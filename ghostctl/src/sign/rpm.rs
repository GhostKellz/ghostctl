// RPM package signing via Azure Key Vault
//
// Two modes:
// 1. Detached: produces .sig + .sig.json files (default)
// 2. Native: embeds OpenPGP signature in RPM signature header (--native)
//    Verifiable with `rpm -K` after importing the public key

use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

use super::config::{SigningConfig, pgp_key_created_at};
use super::hash::{DigestAlgorithm, file_digest, hex_digest};
use super::keyvault::KeyVaultClient;
use super::pgp;

/// RPM lead magic bytes
const RPM_MAGIC: [u8; 4] = [0xED, 0xAB, 0xEE, 0xDB];

/// RPM metadata extracted from the package
#[derive(Debug, Default)]
struct RpmInfo {
    name: String,
    arch: String,
}

/// Metadata written alongside an RPM detached signature
#[derive(Serialize)]
struct RpmSignatureMetadata {
    version: u32,
    format: &'static str,
    algorithm: String,
    key_id: String,
    digest_algorithm: String,
    file_digest: String,
    file_name: String,
    file_size: u64,
    signed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpm_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpm_arch: Option<String>,
}

/// Extract basic RPM info from the lead (first 96 bytes)
fn extract_rpm_info(path: &Path) -> Option<RpmInfo> {
    let mut file = fs::File::open(path).ok()?;
    let mut lead = [0u8; 96];
    file.read_exact(&mut lead).ok()?;

    // Verify magic
    if lead[0..4] != RPM_MAGIC {
        return None;
    }

    // Name is at bytes 10..76 (66 bytes, null-terminated)
    let name_bytes = &lead[10..76];
    let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(66);
    let name = String::from_utf8_lossy(&name_bytes[..name_end]).to_string();

    // Architecture is encoded in arch_num at bytes 8..10 (big-endian u16)
    let arch_num = u16::from_be_bytes([lead[8], lead[9]]);
    let arch = match arch_num {
        1 => "i386".to_string(),
        2 => "alpha".to_string(),
        3 => "sparc".to_string(),
        4 => "mips".to_string(),
        5 => "ppc".to_string(),
        6 => "m68k".to_string(),
        8 => "mipsel".to_string(),
        9 => "arm".to_string(),
        11 => "s390".to_string(),
        12 => "s390x".to_string(),
        13 => "ppc64".to_string(),
        14 => "sh".to_string(),
        15 => "x86_64".to_string(),
        16 => "aarch64".to_string(),
        _ => format!("unknown({})", arch_num),
    };

    Some(RpmInfo { name, arch })
}

/// Fallback: parse RPM info from filename
fn info_from_filename(path: &Path) -> RpmInfo {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Common RPM naming: name-version-release.arch.rpm
    // Try to extract arch from the last dot-separated component before .rpm
    let parts: Vec<&str> = stem.rsplitn(2, '.').collect();
    let arch = if parts.len() == 2 {
        parts[0].to_string()
    } else {
        String::new()
    };

    let name = if parts.len() == 2 {
        parts[1].to_string()
    } else {
        stem.to_string()
    };

    RpmInfo { name, arch }
}

/// Get current UTC time as ISO 8601
fn utc_now() -> String {
    use std::process::Command;
    Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
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
        .unwrap_or_else(|| "unknown".to_string())
}

