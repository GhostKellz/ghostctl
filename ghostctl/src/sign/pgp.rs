// Minimal OpenPGP v4 packet builder for native RPM/DEB signing
//
// Builds OpenPGP v4 signature packets from raw RSA signatures obtained
// via Azure Key Vault. Follows RFC 4880 for packet formats.
//
// The approach: compute a PGP-contextualized hash locally (data + hashed
// subpackets + v4 trailer), send that hash to KV for PKCS#1 v1.5 signing,
// then wrap the result in a proper OpenPGP signature packet.

use sha1::Sha1;
use sha2::{Digest, Sha256, Sha384, Sha512};

use super::hash::DigestAlgorithm;

// OpenPGP constants (RFC 4880)
const PGP_SIG_BINARY: u8 = 0x00;
const PGP_PUBKEY_RSA: u8 = 1;
const PGP_HASH_SHA256: u8 = 8;
const PGP_HASH_SHA384: u8 = 9;
const PGP_HASH_SHA512: u8 = 10;

// Subpacket types
const SUBPKT_CREATION_TIME: u8 = 2;
const SUBPKT_ISSUER: u8 = 16;

/// RSA public key extracted from an X.509 certificate
#[derive(Debug, Clone)]
pub struct RsaPublicKey {
    pub modulus: Vec<u8>,
    pub exponent: Vec<u8>,
}

/// OpenPGP key identity (fingerprint + key ID)
#[derive(Debug, Clone)]
pub struct PgpKeyIdentity {
    pub fingerprint: [u8; 20],
    pub key_id: [u8; 8],
}

/// Context for building a PGP signature
pub struct PgpSignatureContext {
    pub key: RsaPublicKey,
    pub identity: PgpKeyIdentity,
    pub hash_algorithm: DigestAlgorithm,
    pub creation_time: u32,
}

/// Encode an integer as an OpenPGP MPI (Multi-Precision Integer).
/// Format: 2-byte big-endian bit count, then the integer bytes.
pub fn encode_mpi(value: &[u8]) -> Vec<u8> {
    // Strip leading zeros
    let start = value.iter().position(|&b| b != 0).unwrap_or(value.len());
    let trimmed = if start >= value.len() {
        &[0u8] as &[u8]
    } else {
        &value[start..]
    };

    // Count bits in the first byte
    let first_byte = trimmed[0];
    let bit_count = if first_byte == 0 {
        0u16
    } else {
        let leading = first_byte.leading_zeros() as u16;
        (trimmed.len() as u16) * 8 - leading
    };

    let mut out = Vec::with_capacity(2 + trimmed.len());
    out.extend_from_slice(&bit_count.to_be_bytes());
    out.extend_from_slice(trimmed);
    out
}

/// Frame data as an OpenPGP old-format packet.
/// Old-format is used for compatibility with RPM/DEB tooling.
fn packet_frame(packet_type: u8, body: &[u8]) -> Vec<u8> {
    let len = body.len();
    let tag = 0x80 | ((packet_type & 0x0F) << 2);

    let mut out = Vec::with_capacity(6 + len);
    if len < 256 {
        out.push(tag); // one-byte length
        out.push(len as u8);
    } else if len < 65536 {
        out.push(tag | 0x01); // two-byte length
        out.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        out.push(tag | 0x02); // four-byte length
        out.extend_from_slice(&(len as u32).to_be_bytes());
    }
    out.extend_from_slice(body);
    out
}

/// Map DigestAlgorithm to OpenPGP hash algorithm ID
pub fn pgp_hash_id(alg: DigestAlgorithm) -> u8 {
    match alg {
        DigestAlgorithm::Sha256 => PGP_HASH_SHA256,
        DigestAlgorithm::Sha384 => PGP_HASH_SHA384,
        DigestAlgorithm::Sha512 => PGP_HASH_SHA512,
    }
}

/// Build the hashed area for a v4 signature.
/// Contains signature creation time subpacket.
fn build_hashed_area(ctx: &PgpSignatureContext) -> Vec<u8> {
    // Creation time subpacket: length(5) + type(2) + 4-byte BE timestamp
    let mut subpackets = Vec::new();
    subpackets.push(5); // subpacket length (1 + 4)
    subpackets.push(SUBPKT_CREATION_TIME);
    subpackets.extend_from_slice(&ctx.creation_time.to_be_bytes());
    subpackets
}

/// Build the unhashed area for a v4 signature.
/// Contains issuer key ID subpacket.
fn build_unhashed_area(ctx: &PgpSignatureContext) -> Vec<u8> {
    let mut subpackets = Vec::new();
    subpackets.push(9); // subpacket length (1 + 8)
    subpackets.push(SUBPKT_ISSUER);
    subpackets.extend_from_slice(&ctx.identity.key_id);
    subpackets
}

/// Build the v4 hash trailer appended after the hashed area.
/// Format: [0x04, 0xFF, 4-byte BE length of (header + hashed area)]
fn build_hash_trailer(hashed_header_len: usize) -> Vec<u8> {
    let mut trailer = Vec::with_capacity(6);
    trailer.push(0x04); // version 4
    trailer.push(0xFF);
    trailer.extend_from_slice(&(hashed_header_len as u32).to_be_bytes());
    trailer
}

/// Compute the PGP-contextualized hash over data.
///
/// The hash covers: data || v4_sig_header || hashed_area || trailer
///
/// Returns (full_digest, two_byte_hash_prefix).
pub fn pgp_hash(data: &[u8], ctx: &PgpSignatureContext) -> (Vec<u8>, [u8; 2]) {
    let hashed_area = build_hashed_area(ctx);
    let hash_id = pgp_hash_id(ctx.hash_algorithm);

    // v4 signature header bytes that go into the hash:
    // version(1) + sig_type(1) + pubkey_algo(1) + hash_algo(1) + hashed_area_len(2)
    let mut sig_header = vec![
        0x04,           // version 4
        PGP_SIG_BINARY, // binary document
        PGP_PUBKEY_RSA, // RSA
        hash_id,
    ];
    sig_header.extend_from_slice(&(hashed_area.len() as u16).to_be_bytes());
    sig_header.extend_from_slice(&hashed_area);

    let trailer = build_hash_trailer(sig_header.len());

    let digest = match ctx.hash_algorithm {
        DigestAlgorithm::Sha256 => {
            let mut h = Sha256::new();
            h.update(data);
            h.update(&sig_header);
            h.update(&trailer);
            h.finalize().to_vec()
        }
        DigestAlgorithm::Sha384 => {
            let mut h = Sha384::new();
            h.update(data);
            h.update(&sig_header);
            h.update(&trailer);
            h.finalize().to_vec()
        }
        DigestAlgorithm::Sha512 => {
            let mut h = Sha512::new();
            h.update(data);
            h.update(&sig_header);
            h.update(&trailer);
            h.finalize().to_vec()
        }
    };

    let prefix = [digest[0], digest[1]];
    (digest, prefix)
}

