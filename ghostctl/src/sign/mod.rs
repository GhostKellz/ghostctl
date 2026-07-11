// Azure Key Vault code signing [EXPERIMENTAL]
//
// Signs files using Azure Key Vault-backed keys. The private key never
// leaves Key Vault -- files are hashed locally and the digest is sent
// to Key Vault for signing via REST API.

pub mod asn1;
pub mod auth;
pub mod config;
pub mod deb;
pub mod errors;
pub mod generic;
pub mod hash;
pub mod keyvault;
pub mod pacman;
pub mod pe;
pub mod pgp;
pub mod rpm;
pub mod timestamp;

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::io::Write;
use std::path::Path;

use auth::AuthMethod;
use config::{SigningConfig, pgp_key_created_at, validate_name};
use hash::FileFormat;
use keyvault::KeyVaultClient;

pub fn command() -> Command {
    Command::new("sign")
        .about("[EXPERIMENTAL] Code signing via Azure Key Vault")
        .long_about(
            "Sign files using Azure Key Vault-backed keys.\n\
             The private key never leaves Key Vault -- files are hashed locally\n\
             and the hash is sent to Key Vault for signing.\n\n\
             Supports: generic files (detached .sig), Windows PE, RPM, DEB, and Arch Linux packages.\n\n\
             This feature is experimental and may change in future releases.",
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("file")
                .about("Sign a file using Azure Key Vault")
                .arg(Arg::new("FILE").required(true).help("File to sign"))
                .arg(
                    Arg::new("format")
                        .long("format")
                        .value_parser(["auto", "generic", "pe", "rpm", "deb", "pacman"])
                        .default_value("auto")
                        .help("Signing format (auto-detect by default)"),
                )
                .arg(
                    Arg::new("vault-url")
                        .long("vault-url")
                        .value_name("URL")
                        .help("Azure Key Vault URL (overrides config)"),
                )
                .arg(
                    Arg::new("cert-name")
                        .long("cert-name")
                        .value_name("NAME")
                        .help("Key/certificate name in Key Vault (overrides config)"),
                )
                .arg(
                    Arg::new("algorithm")
                        .long("algorithm")
                        .short('a')
                        .value_parser(["RS256", "RS384", "RS512", "ES256", "ES384", "ES512"])
                        .help("Signing algorithm (default: from config or RS256)"),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .value_name("PATH")
                        .help("Output path for signature file"),
                )
                .arg(
                    Arg::new("auth")
                        .long("auth")
                        .value_parser(["cli", "sp"])
                        .help("Authentication method override"),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Show what would be signed without calling Key Vault"),
                )
                .arg(
                    Arg::new("timestamp")
                        .long("timestamp")
                        .action(ArgAction::SetTrue)
                        .help("Request RFC 3161 timestamp (default for PE if TSA URL configured)"),
                )
                .arg(
                    Arg::new("no-timestamp")
                        .long("no-timestamp")
                        .action(ArgAction::SetTrue)
                        .help("Disable timestamping"),
                )
                .arg(
                    Arg::new("native")
                        .long("native")
                        .action(ArgAction::SetTrue)
                        .help("Use native package signing (RPM: embed in header, DEB: dpkg-sig format)"),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .action(ArgAction::SetTrue)
                        .help("Verbose output"),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Show or initialize signing configuration")
                .arg(
                    Arg::new("init")
                        .long("init")
                        .action(ArgAction::SetTrue)
                        .help("Interactive signing configuration setup"),
                ),
        )
        .subcommand(
            Command::new("status").about("Check signing dependencies and Azure connectivity"),
        )
        .subcommand(
            Command::new("export-key")
                .about("Export the signing public key from Azure Key Vault")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .value_parser(["pgp", "pem", "der"])
                        .default_value("pgp")
                        .help("Export format: pgp (ASCII-armored OpenPGP), pem (X.509 PEM), der (raw DER)"),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .value_name("PATH")
                        .help("Output file (stdout if omitted)"),
                )
                .arg(
                    Arg::new("vault-url")
                        .long("vault-url")
                        .value_name("URL")
                        .help("Azure Key Vault URL (overrides config)"),
                )
                .arg(
                    Arg::new("cert-name")
                        .long("cert-name")
                        .value_name("NAME")
                        .help("Key/certificate name in Key Vault (overrides config)"),
                )
                .arg(
                    Arg::new("auth")
                        .long("auth")
                        .value_parser(["cli", "sp"])
                        .help("Authentication method override"),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify a file signature against Azure Key Vault certificate")
                .arg(Arg::new("FILE").required(true).help("File to verify"))
                .arg(
                    Arg::new("signature")
                        .long("signature")
                        .short('s')
                        .value_name("PATH")
                        .help("Signature file path (default: FILE.sig)"),
                )
                .arg(
                    Arg::new("vault-url")
                        .long("vault-url")
                        .value_name("URL")
                        .help("Azure Key Vault URL (overrides config)"),
                )
                .arg(
                    Arg::new("cert-name")
                        .long("cert-name")
                        .value_name("NAME")
                        .help("Key/certificate name in Key Vault (overrides config)"),
                )
                .arg(
                    Arg::new("auth")
                        .long("auth")
                        .value_parser(["cli", "sp"])
                        .help("Authentication method override"),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .action(ArgAction::SetTrue)
                        .help("Verbose output"),
                ),
        )
        .subcommand(
            Command::new("list-keys")
                .about("List signing certificates in Azure Key Vault")
                .arg(
                    Arg::new("vault-url")
                        .long("vault-url")
                        .value_name("URL")
                        .help("Azure Key Vault URL (overrides config)"),
                )
                .arg(
                    Arg::new("auth")
                        .long("auth")
                        .value_parser(["cli", "sp"])
                        .help("Authentication method override"),
                ),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("file", sub)) => handle_sign_file(sub),
        Some(("config", sub)) => handle_config(sub),
        Some(("status", _)) => handle_status(),
        Some(("export-key", sub)) => handle_export_key(sub),
        Some(("verify", sub)) => handle_verify(sub),
        Some(("list-keys", sub)) => handle_list_keys(sub),
        _ => unreachable!(),
    }
}