/// Sign an RPM package with a detached signature
pub fn sign_rpm(
    path: &Path,
    kv: &mut KeyVaultClient,
    config: &SigningConfig,
    output: Option<&Path>,
    verbose: bool,
) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path.display()))?;

    let rpm_info = extract_rpm_info(path).unwrap_or_else(|| info_from_filename(path));

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", file_meta.len());
        println!("Format: RPM Package");
        if !rpm_info.name.is_empty() {
            println!("RPM Name: {}", rpm_info.name);
        }
        if !rpm_info.arch.is_empty() {
            println!("RPM Arch: {}", rpm_info.arch);
        }
        println!("Algorithm: {}", config.algorithm);
        println!("Digest: {}", digest_alg.name());
        println!();
    }

    // Compute digest
    if verbose {
        println!("Computing {} digest...", digest_alg.name());
    }
    let digest = file_digest(path, digest_alg).context("Failed to compute file digest")?;
    let digest_hex = hex_digest(&digest);

    if verbose {
        println!("Digest: {}", digest_hex);
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
    let key_id = KeyVaultClient::key_id(&sign_response).to_string();

    if verbose {
        println!("Signature received ({} bytes)", signature_bytes.len());
    }

    // Determine output paths
    let sig_path = match output {
        Some(p) => p.to_path_buf(),
        None => path.with_extension("rpm.sig"),
    };
    let meta_path = sig_path.with_extension("sig.json");

    // Write signature file
    fs::write(&sig_path, &signature_bytes)
        .with_context(|| format!("Failed to write signature: {}", sig_path.display()))?;

    // Write metadata
    let metadata = RpmSignatureMetadata {
        version: 1,
        format: "rpm",
        algorithm: config.algorithm.clone(),
        key_id,
        digest_algorithm: digest_alg.name().to_string(),
        file_digest: digest_hex,
        file_name,
        file_size: file_meta.len(),
        signed_at: utc_now(),
        rpm_name: if rpm_info.name.is_empty() {
            None
        } else {
            Some(rpm_info.name)
        },
        rpm_arch: if rpm_info.arch.is_empty() {
            None
        } else {
            Some(rpm_info.arch)
        },
    };

    let meta_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize signature metadata")?;
    fs::write(&meta_path, &meta_json)
        .with_context(|| format!("Failed to write metadata: {}", meta_path.display()))?;

    println!("Signed (RPM detached): {}", path.display());
    println!("  Signature: {}", sig_path.display());
    println!("  Metadata:  {}", meta_path.display());

    Ok(())
}

/// Dry-run for RPM signing
pub fn dry_run_rpm(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path.display()))?;

    let rpm_info = extract_rpm_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);
    let digest = file_digest(path, digest_alg).context("Failed to compute file digest")?;
    let sig_path = path.with_extension("rpm.sig");

    println!("[DRY RUN] Would sign (RPM detached): {}", path.display());
    println!("  File size:  {} bytes", file_meta.len());
    println!("  Format:     RPM Package");
    if !rpm_info.name.is_empty() {
        println!("  RPM Name:   {}", rpm_info.name);
    }
    if !rpm_info.arch.is_empty() {
        println!("  RPM Arch:   {}", rpm_info.arch);
    }
    println!("  Algorithm:  {}", config.algorithm);
    println!(
        "  Digest:     {} = {}",
        digest_alg.name(),
        hex_digest(&digest)
    );
    println!("  Vault:      {}", config.vault_url);
    println!("  Key:        {}", config.cert_name);
    println!("  Output:     {}", sig_path.display());
    println!();
    println!("No Key Vault calls were made.");

    Ok(())
}

// --- Native RPM signing ---

/// RPM header magic: 0x8EADE801
const RPM_HEADER_MAGIC: [u8; 4] = [0x8E, 0xAD, 0xE8, 0x01];

/// RPMSIGTAG_RSA (tag 268) - header-only RSA signature
const RPMSIGTAG_RSA: u32 = 268;

/// RPM header tag data type: BIN (7)
const RPM_BIN_TYPE: u32 = 7;

/// Parsed RPM layout
struct RpmLayout {
    lead: Vec<u8>,                    // 96-byte lead
    sig_header_raw: Vec<u8>,          // original signature header (magic + header + padding)
    main_header_and_payload: Vec<u8>, // everything after sig section
    main_header_start: usize,         // offset of main header within the full RPM
}

