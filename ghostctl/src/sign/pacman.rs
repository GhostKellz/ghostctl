// Arch Linux package signing via Azure Key Vault
//
// Produces detached OpenPGP binary signatures (.sig files) compatible
// with pacman's signature verification. The .sig file is a raw OpenPGP
// v4 signature packet -- the same format `gpg --detach-sign` produces.
//
// Verification: pacman-key --verify <package.pkg.tar.zst> <package.pkg.tar.zst.sig>
// Or implicitly by pacman when SigLevel is set in pacman.conf.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::config::{SigningConfig, pgp_key_created_at};
use super::hash::{DigestAlgorithm, file_digest, hex_digest};
use super::keyvault::KeyVaultClient;
use super::pgp;

/// Sign an Arch Linux package with a detached OpenPGP signature.
///
/// Produces a `.sig` file containing a raw OpenPGP v4 binary signature
/// packet, compatible with pacman's signature verification.
pub fn sign_pacman(
    path: &Path,
    kv: &mut KeyVaultClient,
    config: &SigningConfig,
    output: Option<&Path>,
    verbose: bool,
) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta =
        fs::metadata(path).with_context(|| format!("Cannot read metadata: {}", path.display()))?;

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);

    if verbose {
        println!("File: {}", path.display());
        println!("Size: {} bytes", file_meta.len());
        println!("Format: Arch Linux Package");
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

    // Read the entire file for PGP hashing
    let file_data =
        fs::read(path).with_context(|| format!("Cannot read file: {}", path.display()))?;

    let ctx = pgp::PgpSignatureContext {
        key: rsa_key,
        identity,
        hash_algorithm: digest_alg,
        creation_time,
    };

    // Compute PGP-contextualized hash
    if verbose {
        println!("Computing PGP-contextualized hash...");
    }
    let (digest, hash_prefix) = pgp::pgp_hash(&file_data, &ctx);

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

    // Build OpenPGP signature packet (raw binary, not ASCII armored)
    let pgp_packet = pgp::build_signature_packet(&ctx, &raw_sig, hash_prefix);

    if verbose {
        println!("OpenPGP signature packet: {} bytes", pgp_packet.len());
    }

    // Write .sig file
    let sig_path = match output {
        Some(p) => p.to_path_buf(),
        None => {
            let mut sig = path.as_os_str().to_os_string();
            sig.push(".sig");
            sig.into()
        }
    };

    fs::write(&sig_path, &pgp_packet)
        .with_context(|| format!("Failed to write signature: {}", sig_path.display()))?;

    println!("Signed (Arch package): {}", path.display());
    println!("  Signature: {}", sig_path.display());
    println!(
        "  Verify:    pacman-key --verify {} {}",
        path.display(),
        sig_path.display()
    );

    Ok(())
}

/// Dry-run for Arch package signing
pub fn dry_run_pacman(path: &Path, config: &SigningConfig) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let file_meta =
        fs::metadata(path).with_context(|| format!("Cannot read metadata: {}", path.display()))?;

    let digest_alg = DigestAlgorithm::from_sign_algorithm(&config.algorithm);
    let digest = file_digest(path, digest_alg).context("Failed to compute file digest")?;

    let mut sig_path = path.as_os_str().to_os_string();
    sig_path.push(".sig");

    println!("[DRY RUN] Would sign (Arch package): {}", path.display());
    println!("  File size:  {} bytes", file_meta.len());
    println!("  Format:     Arch Linux Package");
    println!("  Algorithm:  {}", config.algorithm);
    println!(
        "  Digest:     {} = {}",
        digest_alg.name(),
        hex_digest(&digest)
    );
    println!("  Vault:      {}", config.vault_url);
    println!("  Key:        {}", config.cert_name);
    println!("  Output:     {}", sig_path.to_string_lossy());
    println!();
    println!("No Key Vault calls were made.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dry_run_pacman() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-1.0-1-x86_64.pkg.tar.zst");
        fs::write(&path, b"fake arch package content").unwrap();

        let config = SigningConfig {
            vault_url: "https://test.vault.azure.net".to_string(),
            cert_name: "test-cert".to_string(),
            algorithm: "RS256".to_string(),
            ..Default::default()
        };

        let result = dry_run_pacman(&path, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dry_run_pacman_db() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("myrepo.db.tar.gz");
        fs::write(&path, b"fake repo database content").unwrap();

        let config = SigningConfig {
            vault_url: "https://test.vault.azure.net".to_string(),
            cert_name: "test-cert".to_string(),
            algorithm: "RS256".to_string(),
            ..Default::default()
        };

        let result = dry_run_pacman(&path, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sig_path_construction() {
        let path = Path::new("/tmp/test-1.0-1-x86_64.pkg.tar.zst");
        let mut sig_path = path.as_os_str().to_os_string();
        sig_path.push(".sig");
        assert_eq!(
            sig_path.to_string_lossy(),
            "/tmp/test-1.0-1-x86_64.pkg.tar.zst.sig"
        );
    }
}
