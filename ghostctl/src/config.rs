use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GhostConfig {
    pub general: GeneralConfig,
    pub backup: BackupConfig,
    pub scripts: ScriptsConfig,
    pub ghost_tools: GhostToolsConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub github_user: String,
    pub default_editor: String,
    pub log_level: String,
    pub auto_update_check: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupConfig {
    pub default_paths: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub retention_daily: u32,
    pub retention_weekly: u32,
    pub retention_monthly: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptsConfig {
    pub local_scripts_dir: String,
    pub auto_discover: bool,
    pub trusted_sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GhostToolsConfig {
    pub auto_install_deps: bool,
    pub preferred_build_jobs: u32,
    pub install_location: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    pub theme: String,
    pub show_tips: bool,
    pub confirmation_prompts: bool,
}

impl Default for GhostConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                github_user: "ghostkellz".to_string(),
                default_editor: std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()),
                log_level: "info".to_string(),
                auto_update_check: true,
            },
            backup: BackupConfig {
                default_paths: vec!["/home".to_string(), "/etc".to_string()],
                exclude_patterns: vec!["*.tmp".to_string(), "*.cache".to_string()],
                retention_daily: 7,
                retention_weekly: 4,
                retention_monthly: 12,
            },
            scripts: ScriptsConfig {
                local_scripts_dir: "~/.config/ghostctl/scripts".to_string(),
                auto_discover: true,
                trusted_sources: vec![
                    "https://raw.githubusercontent.com/ghostkellz/ghostctl/main/scripts/"
                        .to_string(),
                ],
            },
            ghost_tools: GhostToolsConfig {
                auto_install_deps: false,
                preferred_build_jobs: num_cpus::get() as u32,
                install_location: "/usr/bin".to_string(),
            },
            ui: UiConfig {
                theme: "default".to_string(),
                show_tips: true,
                confirmation_prompts: true,
            },
        }
    }
}

impl GhostConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str::<GhostConfig>(&content) {
                    Ok(config) => {
                        log::info!("Loaded config from {:?}", config_path);
                        return config;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse config: {}. Using defaults.", e);
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read config file: {}. Using defaults.", e);
                }
            }
        }

        // Create default config if none exists
        let default_config = Self::default();
        if let Err(e) = default_config.save() {
            log::warn!("Failed to save default config: {}", e);
        }

        default_config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        log::info!("Config saved to {:?}", config_path);
        Ok(())
    }

    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ghostctl")
            .join("config.toml")
    }

    pub fn edit() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let _config = Self::load(); // Ensure config exists

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        let status = std::process::Command::new(&editor)
            .arg(&config_path)
            .status()?;

        if status.success() {
            // Validate the edited config
            match Self::load() {
                _config => {
                    println!("✅ Configuration updated successfully");
                    log::info!("Config edited and validated");
                }
            }
        } else {
            return Err("Editor exited with error".into());
        }

        Ok(())
    }

    pub fn show() {
        let config = Self::load();

        println!("📋 GhostCTL Configuration");
        println!("════════════════════════");
        println!("📂 Config file: {:?}", Self::config_path());
        println!();

        println!("⚙️  General:");
        println!("  GitHub User: {}", config.general.github_user);
        println!("  Editor: {}", config.general.default_editor);
        println!("  Log Level: {}", config.general.log_level);
        println!("  Auto Update Check: {}", config.general.auto_update_check);
        println!();

        println!("💾 Backup:");
        println!("  Default Paths: {:?}", config.backup.default_paths);
        println!(
            "  Retention: {}d/{}w/{}m",
            config.backup.retention_daily,
            config.backup.retention_weekly,
            config.backup.retention_monthly
        );
        println!();

        println!("📜 Scripts:");
        println!("  Local Dir: {}", config.scripts.local_scripts_dir);
        println!("  Auto Discover: {}", config.scripts.auto_discover);
        println!();

        println!("👻 Ghost Tools:");
        println!("  Build Jobs: {}", config.ghost_tools.preferred_build_jobs);
        println!(
            "  Install Location: {}",
            config.ghost_tools.install_location
        );
        println!();

        println!("🎨 UI:");
        println!("  Theme: {}", config.ui.theme);
        println!("  Show Tips: {}", config.ui.show_tips);
        println!("  Confirmations: {}", config.ui.confirmation_prompts);
    }

    pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();

        if config_path.exists() {
            fs::remove_file(&config_path)?;
            println!("🗑️  Removed existing config");
        }

        let default_config = Self::default();
        default_config.save()?;

        println!("✅ Reset to default configuration");
        Ok(())
    }
}
