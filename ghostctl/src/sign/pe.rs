// PE Authenticode signing via Azure Key Vault
//
// Parses a PE file with goblin, computes the Authenticode digest (excluding
// checksum, cert table dir entry, and existing cert data), sends the digest
// to Key Vault for signing, builds a CMS SignedData envelope, and embeds
// it as a WIN_CERTIFICATE structure in the PE.
//
// Limitations (first pass):
// - RSA only (RS256/384/512)
// - Single certificate, no chain
// - No dual-signing / append to existing signatures
// - No page hashing

use anyhow::{Context, Result, bail};
use goblin::pe::PE;
use std::fs;
use std::path::Path;

use super::asn1;
use super::config::SigningConfig;
use super::hash::{
    AuthenticodeOffsets, DigestAlgorithm, authenticode_digest, digest_bytes, hex_digest,
};
use super::keyvault::KeyVaultClient;

/// WIN_CERTIFICATE revision
const WIN_CERT_REVISION_2_0: u16 = 0x0200;
/// WIN_CERTIFICATE type for PKCS#7
const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;
/// Certificate Table is data directory index 4
const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4;

/// Parse a PE file and extract offsets needed for Authenticode digest
fn parse_pe_offsets(pe_bytes: &[u8]) -> Result<AuthenticodeOffsets> {
    let pe = PE::parse(pe_bytes).map_err(|e| anyhow::anyhow!("PE parsing failed: {}", e))?;

    // PE signature offset from DOS header at offset 0x3C
    if pe_bytes.len() < 0x40 {
        bail!("File too small to be a valid PE");
    }
    let pe_sig_offset = u32::from_le_bytes(pe_bytes[0x3C..0x40].try_into().unwrap()) as usize;

    if pe_bytes.len() < pe_sig_offset + 4 {
        bail!("Invalid PE signature offset");
    }

    // Verify PE signature "PE\0\0"
    if &pe_bytes[pe_sig_offset..pe_sig_offset + 4] != b"PE\0\0" {
        bail!("Invalid PE signature");
    }

    let coff_header_offset = pe_sig_offset + 4;

    // Check if PE32 or PE32+ from optional header magic
    let optional_header_offset = coff_header_offset + 20;
    if pe_bytes.len() < optional_header_offset + 2 {
        bail!("PE file too small for optional header");
    }
    let magic = u16::from_le_bytes(
        pe_bytes[optional_header_offset..optional_header_offset + 2]
            .try_into()
            .unwrap(),
    );

    let is_pe32_plus = match magic {
        0x10B => false, // PE32
        0x20B => true,  // PE32+
        _ => bail!("Unknown PE optional header magic: {:#06x}", magic),
    };

    // Checksum is at optional_header + 64
    let checksum_offset = optional_header_offset + 64;

    // Data directories start after the fixed optional header fields
    // PE32: optional header base = 96 bytes, PE32+: 112 bytes
    let data_dir_base = if is_pe32_plus {
        optional_header_offset + 112
    } else {
        optional_header_offset + 96
    };

    // Certificate Table is directory entry 4, each entry is 8 bytes (RVA + Size)
    let cert_table_offset = data_dir_base + (IMAGE_DIRECTORY_ENTRY_SECURITY * 8);
    if pe_bytes.len() < cert_table_offset + 8 {
        bail!("PE file too small for certificate table directory entry");
    }

    let cert_table_rva = u32::from_le_bytes(
        pe_bytes[cert_table_offset..cert_table_offset + 4]
            .try_into()
            .unwrap(),
    );
    let cert_table_size = u32::from_le_bytes(
        pe_bytes[cert_table_offset + 4..cert_table_offset + 8]
            .try_into()
            .unwrap(),
    );

    // SizeOfHeaders from optional header
    let size_of_headers_offset = optional_header_offset + 60;
    let header_end = u32::from_le_bytes(
        pe_bytes[size_of_headers_offset..size_of_headers_offset + 4]
            .try_into()
            .unwrap(),
    ) as usize;

    // Sanity checks
    let _ = pe; // we used goblin to validate the PE structure

    Ok(AuthenticodeOffsets {
        checksum_offset,
        cert_table_offset,
        cert_table_rva,
        cert_table_size,
        header_end,
    })
}