/// Build a complete OpenPGP v4 signature packet from a raw RSA signature.
///
/// The raw_sig comes from Azure KV and is the PKCS#1 v1.5 padded signature
/// over the PGP-contextualized hash.
pub fn build_signature_packet(
    ctx: &PgpSignatureContext,
    raw_sig: &[u8],
    hash_prefix: [u8; 2],
) -> Vec<u8> {
    let hashed_area = build_hashed_area(ctx);
    let unhashed_area = build_unhashed_area(ctx);
    let hash_id = pgp_hash_id(ctx.hash_algorithm);

    let mut body = vec![
        0x04,           // version 4
        PGP_SIG_BINARY, // binary document
        PGP_PUBKEY_RSA, // RSA
        hash_id,        // hash algorithm
    ];

    // Hashed subpacket area
    body.extend_from_slice(&(hashed_area.len() as u16).to_be_bytes());
    body.extend_from_slice(&hashed_area);

    // Unhashed subpacket area
    body.extend_from_slice(&(unhashed_area.len() as u16).to_be_bytes());
    body.extend_from_slice(&unhashed_area);

    // Hash prefix (left 16 bits of hash)
    body.extend_from_slice(&hash_prefix);

    // RSA signature as MPI
    body.extend_from_slice(&encode_mpi(raw_sig));

    // Wrap in packet framing (packet type 2 = signature)
    packet_frame(2, &body)
}

/// Wrap a signature packet in ASCII armor (PGP SIGNATURE).
pub fn ascii_armor_signature(packet: &[u8]) -> String {
    let b64 = base64_encode(packet);
    let crc = crc24(packet);
    let crc_bytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
    let crc_b64 = base64_encode(&crc_bytes);

    let mut out = String::new();
    out.push_str("-----BEGIN PGP SIGNATURE-----\n\n");

    // Wrap base64 at 76 characters
    for chunk in b64.as_bytes().chunks(76) {
        out.push_str(&String::from_utf8_lossy(chunk));
        out.push('\n');
    }

    out.push('=');
    out.push_str(&crc_b64);
    out.push('\n');
    out.push_str("-----END PGP SIGNATURE-----\n");
    out
}

/// Simple base64 encoding (standard alphabet with padding)
fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

/// CRC-24 as specified by RFC 4880 for ASCII armor.
pub fn crc24(data: &[u8]) -> u32 {
    const CRC24_INIT: u32 = 0x00B704CE;
    const CRC24_POLY: u32 = 0x01864CFB;

    let mut crc = CRC24_INIT;
    for &byte in data {
        crc ^= (byte as u32) << 16;
        for _ in 0..8 {
            crc <<= 1;
            if crc & 0x01000000 != 0 {
                crc ^= CRC24_POLY;
            }
        }
    }
    crc & 0x00FFFFFF
}

/// Extract RSA public key (modulus, exponent) from an X.509 DER certificate.
///
/// Walks the ASN.1 structure to find SubjectPublicKeyInfo → RSAPublicKey.
pub fn extract_rsa_pubkey(cert_der: &[u8]) -> Option<RsaPublicKey> {
    // X.509 Certificate structure:
    // SEQUENCE { tbsCertificate, signatureAlgorithm, signatureValue }
    // tbsCertificate: SEQUENCE { version, serialNumber, signature, issuer,
    //                            validity, subject, subjectPublicKeyInfo, ... }
    // subjectPublicKeyInfo: SEQUENCE { algorithm, BIT STRING { RSAPublicKey } }
    // RSAPublicKey: SEQUENCE { modulus INTEGER, exponent INTEGER }

    let (_, cert_content) = read_der_tag_len(cert_der, 0x30)?; // Certificate SEQUENCE
    let (_, tbs_content) = read_der_tag_len(cert_content, 0x30)?; // TBSCertificate SEQUENCE

    // Walk through TBSCertificate fields to find subjectPublicKeyInfo (7th field, index 6)
    let mut pos = 0;
    let mut field_idx = 0;

    while pos < tbs_content.len() && field_idx < 6 {
        let tag = tbs_content[pos];
        pos += 1;
        let (len, consumed) = read_der_length(&tbs_content[pos..])?;
        pos += consumed + len;
        field_idx += 1;

        // Skip context-tagged version field [0] if present
        if tag == 0xA0 && field_idx == 1 {
            // version is optional, already consumed, continue
        }
    }

    if field_idx != 6 || pos >= tbs_content.len() {
        return None;
    }

    // Now at subjectPublicKeyInfo
    let spki = &tbs_content[pos..];
    let (_, spki_content) = read_der_tag_len(spki, 0x30)?;

    // Skip AlgorithmIdentifier
    let (_, alg_content_with_rest) = read_der_tag_len(spki_content, 0x30)?;
    let alg_total = der_element_size(spki_content)?;
    let after_alg = &spki_content[alg_total..];

    // BIT STRING containing RSAPublicKey
    if after_alg.is_empty() || after_alg[0] != 0x03 {
        return None;
    }
    let (_, bs_content) = read_der_tag_len(after_alg, 0x03)?;

    // Skip the "unused bits" byte (should be 0x00)
    if bs_content.is_empty() {
        return None;
    }
    let rsa_key_der = &bs_content[1..];

    // RSAPublicKey SEQUENCE { modulus INTEGER, exponent INTEGER }
    let (_, rsa_content) = read_der_tag_len(rsa_key_der, 0x30)?;

    // modulus INTEGER
    let (_, mod_content) = read_der_tag_len(rsa_content, 0x02)?;
    let mod_total = der_element_size(rsa_content)?;

    // Strip leading zero from positive integer encoding
    let modulus = strip_leading_zero(mod_content);

    // exponent INTEGER
    let exp_data = &rsa_content[mod_total..];
    let (_, exp_content) = read_der_tag_len(exp_data, 0x02)?;
    let exponent = strip_leading_zero(exp_content);

    // Suppress unused variable warning
    let _ = alg_content_with_rest;

    Some(RsaPublicKey {
        modulus: modulus.to_vec(),
        exponent: exponent.to_vec(),
    })
}

