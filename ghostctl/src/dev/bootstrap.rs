use chrono::{Datelike, Utc};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct GhostctlConfig {
    pub github_user: String,
    pub default_license: Option<String>,
}

/// Validate repository name to prevent path traversal and shell injection
fn validate_repo_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Repository name cannot be empty");
    }
    if name.len() > 100 {
        return Err("Repository name too long (max 100 characters)");
    }
    // Only allow alphanumeric, hyphen, underscore, and dot (no leading dot)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err("Repository name contains invalid characters");
    }
    if name.starts_with('.') || name.starts_with('-') {
        return Err("Repository name cannot start with '.' or '-'");
    }
    if name.contains("..") {
        return Err("Repository name cannot contain '..'");
    }
    Ok(())
}

/// Validate GitHub username
fn validate_github_user(user: &str) -> Result<(), &'static str> {
    if user.is_empty() {
        return Err("GitHub username cannot be empty");
    }
    if user.len() > 39 {
        return Err("GitHub username too long");
    }
    // GitHub usernames: alphanumeric and hyphens, no consecutive hyphens, no leading/trailing hyphen
    if !user
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-')
    {
        return Err("GitHub username contains invalid characters");
    }
    if user.starts_with('-') || user.ends_with('-') || user.contains("--") {
        return Err("Invalid GitHub username format");
    }
    Ok(())
}

fn load_config() -> GhostctlConfig {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/config.yaml"),
        None => {
            return GhostctlConfig {
                github_user: "ghostkellz".to_string(),
                default_license: Some("MIT".to_string()),
            };
        }
    };
    if let Ok(contents) = fs::read_to_string(&config_path) {
        if let Ok(cfg) = serde_yaml::from_str::<GhostctlConfig>(&contents) {
            // Validate loaded config
            if validate_github_user(&cfg.github_user).is_ok() {
                return cfg;
            }
            eprintln!("Warning: Invalid GitHub username in config, using default");
        }
    }
    GhostctlConfig {
        github_user: "ghostkellz".to_string(),
        default_license: Some("MIT".to_string()),
    }
}

pub fn bootstrap_repo(repo: &str) {
    // Validate repository name first
    if let Err(e) = validate_repo_name(repo) {
        eprintln!("Invalid repository name: {}", e);
        return;
    }

    let cfg = load_config();

    // Validate GitHub user from config
    if let Err(e) = validate_github_user(&cfg.github_user) {
        eprintln!("Invalid GitHub username in config: {}", e);
        return;
    }

    let base_path = "/data/projects";
    let repo_path = format!("{}/{}", base_path, repo);

    // Verify base path exists
    if !Path::new(base_path).exists() {
        eprintln!("Base path {} does not exist", base_path);
        return;
    }

    // Create directory
    if !Path::new(&repo_path).exists() {
        if let Err(e) = fs::create_dir(&repo_path) {
            eprintln!("Failed to create repo directory: {}", e);
            return;
        }
    }

    if let Err(e) = std::env::set_current_dir(&repo_path) {
        eprintln!("Failed to change to repo directory: {}", e);
        return;
    }

    // Create README.md
    if let Err(e) = fs::write("README.md", format!("# {}", repo)) {
        eprintln!("Failed to write README.md: {}", e);
        return;
    }

    // Create LICENSE
    let mut license = match fs::File::create("LICENSE") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create LICENSE: {}", e);
            return;
        }
    };

    let license_text = match cfg.default_license.as_deref() {
        Some("MIT") | _ => format!(
            "MIT License\n\nCopyright (c) {} {}\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software...",
            Utc::now().year(),
            cfg.github_user
        ),
    };

    if let Err(e) = writeln!(license, "{}", license_text) {
        eprintln!("Failed to write LICENSE: {}", e);
        return;
    }

    // Git init, add, commit - using direct args, no shell interpolation
    if let Err(e) = Command::new("git").arg("init").status() {
        eprintln!("Failed to initialize git: {}", e);
        return;
    }

    if let Err(e) = Command::new("git").args(["add", "."]).status() {
        eprintln!("Failed to stage files: {}", e);
        return;
    }

    if let Err(e) = Command::new("git")
        .args(["commit", "-m", "Initial commit: bootstrap"])
        .status()
    {
        eprintln!("Failed to create initial commit: {}", e);
        return;
    }

    // Create GitHub repo and push - using separate validated args
    let full_repo = format!("{}/{}", cfg.github_user, repo);
    match Command::new("gh")
        .args([
            "repo",
            "create",
            &full_repo,
            "--public",
            "--source=.",
            "--remote=origin",
            "--push",
        ])
        .status()
    {
        Ok(status) if !status.success() => {
            eprintln!("GitHub repo creation failed with exit code: {:?}", status.code());
            return;
        }
        Err(e) => {
            eprintln!("Failed to create GitHub repo: {}", e);
            return;
        }
        _ => {}
    }

    // Set remote to SSH - using validated components
    let ssh_url = format!("git@github.com:{}/{}.git", cfg.github_user, repo);
    if let Err(e) = Command::new("git")
        .args(["remote", "set-url", "origin", &ssh_url])
        .status()
    {
        eprintln!("Failed to set remote URL: {}", e);
        return;
    }

    println!(
        "\n Repo '{}' created at {} and pushed via SSH",
        repo, repo_path
    );

    // Show remote info
    if let Err(e) = Command::new("git").args(["remote", "-v"]).status() {
        eprintln!("Failed to show remote info: {}", e);
    }
}