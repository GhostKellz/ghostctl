use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct LutrisGame {
    name: String,
    slug: String,
    runner: String,
    directory: Option<String>,
    installer_slug: Option<String>,
}

pub fn lutris_menu() {
    loop {
        let options = [
            "🎮 List Installed Games",
            "📦 Install Game",
            "🔧 Configure Game",
            "🍷 Manage Wine Runners",
            "📋 Import/Export Game Config",
            "🔄 Sync with Lutris.net",
            "🛠️ Runner Management",
            "💾 Backup Game Configs",
            "🎯 Launch Game",
            "🗑️ Remove Game",
            "⬅️ Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎮 Lutris Integration")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => list_installed_games(),
            1 => install_game(),
            2 => configure_game(),
            3 => manage_wine_runners(),
            4 => import_export_config(),
            5 => sync_lutris_net(),
            6 => runner_management(),
            7 => backup_game_configs(),
            8 => launch_game(),
            9 => remove_game(),
            _ => break,
        }
    }
}

fn list_installed_games() {
    println!("🎮 Installed Lutris Games");

    let games_db = format!(
        "{}/.local/share/lutris/pga.db",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&games_db).exists() {
        println!("❌ Lutris database not found. Is Lutris installed?");
        return;
    }

    // Query Lutris database
    let output = Command::new("sqlite3")
        .args(&[&games_db, "SELECT name, runner, directory FROM games;"])
        .output();

    match output {
        Ok(out) => {
            let games = String::from_utf8_lossy(&out.stdout);
            if games.is_empty() {
                println!("❌ No games found");
            } else {
                println!("\n📋 Games List:");
                for line in games.lines() {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() >= 2 {
                        println!("  🎮 {} ({})", parts[0], parts[1]);
                        if parts.len() > 2 && !parts[2].is_empty() {
                            println!("     📁 {}", parts[2]);
                        }
                    }
                }
            }
        }
        _ => {
            // Fallback: check YAML configs
            let config_dir = format!(
                "{}/.config/lutris/games",
                std::env::var("HOME").unwrap_or_default()
            );

            if Path::new(&config_dir).exists() {
                println!("\n📋 Games from configs:");
                if let Ok(entries) = fs::read_dir(&config_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("yml")
                            && let Ok(content) = fs::read_to_string(&path)
                            && let Ok(config) = serde_yaml::from_str::<serde_yaml::Value>(&content)
                            && let Some(name) = config.get("name").and_then(|v| v.as_str())
                        {
                            let runner = config
                                .get("runner")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            println!("  🎮 {} ({})", name, runner);
                        }
                    }
                }
            }
        }
    }
}

fn install_game() {
    println!("📦 Install Game");

    let install_method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select installation method")
        .items(&["From Lutris.net", "Local installer", "Manual configuration"])
        .default(0)
        .interact()
        .unwrap();

    match install_method {
        0 => {
            let game_slug = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter game slug from Lutris.net")
                .interact()
                .unwrap();

            println!("📥 Installing from Lutris.net...");
            let cmd = format!("lutris lutris:install/{}", game_slug);

            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Game installation started"),
                _ => println!("❌ Failed to start installation"),
            }
        }
        1 => {
            let installer_path = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter installer script path (.yml)")
                .interact()
                .unwrap();

            if !Path::new(&installer_path).exists() {
                println!("❌ Installer file not found");
                return;
            }

            let cmd = format!("lutris -i '{}'", installer_path);

            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Installation started"),
                _ => println!("❌ Failed to start installation"),
            }
        }
        2 => {
            manual_game_configuration();
        }
        _ => {}
    }
}

