use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

use super::config::SigningConfig;
use super::hash::{DigestAlgorithm, file_digest, hex_digest};
use super::keyvault::KeyVaultClient;

/// Metadata written alongside a detached signature
#[derive(Serialize)]
struct SignatureMetadata {
    /// Metadata format version
    version: u32,
    /// Signing algorithm used (e.g. RS256)
    algorithm: String,
    /// Azure Key Vault key identifier
    key_id: String,
    /// Hash algorithm used for the digest
    digest_algorithm: String,
    /// Hex-encoded digest of the original file
    file_digest: String,
    /// Original file name
    file_name: String,
    /// File size in bytes
    file_size: u64,
    /// UTC timestamp of signing
    signed_at: String,
}

/// Sign a file and produce a detached signature (.sig) and metadata (.sig.json)
pub fn sign_generic(
    path: &Path,
    kv: &mut KeyVaultClient,
    config: &SigningConfig,
    output: Option<&Path>,
    verbose: bool,
) -> Result<()> {
    // Validate file exists
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path.display()))?;

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Determine digest algorithm from signing algorithm
    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", file_meta.len());
        println!("Algorithm: {}", config.algorithm);
        println!("Digest: {}", digest_alg.name());
        println!("Vault: {}", config.vault_url);
        println!("Key: {}", config.cert_name);
        println!();
    }

    // Compute file digest
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
        println!("Key ID: {}", key_id);
    }

    // Determine output paths
    let sig_path = match output {
        Some(p) => p.to_path_buf(),
        None => path.with_extension(format!(
            "{}.sig",
            path.extension().and_then(|e| e.to_str()).unwrap_or("bin")
        )),
    };
    let meta_path = sig_path.with_extension("sig.json");

    // Write signature file
    fs::write(&sig_path, &signature_bytes)
        .with_context(|| format!("Failed to write signature: {}", sig_path.display()))?;

    // Write metadata
    let now = chrono_now_utc();
    let metadata = SignatureMetadata {
        version: 1,
        algorithm: config.algorithm.clone(),
        key_id,
        digest_algorithm: digest_alg.name().to_string(),
        file_digest: digest_hex,
        file_name,
        file_size: file_meta.len(),
        signed_at: now,
    };

    let meta_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize signature metadata")?;
    fs::write(&meta_path, &meta_json)
        .with_context(|| format!("Failed to write metadata: {}", meta_path.display()))?;

    println!("Signed: {}", path.display());
    println!("  Signature: {}", sig_path.display());
    println!("  Metadata:  {}", meta_path.display());

    Ok(())
}

/// Dry-run: show what would be signed without calling Key Vault
pub fn dry_run_generic(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path.display()))?;

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);
    let digest = file_digest(path, digest_alg).context("Failed to compute file digest")?;
    let digest_hex = hex_digest(&digest);

    let sig_path = path.with_extension(format!(
        "{}.sig",
        path.extension().and_then(|e| e.to_str()).unwrap_or("bin")
    ));

    println!("[DRY RUN] Would sign: {}", path.display());
    println!("  File size:  {} bytes", file_meta.len());
    println!("  Algorithm:  {}", config.algorithm);
    println!("  Digest:     {} = {}", digest_alg.name(), digest_hex);
    println!("  Vault:      {}", config.vault_url);
    println!("  Key:        {}", config.cert_name);
    println!("  Output:     {}", sig_path.display());
    println!();
    println!("No Key Vault calls were made.");

    Ok(())
}

/// Get current UTC time as ISO 8601 string without external chrono dependency.
/// Format: 2026-05-24T12:00:00Z
fn chrono_now_utc() -> String {
    use std::process::Command;

    // Use `date` command for UTC timestamp
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