/// Parse an RPM file into its structural sections.
fn parse_rpm_layout(data: &[u8]) -> Result<RpmLayout> {
    if data.len() < 96 {
        anyhow::bail!("RPM too small: {} bytes", data.len());
    }

    // Lead: first 96 bytes
    if data[0..4] != RPM_MAGIC {
        anyhow::bail!("Not an RPM file (bad magic)");
    }
    let lead = data[0..96].to_vec();

    // Signature header starts at byte 96
    let mut pos = 96;

    // Parse signature header
    if pos + 16 > data.len() {
        anyhow::bail!("RPM truncated: no signature header");
    }
    if data[pos..pos + 4] != RPM_HEADER_MAGIC {
        anyhow::bail!("Bad RPM signature header magic");
    }

    let _sig_version =
        u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);
    let sig_nindex =
        u32::from_be_bytes([data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11]]);
    let sig_hsize = u32::from_be_bytes([
        data[pos + 12],
        data[pos + 13],
        data[pos + 14],
        data[pos + 15],
    ]);

    let sig_header_len = 16 + (sig_nindex as usize * 16) + sig_hsize as usize;
    if pos + sig_header_len > data.len() {
        anyhow::bail!("RPM truncated: signature header extends beyond file");
    }

    // Signature section is 8-byte aligned
    let sig_total = sig_header_len + ((8 - (sig_header_len % 8)) % 8);
    let sig_header_raw = data[pos..pos + sig_total].to_vec();

    pos += sig_total;
    let main_header_start = pos;

    let main_header_and_payload = data[pos..].to_vec();

    Ok(RpmLayout {
        lead,
        sig_header_raw,
        main_header_and_payload,
        main_header_start,
    })
}

/// An entry in an RPM header index
#[derive(Clone)]
struct RpmIndexEntry {
    tag: u32,
    data_type: u32,
    offset: u32,
    count: u32,
}

/// Parse RPM header index entries and data store.
fn parse_rpm_header(header_data: &[u8]) -> Result<(Vec<RpmIndexEntry>, Vec<u8>)> {
    if header_data.len() < 16 || header_data[0..4] != RPM_HEADER_MAGIC {
        anyhow::bail!("Bad RPM header magic");
    }

    let nindex = u32::from_be_bytes([
        header_data[8],
        header_data[9],
        header_data[10],
        header_data[11],
    ]) as usize;
    let hsize = u32::from_be_bytes([
        header_data[12],
        header_data[13],
        header_data[14],
        header_data[15],
    ]) as usize;

    let index_start = 16;
    let index_end = index_start + nindex * 16;
    let store_end = index_end + hsize;

    if store_end > header_data.len() {
        anyhow::bail!("RPM header truncated");
    }

    let mut entries = Vec::with_capacity(nindex);
    for i in 0..nindex {
        let off = index_start + i * 16;
        entries.push(RpmIndexEntry {
            tag: u32::from_be_bytes([
                header_data[off],
                header_data[off + 1],
                header_data[off + 2],
                header_data[off + 3],
            ]),
            data_type: u32::from_be_bytes([
                header_data[off + 4],
                header_data[off + 5],
                header_data[off + 6],
                header_data[off + 7],
            ]),
            offset: u32::from_be_bytes([
                header_data[off + 8],
                header_data[off + 9],
                header_data[off + 10],
                header_data[off + 11],
            ]),
            count: u32::from_be_bytes([
                header_data[off + 12],
                header_data[off + 13],
                header_data[off + 14],
                header_data[off + 15],
            ]),
        });
    }

    let store = header_data[index_end..store_end].to_vec();
    Ok((entries, store))
}