fn handle_sign_file(matches: &ArgMatches) -> Result<()> {
    let file_path = matches.get_one::<String>("FILE").unwrap();
    let path = Path::new(file_path);

    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    // Build effective config from stored config + CLI overrides
    let cfg = load_effective_config(matches)?;

    // Validate config
    if cfg.vault_url.is_empty() {
        anyhow::bail!(
            "No vault URL configured. Run 'ghostctl sign config --init' or pass --vault-url"
        );
    }
    if cfg.cert_name.is_empty() {
        anyhow::bail!(
            "No certificate name configured. Run 'ghostctl sign config --init' or pass --cert-name"
        );
    }
    if !validate_name(&cfg.cert_name) {
        anyhow::bail!(
            "Invalid certificate name '{}'. Must be 1-127 alphanumeric/hyphen characters.",
            cfg.cert_name
        );
    }

    let verbose = matches.get_flag("verbose");
    let dry_run = matches.get_flag("dry-run");
    let native = matches.get_flag("native");

    // Detect file format
    let format_override = matches.get_one::<String>("format").map(|s| s.as_str());
    let format = match format_override {
        Some("generic") => FileFormat::Generic,
        Some("pe") => FileFormat::Pe,
        Some("rpm") => FileFormat::Rpm,
        Some("deb") => FileFormat::Deb,
        Some("pacman") => FileFormat::Pacman,
        _ => FileFormat::detect(path).context("Failed to detect file format")?,
    };

    if verbose {
        println!("Detected format: {}", format.name());
    }

    // Determine timestamp behavior
    let explicit_timestamp = matches.get_flag("timestamp");
    let use_timestamp = if matches.get_flag("no-timestamp") {
        false
    } else if explicit_timestamp {
        true
    } else {
        // Default: timestamp PE files if TSA URL is configured
        format == FileFormat::Pe && !cfg.tsa_url.is_empty()
    };

    let output = matches
        .get_one::<String>("output")
        .map(|s| Path::new(s.as_str()));

    validate_algorithm_for_format(format, &cfg)?;

    // Dry run: show format-specific info
    if dry_run {
        return match format {
            FileFormat::Pe => pe::dry_run_pe(path, &cfg),
            FileFormat::Rpm if native => rpm::dry_run_rpm_native(path, &cfg),
            FileFormat::Deb if native => deb::dry_run_deb_native(path, &cfg),
            FileFormat::Rpm => rpm::dry_run_rpm(path, &cfg),
            FileFormat::Deb => deb::dry_run_deb(path, &cfg),
            FileFormat::Pacman => pacman::dry_run_pacman(path, &cfg),
            FileFormat::Generic => generic::dry_run_generic(path, &cfg),
        };
    }

    // Authenticate and sign
    let auth = AuthMethod::from_config(&cfg).context("Failed to configure authentication")?;
    let mut kv = KeyVaultClient::new(&cfg.vault_url, auth)
        .context("Failed to connect to Azure Key Vault")?;

    match format {
        FileFormat::Pe => pe::sign_pe(
            path,
            &mut kv,
            &cfg,
            output,
            verbose,
            use_timestamp,
            explicit_timestamp,
        ),
        FileFormat::Rpm if native => rpm::sign_rpm_native(path, &mut kv, &cfg, output, verbose),
        FileFormat::Deb if native => deb::sign_deb_native(path, &mut kv, &cfg, output, verbose),
        FileFormat::Rpm => rpm::sign_rpm(path, &mut kv, &cfg, output, verbose),
        FileFormat::Deb => deb::sign_deb(path, &mut kv, &cfg, output, verbose),
        FileFormat::Pacman => pacman::sign_pacman(path, &mut kv, &cfg, output, verbose),
        FileFormat::Generic => generic::sign_generic(path, &mut kv, &cfg, output, verbose),
    }
}