/// Compute OpenPGP v4 key fingerprint and key ID from an RSA public key.
///
/// Fingerprint = SHA-1(0x99 || 2-byte pubkey body length || pubkey body)
/// Key ID = last 8 bytes of fingerprint
pub fn compute_key_identity(key: &RsaPublicKey, creation_time: u32) -> PgpKeyIdentity {
    let pubkey_body = build_pubkey_body(key, creation_time);

    let mut hasher = Sha1::new();
    hasher.update([0x99]);
    hasher.update((pubkey_body.len() as u16).to_be_bytes());
    hasher.update(&pubkey_body);

    let hash = hasher.finalize();
    let mut fingerprint = [0u8; 20];
    fingerprint.copy_from_slice(&hash);

    let mut key_id = [0u8; 8];
    key_id.copy_from_slice(&fingerprint[12..20]);

    PgpKeyIdentity {
        fingerprint,
        key_id,
    }
}

/// Build the body of a v4 public key packet (for fingerprinting and export).
fn build_pubkey_body(key: &RsaPublicKey, creation_time: u32) -> Vec<u8> {
    let n_mpi = encode_mpi(&key.modulus);
    let e_mpi = encode_mpi(&key.exponent);

    let mut body = Vec::new();
    body.push(0x04); // version 4
    body.extend_from_slice(&creation_time.to_be_bytes());
    body.push(PGP_PUBKEY_RSA);
    body.extend_from_slice(&n_mpi);
    body.extend_from_slice(&e_mpi);
    body
}

/// Build an exportable public key packet (for `rpm --import`).
pub fn build_public_key_packet(key: &RsaPublicKey, creation_time: u32) -> Vec<u8> {
    let body = build_pubkey_body(key, creation_time);
    // Packet type 6 = Public Key
    packet_frame(6, &body)
}

/// Export a public key in ASCII-armored format (for `rpm --import`).
pub fn ascii_armor_public_key(key: &RsaPublicKey, creation_time: u32) -> String {
    let packet = build_public_key_packet(key, creation_time);
    let b64 = base64_encode(&packet);
    let crc = crc24(&packet);
    let crc_bytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
    let crc_b64 = base64_encode(&crc_bytes);

    let mut out = String::new();
    out.push_str("-----BEGIN PGP PUBLIC KEY BLOCK-----\n\n");
    for chunk in b64.as_bytes().chunks(76) {
        out.push_str(&String::from_utf8_lossy(chunk));
        out.push('\n');
    }
    out.push('=');
    out.push_str(&crc_b64);
    out.push('\n');
    out.push_str("-----END PGP PUBLIC KEY BLOCK-----\n");
    out
}

/// Extract the Subject CN from an X.509 DER certificate.
pub fn extract_subject_cn(cert_der: &[u8]) -> Option<String> {
    // Navigate to TBSCertificate → Subject
    let (_, cert_content) = read_der_tag_len(cert_der, 0x30)?;
    let (_, tbs_content) = read_der_tag_len(cert_content, 0x30)?;

    // Walk to subject (field index 5)
    let mut pos = 0;
    let mut field_idx = 0;

    while pos < tbs_content.len() && field_idx < 5 {
        let tag = tbs_content[pos];
        pos += 1;
        let (len, consumed) = read_der_length(&tbs_content[pos..])?;
        pos += consumed + len;
        field_idx += 1;

        if tag == 0xA0 && field_idx == 1 {
            // context-tagged version, already consumed
        }
    }

    if field_idx != 5 || pos >= tbs_content.len() {
        return None;
    }

    // Subject is a SEQUENCE of SET of SEQUENCE { OID, value }
    let subject = &tbs_content[pos..];
    let (_, subject_content) = read_der_tag_len(subject, 0x30)?;

    // OID for CN: 2.5.4.3
    let cn_oid: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x03];

    // Search through RDN sets for CN
    let mut spos = 0;
    while spos < subject_content.len() {
        let set_start = spos;
        if subject_content[spos] != 0x31 {
            break;
        }
        spos += 1;
        let (set_len, set_consumed) = read_der_length(&subject_content[spos..])?;
        spos += set_consumed;
        let set_content = &subject_content[spos..spos + set_len];
        spos += set_len;

        // Each SET contains SEQUENCE { OID, value }
        if let Some((_, seq_content)) = read_der_tag_len(set_content, 0x30)
            && seq_content.len() > cn_oid.len()
            && seq_content.starts_with(cn_oid)
        {
            // Found CN, extract value
            let val_start = cn_oid.len();
            let val_data = &seq_content[val_start..];
            if !val_data.is_empty() {
                let val_tag = val_data[0];
                if let Some((_, val_content)) = read_der_tag_len(val_data, val_tag) {
                    return Some(String::from_utf8_lossy(val_content).to_string());
                }
            }
        }

        let _ = set_start;
    }

    None
}

/// Inline MD5 implementation for dpkg-sig compatibility.
/// Only used in one place (DEB signing control text), so kept inline.
pub fn md5_digest(data: &[u8]) -> [u8; 16] {
    // Constants
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    // Pre-processing: add padding
    let orig_len_bits = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&orig_len_bits.to_le_bytes());

    // Initialize hash values
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    // Process each 64-byte chunk
    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
        }

        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | ((!b) & d), i),
                16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | (!d)), (7 * i) % 16),
            };

            let f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&a0.to_le_bytes());
    result[4..8].copy_from_slice(&b0.to_le_bytes());
    result[8..12].copy_from_slice(&c0.to_le_bytes());
    result[12..16].copy_from_slice(&d0.to_le_bytes());
    result
}

/// SHA-1 digest (for dpkg-sig member hashes)
pub fn sha1_digest(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 20];
    out.copy_from_slice(&result);
    out
}

/// Format bytes as hex string
pub fn hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

// --- DER parsing helpers ---