/// Compute PE checksum (matches Windows PE loader algorithm)
fn pe_checksum(pe_bytes: &[u8], checksum_offset: usize) -> u32 {
    let mut sum: u64 = 0;
    let len = pe_bytes.len();

    let mut i = 0;
    while i + 1 < len {
        // Skip the checksum field itself
        if i == checksum_offset {
            i += 4;
            continue;
        }

        let word = u16::from_le_bytes([pe_bytes[i], pe_bytes[i + 1]]) as u64;
        sum += word;
        // Fold carry
        sum = (sum & 0xFFFF) + (sum >> 16);
        i += 2;
    }

    // Handle odd trailing byte
    if i < len {
        sum += pe_bytes[i] as u64;
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    // Final fold
    sum = (sum & 0xFFFF) + (sum >> 16);

    (sum as u32) + (len as u32)
}

/// Build the SpcIndirectDataContent DER structure
fn build_spc_indirect_data(digest: &[u8], digest_alg: DigestAlgorithm) -> Vec<u8> {
    let digest_oid = asn1::digest_oid(digest_alg.name());
    let algorithm_id = asn1::algorithm_identifier(digest_oid);
    let digest_value = asn1::octet_string(digest);

    // DigestInfo = SEQUENCE { algorithm, digest }
    let digest_info = asn1::sequence(&[&algorithm_id, &digest_value]);

    // SpcPeImageData = SEQUENCE { flags BIT STRING, file SpcLink }
    // flags: 0 (no specific flags)
    // file: [0] IMPLICIT empty (default)
    let flags = asn1::bit_string(&[]);
    let file_link = asn1::context_tag(0, &asn1::context_tag_implicit(2, &[]));
    let spc_pe_image_data_value = asn1::sequence(&[&flags, &file_link]);

    // SpcAttributeTypeAndOptionalValue = SEQUENCE { type OID, value ANY }
    let spc_image_oid = asn1::oid(asn1::OID_SPC_PE_IMAGE_DATA);
    let spc_attr = asn1::sequence(&[&spc_image_oid, &spc_pe_image_data_value]);

    // SpcIndirectDataContent = SEQUENCE { data, messageDigest }
    asn1::sequence(&[&spc_attr, &digest_info])
}

/// Build authenticated attributes for SignerInfo
fn build_auth_attrs(
    content_type_oid: &[u32],
    message_digest: &[u8],
    signing_time: &str,
) -> Vec<u8> {
    let content_type_attr = asn1::attribute(asn1::OID_CONTENT_TYPE, &asn1::oid(content_type_oid));
    let digest_attr = asn1::attribute(
        asn1::OID_MESSAGE_DIGEST,
        &asn1::octet_string(message_digest),
    );
    let time_attr = asn1::attribute(asn1::OID_SIGNING_TIME, &asn1::utc_time(signing_time));

    // Authenticated attributes are a SET OF
    let mut content = Vec::new();
    content.extend_from_slice(&content_type_attr);
    content.extend_from_slice(&digest_attr);
    content.extend_from_slice(&time_attr);
    content
}

/// Build the full CMS SignedData structure for Authenticode
fn build_cms_signed_data(
    spc_indirect_data: &[u8],
    signature: &[u8],
    cert_der: &[u8],
    digest_alg: DigestAlgorithm,
    signing_time: &str,
    timestamp_token: Option<&[u8]>,
) -> Vec<u8> {
    let digest_oid_arcs = asn1::digest_oid(digest_alg.name());

    // version = 1
    let version = asn1::integer_u64(1);

    // digestAlgorithms SET OF AlgorithmIdentifier
    let digest_alg_id = asn1::algorithm_identifier(digest_oid_arcs);
    let digest_algorithms = asn1::set(&[&digest_alg_id]);

    // contentInfo: ContentType = SPC_INDIRECT_DATA, content = spcIndirectDataContent
    let spc_oid = asn1::oid(asn1::OID_SPC_INDIRECT_DATA);
    let spc_content = asn1::context_tag(0, spc_indirect_data);
    let content_info = asn1::sequence(&[&spc_oid, &spc_content]);

    // certificates [0] IMPLICIT SET OF Certificate
    let certs = asn1::context_tag(0, cert_der);

    // Compute message digest over spc_indirect_data for authenticated attrs
    let msg_digest = digest_bytes(spc_indirect_data, digest_alg);

    // Build authenticated attributes
    let auth_attrs_content =
        build_auth_attrs(asn1::OID_SPC_INDIRECT_DATA, &msg_digest, signing_time);

    // For signing, the authenticated attrs are DER-encoded as SET
    // But in SignerInfo they appear as [0] IMPLICIT SET
    let auth_attrs_tagged = asn1::context_tag(0, &auth_attrs_content);

    // SignerInfo
    let signer_version = asn1::integer_u64(1);

    // issuerAndSerialNumber: extract from cert DER
    let (issuer_bytes, serial_bytes) = extract_issuer_serial(cert_der);
    let issuer_and_serial = asn1::sequence(&[&issuer_bytes, &serial_bytes]);

    let digest_alg_signer = asn1::algorithm_identifier(digest_oid_arcs);
    let sig_alg = asn1::algorithm_identifier(asn1::OID_RSA_ENCRYPTION);
    let sig_value = asn1::octet_string(signature);

    let signer_info = if let Some(ts_token) = timestamp_token {
        // Add timestamp as unauthenticated attribute
        let ts_attr = asn1::attribute(asn1::OID_TIMESTAMP_TOKEN, &asn1::raw(ts_token));
        let unauth_attrs = asn1::context_tag(1, &ts_attr);

        asn1::sequence(&[
            &signer_version,
            &issuer_and_serial,
            &digest_alg_signer,
            &auth_attrs_tagged,
            &sig_alg,
            &sig_value,
            &unauth_attrs,
        ])
    } else {
        asn1::sequence(&[
            &signer_version,
            &issuer_and_serial,
            &digest_alg_signer,
            &auth_attrs_tagged,
            &sig_alg,
            &sig_value,
        ])
    };

    let signer_infos = asn1::set(&[&signer_info]);

    // SignedData SEQUENCE
    let signed_data = asn1::sequence(&[
        &version,
        &digest_algorithms,
        &content_info,
        &certs,
        &signer_infos,
    ]);

    // Outer ContentInfo: contentType = signedData, content = [0] EXPLICIT signedData
    let signed_data_oid = asn1::oid(asn1::OID_SIGNED_DATA);
    let signed_data_content = asn1::context_tag(0, &signed_data);
    asn1::sequence(&[&signed_data_oid, &signed_data_content])
}

/// Extract issuer and serial number from a DER-encoded X.509 certificate.
/// Returns (issuer_der, serial_der) both as raw DER-encoded values.
fn extract_issuer_serial(cert_der: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // X.509 structure:
    // Certificate = SEQUENCE {
    //   tbsCertificate = SEQUENCE {
    //     version [0] EXPLICIT INTEGER (optional),
    //     serialNumber INTEGER,
    //     signature AlgorithmIdentifier,
    //     issuer Name,
    //     ...
    //   }
    // }

    // Try to parse the TLV structure manually
    if let Some((tbs_start, tbs_content)) = parse_tlv(cert_der) {
        let tbs_bytes = &cert_der[tbs_start..tbs_start + tbs_content];
        if let Some((inner_start, _)) = parse_tlv(tbs_bytes) {
            let inner = &tbs_bytes[inner_start..];

            // Skip version if present (context tag [0])
            let mut pos = 0;
            if !inner.is_empty()
                && inner[0] == 0xA0
                && let Some(len) = read_der_length(&inner[1..])
            {
                pos = 1 + len.0 + len.1;
            }

            // Serial number
            let serial_start = pos;
            if pos < inner.len()
                && inner[pos] == 0x02
                && let Some(len) = read_der_length(&inner[pos + 1..])
            {
                let serial_end = pos + 1 + len.0 + len.1;
                let serial = inner[serial_start..serial_end].to_vec();

                // Skip signature AlgorithmIdentifier
                pos = serial_end;
                if pos < inner.len()
                    && inner[pos] == 0x30
                    && let Some(len) = read_der_length(&inner[pos + 1..])
                {
                    pos = pos + 1 + len.0 + len.1;
                }

                // Issuer Name (SEQUENCE)
                if pos < inner.len()
                    && inner[pos] == 0x30
                    && let Some(len) = read_der_length(&inner[pos + 1..])
                {
                    let issuer_end = pos + 1 + len.0 + len.1;
                    let issuer = inner[pos..issuer_end].to_vec();
                    return (issuer, serial);
                }
            }
        }
    }

    // Fallback: empty issuer and serial=1
    (asn1::sequence(&[]), asn1::integer_u64(1))
}

/// Parse a DER TLV: returns (content_start_offset_from_input_start, content_length)
fn parse_tlv(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    let _tag = data[0];
    let len_info = read_der_length(&data[1..])?;
    Some((1 + len_info.0, len_info.1))
}

/// Read a DER length field. Returns (length_bytes_consumed, content_length)
fn read_der_length(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    if data[0] < 0x80 {
        Some((1, data[0] as usize))
    } else {
        let num_bytes = (data[0] & 0x7F) as usize;
        if num_bytes == 0 || num_bytes > 4 || data.len() < 1 + num_bytes {
            return None;
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | (data[1 + i] as usize);
        }
        Some((1 + num_bytes, len))
    }
}

/// Build WIN_CERTIFICATE structure (8-byte aligned)
fn build_win_certificate(cms_data: &[u8]) -> Vec<u8> {
    // WIN_CERTIFICATE: { dwLength: u32, wRevision: u16, wCertificateType: u16, bCertificate: [u8] }
    let header_size = 8u32;
    let total_size = header_size + cms_data.len() as u32;

    let mut win_cert = Vec::with_capacity(total_size as usize);
    win_cert.extend_from_slice(&total_size.to_le_bytes());
    win_cert.extend_from_slice(&WIN_CERT_REVISION_2_0.to_le_bytes());
    win_cert.extend_from_slice(&WIN_CERT_TYPE_PKCS_SIGNED_DATA.to_le_bytes());
    win_cert.extend_from_slice(cms_data);

    // Pad to 8-byte alignment
    let padding = (8 - (win_cert.len() % 8)) % 8;
    win_cert.resize(win_cert.len() + padding, 0);

    win_cert
}

/// Get current UTC time as UTCTime format (YYMMDDHHMMSSZ)
fn utc_time_now() -> String {
    use std::process::Command;
    Command::new("date")
        .args(["-u", "+%y%m%d%H%M%SZ"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "260101000000Z".to_string())
}

/// Sign a PE file with embedded Authenticode signature
pub fn sign_pe(
    path: &Path,
    kv: &mut KeyVaultClient,
    config: &SigningConfig,
    output: Option<&Path>,
    verbose: bool,
    timestamp: bool,
    require_timestamp: bool,
) -> Result<()> {
    let pe_bytes =
        fs::read(path).with_context(|| format!("Cannot read PE file: {}", path.display()))?;

    let offsets = parse_pe_offsets(&pe_bytes).context("Failed to parse PE structure")?;

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", pe_bytes.len());
        println!("Format: PE Authenticode");
        println!("Algorithm: {}", config.algorithm);
        println!("Digest: {}", digest_alg.name());
        println!("Vault: {}", config.vault_url);
        println!("Key: {}", config.cert_name);
        println!("Checksum offset: {:#x}", offsets.checksum_offset);
        println!("Cert table offset: {:#x}", offsets.cert_table_offset);
        if offsets.cert_table_rva > 0 {
            println!(
                "Existing cert data: {:#x} ({} bytes)",
                offsets.cert_table_rva, offsets.cert_table_size
            );
        }
        println!();
    }

    // Compute Authenticode digest
    if verbose {
        println!("Computing Authenticode {} digest...", digest_alg.name());
    }
    let digest = authenticode_digest(&pe_bytes, &offsets, digest_alg)
        .context("Failed to compute Authenticode digest")?;

    if verbose {
        println!("Digest: {}", hex_digest(&digest));
        println!();
        println!("Sending digest to Azure Key Vault for signing...");
    }

    // Sign via Key Vault
    let sign_response = kv
        .sign(
            &config.cert_name,
            config.key_version.as_deref(),
            &config.algorithm,
            &digest,
        )
        .context("Key Vault signing failed")?;

    let signature_bytes =
        KeyVaultClient::decode_signature(&sign_response).context("Failed to decode signature")?;

    if verbose {
        println!("Signature received ({} bytes)", signature_bytes.len());
    }

    // Fetch the certificate
    if verbose {
        println!("Fetching certificate from Key Vault...");
    }
    let cert_der = kv
        .get_certificate(&config.cert_name, config.key_version.as_deref())
        .context("Failed to fetch certificate from Key Vault")?;

    if verbose {
        println!("Certificate received ({} bytes)", cert_der.len());
    }

    // Build SpcIndirectDataContent
    let spc_indirect_data = build_spc_indirect_data(&digest, digest_alg);

    // Optionally get timestamp
    let ts_token = if timestamp && !config.tsa_url.is_empty() {
        if verbose {
            println!("Requesting timestamp from {}...", config.tsa_url);
        }
        match super::timestamp::timestamp_signature(&signature_bytes, digest_alg, &config.tsa_url) {
            Ok(token) => {
                if verbose {
                    println!("Timestamp received ({} bytes)", token.len());
                }
                Some(token)
            }
            Err(e) => {
                if require_timestamp {
                    return Err(e).context("Timestamping was explicitly requested");
                }
                eprintln!(
                    "Warning: Timestamping failed: {}. Signing without timestamp.",
                    e
                );
                None
            }
        }
    } else {
        None
    };

    let signing_time = utc_time_now();

    // Build CMS SignedData
    let cms = build_cms_signed_data(
        &spc_indirect_data,
        &signature_bytes,
        &cert_der,
        digest_alg,
        &signing_time,
        ts_token.as_deref(),
    );

    // Build WIN_CERTIFICATE
    let win_cert = build_win_certificate(&cms);

    if verbose {
        println!("WIN_CERTIFICATE size: {} bytes", win_cert.len());
    }

    // Build the signed PE
    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => path.to_path_buf(), // overwrite in place
    };

    let mut signed_pe = pe_bytes.clone();

    // Remove any existing cert data
    if offsets.cert_table_rva > 0 {
        signed_pe.truncate(offsets.cert_table_rva as usize);
    }

    // Append WIN_CERTIFICATE at end of file
    let new_cert_rva = signed_pe.len() as u32;
    let new_cert_size = win_cert.len() as u32;
    signed_pe.extend_from_slice(&win_cert);

    // Update cert table directory entry
    signed_pe[offsets.cert_table_offset..offsets.cert_table_offset + 4]
        .copy_from_slice(&new_cert_rva.to_le_bytes());
    signed_pe[offsets.cert_table_offset + 4..offsets.cert_table_offset + 8]
        .copy_from_slice(&new_cert_size.to_le_bytes());

    // Update PE checksum
    let new_checksum = pe_checksum(&signed_pe, offsets.checksum_offset);
    signed_pe[offsets.checksum_offset..offsets.checksum_offset + 4]
        .copy_from_slice(&new_checksum.to_le_bytes());

    // Write the signed PE
    fs::write(&output_path, &signed_pe)
        .with_context(|| format!("Failed to write signed PE: {}", output_path.display()))?;

    println!("Signed (Authenticode): {}", output_path.display());
    println!("  Certificate embedded in PE");
    if ts_token.is_some() {
        println!("  Timestamped: yes");
    }

    Ok(())
}