fn validate_algorithm_for_format(format: FileFormat, cfg: &SigningConfig) -> Result<()> {
    let is_ec = matches!(cfg.algorithm.as_str(), "ES256" | "ES384" | "ES512");
    if is_ec && format != FileFormat::Generic {
        anyhow::bail!(
            "{} signing currently requires an RSA Key Vault key. Use RS256, RS384, or RS512.",
            format.name()
        );
    }

    Ok(())
}

fn handle_config(matches: &ArgMatches) -> Result<()> {
    if matches.get_flag("init") {
        return config_init();
    }

    // Show current config
    let ghost_cfg = crate::config::GhostConfig::load();
    match &ghost_cfg.signing {
        Some(cfg) => {
            println!("Signing Configuration");
            println!("=====================");
            println!("Vault URL:    {}", cfg.vault_url);
            println!("Cert/Key:     {}", cfg.cert_name);
            println!(
                "Key Version:  {}",
                cfg.key_version.as_deref().unwrap_or("(latest)")
            );
            println!("Algorithm:    {}", cfg.algorithm);
            println!("Auth Method:  {}", cfg.auth_method);
            println!("TSA URL:      {}", cfg.tsa_url);
            if let Some(tid) = &cfg.tenant_id {
                println!("Tenant ID:    {}", tid);
            }
            if let Some(cid) = &cfg.client_id {
                println!("Client ID:    {}", cid);
            }
            println!();
            println!(
                "Config file: {}",
                crate::config::GhostConfig::config_path().display()
            );
        }
        None => {
            println!("No signing configuration found.");
            println!("Run 'ghostctl sign config --init' to set up signing.");
        }
    }

    Ok(())
}