/// Read a DER tag and length, returning the content slice.
fn read_der_tag_len(data: &[u8], expected_tag: u8) -> Option<(usize, &[u8])> {
    if data.is_empty() || data[0] != expected_tag {
        return None;
    }
    let (len, consumed) = read_der_length(&data[1..])?;
    let start = 1 + consumed;
    let end = start + len;
    if end > data.len() {
        return None;
    }
    Some((start, &data[start..end]))
}

/// Read a DER length value, returning (length, bytes_consumed).
fn read_der_length(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    if data[0] < 0x80 {
        Some((data[0] as usize, 1))
    } else {
        let num_bytes = (data[0] & 0x7F) as usize;
        if num_bytes == 0 || num_bytes > 4 || data.len() < 1 + num_bytes {
            return None;
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        Some((len, 1 + num_bytes))
    }
}

/// Get the total size of a DER element (tag + length + content).
fn der_element_size(data: &[u8]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    let (len, consumed) = read_der_length(&data[1..])?;
    Some(1 + consumed + len)
}

/// Strip leading zero byte from a positive integer encoding.
fn strip_leading_zero(data: &[u8]) -> &[u8] {
    if data.len() > 1 && data[0] == 0x00 {
        &data[1..]
    } else {
        data
    }
}

// --- RSA Verification ---
//
// Inline big-number arithmetic for RSA public key verification.
// Only needed for signature verification (not signing, which happens in KV).
// Uses schoolbook algorithms on big-endian byte arrays.

/// Result of signature verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyResult {
    /// Signature is cryptographically valid
    Valid,
    /// Hash prefix in signature packet does not match computed hash
    HashMismatch,
    /// RSA signature is invalid (decrypted value does not match expected digest)
    SignatureInvalid,
    /// Signature format is not supported
    UnsupportedFormat,
}

/// Parsed OpenPGP v4 signature packet
#[derive(Debug)]
pub struct ParsedSignature {
    pub sig_type: u8,
    pub hash_algorithm: DigestAlgorithm,
    pub creation_time: u32,
    pub key_id: [u8; 8],
    pub hash_prefix: [u8; 2],
    pub rsa_signature: Vec<u8>,
    pub hashed_area: Vec<u8>,
}

/// Compare two big-endian byte arrays: returns Ordering
fn bignum_cmp(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    // Strip leading zeros for comparison
    let a = strip_leading_zeros(a);
    let b = strip_leading_zeros(b);
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    }
}

/// Strip leading zero bytes from a big-endian number
fn strip_leading_zeros(data: &[u8]) -> &[u8] {
    let start = data
        .iter()
        .position(|&b| b != 0)
        .unwrap_or(data.len().saturating_sub(1));
    &data[start..]
}

/// Subtract b from a (big-endian), assuming a >= b. Returns result with same length as a.
fn bignum_sub(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len().max(b.len());
    let mut result = vec![0u8; n];
    let mut borrow: i16 = 0;

    for i in 0..n {
        let ai = if i < a.len() {
            a[a.len() - 1 - i] as i16
        } else {
            0
        };
        let bi = if i < b.len() {
            b[b.len() - 1 - i] as i16
        } else {
            0
        };
        let diff = ai - bi - borrow;
        if diff < 0 {
            result[n - 1 - i] = (diff + 256) as u8;
            borrow = 1;
        } else {
            result[n - 1 - i] = diff as u8;
            borrow = 0;
        }
    }
    result
}

/// Multiply two big-endian byte arrays, return product
fn bignum_mul(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len() + b.len();
    let mut result = vec![0u32; n];

    // Work from least-significant byte. Index i in a corresponds to
    // a[a.len()-1-i] (value contribution = a_byte * 256^i).
    for i in 0..a.len() {
        for j in 0..b.len() {
            // pos = power of 256 for this partial product
            let pos = i + j;
            result[n - 1 - pos] += a[a.len() - 1 - i] as u32 * b[b.len() - 1 - j] as u32;
        }
    }

    // Propagate carries (from least significant to most significant)
    for i in (1..n).rev() {
        result[i - 1] += result[i] >> 8;
        result[i] &= 0xFF;
    }
    // Handle final carry (may be multi-byte in extreme cases)
    let mut out = Vec::with_capacity(n + 1);
    let extra = result[0] >> 8;
    result[0] &= 0xFF;
    if extra > 0 {
        // For schoolbook multiply, extra is at most ~255*255 >> 8 propagated,
        // but handle multi-byte just in case
        if extra > 255 {
            out.push((extra >> 8) as u8);
        }
        out.push(extra as u8);
    }
    for &v in &result {
        out.push(v as u8);
    }
    out
}

/// Compute a mod m for big-endian byte arrays.
/// Uses repeated subtraction with shifting for efficiency.
fn bignum_mod(a: &[u8], m: &[u8]) -> Vec<u8> {
    let a = strip_leading_zeros(a);
    let m = strip_leading_zeros(m);

    if bignum_cmp(a, m) == std::cmp::Ordering::Less {
        return a.to_vec();
    }

    // Binary long division
    let mut remainder = Vec::new();
    for &byte in a {
        for bit_pos in (0..8).rev() {
            // Shift remainder left by 1 bit and add current bit
            let bit = (byte >> bit_pos) & 1;
            shift_left_one(&mut remainder);
            if bit == 1 {
                if let Some(last) = remainder.last_mut() {
                    *last |= 1;
                } else {
                    remainder.push(1);
                }
            }

            // If remainder >= m, subtract m
            if bignum_cmp(&remainder, m) != std::cmp::Ordering::Less {
                remainder = bignum_sub(&remainder, m);
                // Strip leading zeros
                let start = remainder
                    .iter()
                    .position(|&b| b != 0)
                    .unwrap_or(remainder.len().saturating_sub(1));
                remainder = remainder[start..].to_vec();
            }
        }
    }

    if remainder.is_empty() {
        vec![0]
    } else {
        remainder
    }
}

/// Shift a big-endian byte array left by 1 bit
fn shift_left_one(data: &mut Vec<u8>) {
    let mut carry = 0u8;
    for byte in data.iter_mut().rev() {
        let new_carry = *byte >> 7;
        *byte = (*byte << 1) | carry;
        carry = new_carry;
    }
    if carry != 0 {
        data.insert(0, carry);
    }
}

