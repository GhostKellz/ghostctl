// DEB package signing via Azure Key Vault
//
// Two modes:
// 1. Detached: produces .sig + .sig.json files (default)
// 2. Native: adds _gpgbuilder ar member with dpkg-sig format (--native)
//    Verifiable with `dpkg-sig --verify`

use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

use super::config::SigningConfig;
use super::hash::{DigestAlgorithm, file_digest, hex_digest};
use super::keyvault::KeyVaultClient;
use super::pgp;

/// AR archive magic
const AR_MAGIC: &[u8; 8] = b"!<arch>\n";

/// DEB metadata extracted from the package
#[derive(Debug, Default)]
struct DebInfo {
    package: String,
    version: String,
    architecture: String,
}

/// Metadata written alongside a DEB detached signature
#[derive(Serialize)]
struct DebSignatureMetadata {
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
    deb_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deb_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deb_architecture: Option<String>,
}

/// Try to extract DEB package info from the ar archive.
/// Looks for a control.tar* member and parses the control file inside.
fn extract_deb_info(path: &Path) -> Option<DebInfo> {
    let data = fs::read(path).ok()?;

    // Verify AR magic
    if data.len() < 8 || &data[0..8] != AR_MAGIC {
        return None;
    }

    // Parse AR members looking for control.tar*
    let mut pos = 8;
    while pos + 60 <= data.len() {
        // AR member header: 60 bytes
        let name = String::from_utf8_lossy(&data[pos..pos + 16])
            .trim()
            .trim_end_matches('/')
            .to_string();
        let size_str = String::from_utf8_lossy(&data[pos + 48..pos + 58])
            .trim()
            .to_string();
        let size: usize = size_str.parse().ok()?;

        // Check end marker
        if &data[pos + 58..pos + 60] != b"`\n" {
            break;
        }

        let member_start = pos + 60;
        let member_end = member_start + size;

        if name.starts_with("control.tar") {
            // Try to extract control file from the tarball
            let member_data = &data[member_start..member_end.min(data.len())];
            if let Some(info) = parse_control_tar(member_data, &name) {
                return Some(info);
            }
        }

        // Next member (2-byte aligned)
        pos = member_end + (member_end % 2);
    }

    // Fallback: parse from filename
    Some(info_from_filename(path))
}

/// Parse a control.tar* to extract the control file content
fn parse_control_tar(data: &[u8], member_name: &str) -> Option<DebInfo> {
    // Decompress if needed
    let tar_data = if member_name.ends_with(".gz") {
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).ok()?;
        decompressed
    } else if member_name.ends_with(".xz") {
        // xz decompression would need an additional dep, skip for now
        return None;
    } else if member_name.ends_with(".zst") {
        return None;
    } else {
        data.to_vec()
    };

    // Parse tar looking for ./control or control
    let mut pos = 0;
    while pos + 512 <= tar_data.len() {
        // Tar header: name is first 100 bytes
        let name = String::from_utf8_lossy(&tar_data[pos..pos + 100])
            .trim_matches('\0')
            .trim_start_matches("./")
            .to_string();

        // File size at offset 124, 12 bytes, octal
        let size_str = String::from_utf8_lossy(&tar_data[pos + 124..pos + 136])
            .trim_matches('\0')
            .trim()
            .to_string();
        let size: usize = usize::from_str_radix(&size_str, 8).unwrap_or(0);

        if name == "control" && size > 0 {
            let content_start = pos + 512;
            let content_end = (content_start + size).min(tar_data.len());
            let control_text =
                String::from_utf8_lossy(&tar_data[content_start..content_end]).to_string();
            return Some(parse_control_fields(&control_text));
        }

        // Skip to next tar entry (512-byte aligned)
        let data_blocks = size.div_ceil(512);
        pos += 512 + (data_blocks * 512);
    }

    None
}