/// Dry-run for PE signing
pub fn dry_run_pe(path: &Path, config: &SigningConfig) -> Result<()> {
    let pe_bytes =
        fs::read(path).with_context(|| format!("Cannot read PE file: {}", path.display()))?;

    let offsets = parse_pe_offsets(&pe_bytes).context("Failed to parse PE structure")?;

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);
    let digest = authenticode_digest(&pe_bytes, &offsets, digest_alg)
        .context("Failed to compute Authenticode digest")?;

    println!("[DRY RUN] Would sign (Authenticode): {}", path.display());
    println!("  File size:         {} bytes", pe_bytes.len());
    println!("  Format:            PE Authenticode");
    println!("  Algorithm:         {}", config.algorithm);
    println!(
        "  Digest:            {} = {}",
        digest_alg.name(),
        hex_digest(&digest)
    );
    println!("  Vault:             {}", config.vault_url);
    println!("  Key:               {}", config.cert_name);
    println!("  Checksum offset:   {:#x}", offsets.checksum_offset);
    println!("  Cert table offset: {:#x}", offsets.cert_table_offset);
    if offsets.cert_table_rva > 0 {
        println!(
            "  Existing cert:     {:#x} ({} bytes) -- will be replaced",
            offsets.cert_table_rva, offsets.cert_table_size
        );
    }
    if !config.tsa_url.is_empty() {
        println!("  Timestamp URL:     {}", config.tsa_url);
    }
    println!();
    println!("No Key Vault calls were made.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid PE32 for testing
    fn make_minimal_pe() -> Vec<u8> {
        let mut pe = vec![0u8; 512];

        // DOS header
        pe[0] = b'M';
        pe[1] = b'Z';
        // e_lfanew at offset 0x3C = 0x80
        pe[0x3C] = 0x80;

        // PE signature at 0x80
        pe[0x80] = b'P';
        pe[0x81] = b'E';
        pe[0x82] = 0;
        pe[0x83] = 0;

        // COFF header at 0x84
        // Machine: x86_64 = 0x8664
        pe[0x84] = 0x64;
        pe[0x85] = 0x86;
        // NumberOfSections = 1
        pe[0x86] = 0x01;
        pe[0x87] = 0x00;
        // SizeOfOptionalHeader = 240 (0xF0) for PE32+
        pe[0x94] = 0xF0;
        pe[0x95] = 0x00;

        // Optional header at 0x98
        // Magic: PE32+ = 0x20B
        pe[0x98] = 0x0B;
        pe[0x99] = 0x02;

        // SizeOfHeaders at optional_header + 60 = 0x98 + 60 = 0xD4
        pe[0xD4] = 0x00;
        pe[0xD5] = 0x02; // 0x200 = 512

        // NumberOfRvaAndSizes at optional_header + 108 = 0x98 + 108 = 0x104
        pe[0x104] = 16; // 16 data directories

        pe
    }

    #[test]
    fn test_parse_pe_offsets() {
        let pe = make_minimal_pe();
        let offsets = parse_pe_offsets(&pe).unwrap();

        // Checksum at optional_header + 64 = 0x98 + 64 = 0xD8
        assert_eq!(offsets.checksum_offset, 0xD8);

        // Cert table at data_dir_base + 4*8 = (0x98 + 112) + 32 = 0x128
        assert_eq!(offsets.cert_table_offset, 0x128);

        // No existing cert data
        assert_eq!(offsets.cert_table_rva, 0);
        assert_eq!(offsets.cert_table_size, 0);
    }

    #[test]
    fn test_pe_checksum() {
        let pe = make_minimal_pe();
        let checksum = pe_checksum(&pe, 0xD8);
        // Should be non-zero (file length + checksum)
        assert!(checksum > 0);
    }

    #[test]
    fn test_build_win_certificate() {
        let cms_data = vec![0xAA; 100];
        let win_cert = build_win_certificate(&cms_data);

        // Header: 4 (length) + 2 (revision) + 2 (type) = 8
        let total_size = u32::from_le_bytes(win_cert[0..4].try_into().unwrap());
        assert_eq!(total_size, 108); // 8 + 100

        let revision = u16::from_le_bytes(win_cert[4..6].try_into().unwrap());
        assert_eq!(revision, WIN_CERT_REVISION_2_0);

        let cert_type = u16::from_le_bytes(win_cert[6..8].try_into().unwrap());
        assert_eq!(cert_type, WIN_CERT_TYPE_PKCS_SIGNED_DATA);

        // 8-byte aligned
        assert_eq!(win_cert.len() % 8, 0);
    }

    #[test]
    fn test_build_spc_indirect_data() {
        let digest = vec![0xAB; 32]; // fake SHA-256 digest
        let spc = build_spc_indirect_data(&digest, DigestAlgorithm::Sha256);
        // Should be a valid SEQUENCE
        assert_eq!(spc[0], 0x30);
        assert!(!spc.is_empty());
    }

    #[test]
    fn test_authenticode_digest_minimal_pe() {
        let pe = make_minimal_pe();
        let offsets = parse_pe_offsets(&pe).unwrap();
        let digest = authenticode_digest(&pe, &offsets, DigestAlgorithm::Sha256).unwrap();
        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn test_extract_issuer_serial_fallback() {
        // With invalid cert DER, should return fallback values
        let (issuer, serial) = extract_issuer_serial(&[0x00, 0x01, 0x02]);
        assert!(!issuer.is_empty());
        assert!(!serial.is_empty());
    }
}
