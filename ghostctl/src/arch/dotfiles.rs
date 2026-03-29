use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn dotfiles_menu() {
    println!("📁 Dotfiles Management");
    println!("=====================");

    let options = [
        "🔍 Find dotfiles in home directory",
        "💾 Backup dotfiles",
        "📊 Track with Git",
        "🔄 Sync with remote repository",
        "♻️  Restore from backup",
        "📋 List tracked dotfiles",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Dotfiles Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => find_dotfiles(),
        1 => backup_dotfiles(),
        2 => init_dotfiles_repo(),
        3 => sync_dotfiles(),
        4 => restore_dotfiles(),
        5 => list_tracked_dotfiles(),
        _ => return,
    }
}

pub fn find_dotfiles() {
    println!("🔍 Finding dotfiles in home directory...");

    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    let mut dotfiles = Vec::new();

    // Common dotfiles to look for
    let common_dotfiles = [
        ".bashrc",
        ".zshrc",
        ".vimrc",
        ".gitconfig",
        ".tmux.conf",
        ".config/nvim",
        ".config/alacritty",
        ".config/kitty",
        ".config/fish",
        ".config/starship.toml",
        ".config/ghostty",
        ".ssh/config",
    ];

    for dotfile in &common_dotfiles {
        let path = PathBuf::from(&home).join(dotfile);
        if path.exists() {
            dotfiles.push(path.display().to_string());
        }
    }

    // Also find all dot directories in home (but not deeply nested)
    if let Ok(entries) = fs::read_dir(&home) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name()
                && let Some(name_str) = name.to_str()
                && name_str.starts_with('.')
                && path.is_dir()
            {
                dotfiles.push(path.display().to_string());
            }
        }
    }

    if dotfiles.is_empty() {
        println!("❌ No dotfiles found");
        return;
    }

    println!("\n📋 Found {} dotfiles:", dotfiles.len());
    for dotfile in &dotfiles {
        println!("  • {}", dotfile);
    }

    // Ask if user wants to do something with them
    let options = ["Backup selected", "Track with Git", "View details", "Exit"];
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => backup_selected_dotfiles(dotfiles),
        1 => track_selected_dotfiles(dotfiles),
        2 => show_dotfile_details(dotfiles),
        _ => {}
    }
}

fn backup_selected_dotfiles(dotfiles: Vec<String>) {
    let indices = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select dotfiles to backup")
        .items(&dotfiles)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    if indices.is_empty() {
        println!("❌ No dotfiles selected");
        return;
    }

    let backup_dir: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup directory")
        .default(format!(
            "{}/dotfiles-backup-{}",
            std::env::var("HOME").unwrap_or_else(|_| ".".to_string()),
            chrono::Local::now().format("%Y%m%d-%H%M%S")
        ))
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let _ = fs::create_dir_all(&backup_dir);

    for idx in indices {
        let dotfile = &dotfiles[idx];
        let Some(filename) = PathBuf::from(dotfile).file_name().map(|f| f.to_owned()) else {
            continue;
        };
        let dest = PathBuf::from(&backup_dir).join(filename);

        println!("  Backing up {} -> {}", dotfile, dest.display());

        if PathBuf::from(dotfile).is_dir() {
            let _ = Command::new("cp")
                .args(["-r", dotfile, &dest.to_string_lossy()])
                .status();
        } else {
            let _ = fs::copy(dotfile, &dest);
        }
    }

    println!("✅ Backup completed to: {}", backup_dir);
}