/// Parse Debian control file fields
fn parse_control_fields(control: &str) -> DebInfo {
    let mut info = DebInfo::default();

    for line in control.lines() {
        if let Some(val) = line.strip_prefix("Package:") {
            info.package = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("Version:") {
            info.version = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("Architecture:") {
            info.architecture = val.trim().to_string();
        }
    }

    info
}

/// Fallback: parse info from filename
fn info_from_filename(path: &Path) -> DebInfo {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Common DEB naming: package_version_arch.deb
    let parts: Vec<&str> = stem.splitn(3, '_').collect();
    DebInfo {
        package: parts.first().unwrap_or(&"unknown").to_string(),
        version: parts.get(1).unwrap_or(&"").to_string(),
        architecture: parts.get(2).unwrap_or(&"").to_string(),
    }
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

/// Sign a DEB package with a detached signature
pub fn sign_deb(
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

    let deb_info = extract_deb_info(path).unwrap_or_else(|| info_from_filename(path));

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", file_meta.len());
        println!("Format: DEB Package");
        if !deb_info.package.is_empty() {
            println!("Package: {}", deb_info.package);
        }
        if !deb_info.version.is_empty() {
            println!("Version: {}", deb_info.version);
        }
        if !deb_info.architecture.is_empty() {
            println!("Arch: {}", deb_info.architecture);
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
        None => path.with_extension("deb.sig"),
    };
    let meta_path = sig_path.with_extension("sig.json");

    // Write signature file
    fs::write(&sig_path, &signature_bytes)
        .with_context(|| format!("Failed to write signature: {}", sig_path.display()))?;

    // Write metadata
    let metadata = DebSignatureMetadata {
        version: 1,
        format: "deb",
        algorithm: config.algorithm.clone(),
        key_id,
        digest_algorithm: digest_alg.name().to_string(),
        file_digest: digest_hex,
        file_name,
        file_size: file_meta.len(),
        signed_at: utc_now(),
        deb_package: if deb_info.package.is_empty() {
            None
        } else {
            Some(deb_info.package)
        },
        deb_version: if deb_info.version.is_empty() {
            None
        } else {
            Some(deb_info.version)
        },
        deb_architecture: if deb_info.architecture.is_empty() {
            None
        } else {
            Some(deb_info.architecture)
        },
    };

    let meta_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize signature metadata")?;
    fs::write(&meta_path, &meta_json)
        .with_context(|| format!("Failed to write metadata: {}", meta_path.display()))?;

    println!("Signed (DEB detached): {}", path.display());
    println!("  Signature: {}", sig_path.display());
    println!("  Metadata:  {}", meta_path.display());

    Ok(())
}

/// Dry-run for DEB signing
pub fn dry_run_deb(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path.display()))?;

    let deb_info = extract_deb_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);
    let digest = file_digest(path, digest_alg).context("Failed to compute file digest")?;
    let sig_path = path.with_extension("deb.sig");

    println!("[DRY RUN] Would sign (DEB detached): {}", path.display());
    println!("  File size:  {} bytes", file_meta.len());
    println!("  Format:     DEB Package");
    if !deb_info.package.is_empty() {
        println!("  Package:    {}", deb_info.package);
    }
    if !deb_info.version.is_empty() {
        println!("  Version:    {}", deb_info.version);
    }
    if !deb_info.architecture.is_empty() {
        println!("  Arch:       {}", deb_info.architecture);
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

// --- Native DEB signing ---

/// An AR archive member
struct ArMember {
    name: String,
    timestamp: u64,
    owner_id: u32,
    group_id: u32,
    mode: u32,
    data: Vec<u8>,
}

/// Parse all AR archive members from raw data.
fn parse_ar_members(data: &[u8]) -> Result<Vec<ArMember>> {
    if data.len() < 8 || &data[0..8] != AR_MAGIC {
        anyhow::bail!("Not an AR archive (bad magic)");
    }

    let mut members = Vec::new();
    let mut pos = 8;

    while pos + 60 <= data.len() {
        let name = String::from_utf8_lossy(&data[pos..pos + 16])
            .trim()
            .trim_end_matches('/')
            .to_string();

        let timestamp: u64 = String::from_utf8_lossy(&data[pos + 16..pos + 28])
            .trim()
            .parse()
            .unwrap_or(0);

        let owner_id: u32 = String::from_utf8_lossy(&data[pos + 28..pos + 34])
            .trim()
            .parse()
            .unwrap_or(0);

        let group_id: u32 = String::from_utf8_lossy(&data[pos + 34..pos + 40])
            .trim()
            .parse()
            .unwrap_or(0);

        let mode: u32 =
            u32::from_str_radix(String::from_utf8_lossy(&data[pos + 40..pos + 48]).trim(), 8)
                .unwrap_or(0o100644);

        let size: usize = String::from_utf8_lossy(&data[pos + 48..pos + 58])
            .trim()
            .parse()
            .context("Invalid AR member size")?;

        if &data[pos + 58..pos + 60] != b"`\n" {
            anyhow::bail!("Invalid AR member header end marker");
        }

        let member_start = pos + 60;
        let member_end = member_start + size;
        if member_end > data.len() {
            anyhow::bail!("AR member extends beyond file");
        }

        members.push(ArMember {
            name,
            timestamp,
            owner_id,
            group_id,
            mode,
            data: data[member_start..member_end].to_vec(),
        });

        // Next member (2-byte aligned)
        pos = member_end + (member_end % 2);
    }

    Ok(members)
}

/// Build an AR archive member entry (header + data + alignment padding).
fn build_ar_member(name: &str, data: &[u8], timestamp: u64) -> Vec<u8> {
    let mut header = Vec::with_capacity(60 + data.len() + 1);

    // Name: 16 bytes, "name/" left-justified, padded with spaces
    let name_with_slash = format!("{}/", name);
    let ar_name = format!("{:<16}", name_with_slash);
    header.extend_from_slice(ar_name.as_bytes());

    // Timestamp: 12 bytes
    header.extend_from_slice(format!("{:<12}", timestamp).as_bytes());

    // Owner ID: 6 bytes
    header.extend_from_slice(b"0     ");

    // Group ID: 6 bytes
    header.extend_from_slice(b"0     ");

    // File mode: 8 bytes (octal)
    header.extend_from_slice(b"100644  ");

    // Size: 10 bytes
    header.extend_from_slice(format!("{:<10}", data.len()).as_bytes());

    // End marker
    header.extend_from_slice(b"`\n");

    // Data
    header.extend_from_slice(data);

    // 2-byte alignment
    if !data.len().is_multiple_of(2) {
        header.push(b'\n');
    }

    header
}

/// Build dpkg-sig control text listing member hashes.
fn build_dpkg_sig_control(members: &[ArMember], signer: &str) -> String {
    let now_rfc2822 = chrono::Utc::now().to_rfc2822();

    let mut text = String::new();
    text.push_str("Version: 4\n");
    text.push_str(&format!("Signer: {}\n", signer));
    text.push_str(&format!("Date: {}\n", now_rfc2822));
    text.push_str("Role: builder\n");
    text.push_str("Files: \n");

    for member in members {
        let md5 = pgp::hex(&pgp::md5_digest(&member.data));
        let sha1 = pgp::hex(&pgp::sha1_digest(&member.data));
        text.push_str(&format!(
            "\t{} {} {} {}\n",
            md5,
            sha1,
            member.data.len(),
            member.name
        ));
    }

    text
}

/// Sign a DEB package with native dpkg-sig format.
pub fn sign_deb_native(
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

    let mut members = parse_ar_members(&data)
        .with_context(|| format!("Failed to parse DEB: {}", path.display()))?;

    // Remove any existing _gpgbuilder member
    members.retain(|m| !m.name.starts_with("_gpgbuilder"));

    let deb_info = extract_deb_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", data.len());
        println!("Format: DEB Package (native signing)");
        if !deb_info.package.is_empty() {
            println!("Package: {}", deb_info.package);
        }
        if !deb_info.version.is_empty() {
            println!("Version: {}", deb_info.version);
        }
        if !deb_info.architecture.is_empty() {
            println!("Arch: {}", deb_info.architecture);
        }
        println!("AR members: {}", members.len());
        println!("Algorithm: {}", config.algorithm);
        println!();
    }

    // Fetch certificate from KV
    if verbose {
        println!("Fetching certificate from Key Vault...");
    }
    let cert_der = kv
        .get_certificate(&config.cert_name, config.key_version.as_deref())
        .context("Failed to fetch certificate from Key Vault")?;

    let rsa_key = pgp::extract_rsa_pubkey(&cert_der)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract RSA public key from certificate"))?;

    let signer = pgp::extract_subject_cn(&cert_der).unwrap_or_else(|| "Unknown Signer".to_string());

    let creation_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as u32;

    let identity = pgp::compute_key_identity(&rsa_key, creation_time);

    if verbose {
        println!("Signer: {}", signer);
        println!("Key fingerprint: {}", pgp::hex(&identity.fingerprint));
        println!("Key ID: {}", pgp::hex(&identity.key_id));
        println!();
    }

    // Build dpkg-sig control text
    let control_text = build_dpkg_sig_control(&members, &signer);

    if verbose {
        println!("Computing PGP-contextualized hash over control text...");
    }

    let ctx = pgp::PgpSignatureContext {
        key: rsa_key,
        identity,
        hash_algorithm: digest_alg,
        creation_time,
    };

    let (digest, hash_prefix) = pgp::pgp_hash(control_text.as_bytes(), &ctx);

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

    // Build OpenPGP signature packet and ASCII armor it
    let pgp_packet = pgp::build_signature_packet(&ctx, &raw_sig, hash_prefix);
    let armored = pgp::ascii_armor_signature(&pgp_packet);

    // Build _gpgbuilder content: control text + armored signature
    let mut gpg_content = control_text;
    gpg_content.push_str(&armored);

    // Reconstruct the .deb: ar magic + all original members + _gpgbuilder
    let mut output_data = Vec::with_capacity(data.len() + gpg_content.len() + 200);
    output_data.extend_from_slice(AR_MAGIC);

    for member in &members {
        let mut member_header = Vec::with_capacity(60);
        let ar_name = format!("{:<15}/", member.name);
        member_header.extend_from_slice(ar_name.as_bytes());
        member_header.extend_from_slice(format!("{:<12}", member.timestamp).as_bytes());
        member_header.extend_from_slice(format!("{:<6}", member.owner_id).as_bytes());
        member_header.extend_from_slice(format!("{:<6}", member.group_id).as_bytes());
        member_header.extend_from_slice(format!("{:<8o}", member.mode).as_bytes());
        member_header.extend_from_slice(format!("{:<10}", member.data.len()).as_bytes());
        member_header.extend_from_slice(b"`\n");

        output_data.extend_from_slice(&member_header);
        output_data.extend_from_slice(&member.data);
        if member.data.len() % 2 != 0 {
            output_data.push(b'\n');
        }
    }

    // Add _gpgbuilder member
    let gpg_member = build_ar_member("_gpgbuilder", gpg_content.as_bytes(), creation_time as u64);
    output_data.extend_from_slice(&gpg_member);

    // Write output
    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => path.to_path_buf(),
    };
    fs::write(&output_path, &output_data)
        .with_context(|| format!("Failed to write signed DEB: {}", output_path.display()))?;

    println!("Signed (DEB native): {}", output_path.display());
    println!("  Verify with: dpkg-sig --verify {}", output_path.display());

    Ok(())
}