fn config_init() -> Result<()> {
    println!("Signing Configuration Setup");
    println!("===========================");
    println!();

    let vault_url: String = Input::new()
        .with_prompt("Azure Key Vault URL")
        .with_initial_text("https://")
        .interact_text()
        .context("Input cancelled")?;

    let cert_name: String = Input::new()
        .with_prompt("Certificate/Key name in Key Vault")
        .interact_text()
        .context("Input cancelled")?;

    if !validate_name(&cert_name) {
        anyhow::bail!("Invalid name. Must be 1-127 alphanumeric/hyphen characters.");
    }

    let algorithm_options = ["RS256", "RS384", "RS512", "ES256", "ES384", "ES512"];
    let alg_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Signing algorithm")
        .items(&algorithm_options)
        .default(0)
        .interact()
        .context("Selection cancelled")?;
    let algorithm = algorithm_options[alg_idx].to_string();

    let auth_options = [
        "Azure CLI (az login session)",
        "Service Principal (client credentials)",
    ];
    let auth_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Authentication method")
        .items(&auth_options)
        .default(0)
        .interact()
        .context("Selection cancelled")?;

    let auth_method = if auth_idx == 0 {
        "cli".to_string()
    } else {
        "service_principal".to_string()
    };

    let mut tenant_id = None;
    let mut client_id = None;

    if auth_method == "service_principal" {
        let tid: String = Input::new()
            .with_prompt("Azure Tenant ID")
            .interact_text()
            .context("Input cancelled")?;
        tenant_id = Some(tid);

        let cid: String = Input::new()
            .with_prompt("Azure Client ID (Application ID)")
            .interact_text()
            .context("Input cancelled")?;
        client_id = Some(cid);

        println!();
        println!("Note: Client secret should be set via AZURE_CLIENT_SECRET environment variable.");
        println!("It is never stored in the config file.");
    }

    let signing_config = SigningConfig {
        vault_url,
        cert_name,
        key_version: None,
        tenant_id,
        client_id,
        algorithm,
        tsa_url: "http://timestamp.digicert.com".to_string(),
        pgp_key_created_at: Some(0),
        auth_method,
    };

    // Load and update config
    let mut ghost_cfg = crate::config::GhostConfig::load();
    ghost_cfg.signing = Some(signing_config);

    ghost_cfg
        .save()
        .map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))?;

    println!();
    println!("Signing configuration saved.");
    println!(
        "Config file: {}",
        crate::config::GhostConfig::config_path().display()
    );

    Ok(())
}

fn handle_status() -> Result<()> {
    println!("Signing Status");
    println!("==============");
    println!();

    // Check Azure CLI
    let az_installed = auth::check_az_cli().unwrap_or(false);
    let status_char = if az_installed { "+" } else { "-" };
    println!("{} Azure CLI installed", status_char);

    if az_installed {
        let az_session = auth::check_az_session().unwrap_or(false);
        let status_char = if az_session { "+" } else { "-" };
        println!("{} Azure CLI session active", status_char);
        if !az_session {
            println!("  Run: az login");
        }
    } else {
        println!("  Install: https://docs.microsoft.com/en-us/cli/azure/install-azure-cli");
    }

    // Check config
    let ghost_cfg = crate::config::GhostConfig::load();
    match &ghost_cfg.signing {
        Some(cfg) => {
            println!("+ Signing config present");
            println!("  Vault: {}", cfg.vault_url);
            println!("  Key:   {}", cfg.cert_name);
            println!("  Auth:  {}", cfg.auth_method);

            // Try to connect to vault and check key
            if az_installed || cfg.auth_method == "service_principal" {
                print!("  Checking vault connectivity... ");
                match check_vault_key(cfg) {
                    Ok(info) => {
                        println!("OK");
                        println!("  Key type: {}", info.key_type);
                        println!("  Enabled:  {}", if info.enabled { "yes" } else { "NO" });
                    }
                    Err(e) => {
                        println!("FAILED");
                        println!("  Error: {}", e);
                    }
                }
            }
        }
        None => {
            println!("- No signing configuration");
            println!("  Run: ghostctl sign config --init");
        }
    }

    // Check external tools
    println!();
    let osslsigncode = which_exists("osslsigncode");
    let status_char = if osslsigncode { "+" } else { "-" };
    println!("{} osslsigncode (PE signature verification)", status_char);

    let dpkg_sig = which_exists("dpkg-sig");
    let status_char = if dpkg_sig { "+" } else { "-" };
    println!("{} dpkg-sig (DEB signing)", status_char);

    let rpmsign = which_exists("rpmsign");
    let status_char = if rpmsign { "+" } else { "-" };
    println!("{} rpmsign (RPM signing)", status_char);

    Ok(())
}