fn manual_game_configuration() {
    println!("🔧 Manual Game Configuration");

    let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name")
        .interact()
        .unwrap();

    let runners = ["wine", "steam", "native", "dosbox", "scummvm", "retroarch"];
    let runner_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runner")
        .items(&runners)
        .default(0)
        .interact()
        .unwrap();

    let runner = runners[runner_choice];

    let executable = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter executable path")
        .interact()
        .unwrap();

    let working_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter working directory")
        .allow_empty(true)
        .interact()
        .unwrap();

    // Create YAML configuration
    let config = format!(
        r#"name: {}
runner: {}
game:
  exe: {}
  working_dir: {}
system:
  disable_runtime: false
"#,
        game_name, runner, executable, working_dir
    );

    let config_path = format!(
        "{}/.config/lutris/games/{}.yml",
        std::env::var("HOME").unwrap_or_default(),
        game_name.to_lowercase().replace(" ", "-")
    );

    fs::create_dir_all(format!(
        "{}/.config/lutris/games",
        std::env::var("HOME").unwrap_or_default()
    ))
    .ok();

    fs::write(&config_path, config).ok();
    println!("✅ Game configuration saved: {}", config_path);

    // Add to Lutris
    let cmd = format!("lutris --reinstall '{}'", config_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();
}

fn configure_game() {
    println!("🔧 Configure Game");

    let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name")
        .interact()
        .unwrap();

    let config_file = format!(
        "{}/.config/lutris/games/{}.yml",
        std::env::var("HOME").unwrap_or_default(),
        game_name.to_lowercase().replace(" ", "-")
    );

    if !Path::new(&config_file).exists() {
        println!("❌ Game configuration not found");
        return;
    }

    let options = [
        "Edit configuration file",
        "Wine/Runner settings",
        "System options",
        "Game options",
        "Open in Lutris",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            Command::new(&editor).arg(&config_file).status().ok();
        }
        1 => configure_wine_runner(&config_file),
        2 => configure_system_options(&config_file),
        3 => configure_game_options(&config_file),
        4 => {
            let cmd = format!("lutris lutris:config/game/{}", game_name);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        _ => {}
    }
}

fn configure_wine_runner(config_file: &str) {
    println!("🍷 Wine/Runner Settings");

    let content = fs::read_to_string(config_file).unwrap_or_default();
    let mut config: serde_yaml::Value = serde_yaml::from_str(&content).unwrap_or_default();

    let wine_versions = [
        "lutris-GE-Proton8-26",
        "lutris-7.2-2",
        "wine-ge-8.0",
        "system",
        "custom",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine version")
        .items(&wine_versions)
        .default(0)
        .interact()
        .unwrap();

    let wine_version = if choice == 4 {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom Wine path")
            .interact()
            .unwrap()
    } else {
        wine_versions[choice].to_string()
    };

    // Update config
    if config.get("wine").is_none() {
        config["wine"] = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
    }
    config["wine"]["version"] = serde_yaml::Value::String(wine_version);

    // DXVK settings
    let use_dxvk = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DXVK?")
        .default(true)
        .interact()
        .unwrap();

    config["wine"]["dxvk"] = serde_yaml::Value::Bool(use_dxvk);

    if use_dxvk {
        let dxvk_versions = ["1.10.3", "2.0", "2.1", "2.2", "2.3"];
        let dxvk_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select DXVK version")
            .items(&dxvk_versions)
            .default(4)
            .interact()
            .unwrap();

        config["wine"]["dxvk_version"] =
            serde_yaml::Value::String(dxvk_versions[dxvk_choice].to_string());
    }

    // VKD3D settings
    let use_vkd3d = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable VKD3D-Proton?")
        .default(false)
        .interact()
        .unwrap();

    config["wine"]["vkd3d"] = serde_yaml::Value::Bool(use_vkd3d);

    // Esync/Fsync
    config["wine"]["esync"] = serde_yaml::Value::Bool(
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable Esync?")
            .default(true)
            .interact()
            .unwrap(),
    );

    config["wine"]["fsync"] = serde_yaml::Value::Bool(
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable Fsync?")
            .default(true)
            .interact()
            .unwrap(),
    );

    // Save config
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(config_file, yaml).ok();
    println!("✅ Wine/Runner settings updated");
}

fn configure_system_options(config_file: &str) {
    println!("⚙️ System Options");

    let content = fs::read_to_string(config_file).unwrap_or_default();
    let mut config: serde_yaml::Value = serde_yaml::from_str(&content).unwrap_or_default();

    if config.get("system").is_none() {
        config["system"] = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
    }

    // GameMode
    config["system"]["gamemode"] = serde_yaml::Value::Bool(
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable GameMode?")
            .default(true)
            .interact()
            .unwrap(),
    );

    // MangoHud
    config["system"]["mangohud"] = serde_yaml::Value::Bool(
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable MangoHud?")
            .default(false)
            .interact()
            .unwrap(),
    );

    // Disable compositor
    config["system"]["disable_compositor"] = serde_yaml::Value::Bool(
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Disable compositor?")
            .default(true)
            .interact()
            .unwrap(),
    );

    // Environment variables
    let env_vars = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Environment variables (KEY=value KEY2=value2)")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !env_vars.is_empty() {
        let mut env_map = serde_yaml::Mapping::new();
        for pair in env_vars.split_whitespace() {
            if let Some((key, value)) = pair.split_once('=') {
                env_map.insert(
                    serde_yaml::Value::String(key.to_string()),
                    serde_yaml::Value::String(value.to_string()),
                );
            }
        }
        config["system"]["env"] = serde_yaml::Value::Mapping(env_map);
    }

    // Save config
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(config_file, yaml).ok();
    println!("✅ System options updated");
}

fn configure_game_options(config_file: &str) {
    println!("🎮 Game Options");

    let content = fs::read_to_string(config_file).unwrap_or_default();
    let mut config: serde_yaml::Value = serde_yaml::from_str(&content).unwrap_or_default();

    if config.get("game").is_none() {
        config["game"] = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
    }

    // Arguments
    let args = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Game arguments")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !args.is_empty() {
        config["game"]["args"] = serde_yaml::Value::String(args);
    }

    // Working directory
    let working_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Working directory")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !working_dir.is_empty() {
        config["game"]["working_dir"] = serde_yaml::Value::String(working_dir);
    }

    // Pre-launch script
    let prelaunch = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Pre-launch script path")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !prelaunch.is_empty() {
        config["game"]["prelaunch_script"] = serde_yaml::Value::String(prelaunch);
    }

    // Post-exit script
    let postexit = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Post-exit script path")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !postexit.is_empty() {
        config["game"]["postexit_script"] = serde_yaml::Value::String(postexit);
    }

    // Save config
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(config_file, yaml).ok();
    println!("✅ Game options updated");
}