/// Modular exponentiation: base^exp mod modulus (big-endian byte arrays).
/// Uses square-and-multiply algorithm.
pub fn mod_exp(base: &[u8], exp: &[u8], modulus: &[u8]) -> Vec<u8> {
    let modulus = strip_leading_zeros(modulus);

    // Handle trivial cases
    if modulus == [1] {
        return vec![0];
    }

    let base_mod = bignum_mod(base, modulus);
    let mut result = vec![1u8];

    // Iterate through exponent bits (MSB first)
    let exp = strip_leading_zeros(exp);
    for &byte in exp {
        for bit_pos in (0..8).rev() {
            // Square
            let squared = bignum_mul(&result, &result);
            result = bignum_mod(&squared, modulus);

            // Multiply if bit is set
            if (byte >> bit_pos) & 1 == 1 {
                let product = bignum_mul(&result, &base_mod);
                result = bignum_mod(&product, modulus);
            }
        }
    }

    result
}

// PKCS#1 v1.5 DigestInfo prefixes (DER-encoded AlgorithmIdentifier + hash)
const PKCS1_SHA256_PREFIX: &[u8] = &[
    0x30, 0x31, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05,
    0x00, 0x04, 0x20,
];
const PKCS1_SHA384_PREFIX: &[u8] = &[
    0x30, 0x41, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05,
    0x00, 0x04, 0x30,
];
const PKCS1_SHA512_PREFIX: &[u8] = &[
    0x30, 0x51, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05,
    0x00, 0x04, 0x40,
];

/// Verify an RSA PKCS#1 v1.5 signature.
///
/// Decrypts the signature with the public key (sig^e mod n),
/// then verifies PKCS padding and DigestInfo structure match the expected digest.
pub fn rsa_verify_pkcs1v15(
    key: &RsaPublicKey,
    signature: &[u8],
    expected_digest: &[u8],
    hash_algorithm: DigestAlgorithm,
) -> bool {
    // Compute sig^e mod n
    let decrypted = mod_exp(signature, &key.exponent, &key.modulus);

    // Pad decrypted to modulus length (leading zeros may have been stripped)
    let mod_len = key.modulus.len();
    let mut padded = vec![0u8; mod_len];
    let offset = mod_len.saturating_sub(decrypted.len());
    padded[offset..].copy_from_slice(&decrypted);

    // PKCS#1 v1.5 format: 0x00 0x01 [0xFF padding] 0x00 [DigestInfo]
    if padded.len() < 11 || padded[0] != 0x00 || padded[1] != 0x01 {
        return false;
    }

    // Find the 0x00 separator after the 0xFF padding
    let mut sep_pos = 2;
    while sep_pos < padded.len() && padded[sep_pos] == 0xFF {
        sep_pos += 1;
    }
    if sep_pos >= padded.len() || padded[sep_pos] != 0x00 {
        return false;
    }
    sep_pos += 1; // skip the 0x00

    let digest_info = &padded[sep_pos..];

    // Get the expected DigestInfo prefix for the hash algorithm
    let prefix = match hash_algorithm {
        DigestAlgorithm::Sha256 => PKCS1_SHA256_PREFIX,
        DigestAlgorithm::Sha384 => PKCS1_SHA384_PREFIX,
        DigestAlgorithm::Sha512 => PKCS1_SHA512_PREFIX,
    };

    // Check prefix + digest
    if digest_info.len() != prefix.len() + expected_digest.len() {
        return false;
    }
    if !digest_info.starts_with(prefix) {
        return false;
    }
    digest_info[prefix.len()..] == *expected_digest
}

/// Parse an OpenPGP v4 signature packet.
///
/// Handles both old-format and new-format packet framing.
/// Returns the parsed components needed for verification.
pub fn parse_signature_packet(data: &[u8]) -> Option<ParsedSignature> {
    if data.is_empty() {
        return None;
    }

    // Parse packet header
    let tag_byte = data[0];
    if tag_byte & 0x80 == 0 {
        return None; // not a valid PGP packet
    }

    let (packet_type, body_start, body_len);
    if tag_byte & 0x40 != 0 {
        // New-format packet
        packet_type = tag_byte & 0x3F;
        let (len, consumed) = parse_new_packet_length(&data[1..])?;
        body_start = 1 + consumed;
        body_len = len;
    } else {
        // Old-format packet
        packet_type = (tag_byte & 0x3C) >> 2;
        let length_type = tag_byte & 0x03;
        match length_type {
            0 => {
                if data.len() < 2 {
                    return None;
                }
                body_start = 2;
                body_len = data[1] as usize;
            }
            1 => {
                if data.len() < 3 {
                    return None;
                }
                body_start = 3;
                body_len = u16::from_be_bytes([data[1], data[2]]) as usize;
            }
            2 => {
                if data.len() < 5 {
                    return None;
                }
                body_start = 5;
                body_len = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
            }
            _ => return None, // indeterminate length not supported
        }
    }

    // Must be signature packet (type 2)
    if packet_type != 2 {
        return None;
    }

    if body_start + body_len > data.len() {
        return None;
    }
    let body = &data[body_start..body_start + body_len];

    // Parse v4 signature packet body
    if body.len() < 6 || body[0] != 0x04 {
        return None; // only v4 supported
    }

    let sig_type = body[1];
    let pubkey_algo = body[2];
    let hash_algo = body[3];

    if pubkey_algo != PGP_PUBKEY_RSA {
        return None; // only RSA supported
    }

    let hash_algorithm = match hash_algo {
        PGP_HASH_SHA256 => DigestAlgorithm::Sha256,
        PGP_HASH_SHA384 => DigestAlgorithm::Sha384,
        PGP_HASH_SHA512 => DigestAlgorithm::Sha512,
        _ => return None,
    };

    // Hashed subpacket area
    let hashed_len = u16::from_be_bytes([body[4], body[5]]) as usize;
    if 6 + hashed_len > body.len() {
        return None;
    }
    let hashed_subpackets = &body[6..6 + hashed_len];

    // Build the full hashed area for hash reconstruction
    // (sig header + hashed subpackets as they appear in the packet)
    let hashed_area = body[..6 + hashed_len].to_vec();

    // Parse creation time from hashed subpackets
    let creation_time = parse_subpacket_creation_time(hashed_subpackets).unwrap_or(0);

    // Unhashed subpacket area
    let unhashed_start = 6 + hashed_len;
    if unhashed_start + 2 > body.len() {
        return None;
    }
    let unhashed_len =
        u16::from_be_bytes([body[unhashed_start], body[unhashed_start + 1]]) as usize;
    let unhashed_end = unhashed_start + 2 + unhashed_len;
    if unhashed_end > body.len() {
        return None;
    }
    let unhashed_subpackets = &body[unhashed_start + 2..unhashed_end];

    // Parse key ID from unhashed subpackets
    let mut key_id = [0u8; 8];
    if let Some(kid) = parse_subpacket_issuer(unhashed_subpackets) {
        key_id = kid;
    }

    // Hash prefix (2 bytes after unhashed area)
    if unhashed_end + 2 > body.len() {
        return None;
    }
    let hash_prefix = [body[unhashed_end], body[unhashed_end + 1]];

    // RSA signature MPI
    let mpi_start = unhashed_end + 2;
    if mpi_start + 2 > body.len() {
        return None;
    }
    let mpi_bits = u16::from_be_bytes([body[mpi_start], body[mpi_start + 1]]) as usize;
    let mpi_bytes = mpi_bits.div_ceil(8);
    if mpi_start + 2 + mpi_bytes > body.len() {
        return None;
    }
    let rsa_signature = body[mpi_start + 2..mpi_start + 2 + mpi_bytes].to_vec();

    Some(ParsedSignature {
        sig_type,
        hash_algorithm,
        creation_time,
        key_id,
        hash_prefix,
        rsa_signature,
        hashed_area,
    })
}