/// Check if a command exists on PATH
fn which_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

/// Try to connect to Key Vault and check the configured key
fn check_vault_key(cfg: &SigningConfig) -> Result<keyvault::KeyInfo> {
    let auth = AuthMethod::from_config(cfg)?;
    let mut kv = KeyVaultClient::new(&cfg.vault_url, auth)?;
    kv.check_key(&cfg.cert_name)
}

/// Build an effective SigningConfig from stored config + CLI overrides
fn load_effective_config(matches: &ArgMatches) -> Result<SigningConfig> {
    let ghost_cfg = crate::config::GhostConfig::load();
    let mut cfg = ghost_cfg.signing.unwrap_or_default();

    // Apply CLI overrides
    if let Some(url) = matches.get_one::<String>("vault-url") {
        cfg.vault_url = url.clone();
    }
    if let Some(name) = matches.get_one::<String>("cert-name") {
        cfg.cert_name = name.clone();
    }
    if let Some(alg) = matches.get_one::<String>("algorithm") {
        cfg.algorithm = alg.clone();
    }
    if let Some(auth) = matches.get_one::<String>("auth") {
        cfg.auth_method = match auth.as_str() {
            "sp" => "service_principal".to_string(),
            other => other.to_string(),
        };
    }

    Ok(cfg)
}

/// Build a minimal config from subcommand matches that only have vault-url + auth
fn load_minimal_config(matches: &ArgMatches) -> Result<SigningConfig> {
    let ghost_cfg = crate::config::GhostConfig::load();
    let mut cfg = ghost_cfg.signing.unwrap_or_default();

    if let Some(url) = matches.get_one::<String>("vault-url") {
        cfg.vault_url = url.clone();
    }
    if let Some(name) = matches.get_one::<String>("cert-name") {
        cfg.cert_name = name.clone();
    }
    if let Some(auth) = matches.get_one::<String>("auth") {
        cfg.auth_method = match auth.as_str() {
            "sp" => "service_principal".to_string(),
            other => other.to_string(),
        };
    }

    Ok(cfg)
}