fn track_selected_dotfiles(dotfiles: Vec<String>) {
    println!("📊 Setting up Git tracking for dotfiles...");

    let dotfiles_dir = format!(
        "{}/.dotfiles",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    // Initialize git repo if it doesn't exist
    if !PathBuf::from(&dotfiles_dir).join(".git").exists() {
        let _ = fs::create_dir_all(&dotfiles_dir);
        let _ = Command::new("git")
            .current_dir(&dotfiles_dir)
            .args(["init"])
            .status();
    }

    let indices = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select dotfiles to track")
        .items(&dotfiles)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    for idx in indices {
        let dotfile = &dotfiles[idx];
        let dotfile_path = PathBuf::from(dotfile);
        let Some(filename) = dotfile_path.file_name() else {
            continue;
        };
        let dest = PathBuf::from(&dotfiles_dir).join(filename);

        // Move file and create symlink
        println!("  Tracking {}", dotfile);
        let _ = Command::new("cp")
            .args(["-r", dotfile, &dest.to_string_lossy()])
            .status();

        // Add to git
        let _ = Command::new("git")
            .current_dir(&dotfiles_dir)
            .args(["add", &filename.to_string_lossy()])
            .status();
    }

    println!("✅ Dotfiles tracked in: {}", dotfiles_dir);
    println!("💡 Don't forget to create symlinks and commit your changes!");
}

fn show_dotfile_details(dotfiles: Vec<String>) {
    for dotfile in dotfiles {
        let path = PathBuf::from(&dotfile);
        if let Ok(metadata) = fs::metadata(&path) {
            println!("\n📄 {}", dotfile);
            println!("  Size: {} bytes", metadata.len());
            println!(
                "  Type: {}",
                if metadata.is_dir() {
                    "Directory"
                } else {
                    "File"
                }
            );

            if !metadata.is_dir() {
                // Show first few lines for files
                if let Ok(content) = fs::read_to_string(&path) {
                    let lines: Vec<&str> = content.lines().take(3).collect();
                    if !lines.is_empty() {
                        println!("  Preview:");
                        for line in lines {
                            println!("    {}", line);
                        }
                    }
                }
            }
        }
    }
}

pub fn backup_dotfiles() {
    println!("💾 Backing up dotfiles...");
    find_dotfiles();
}

pub fn init_dotfiles_repo() {
    println!("📊 Initializing dotfiles repository...");

    let dotfiles_dir = format!(
        "{}/.dotfiles",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    if PathBuf::from(&dotfiles_dir).join(".git").exists() {
        println!("⚠️  Git repository already exists at {}", dotfiles_dir);
        return;
    }

    let _ = fs::create_dir_all(&dotfiles_dir);

    let status = Command::new("git")
        .current_dir(&dotfiles_dir)
        .args(["init"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Git repository initialized at {}", dotfiles_dir);

            // Create a basic README
            let readme_content = "# Dotfiles\n\nMy personal dotfiles managed by ghostctl.\n";
            let _ = fs::write(
                PathBuf::from(&dotfiles_dir).join("README.md"),
                readme_content,
            );

            // Create install script template
            let install_script =
                "#!/bin/bash\n# Dotfiles installation script\n\n# Create symlinks here\n";
            let _ = fs::write(
                PathBuf::from(&dotfiles_dir).join("install.sh"),
                install_script,
            );

            println!("📝 Created README.md and install.sh templates");
        }
        _ => println!("❌ Failed to initialize git repository"),
    }
}

pub fn sync_dotfiles() {
    println!("🔄 Syncing dotfiles with remote repository...");

    let dotfiles_dir = format!(
        "{}/.dotfiles",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    if !PathBuf::from(&dotfiles_dir).join(".git").exists() {
        println!("❌ No git repository found. Initialize first with 'track' option.");
        return;
    }

    // Check for remote
    let output = Command::new("git")
        .current_dir(&dotfiles_dir)
        .args(["remote", "-v"])
        .output();

    if let Ok(output) = output
        && output.stdout.is_empty()
    {
        println!("⚠️  No remote repository configured.");
        let add_remote = match dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to add a remote?")
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        if add_remote {
            let remote_url: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Remote URL (e.g., git@github.com:username/dotfiles.git)")
                .interact_text()
            {
                Ok(u) => u,
                Err(_) => return,
            };

            let _ = Command::new("git")
                .current_dir(&dotfiles_dir)
                .args(["remote", "add", "origin", &remote_url])
                .status();
        }
    }

    // Pull latest changes
    println!("⬇️  Pulling latest changes...");
    let _ = Command::new("git")
        .current_dir(&dotfiles_dir)
        .args(["pull"])
        .status();

    // Push changes
    println!("⬆️  Pushing local changes...");
    let _ = Command::new("git")
        .current_dir(&dotfiles_dir)
        .args(["push"])
        .status();

    println!("✅ Sync completed");
}

pub fn restore_dotfiles() {
    println!("♻️  Restoring dotfiles from backup...");

    let backup_dir: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup directory path")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    if !PathBuf::from(&backup_dir).exists() {
        println!("❌ Backup directory does not exist");
        return;
    }

    // List files in backup
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    if files.is_empty() {
        println!("❌ No files found in backup directory");
        return;
    }

    let indices = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select files to restore")
        .items(&files)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

    for idx in indices {
        let file = &files[idx];
        let src = PathBuf::from(&backup_dir).join(file);
        let dest = PathBuf::from(&home).join(file);

        println!("  Restoring {} -> {}", src.display(), dest.display());

        if src.is_dir() {
            let _ = Command::new("cp")
                .args(["-r", &src.to_string_lossy(), &dest.to_string_lossy()])
                .status();
        } else {
            let _ = fs::copy(&src, &dest);
        }
    }

    println!("✅ Restore completed");
}

pub fn list_tracked_dotfiles() {
    println!("📋 Listing tracked dotfiles...");

    let dotfiles_dir = format!(
        "{}/.dotfiles",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    if !PathBuf::from(&dotfiles_dir).join(".git").exists() {
        println!("❌ No dotfiles repository found");
        return;
    }

    let output = Command::new("git")
        .current_dir(&dotfiles_dir)
        .args(&["ls-files"])
        .output();

    match output {
        Ok(output) => {
            let files = String::from_utf8_lossy(&output.stdout);
            if files.is_empty() {
                println!("❌ No files tracked yet");
            } else {
                println!("\n📁 Tracked files:");
                for file in files.lines() {
                    println!("  • {}", file);
                }
            }
        }
        Err(_) => println!("❌ Failed to list tracked files"),
    }
}