/// Parse new-format packet length encoding
fn parse_new_packet_length(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    match data[0] {
        0..=191 => Some((data[0] as usize, 1)),
        192..=223 => {
            if data.len() < 2 {
                return None;
            }
            let len = ((data[0] as usize - 192) << 8) + data[1] as usize + 192;
            Some((len, 2))
        }
        255 => {
            if data.len() < 5 {
                return None;
            }
            let len = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
            Some((len, 5))
        }
        _ => None, // partial body lengths not supported
    }
}

/// Extract creation time from hashed subpackets
fn parse_subpacket_creation_time(subpackets: &[u8]) -> Option<u32> {
    let mut pos = 0;
    while pos < subpackets.len() {
        let (spkt_len, consumed) = parse_subpacket_length(&subpackets[pos..])?;
        pos += consumed;
        if spkt_len == 0 || pos + spkt_len > subpackets.len() {
            break;
        }
        let spkt_type = subpackets[pos];
        if spkt_type == SUBPKT_CREATION_TIME && spkt_len >= 5 {
            let time = u32::from_be_bytes([
                subpackets[pos + 1],
                subpackets[pos + 2],
                subpackets[pos + 3],
                subpackets[pos + 4],
            ]);
            return Some(time);
        }
        pos += spkt_len;
    }
    None
}

/// Extract issuer key ID from subpackets
fn parse_subpacket_issuer(subpackets: &[u8]) -> Option<[u8; 8]> {
    let mut pos = 0;
    while pos < subpackets.len() {
        let (spkt_len, consumed) = parse_subpacket_length(&subpackets[pos..])?;
        pos += consumed;
        if spkt_len == 0 || pos + spkt_len > subpackets.len() {
            break;
        }
        let spkt_type = subpackets[pos];
        if spkt_type == SUBPKT_ISSUER && spkt_len >= 9 {
            let mut kid = [0u8; 8];
            kid.copy_from_slice(&subpackets[pos + 1..pos + 9]);
            return Some(kid);
        }
        pos += spkt_len;
    }
    None
}

/// Parse subpacket length (1, 2, or 5 bytes)
fn parse_subpacket_length(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    match data[0] {
        0..=191 => Some((data[0] as usize, 1)),
        192..=254 => {
            if data.len() < 2 {
                return None;
            }
            let len = ((data[0] as usize - 192) << 8) + data[1] as usize + 192;
            Some((len, 2))
        }
        255 => {
            if data.len() < 5 {
                return None;
            }
            let len = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
            Some((len, 5))
        }
    }
}