/// Build a new signature header with the PGP signature injected.
///
/// Removes any existing RPMSIGTAG_RSA entry and adds the new one.
fn build_signature_header(entries: &[RpmIndexEntry], store: &[u8], pgp_sig: &[u8]) -> Vec<u8> {
    // Filter out existing RSA sig entry
    let mut new_entries: Vec<RpmIndexEntry> = entries
        .iter()
        .filter(|e| e.tag != RPMSIGTAG_RSA)
        .cloned()
        .collect();

    // Build new data store: original data + pgp signature
    let mut new_store = store.to_vec();
    let sig_offset = new_store.len() as u32;
    new_store.extend_from_slice(pgp_sig);

    // Add new RSA sig entry
    new_entries.push(RpmIndexEntry {
        tag: RPMSIGTAG_RSA,
        data_type: RPM_BIN_TYPE,
        offset: sig_offset,
        count: pgp_sig.len() as u32,
    });

    // Sort entries by tag (RPM requirement)
    new_entries.sort_by_key(|e| e.tag);

    // Build header
    let nindex = new_entries.len() as u32;
    let hsize = new_store.len() as u32;

    let mut header = Vec::new();
    // Magic + version (reserved 4 bytes)
    header.extend_from_slice(&RPM_HEADER_MAGIC);
    header.extend_from_slice(&[0u8; 4]); // reserved
    header.extend_from_slice(&nindex.to_be_bytes());
    header.extend_from_slice(&hsize.to_be_bytes());

    // Index entries
    for entry in &new_entries {
        header.extend_from_slice(&entry.tag.to_be_bytes());
        header.extend_from_slice(&entry.data_type.to_be_bytes());
        header.extend_from_slice(&entry.offset.to_be_bytes());
        header.extend_from_slice(&entry.count.to_be_bytes());
    }

    // Data store
    header.extend_from_slice(&new_store);

    // 8-byte alignment padding
    let pad = (8 - (header.len() % 8)) % 8;
    header.extend(std::iter::repeat_n(0u8, pad));

    header
}

/// Sign an RPM package with a native OpenPGP signature embedded in the header.
pub fn sign_rpm_native(
    path: &Path,
    kv: &mut KeyVaultClient,
    config: &SigningConfig,
    output: Option<&Path>,
    verbose: bool,
) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let data = fs::read(path).with_context(|| format!("Cannot read file: {}", path.display()))?;

    let layout = parse_rpm_layout(&data)
        .with_context(|| format!("Failed to parse RPM: {}", path.display()))?;

    let rpm_info = extract_rpm_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", data.len());
        println!("Format: RPM Package (native signing)");
        if !rpm_info.name.is_empty() {
            println!("RPM Name: {}", rpm_info.name);
        }
        if !rpm_info.arch.is_empty() {
            println!("RPM Arch: {}", rpm_info.arch);
        }
        println!("Algorithm: {}", config.algorithm);
        println!("Digest: {}", digest_alg.name());
        println!();
    }

    // Fetch certificate from KV to extract RSA public key
    if verbose {
        println!("Fetching certificate from Key Vault...");
    }
    let cert_der = kv
        .get_certificate(&config.cert_name, config.key_version.as_deref())
        .context("Failed to fetch certificate from Key Vault")?;

    let rsa_key = pgp::extract_rsa_pubkey(&cert_der)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract RSA public key from certificate"))?;

    let creation_time = pgp_key_created_at(config);
    let identity = pgp::compute_key_identity(&rsa_key, creation_time);

    if verbose {
        println!("Key fingerprint: {}", pgp::hex(&identity.fingerprint));
        println!("Key ID: {}", pgp::hex(&identity.key_id));
        println!();
    }

    let ctx = pgp::PgpSignatureContext {
        key: rsa_key,
        identity,
        hash_algorithm: digest_alg,
        creation_time,
    };

    // For RPM, the signature covers the main header only (header-only signature)
    if verbose {
        println!("Computing PGP-contextualized hash over main header...");
    }
    let (digest, hash_prefix) = pgp::pgp_hash(&layout.main_header_and_payload, &ctx);

    if verbose {
        println!("Digest: {}", pgp::hex(&digest));
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

    let raw_sig =
        KeyVaultClient::decode_signature(&sign_response).context("Failed to decode signature")?;

    if verbose {
        println!("Signature received ({} bytes)", raw_sig.len());
    }

    // Build OpenPGP signature packet
    let pgp_packet = pgp::build_signature_packet(&ctx, &raw_sig, hash_prefix);

    if verbose {
        println!("OpenPGP signature packet: {} bytes", pgp_packet.len());
    }

    // Parse existing signature header and inject the PGP signature
    let (sig_entries, sig_store) =
        parse_rpm_header(&layout.sig_header_raw).context("Failed to parse RPM signature header")?;

    let new_sig_header = build_signature_header(&sig_entries, &sig_store, &pgp_packet);

    // Reconstruct RPM: lead + new sig header + main header + payload
    let mut output_data = Vec::with_capacity(
        layout.lead.len() + new_sig_header.len() + layout.main_header_and_payload.len(),
    );
    output_data.extend_from_slice(&layout.lead);
    output_data.extend_from_slice(&new_sig_header);
    output_data.extend_from_slice(&layout.main_header_and_payload);

    // Write output
    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => path.to_path_buf(), // overwrite in place
    };
    fs::write(&output_path, &output_data)
        .with_context(|| format!("Failed to write signed RPM: {}", output_path.display()))?;

    println!("Signed (RPM native): {}", output_path.display());
    println!("  Verify with: rpm -K {}", output_path.display());
    println!("  Import key:  rpm --import <(ghostctl sign export-key)");

    Ok(())
}