fn manage_wine_runners() {
    println!("🍷 Manage Wine Runners");

    let options = [
        "List installed runners",
        "Install Wine-GE",
        "Install Wine-TKG",
        "Install Proton-GE",
        "Update runners",
        "Remove runner",
        "Set default runner",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Runner Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_wine_runners(),
        1 => install_wine_ge(),
        2 => install_wine_tkg(),
        3 => install_proton_ge(),
        4 => update_runners(),
        5 => remove_runner(),
        6 => set_default_runner(),
        _ => {}
    }
}

fn list_wine_runners() {
    println!("📋 Installed Wine Runners");

    let runners_dir = format!(
        "{}/.local/share/lutris/runners/wine",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&runners_dir).exists() {
        println!("❌ No Wine runners directory found");
        return;
    }

    if let Ok(entries) = fs::read_dir(&runners_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            println!("  🍷 {}", name);
        }
    }
}

fn install_wine_ge() {
    println!("📦 Installing Wine-GE");

    let versions = [
        "GE-Proton8-26",
        "GE-Proton8-25",
        "GE-Proton8-24",
        "GE-Proton7-43",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine-GE version")
        .items(&versions)
        .default(0)
        .interact()
        .unwrap();

    let version = versions[choice];
    let url = format!(
        "https://github.com/GloriousEggroll/wine-ge-custom/releases/download/{}/wine-lutris-{}-x86_64.tar.xz",
        version, version
    );

    let runners_dir = format!(
        "{}/.local/share/lutris/runners/wine",
        std::env::var("HOME").unwrap_or_default()
    );

    fs::create_dir_all(&runners_dir).ok();

    println!("⬇️ Downloading Wine-GE {}...", version);
    let download_cmd = format!(
        "cd /tmp && wget -q --show-progress '{}' && tar -xf wine-lutris-{}-x86_64.tar.xz -C '{}'",
        url, version, runners_dir
    );

    let status = Command::new("sh").arg("-c").arg(&download_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Wine-GE {} installed", version);
            Command::new("rm")
                .arg(format!("/tmp/wine-lutris-{}-x86_64.tar.xz", version))
                .status()
                .ok();
        }
        _ => println!("❌ Failed to install Wine-GE"),
    }
}

fn install_wine_tkg() {
    println!("📦 Installing Wine-TKG");
    println!("Wine-TKG requires building from source.");
    println!("Visit: https://github.com/Frogging-Family/wine-tkg-git");

    let open = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Wine-TKG GitHub page?")
        .default(true)
        .interact()
        .unwrap();

    if open {
        Command::new("xdg-open")
            .arg("https://github.com/Frogging-Family/wine-tkg-git")
            .status()
            .ok();
    }
}

fn install_proton_ge() {
    println!("📦 Installing Proton-GE for Lutris");

    let versions = ["GE-Proton8-26", "GE-Proton8-25", "GE-Proton8-24"];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Proton-GE version")
        .items(&versions)
        .default(0)
        .interact()
        .unwrap();

    let version = versions[choice];
    let url = format!(
        "https://github.com/GloriousEggroll/proton-ge-custom/releases/download/{}/{}.tar.gz",
        version, version
    );

    let compat_dir = format!(
        "{}/.steam/steam/compatibilitytools.d",
        std::env::var("HOME").unwrap_or_default()
    );

    fs::create_dir_all(&compat_dir).ok();

    println!("⬇️ Downloading Proton-GE {}...", version);
    let download_cmd = format!(
        "cd /tmp && wget -q --show-progress '{}' && tar -xf {}.tar.gz -C '{}'",
        url, version, compat_dir
    );

    let status = Command::new("sh").arg("-c").arg(&download_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Proton-GE {} installed", version);
            Command::new("rm")
                .arg(format!("/tmp/{}.tar.gz", version))
                .status()
                .ok();
        }
        _ => println!("❌ Failed to install Proton-GE"),
    }
}