/// Verify a detached OpenPGP signature against file data.
///
/// Parses the signature packet, recomputes the PGP-contextualized hash,
/// and verifies the RSA signature using the provided public key.
pub fn verify_detached_signature(
    file_data: &[u8],
    sig_data: &[u8],
    key: &RsaPublicKey,
    key_creation_time: u32,
) -> VerifyResult {
    let parsed = match parse_signature_packet(sig_data) {
        Some(p) => p,
        None => return VerifyResult::UnsupportedFormat,
    };

    // Reconstruct the PGP hash using the parsed hashed area
    let hashed_area = &parsed.hashed_area;
    let trailer = build_hash_trailer(hashed_area.len());

    let digest = match parsed.hash_algorithm {
        DigestAlgorithm::Sha256 => {
            let mut h = Sha256::new();
            h.update(file_data);
            h.update(hashed_area);
            h.update(&trailer);
            h.finalize().to_vec()
        }
        DigestAlgorithm::Sha384 => {
            let mut h = Sha384::new();
            h.update(file_data);
            h.update(hashed_area);
            h.update(&trailer);
            h.finalize().to_vec()
        }
        DigestAlgorithm::Sha512 => {
            let mut h = Sha512::new();
            h.update(file_data);
            h.update(hashed_area);
            h.update(&trailer);
            h.finalize().to_vec()
        }
    };

    // Quick check: hash prefix
    if digest.len() < 2 || digest[0] != parsed.hash_prefix[0] || digest[1] != parsed.hash_prefix[1]
    {
        return VerifyResult::HashMismatch;
    }

    // Full RSA verification
    let _ = key_creation_time; // used for context but not needed in verification logic
    if rsa_verify_pkcs1v15(key, &parsed.rsa_signature, &digest, parsed.hash_algorithm) {
        VerifyResult::Valid
    } else {
        VerifyResult::SignatureInvalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_mpi_simple() {
        // Value 0x01 = 1 bit = MPI(0x00, 0x01, 0x01)
        let mpi = encode_mpi(&[0x01]);
        assert_eq!(mpi, vec![0x00, 0x01, 0x01]);

        // Value 0x80 = 8 bits
        let mpi = encode_mpi(&[0x80]);
        assert_eq!(mpi, vec![0x00, 0x08, 0x80]);

        // Value 0xFF = 8 bits
        let mpi = encode_mpi(&[0xFF]);
        assert_eq!(mpi, vec![0x00, 0x08, 0xFF]);
    }

    #[test]
    fn test_encode_mpi_multibyte() {
        let mpi = encode_mpi(&[0x01, 0x00]);
        // 0x0100 = 256, bit count = 9
        assert_eq!(&mpi[..2], &[0x00, 0x09]);
        assert_eq!(&mpi[2..], &[0x01, 0x00]);
    }

    #[test]
    fn test_encode_mpi_leading_zeros() {
        // Leading zeros should be stripped
        let mpi = encode_mpi(&[0x00, 0x00, 0x42]);
        assert_eq!(mpi, vec![0x00, 0x07, 0x42]); // 7 bits
    }

    #[test]
    fn test_encode_mpi_zero() {
        let mpi = encode_mpi(&[0x00]);
        assert_eq!(mpi, vec![0x00, 0x00, 0x00]); // 0 bits
    }

    #[test]
    fn test_crc24() {
        // Empty data
        let crc = crc24(&[]);
        assert_eq!(crc, 0x00B704CE); // CRC24_INIT for empty

        // Known test
        let crc = crc24(b"hello");
        assert_ne!(crc, 0); // just verify it produces something
        assert!(crc <= 0x00FFFFFF); // 24-bit
    }

    #[test]
    fn test_packet_frame_short() {
        let body = vec![0x01, 0x02, 0x03];
        let packet = packet_frame(2, &body);
        // Old format: tag = 0x80 | (2 << 2) = 0x88, one-byte length = 3
        assert_eq!(packet[0], 0x88);
        assert_eq!(packet[1], 3);
        assert_eq!(&packet[2..], &body);
    }

    #[test]
    fn test_packet_frame_medium() {
        let body = vec![0xAA; 300];
        let packet = packet_frame(2, &body);
        // Old format: tag = 0x88 | 0x01 = 0x89, two-byte length
        assert_eq!(packet[0], 0x89);
        assert_eq!(u16::from_be_bytes([packet[1], packet[2]]), 300);
    }

    #[test]
    fn test_pgp_hash_deterministic() {
        let key = RsaPublicKey {
            modulus: vec![0x01; 256],
            exponent: vec![0x01, 0x00, 0x01],
        };
        let identity = compute_key_identity(&key, 1700000000);
        let ctx = PgpSignatureContext {
            key,
            identity,
            hash_algorithm: DigestAlgorithm::Sha256,
            creation_time: 1700000000,
        };

        let (digest1, prefix1) = pgp_hash(b"test data", &ctx);
        let (digest2, prefix2) = pgp_hash(b"test data", &ctx);
        assert_eq!(digest1, digest2);
        assert_eq!(prefix1, prefix2);

        // Different data should produce different hash
        let (digest3, _) = pgp_hash(b"other data", &ctx);
        assert_ne!(digest1, digest3);
    }

    #[test]
    fn test_build_signature_packet() {
        let key = RsaPublicKey {
            modulus: vec![0x01; 256],
            exponent: vec![0x01, 0x00, 0x01],
        };
        let identity = compute_key_identity(&key, 1700000000);
        let ctx = PgpSignatureContext {
            key,
            identity,
            hash_algorithm: DigestAlgorithm::Sha256,
            creation_time: 1700000000,
        };

        let fake_sig = vec![0xAB; 256];
        let packet = build_signature_packet(&ctx, &fake_sig, [0xDE, 0xAD]);

        // Should be a valid old-format packet type 2
        assert_eq!(packet[0] & 0xFC, 0x88); // type 2
    }

    #[test]
    fn test_ascii_armor_roundtrip() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let armored = ascii_armor_signature(&data);

        assert!(armored.starts_with("-----BEGIN PGP SIGNATURE-----\n"));
        assert!(armored.ends_with("-----END PGP SIGNATURE-----\n"));
        assert!(armored.contains('=')); // CRC line
    }

    #[test]
    fn test_compute_key_identity() {
        let key = RsaPublicKey {
            modulus: vec![0x01; 256],
            exponent: vec![0x01, 0x00, 0x01],
        };
        let identity = compute_key_identity(&key, 1700000000);

        assert_eq!(identity.fingerprint.len(), 20);
        assert_eq!(identity.key_id.len(), 8);
        // Key ID should be last 8 bytes of fingerprint
        assert_eq!(&identity.key_id, &identity.fingerprint[12..20]);
    }

    #[test]
    fn test_md5_known_values() {
        // MD5("") = d41d8cd98f00b204e9800998ecf8427e
        let empty = md5_digest(b"");
        assert_eq!(hex(&empty), "d41d8cd98f00b204e9800998ecf8427e");

        // MD5("hello world") = 5eb63bbbe01eeed093cb22bb8f5acdc3
        let hello = md5_digest(b"hello world");
        assert_eq!(hex(&hello), "5eb63bbbe01eeed093cb22bb8f5acdc3");

        // MD5("abc") = 900150983cd24fb0d6963f7d28e17f72
        let abc = md5_digest(b"abc");
        assert_eq!(hex(&abc), "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn test_sha1_digest() {
        // SHA-1("hello world") = 2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        let result = sha1_digest(b"hello world");
        assert_eq!(hex(&result), "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }

    #[test]
    fn test_pgp_hash_id_mapping() {
        assert_eq!(pgp_hash_id(DigestAlgorithm::Sha256), 8);
        assert_eq!(pgp_hash_id(DigestAlgorithm::Sha384), 9);
        assert_eq!(pgp_hash_id(DigestAlgorithm::Sha512), 10);
    }

    #[test]
    fn test_build_public_key_packet() {
        let key = RsaPublicKey {
            modulus: vec![0xFF; 256],
            exponent: vec![0x01, 0x00, 0x01],
        };
        let packet = build_public_key_packet(&key, 1700000000);

        // Should be old-format packet type 6 = 0x80 | (6 << 2) = 0x98
        assert_eq!(packet[0] & 0xFC, 0x98);
    }

    #[test]
    fn test_extract_rsa_pubkey_real_cert() {
        // Minimal self-signed RSA cert (DER-encoded) for testing.
        // This is a real valid X.509 structure with a 512-bit RSA key.
        // Generated specifically for unit testing.
        let cert_der = build_test_cert();
        let key = extract_rsa_pubkey(&cert_der);
        assert!(key.is_some(), "Should extract RSA key from valid cert");
        let key = key.unwrap();
        assert!(!key.modulus.is_empty());
        assert!(!key.exponent.is_empty());
    }

    #[test]
    fn test_extract_rsa_pubkey_invalid() {
        assert!(extract_rsa_pubkey(b"not a cert").is_none());
        assert!(extract_rsa_pubkey(&[]).is_none());
    }

    /// Build a minimal X.509 certificate DER for testing.
    /// Structure: Certificate { TBSCertificate { version, serial, sigAlg,
    ///   issuer, validity, subject, subjectPublicKeyInfo }, sigAlg, sig }
    fn build_test_cert() -> Vec<u8> {
        use super::super::asn1;

        // RSA public key: small modulus + exponent
        let mut modulus_bytes = vec![0x00]; // leading zero for positive encoding
        modulus_bytes.extend_from_slice(&[0xBB; 64]); // 512-bit key
        let exponent_bytes = vec![0x01, 0x00, 0x01]; // 65537

        let n_int = asn1::integer(&modulus_bytes);
        let e_int = asn1::integer(&exponent_bytes);
        let rsa_pubkey = asn1::sequence(&[&n_int, &e_int]);

        let bs = asn1::bit_string(&rsa_pubkey);
        let rsa_alg_id = asn1::algorithm_identifier(asn1::OID_RSA_ENCRYPTION);
        let spki = asn1::sequence(&[&rsa_alg_id, &bs]);

        // version [0] EXPLICIT INTEGER 2 (v3)
        let version = asn1::context_tag(0, &asn1::integer_u64(2));

        // serial
        let serial = asn1::integer_u64(1);

        // signature algorithm
        let sig_alg = asn1::algorithm_identifier(asn1::OID_SHA256);

        // issuer (empty SEQUENCE)
        let issuer = asn1::sequence(&[]);

        // validity
        let not_before = asn1::utc_time("250101000000Z");
        let not_after = asn1::utc_time("350101000000Z");
        let validity = asn1::sequence(&[&not_before, &not_after]);

        // subject with CN
        let cn_oid = asn1::oid(&[2, 5, 4, 3]);
        let cn_val = asn1::utf8_string("Test Signer");
        let cn_attr = asn1::sequence(&[&cn_oid, &cn_val]);
        let cn_set = asn1::set(&[&cn_attr]);
        let subject = asn1::sequence(&[&cn_set]);

        let tbs = asn1::sequence(&[
            &version, &serial, &sig_alg, &issuer, &validity, &subject, &spki,
        ]);

        let sig_alg2 = asn1::algorithm_identifier(asn1::OID_SHA256);
        let sig_value = asn1::bit_string(&[0xFF; 64]);

        asn1::sequence(&[&tbs, &sig_alg2, &sig_value])
    }

    #[test]
    fn test_extract_subject_cn() {
        let cert = build_test_cert();
        let cn = extract_subject_cn(&cert);
        assert_eq!(cn, Some("Test Signer".to_string()));
    }

    #[test]
    fn test_mod_exp_small() {
        // 2^10 mod 1000 = 1024 mod 1000 = 24
        let result = mod_exp(&[2], &[10], &[0x03, 0xE8]); // 1000 = 0x03E8
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 24);
    }

    #[test]
    fn test_mod_exp_identity() {
        // x^1 mod m = x mod m
        let result = mod_exp(&[42], &[1], &[100]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_mod_exp_zero_exp() {
        // x^0 mod m = 1 (for m > 1)
        let result = mod_exp(&[42], &[0], &[100]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 1);
    }

    #[test]
    fn test_bignum_mul_simple() {
        // 3 * 7 = 21
        let result = bignum_mul(&[3], &[7]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 21);
    }

    #[test]
    fn test_bignum_mul_large() {
        // 256 * 256 = 65536
        let result = bignum_mul(&[0x01, 0x00], &[0x01, 0x00]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 65536);
    }

    #[test]
    fn test_bignum_mod_simple() {
        // 17 mod 5 = 2
        let result = bignum_mod(&[17], &[5]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 2);
    }

    #[test]
    fn test_bignum_mod_larger() {
        // 1000 mod 7 = 6
        let result = bignum_mod(&[0x03, 0xE8], &[7]);
        let val = result.iter().fold(0u64, |acc, &b| acc * 256 + b as u64);
        assert_eq!(val, 6);
    }

    #[test]
    fn test_parse_signature_packet_roundtrip() {
        let key = RsaPublicKey {
            modulus: vec![0xFF; 256],
            exponent: vec![0x01, 0x00, 0x01],
        };
        let identity = compute_key_identity(&key, 1700000000);
        let ctx = PgpSignatureContext {
            key,
            identity,
            hash_algorithm: DigestAlgorithm::Sha256,
            creation_time: 1700000000,
        };

        let fake_sig = vec![0xAB; 256];
        let packet = build_signature_packet(&ctx, &fake_sig, [0xDE, 0xAD]);

        let parsed = parse_signature_packet(&packet);
        assert!(parsed.is_some(), "Should parse our own signature packet");
        let parsed = parsed.unwrap();
        assert_eq!(parsed.sig_type, PGP_SIG_BINARY);
        assert_eq!(parsed.creation_time, 1700000000);
        assert_eq!(parsed.hash_prefix, [0xDE, 0xAD]);
        assert_eq!(parsed.rsa_signature.len(), 256);
        assert_eq!(parsed.key_id, ctx.identity.key_id);
    }

    #[test]
    fn test_parse_signature_packet_invalid() {
        assert!(parse_signature_packet(&[]).is_none());
        assert!(parse_signature_packet(b"not a packet").is_none());
        assert!(parse_signature_packet(&[0x00]).is_none());
    }

    #[test]
    fn test_verify_result_variants() {
        // Just ensure the enum variants exist and compare
        assert_eq!(VerifyResult::Valid, VerifyResult::Valid);
        assert_ne!(VerifyResult::Valid, VerifyResult::HashMismatch);
        assert_ne!(VerifyResult::Valid, VerifyResult::SignatureInvalid);
        assert_ne!(VerifyResult::Valid, VerifyResult::UnsupportedFormat);
    }
}