fn handle_export_key(matches: &ArgMatches) -> Result<()> {
    let cfg = load_minimal_config(matches)?;

    if cfg.vault_url.is_empty() {
        anyhow::bail!(
            "No vault URL configured. Run 'ghostctl sign config --init' or pass --vault-url"
        );
    }
    if cfg.cert_name.is_empty() {
        anyhow::bail!(
            "No certificate name configured. Run 'ghostctl sign config --init' or pass --cert-name"
        );
    }

    let format = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("pgp");
    let output_path = matches.get_one::<String>("output");

    // Authenticate and fetch certificate
    let auth = AuthMethod::from_config(&cfg).context("Failed to configure authentication")?;
    let mut kv =
        KeyVaultClient::new(&cfg.vault_url, auth).context("Failed to connect to Key Vault")?;
    let cert_der = kv
        .get_certificate(&cfg.cert_name, cfg.key_version.as_deref())
        .context("Failed to fetch certificate from Key Vault")?;

    let output_bytes: Vec<u8> = match format {
        "pgp" => {
            let key = pgp::extract_rsa_pubkey(&cert_der).ok_or_else(|| {
                anyhow::anyhow!("Failed to extract RSA public key from certificate")
            })?;
            let creation_time = pgp_key_created_at(&cfg);
            let identity = pgp::compute_key_identity(&key, creation_time);

            // Print metadata to stderr so piped output stays clean
            let cn = pgp::extract_subject_cn(&cert_der).unwrap_or_default();
            eprintln!("Subject CN:  {}", cn);
            eprintln!("Fingerprint: {}", pgp::hex(&identity.fingerprint));
            eprintln!("Key ID:      {}", pgp::hex(&identity.key_id).to_uppercase());
            eprintln!("Key size:    {}-bit RSA", key.modulus.len() * 8);

            let armored = pgp::ascii_armor_public_key(&key, creation_time);
            armored.into_bytes()
        }
        "pem" => {
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&cert_der);
            let mut pem = String::from("-----BEGIN CERTIFICATE-----\n");
            for chunk in b64.as_bytes().chunks(64) {
                pem.push_str(&String::from_utf8_lossy(chunk));
                pem.push('\n');
            }
            pem.push_str("-----END CERTIFICATE-----\n");

            let cn = pgp::extract_subject_cn(&cert_der).unwrap_or_default();
            eprintln!("Subject CN: {}", cn);
            eprintln!("DER size:   {} bytes", cert_der.len());

            pem.into_bytes()
        }
        "der" => {
            let cn = pgp::extract_subject_cn(&cert_der).unwrap_or_default();
            eprintln!("Subject CN: {}", cn);
            eprintln!("DER size:   {} bytes", cert_der.len());

            cert_der
        }
        _ => unreachable!(),
    };

    match output_path {
        Some(path) => {
            std::fs::write(path, &output_bytes)
                .with_context(|| format!("Failed to write to {}", path))?;
            eprintln!("Wrote {} bytes to {}", output_bytes.len(), path);
        }
        None => {
            std::io::stdout()
                .write_all(&output_bytes)
                .context("Failed to write to stdout")?;
        }
    }

    Ok(())
}