/// Dry-run for native RPM signing
pub fn dry_run_rpm_native(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let data = fs::read(path).with_context(|| format!("Cannot read file: {}", path.display()))?;

    let layout = parse_rpm_layout(&data)
        .with_context(|| format!("Failed to parse RPM: {}", path.display()))?;

    let rpm_info = extract_rpm_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    let (sig_entries, _) =
        parse_rpm_header(&layout.sig_header_raw).context("Failed to parse RPM signature header")?;

    let has_existing_sig = sig_entries.iter().any(|e| e.tag == RPMSIGTAG_RSA);

    println!("[DRY RUN] Would sign (RPM native): {}", path.display());
    println!("  File size:    {} bytes", data.len());
    println!("  Format:       RPM Package (native OpenPGP)");
    if !rpm_info.name.is_empty() {
        println!("  RPM Name:     {}", rpm_info.name);
    }
    if !rpm_info.arch.is_empty() {
        println!("  RPM Arch:     {}", rpm_info.arch);
    }
    println!("  Algorithm:    {}", config.algorithm);
    println!("  Digest:       {}", digest_alg.name());
    println!("  Sig header:   {} entries", sig_entries.len());
    println!(
        "  Existing sig: {}",
        if has_existing_sig {
            "yes (will be replaced)"
        } else {
            "no"
        }
    );
    println!(
        "  Header data:  {} bytes",
        layout.main_header_and_payload.len()
    );
    println!("  Vault:        {}", config.vault_url);
    println!("  Key:          {}", config.cert_name);
    println!();
    println!("No Key Vault calls were made.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_extract_rpm_info_valid() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rpm");
        let mut f = fs::File::create(&path).unwrap();

        // Write RPM lead
        let mut lead = vec![0u8; 96];
        lead[0..4].copy_from_slice(&RPM_MAGIC);
        // Arch: x86_64 = 15
        lead[8] = 0;
        lead[9] = 15;
        // Name at bytes 10..76
        let name = b"my-package-1.0-1";
        lead[10..10 + name.len()].copy_from_slice(name);
        f.write_all(&lead).unwrap();
        drop(f);

        let info = extract_rpm_info(&path).unwrap();
        assert_eq!(info.name, "my-package-1.0-1");
        assert_eq!(info.arch, "x86_64");
    }

    #[test]
    fn test_extract_rpm_info_invalid_magic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rpm");
        fs::write(&path, b"not an rpm file content here").unwrap();

        assert!(extract_rpm_info(&path).is_none());
    }

    #[test]
    fn test_info_from_filename() {
        let path = Path::new("/tmp/my-package-1.0-1.x86_64.rpm");
        let info = info_from_filename(path);
        assert_eq!(info.arch, "x86_64");
        assert!(info.name.contains("my-package"));
    }

    /// Build a minimal valid RPM file for testing native signing.
    fn build_test_rpm() -> Vec<u8> {
        let mut rpm = Vec::new();

        // Lead (96 bytes)
        let mut lead = vec![0u8; 96];
        lead[0..4].copy_from_slice(&RPM_MAGIC);
        lead[4] = 3; // major version
        lead[5] = 0; // minor version
        // type = binary (0)
        lead[8] = 0; // arch high byte
        lead[9] = 15; // x86_64
        let name = b"test-pkg-1.0-1";
        lead[10..10 + name.len()].copy_from_slice(name);
        rpm.extend_from_slice(&lead);

        // Signature header (empty, just magic + 0 entries + 0 store)
        rpm.extend_from_slice(&RPM_HEADER_MAGIC);
        rpm.extend_from_slice(&[0u8; 4]); // reserved
        rpm.extend_from_slice(&0u32.to_be_bytes()); // nindex = 0
        rpm.extend_from_slice(&0u32.to_be_bytes()); // hsize = 0
        // 8-byte align: header is 16 bytes, already aligned

        // Main header (minimal: magic + 0 entries + some data)
        rpm.extend_from_slice(&RPM_HEADER_MAGIC);
        rpm.extend_from_slice(&[0u8; 4]); // reserved
        rpm.extend_from_slice(&0u32.to_be_bytes()); // nindex = 0
        rpm.extend_from_slice(&4u32.to_be_bytes()); // hsize = 4
        rpm.extend_from_slice(b"test"); // 4 bytes of data store

        // Payload (some bytes)
        rpm.extend_from_slice(b"fake payload data here");

        rpm
    }

    #[test]
    fn test_parse_rpm_layout() {
        let rpm = build_test_rpm();
        let layout = parse_rpm_layout(&rpm).unwrap();
        assert_eq!(layout.lead.len(), 96);
        assert!(!layout.main_header_and_payload.is_empty());
    }

    #[test]
    fn test_parse_rpm_header() {
        let rpm = build_test_rpm();
        let layout = parse_rpm_layout(&rpm).unwrap();
        let (entries, store) = parse_rpm_header(&layout.sig_header_raw).unwrap();
        assert_eq!(entries.len(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn test_build_signature_header_injects_sig() {
        let entries = vec![];
        let store = vec![];
        let fake_pgp_sig = vec![0xAA; 100];

        let header = build_signature_header(&entries, &store, &fake_pgp_sig);

        // Should start with RPM header magic
        assert_eq!(&header[0..4], &RPM_HEADER_MAGIC);

        // nindex should be 1 (the RSA sig entry we added)
        let nindex = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
        assert_eq!(nindex, 1);

        // Verify the tag is RPMSIGTAG_RSA
        let tag = u32::from_be_bytes([header[16], header[17], header[18], header[19]]);
        assert_eq!(tag, RPMSIGTAG_RSA);

        // Should be 8-byte aligned
        assert_eq!(header.len() % 8, 0);
    }

    #[test]
    fn test_build_signature_header_replaces_existing() {
        let entries = vec![
            RpmIndexEntry {
                tag: RPMSIGTAG_RSA,
                data_type: RPM_BIN_TYPE,
                offset: 0,
                count: 50,
            },
            RpmIndexEntry {
                tag: 1000,
                data_type: RPM_BIN_TYPE,
                offset: 50,
                count: 10,
            },
        ];
        let store = vec![0xBB; 60];
        let fake_pgp_sig = vec![0xCC; 100];

        let header = build_signature_header(&entries, &store, &fake_pgp_sig);

        // nindex should be 2 (existing non-RSA + new RSA)
        let nindex = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
        assert_eq!(nindex, 2);
    }

    #[test]
    fn test_dry_run_rpm_native() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rpm");
        let rpm = build_test_rpm();
        fs::write(&path, &rpm).unwrap();

        let config = SigningConfig {
            vault_url: "https://test.vault.azure.net".to_string(),
            cert_name: "test-cert".to_string(),
            algorithm: "RS256".to_string(),
            ..Default::default()
        };

        // Should succeed without KV calls
        let result = dry_run_rpm_native(&path, &config);
        assert!(result.is_ok());
    }
}