/// Dry-run for native DEB signing
pub fn dry_run_deb_native(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let data = fs::read(path).with_context(|| format!("Cannot read file: {}", path.display()))?;

    let members = parse_ar_members(&data)
        .with_context(|| format!("Failed to parse DEB: {}", path.display()))?;

    let has_existing_sig = members.iter().any(|m| m.name.starts_with("_gpgbuilder"));
    let deb_info = extract_deb_info(path).unwrap_or_else(|| info_from_filename(path));
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    println!("[DRY RUN] Would sign (DEB native): {}", path.display());
    println!("  File size:    {} bytes", data.len());
    println!("  Format:       DEB Package (dpkg-sig)");
    if !deb_info.package.is_empty() {
        println!("  Package:      {}", deb_info.package);
    }
    if !deb_info.version.is_empty() {
        println!("  Version:      {}", deb_info.version);
    }
    if !deb_info.architecture.is_empty() {
        println!("  Arch:         {}", deb_info.architecture);
    }
    println!("  Algorithm:    {}", config.algorithm);
    println!("  Digest:       {}", digest_alg.name());
    println!("  AR members:   {}", members.len());
    for member in &members {
        println!("    - {} ({} bytes)", member.name, member.data.len());
    }
    println!(
        "  Existing sig: {}",
        if has_existing_sig {
            "yes (will be replaced)"
        } else {
            "no"
        }
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
    fn test_parse_control_fields() {
        let control = "Package: my-app\nVersion: 1.2.3-1\nArchitecture: amd64\nDescription: test\n";
        let info = parse_control_fields(control);
        assert_eq!(info.package, "my-app");
        assert_eq!(info.version, "1.2.3-1");
        assert_eq!(info.architecture, "amd64");
    }

    #[test]
    fn test_info_from_filename() {
        let info = info_from_filename(Path::new("/tmp/myapp_1.0.0_amd64.deb"));
        assert_eq!(info.package, "myapp");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.architecture, "amd64");
    }

    #[test]
    fn test_info_from_filename_no_underscores() {
        let info = info_from_filename(Path::new("/tmp/some-package.deb"));
        assert_eq!(info.package, "some-package");
    }

    #[test]
    fn test_extract_deb_info_invalid() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.deb");
        fs::write(&path, b"not a deb file").unwrap();

        let info = extract_deb_info(&path);
        assert!(info.is_none());
    }

    #[test]
    fn test_extract_deb_info_valid_ar() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.deb");
        let mut f = fs::File::create(&path).unwrap();

        // Write AR magic
        f.write_all(AR_MAGIC).unwrap();

        // Write a debian-binary member (standard first member)
        let member_name = b"debian-binary   ";
        let timestamp = b"0           ";
        let owner = b"0     ";
        let group = b"0     ";
        let mode = b"100644  ";
        let size = b"4         ";
        let end = b"`\n";
        f.write_all(member_name).unwrap();
        f.write_all(timestamp).unwrap();
        f.write_all(owner).unwrap();
        f.write_all(group).unwrap();
        f.write_all(mode).unwrap();
        f.write_all(size).unwrap();
        f.write_all(end).unwrap();
        f.write_all(b"2.0\n").unwrap();
        drop(f);

        // AR is valid but no control.tar, so falls back to filename parsing
        let info = extract_deb_info(&path);
        assert!(info.is_some());
    }

    /// Build a minimal valid DEB (AR archive) for testing native signing.
    fn build_test_deb() -> Vec<u8> {
        let mut deb = Vec::new();
        deb.extend_from_slice(AR_MAGIC);

        // debian-binary member
        let db_data = b"2.0\n";
        deb.extend_from_slice(&build_ar_member("debian-binary", db_data, 1700000000));

        // control.tar (empty, just tar headers would go here)
        let ctrl_data = b"fake control tar data";
        deb.extend_from_slice(&build_ar_member("control.tar", ctrl_data, 1700000000));

        // data.tar (empty)
        let data_data = b"fake data tar content";
        deb.extend_from_slice(&build_ar_member("data.tar", data_data, 1700000000));

        deb
    }

    #[test]
    fn test_parse_ar_members() {
        let deb = build_test_deb();
        let members = parse_ar_members(&deb).unwrap();
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].name, "debian-binary");
        assert_eq!(members[1].name, "control.tar");
        assert_eq!(members[2].name, "data.tar");
    }

    #[test]
    fn test_parse_ar_members_invalid() {
        assert!(parse_ar_members(b"not an ar").is_err());
        assert!(parse_ar_members(b"").is_err());
    }

    #[test]
    fn test_build_ar_member() {
        let data = b"hello world";
        let member = build_ar_member("test.txt", data, 1700000000);

        // Header should be 60 bytes
        assert!(member.len() >= 60 + data.len());

        // Name field
        let name = String::from_utf8_lossy(&member[0..16]);
        assert!(name.starts_with("test.txt"));

        // End marker
        assert_eq!(&member[58..60], b"`\n");

        // Data
        assert_eq!(&member[60..60 + data.len()], data);

        // Odd length should have padding
        assert_eq!(member.len(), 60 + data.len() + 1); // 11 bytes + 1 pad
    }

    #[test]
    fn test_build_dpkg_sig_control() {
        let members = vec![
            ArMember {
                name: "debian-binary".to_string(),
                timestamp: 0,
                owner_id: 0,
                group_id: 0,
                mode: 0o100644,
                data: b"2.0\n".to_vec(),
            },
            ArMember {
                name: "control.tar.gz".to_string(),
                timestamp: 0,
                owner_id: 0,
                group_id: 0,
                mode: 0o100644,
                data: vec![0xAA; 100],
            },
        ];

        let text = build_dpkg_sig_control(&members, "Test Signer");
        assert!(text.starts_with("Version: 4\n"));
        assert!(text.contains("Signer: Test Signer\n"));
        assert!(text.contains("Role: builder\n"));
        assert!(text.contains("debian-binary\n"));
        assert!(text.contains("control.tar.gz\n"));
    }

    #[test]
    fn test_dry_run_deb_native() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.deb");
        let deb = build_test_deb();
        fs::write(&path, &deb).unwrap();

        let config = SigningConfig {
            vault_url: "https://test.vault.azure.net".to_string(),
            cert_name: "test-cert".to_string(),
            algorithm: "RS256".to_string(),
            ..Default::default()
        };

        let result = dry_run_deb_native(&path, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_roundtrip_ar_parse_rebuild() {
        let original = build_test_deb();
        let members = parse_ar_members(&original).unwrap();

        // Rebuild from parsed members
        let mut rebuilt = Vec::new();
        rebuilt.extend_from_slice(AR_MAGIC);
        for member in &members {
            rebuilt.extend_from_slice(&build_ar_member(
                &member.name,
                &member.data,
                member.timestamp,
            ));
        }

        // Re-parse the rebuilt archive
        let reparsed = parse_ar_members(&rebuilt).unwrap();
        assert_eq!(reparsed.len(), members.len());
        for (orig, re) in members.iter().zip(reparsed.iter()) {
            assert_eq!(orig.name, re.name);
            assert_eq!(orig.data, re.data);
        }
    }
}