fn handle_verify(matches: &ArgMatches) -> Result<()> {
    let file_path = matches.get_one::<String>("FILE").unwrap();
    let path = Path::new(file_path);

    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let sig_path_str = matches.get_one::<String>("signature").cloned();
    let sig_path = match &sig_path_str {
        Some(p) => std::path::PathBuf::from(p),
        None => {
            let mut p = path.as_os_str().to_os_string();
            p.push(".sig");
            std::path::PathBuf::from(p)
        }
    };

    if !sig_path.exists() {
        anyhow::bail!(
            "Signature file not found: {}\nSpecify with --signature or place at FILE.sig",
            sig_path.display()
        );
    }

    let verbose = matches.get_flag("verbose");
    let cfg = load_minimal_config(matches)?;

    if cfg.vault_url.is_empty() {
        anyhow::bail!(
            "No vault URL configured. Run 'ghostctl sign config --init' or pass --vault-url"
        );
    }
    if cfg.cert_name.is_empty() {
        anyhow::bail!(
            "No certificate name configured. Run 'ghostctl sign config --init' or pass --cert-name"
        );
    }

    // Read file and signature
    let file_data =
        std::fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;
    let sig_data = std::fs::read(&sig_path)
        .with_context(|| format!("Failed to read signature: {}", sig_path.display()))?;

    // Authenticate and fetch certificate for verification
    let auth = AuthMethod::from_config(&cfg).context("Failed to configure authentication")?;
    let mut kv =
        KeyVaultClient::new(&cfg.vault_url, auth).context("Failed to connect to Key Vault")?;
    let cert_der = kv
        .get_certificate(&cfg.cert_name, cfg.key_version.as_deref())
        .context("Failed to fetch certificate from Key Vault")?;

    let key = pgp::extract_rsa_pubkey(&cert_der)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract RSA public key from certificate"))?;

    // Parse signature to extract metadata
    let parsed = pgp::parse_signature_packet(&sig_data)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse signature packet"))?;

    if verbose {
        let cn = pgp::extract_subject_cn(&cert_der).unwrap_or_default();
        println!("File:        {}", path.display());
        println!("Signature:   {}", sig_path.display());
        println!("Subject CN:  {}", cn);
        println!("Key ID:      {}", pgp::hex(&parsed.key_id).to_uppercase());
        println!("Hash alg:    {:?}", parsed.hash_algorithm);
        println!("Sig type:    0x{:02x}", parsed.sig_type);
        if parsed.creation_time > 0 {
            println!("Signed at:   {} (unix epoch)", parsed.creation_time);
        }
        println!();
    }

    // Use a fixed creation time for key identity (doesn't affect verification)
    let key_creation_time = parsed.creation_time;
    let result = pgp::verify_detached_signature(&file_data, &sig_data, &key, key_creation_time);

    match result {
        pgp::VerifyResult::Valid => {
            println!(
                "GOOD signature from key {}",
                pgp::hex(&parsed.key_id).to_uppercase()
            );
            Ok(())
        }
        pgp::VerifyResult::HashMismatch => {
            anyhow::bail!("BAD signature: hash prefix mismatch (file may have been modified)");
        }
        pgp::VerifyResult::SignatureInvalid => {
            anyhow::bail!(
                "BAD signature: RSA verification failed (wrong key or corrupted signature)"
            );
        }
        pgp::VerifyResult::UnsupportedFormat => {
            anyhow::bail!(
                "Unsupported signature format (only OpenPGP v4 RSA signatures supported)"
            );
        }
    }
}

fn handle_list_keys(matches: &ArgMatches) -> Result<()> {
    let cfg = load_minimal_config(matches)?;

    if cfg.vault_url.is_empty() {
        anyhow::bail!(
            "No vault URL configured. Run 'ghostctl sign config --init' or pass --vault-url"
        );
    }

    let auth = AuthMethod::from_config(&cfg).context("Failed to configure authentication")?;
    let mut kv =
        KeyVaultClient::new(&cfg.vault_url, auth).context("Failed to connect to Key Vault")?;

    let certs = kv
        .list_certificates()
        .context("Failed to list certificates")?;

    if certs.is_empty() {
        println!("No certificates found in {}", cfg.vault_url);
        return Ok(());
    }

    println!("Certificates in {}", cfg.vault_url);
    println!("{}", "=".repeat(60));
    println!("{:<30} {:<10} {:<20}", "Name", "Enabled", "Expires");
    println!("{}", "-".repeat(60));

    for cert in &certs {
        let expires = match cert.expires {
            Some(ts) => chrono_from_epoch(ts),
            None => "never".to_string(),
        };
        println!(
            "{:<30} {:<10} {:<20}",
            cert.name,
            if cert.enabled { "yes" } else { "no" },
            expires
        );
    }

    println!();
    println!("{} certificate(s) total", certs.len());

    Ok(())
}

/// Format a unix epoch timestamp as a human-readable date string
fn chrono_from_epoch(epoch: u64) -> String {
    let secs = epoch;
    let days = secs / 86400;
    // Simple date calculation from epoch days
    // Unix epoch = 1970-01-01
    let mut y = 1970i64;
    let mut remaining = days as i64;

    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let month_days: [i64; 12] = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0usize;
    for (i, &d) in month_days.iter().enumerate() {
        if remaining < d {
            m = i;
            break;
        }
        remaining -= d;
    }

    format!("{:04}-{:02}-{:02}", y, m + 1, remaining + 1)
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