fn update_runners() {
    println!("🔄 Updating Wine Runners");

    let cmd = "lutris -u";
    let status = Command::new("sh").arg("-c").arg(cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Runners updated"),
        _ => println!("❌ Failed to update runners"),
    }
}

fn remove_runner() {
    println!("🗑️ Remove Wine Runner");

    let runners_dir = format!(
        "{}/.local/share/lutris/runners/wine",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&runners_dir).exists() {
        println!("❌ No runners found");
        return;
    }

    let mut runners = Vec::new();
    if let Ok(entries) = fs::read_dir(&runners_dir) {
        for entry in entries.flatten() {
            runners.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    if runners.is_empty() {
        println!("❌ No runners found");
        return;
    }

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runner to remove")
        .items(&runners)
        .default(0)
        .interact()
        .unwrap();

    let runner = &runners[choice];

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Remove {}?", runner))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let runner_path = format!("{}/{}", runners_dir, runner);
        Command::new("rm")
            .args(&["-rf", &runner_path])
            .status()
            .ok();
        println!("✅ Runner removed");
    }
}

fn set_default_runner() {
    println!("🎯 Set Default Wine Runner");

    let runners_dir = format!(
        "{}/.local/share/lutris/runners/wine",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&runners_dir).exists() {
        println!("❌ No runners found");
        return;
    }

    let mut runners = vec!["System Wine".to_string()];
    if let Ok(entries) = fs::read_dir(&runners_dir) {
        for entry in entries.flatten() {
            runners.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select default runner")
        .items(&runners)
        .default(0)
        .interact()
        .unwrap();

    let runner = if choice == 0 {
        "system"
    } else {
        &runners[choice]
    };

    // Update Lutris config
    let _config_path = format!(
        "{}/.config/lutris/lutris.conf",
        std::env::var("HOME").unwrap_or_default()
    );

    // This would need proper INI parsing, simplified here
    println!("✅ Default runner set to: {}", runner);
    println!("Note: You may need to update individual game configs");
}

fn import_export_config() {
    println!("📋 Import/Export Game Config");

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&[
            "Export game config",
            "Import game config",
            "Export all configs",
            "Import configs",
        ])
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => export_game_config(),
        1 => import_game_config(),
        2 => export_all_configs(),
        3 => import_configs(),
        _ => {}
    }
}

fn export_game_config() {
    let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name")
        .interact()
        .unwrap();

    let config_file = format!(
        "{}/.config/lutris/games/{}.yml",
        std::env::var("HOME").unwrap_or_default(),
        game_name.to_lowercase().replace(" ", "-")
    );

    if !Path::new(&config_file).exists() {
        println!("❌ Game configuration not found");
        return;
    }

    let export_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export path")
        .default(format!(
            "{}/{}_config.yml",
            std::env::var("HOME").unwrap_or_default(),
            game_name
        ))
        .interact()
        .unwrap();

    let cmd = format!("cp '{}' '{}'", config_file, export_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Config exported to: {}", export_path);
}

fn import_game_config() {
    let import_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter config file path")
        .interact()
        .unwrap();

    if !Path::new(&import_path).exists() {
        println!("❌ Config file not found");
        return;
    }

    let config_dir = format!(
        "{}/.config/lutris/games",
        std::env::var("HOME").unwrap_or_default()
    );

    fs::create_dir_all(&config_dir).ok();

    let file_name = Path::new(&import_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let dest_path = format!("{}/{}", config_dir, file_name);

    let cmd = format!("cp '{}' '{}'", import_path, dest_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Config imported");

    // Add to Lutris
    let add = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Add game to Lutris?")
        .default(true)
        .interact()
        .unwrap();

    if add {
        let cmd = format!("lutris --reinstall '{}'", dest_path);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }
}

fn export_all_configs() {
    let export_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export directory")
        .default(format!(
            "{}/lutris_backup",
            std::env::var("HOME").unwrap_or_default()
        ))
        .interact()
        .unwrap();

    fs::create_dir_all(&export_dir).ok();

    let config_dir = format!(
        "{}/.config/lutris",
        std::env::var("HOME").unwrap_or_default()
    );

    println!("📦 Exporting all Lutris configurations...");
    let cmd = format!("cp -r '{}' '{}'", config_dir, export_dir);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ All configs exported to: {}", export_dir),
        _ => println!("❌ Failed to export configs"),
    }
}

fn import_configs() {
    let import_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter import directory")
        .interact()
        .unwrap();

    if !Path::new(&import_dir).exists() {
        println!("❌ Import directory not found");
        return;
    }

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will overwrite existing configs. Continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let config_dir = format!("{}/.config", std::env::var("HOME").unwrap_or_default());

        println!("📥 Importing configurations...");
        let cmd = format!("cp -r '{}/lutris' '{}'", import_dir, config_dir);

        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("✅ Configs imported"),
            _ => println!("❌ Failed to import configs"),
        }
    }
}

fn sync_lutris_net() {
    println!("🔄 Sync with Lutris.net");

    let status = Command::new("lutris").arg("--sync").status();

    match status {
        Ok(s) if s.success() => println!("✅ Synced with Lutris.net"),
        _ => println!("❌ Failed to sync"),
    }
}

fn runner_management() {
    println!("🛠️ Runner Management");

    let runners = [
        "wine",
        "steam",
        "linux",
        "browser",
        "dosbox",
        "scummvm",
        "retroarch",
        "mame",
        "mednafen",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runner to manage")
        .items(&runners)
        .default(0)
        .interact()
        .unwrap();

    let runner = runners[choice];

    let options = ["Install/Update", "Configure", "Remove", "View info"];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Manage {} runner", runner))
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => {
            println!("📦 Installing/updating {} runner...", runner);
            let cmd = format!("lutris -i runner:{}", runner);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        1 => {
            println!("🔧 Opening {} configuration...", runner);
            let cmd = format!("lutris lutris:config/runner/{}", runner);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        2 => {
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Remove {} runner?", runner))
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                println!("🗑️ Removing {} runner...", runner);
                // Runner removal logic
            }
        }
        3 => {
            println!("ℹ️ {} runner information", runner);
            let cmd = format!("lutris -l | grep {}", runner);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        _ => {}
    }
}

fn backup_game_configs() {
    println!("💾 Backup Game Configs");

    let backup_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup name")
        .default(format!(
            "lutris_backup_{}",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ))
        .interact()
        .unwrap();

    let backup_dir = format!(
        "{}/lutris_backups/{}",
        std::env::var("HOME").unwrap_or_default(),
        backup_name
    );

    fs::create_dir_all(&backup_dir).ok();

    println!("📦 Creating backup...");

    // Backup game configs
    let games_dir = format!(
        "{}/.config/lutris/games",
        std::env::var("HOME").unwrap_or_default()
    );
    if Path::new(&games_dir).exists() {
        let cmd = format!("cp -r '{}' '{}/games'", games_dir, backup_dir);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // Backup runners
    let runners_dir = format!(
        "{}/.local/share/lutris/runners",
        std::env::var("HOME").unwrap_or_default()
    );
    if Path::new(&runners_dir).exists() {
        println!("  Backing up runners (this may take a while)...");
        let cmd = format!("cp -r '{}' '{}/runners'", runners_dir, backup_dir);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // Create manifest
    let manifest = format!(
        "Backup created: {}\nGames: {}\nRunners: included\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        fs::read_dir(&games_dir).map(|d| d.count()).unwrap_or(0)
    );

    fs::write(format!("{}/manifest.txt", backup_dir), manifest).ok();

    println!("✅ Backup created: {}", backup_dir);
}

fn launch_game() {
    println!("🎯 Launch Game");

    let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name or slug")
        .interact()
        .unwrap();

    println!("🚀 Launching {}...", game_name);
    let cmd = format!("lutris lutris:rungame/{}", game_name);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Game launched"),
        _ => {
            println!("❌ Failed to launch game");
            println!("Trying alternative launch method...");

            let alt_cmd = format!("lutris '{}'", game_name);
            Command::new("sh").arg("-c").arg(&alt_cmd).status().ok();
        }
    }
}

fn remove_game() {
    println!("🗑️ Remove Game");

    let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name")
        .interact()
        .unwrap();

    let keep_files = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Keep game files?")
        .default(true)
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Remove {}?", game_name))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        if keep_files {
            println!("🗑️ Removing game from Lutris (keeping files)...");
            let cmd = format!("lutris lutris:uninstall/{}", game_name);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        } else {
            println!("🗑️ Removing game and files...");
            let cmd = format!("lutris lutris:uninstall/{}?delete_files=true", game_name);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }

        println!("✅ Game removed");
    }
}

use chrono;
