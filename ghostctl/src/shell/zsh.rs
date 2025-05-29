pub fn install_zsh() {
    println!("Installing ZSH...");
    let is_installed = std::process::Command::new("which")
        .arg("zsh")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if is_installed {
        println!("ZSH is already installed.");
    } else {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm zsh || yay -S --noconfirm zsh")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("ZSH installed successfully.");
        } else {
            println!("Failed to install ZSH. Please install it manually.");
            return;
        }
    }
    // Install Oh My Zsh
    let home = std::env::var("HOME").unwrap();
    let omz_dir = format!("{}/.oh-my-zsh", home);
    if !std::path::Path::new(&omz_dir).exists() {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg("RUNZSH=no CHSH=no sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"")
            .status()
            .expect("failed to install Oh My Zsh");
        if status.success() {
            println!("Oh My Zsh installed.");
        } else {
            println!("Failed to install Oh My Zsh.");
        }
    } else {
        println!("Oh My Zsh already installed.");
    }
    // Install Powerlevel10k theme
    let p10k_dir = format!("{}/.oh-my-zsh/custom/themes/powerlevel10k", home);
    if !std::path::Path::new(&p10k_dir).exists() {
        let status = std::process::Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "https://github.com/romkatv/powerlevel10k.git",
                &p10k_dir,
            ])
            .status()
            .expect("failed to clone powerlevel10k");
        if status.success() {
            println!("Powerlevel10k theme installed.");
        } else {
            println!("Failed to install Powerlevel10k theme.");
        }
    } else {
        println!("Powerlevel10k already installed.");
    }
    // Install plugins
    let plugins = [
        (
            "zsh-autosuggestions",
            "https://github.com/zsh-users/zsh-autosuggestions.git",
        ),
        (
            "zsh-syntax-highlighting",
            "https://github.com/zsh-users/zsh-syntax-highlighting.git",
        ),
        (
            "zsh-completions",
            "https://github.com/zsh-users/zsh-completions.git",
        ),
        (
            "zsh-history-substring-search",
            "https://github.com/zsh-users/zsh-history-substring-search.git",
        ),
    ];
    for (name, url) in plugins.iter() {
        let plugin_dir = format!("{}/.oh-my-zsh/custom/plugins/{}", home, name);
        if !std::path::Path::new(&plugin_dir).exists() {
            let status = std::process::Command::new("git")
                .args(["clone", url, &plugin_dir])
                .status()
                .expect("failed to clone plugin");
            if status.success() {
                println!("{} installed.", name);
            } else {
                println!("Failed to install {}.", name);
            }
        } else {
            println!("{} already installed.", name);
        }
    }
    // Update .zshrc
    let zshrc_path = format!("{}/.zshrc", home);
    if let Ok(mut zshrc) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&zshrc_path)
    {
        use std::io::Write;
        writeln!(zshrc, "\n# ghostctl zsh config").ok();
        writeln!(zshrc, "ZSH_THEME=\"powerlevel10k/powerlevel10k\"").ok();
        writeln!(zshrc, "plugins=(git sudo zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-history-substring-search colored-man-pages)").ok();
        writeln!(zshrc, "source $ZSH/oh-my-zsh.sh").ok();
        writeln!(zshrc, "[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh").ok();
        println!(".zshrc updated with Powerlevel10k and plugins.");
    }
}
