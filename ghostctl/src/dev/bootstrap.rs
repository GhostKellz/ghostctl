use std::process::Command;
use std::fs;
use std::io::Write;
use std::path::Path;
use chrono::{Datelike, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GhostctlConfig {
    pub github_user: String,
    pub default_license: Option<String>,
}

fn load_config() -> GhostctlConfig {
    let config_path = dirs::config_dir().unwrap().join("ghostctl/config.yaml");
    if let Ok(contents) = fs::read_to_string(&config_path) {
        if let Ok(cfg) = serde_yaml::from_str::<GhostctlConfig>(&contents) {
            return cfg;
        }
    }
    GhostctlConfig {
        github_user: "ghostkellz".to_string(),
        default_license: Some("MIT".to_string()),
    }
}

pub fn bootstrap_repo(repo: &str) {
    let cfg = load_config();
    let base_path = "/data/projects";
    let repo_path = format!("{}/{}", base_path, repo);

    // Create directory
    if !Path::new(&repo_path).exists() {
        fs::create_dir(&repo_path).expect("Failed to create repo directory");
    }
    std::env::set_current_dir(&repo_path).expect("Failed to cd into repo dir");

    // Create README.md
    fs::write("README.md", format!("# {}", repo)).expect("Failed to write README.md");

    // Create LICENSE
    let mut license = fs::File::create("LICENSE").expect("Failed to create LICENSE");
    let license_text = match cfg.default_license.as_deref() {
        Some("MIT") | _ => format!("MIT License\n\nCopyright (c) {} {}\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software...", chrono::Utc::now().year(), cfg.github_user),
    };
    writeln!(license, "{}", license_text).unwrap();

    // Git init, add, commit
    Command::new("git").arg("init").status().unwrap();
    Command::new("git").args(&["add", "."]).status().unwrap();
    Command::new("git").args(&["commit", "-m", "Initial commit: bootstrap"]).status().unwrap();

    // Create GitHub repo and push
    Command::new("gh")
        .args(&["repo", "create", &format!("{}/{}", cfg.github_user, repo), "--public", "--source=.", "--remote=origin", "--push"])
        .status().unwrap();

    // Set remote to SSH
    Command::new("git")
        .args(&["remote", "set-url", "origin", &format!("git@github.com:{}/{}.git", cfg.github_user, repo)])
        .status().unwrap();

    println!("\n✅ Repo '{}' created at {} and pushed via SSH", repo, repo_path);
    Command::new("git").args(&["remote", "-v"]).status().unwrap();
}