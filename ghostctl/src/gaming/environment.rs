use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn environment_menu() {
    loop {
        let options = [
            "🔧 System Environment Setup",
            "🎮 Gaming-specific Environment Variables",
            "📚 Library & Runtime Setup",
            "🔊 Audio Environment",
            "🖥️  Display Environment",
            "📋 Environment Status",
            "⬅️  Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔧 Gaming Environment Setup")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => system_environment_setup(),
            1 => gaming_environment_variables(),
            2 => library_runtime_setup(),
            3 => audio_environment(),
            4 => display_environment(),
            5 => environment_status(),
            _ => break,
        }
    }
}

fn system_environment_setup() {
    println!("🔧 System Environment Setup");
    println!("===========================");

    println!("💡 Gaming system optimizations:");
    println!("  • Enable multilib repository");
    println!("  • Install gaming libraries");
    println!("  • Configure kernel parameters");
    println!("  • Set up gaming groups");

    let Ok(setup_multilib) = Confirm::new()
        .with_prompt("Setup multilib repository for 32-bit game support?")
        .default(true)
        .interact()
    else {
        return;
    };

    if setup_multilib {
        setup_multilib_repo();
    }

    let Ok(install_libs) = Confirm::new()
        .with_prompt("Install essential gaming libraries?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_libs {
        install_gaming_libraries();
    }
}

fn setup_multilib_repo() {
    println!("📦 Setting up multilib repository...");

    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();

    match multilib_check {
        Ok(out) if out.stdout.is_empty() => {
            println!("🔧 Enabling multilib repository...");
            let status = Command::new("sudo")
                .arg("sed")
                .args(&[
                    "-i",
                    "/^#\\[multilib\\]/,/^#Include = \\/etc\\/pacman.d\\/mirrorlist/ s/^#//",
                    "/etc/pacman.conf",
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Multilib repository enabled");
                    let _ = Command::new("sudo").args(&["pacman", "-Sy"]).status();
                }
                _ => println!("❌ Failed to enable multilib"),
            }
        }
        Ok(_) => println!("✅ Multilib repository already enabled"),
        _ => println!("❌ Could not check multilib status"),
    }
}

fn install_gaming_libraries() {
    println!("📚 Installing essential gaming libraries...");

    let libraries = [
        "lib32-mesa",
        "lib32-vulkan-radeon",
        "lib32-vulkan-intel",
        "lib32-vulkan-mesa-layers",
        "lib32-alsa-plugins",
        "lib32-libpulse",
        "lib32-openal",
        "lib32-gst-plugins-base-libs",
        "vulkan-tools",
        "gamemode",
        "mangohud",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&libraries)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Gaming libraries installed"),
        _ => println!("❌ Failed to install some libraries"),
    }
}

fn gaming_environment_variables() {
    println!("🎮 Gaming Environment Variables");
    println!("===============================");

    println!("💡 Common gaming environment variables:");
    println!("  export DXVK_HUD=fps                    # Show FPS counter");
    println!("  export DXVK_LOG_LEVEL=info             # DXVK logging");
    println!("  export VKD3D_DEBUG=warn                # VKD3D debugging");
    println!("  export PROTON_LOG=1                    # Proton logging");
    println!("  export WINE_LARGE_ADDRESS_AWARE=1      # Wine memory");
    println!("  export __GL_SHADER_DISK_CACHE=1        # NVIDIA shader cache");
    println!("  export __GL_THREADED_OPTIMIZATIONS=1   # NVIDIA threading");

    println!("\n🔧 GameMode variables:");
    println!("  export GAMEMODERUNEXEC=gamemoderun     # GameMode execution");

    println!("\n📊 MangoHud variables:");
    println!("  export MANGOHUD=1                      # Enable MangoHud");
    println!("  export MANGOHUD_CONFIG=fps,cpu,gpu     # Configure overlay");

    let Ok(setup_profile) = Confirm::new()
        .with_prompt("Add gaming environment to ~/.profile?")
        .default(false)
        .interact()
    else {
        return;
    };

    if setup_profile {
        setup_gaming_profile();
    }
}

fn setup_gaming_profile() {
    let gaming_env = r#"
# Gaming Environment Variables
export DXVK_HUD=fps
export VKD3D_DEBUG=warn
export WINE_LARGE_ADDRESS_AWARE=1
export __GL_SHADER_DISK_CACHE=1
export __GL_THREADED_OPTIMIZATIONS=1
"#;

    let profile_path = std::env::home_dir()
        .map(|h| h.join(".profile"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.profile"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&profile_path)
    {
        if writeln!(file, "{}", gaming_env).is_err() {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ Gaming environment added to ~/.profile");
            println!("💡 Source ~/.profile or restart shell to apply changes");
        }
    } else {
        println!("❌ Could not open ~/.profile for writing");
    }
}

fn library_runtime_setup() {
    println!("📚 Library & Runtime Setup");
    println!("==========================");

    let setup_options = [
        "🍷 Wine & Proton Dependencies",
        "🎮 Steam Runtime",
        "📦 Flatpak Gaming Runtimes",
        "🔧 System Libraries Check",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Library Setup")
        .items(&setup_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => setup_wine_dependencies(),
        1 => setup_steam_runtime(),
        2 => setup_flatpak_runtimes(),
        3 => check_system_libraries(),
        _ => return,
    }
}

fn setup_wine_dependencies() {
    println!("🍷 Setting up Wine & Proton Dependencies");

    let wine_deps = [
        "wine",
        "winetricks",
        "dxvk",
        "vkd3d",
        "lib32-freetype2",
        "lib32-gnutls",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&wine_deps)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Wine dependencies installed"),
        _ => println!("❌ Failed to install Wine dependencies"),
    }
}

fn setup_steam_runtime() {
    println!("🎮 Steam Runtime Setup");
    println!("======================");

    println!("💡 Steam runtime is managed automatically by Steam");
    println!("🔧 You can force runtime refresh by:");
    println!("  rm -rf ~/.steam/steam/ubuntu12_32/steam-runtime/");
    println!("  steam --reset");
}

fn setup_flatpak_runtimes() {
    println!("📦 Flatpak Gaming Runtimes");
    println!("==========================");

    let flatpak_check = Command::new("which").arg("flatpak").status();
    match flatpak_check {
        Ok(s) if s.success() => {
            let runtimes = [
                "org.freedesktop.Platform.GL.default//23.08",
                "org.freedesktop.Platform.GL32.default//23.08",
                "org.freedesktop.Platform.VAAPI.Intel//23.08",
                "org.freedesktop.Platform.ffmpeg-full//23.08",
            ];

            for runtime in &runtimes {
                let install_status = Command::new("flatpak")
                    .args(&["install", "-y", "flathub", runtime])
                    .status();

                match install_status {
                    Ok(s) if s.success() => println!("  ✅ {}", runtime),
                    _ => println!("  ❌ Failed: {}", runtime),
                }
            }
        }
        _ => println!("❌ Flatpak not found"),
    }
}

fn check_system_libraries() {
    println!("🔧 System Libraries Check");
    println!("=========================");

    let libraries = [
        ("OpenGL", "glxinfo"),
        ("Vulkan", "vulkaninfo"),
        ("ALSA", "aplay"),
        ("PulseAudio", "pulseaudio"),
        ("DXVK", "dxvk-setup"),
        ("GameMode", "gamemoderun"),
        ("MangoHud", "mangohud"),
    ];

    for (name, cmd) in &libraries {
        let status = Command::new("which").arg(cmd).status();
        match status {
            Ok(s) if s.success() => println!("  ✅ {} available", name),
            _ => println!("  ❌ {} not found", name),
        }
    }
}

fn audio_environment() {
    println!("🔊 Audio Environment Setup");
    println!("==========================");

    println!("💡 Audio system recommendations:");
    println!("  • PipeWire (modern, low-latency)");
    println!("  • PulseAudio (traditional, stable)");
    println!("  • JACK (professional audio)");

    let audio_options = [
        "🎵 Setup PipeWire",
        "🔊 Setup PulseAudio",
        "🎚️  Setup JACK",
        "📋 Audio Status",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Audio Environment")
        .items(&audio_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => setup_pipewire(),
        1 => setup_pulseaudio(),
        2 => setup_jack(),
        3 => audio_status(),
        _ => return,
    }
}

fn setup_pipewire() {
    println!("🎵 Setting up PipeWire");

    let pipewire_packages = [
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "lib32-pipewire",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&pipewire_packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ PipeWire installed");
            println!("🔧 Enable with: systemctl --user enable --now pipewire");
        }
        _ => println!("❌ Failed to install PipeWire"),
    }
}

fn setup_pulseaudio() {
    println!("🔊 Setting up PulseAudio");

    let pulse_packages = [
        "pulseaudio",
        "pulseaudio-alsa",
        "lib32-libpulse",
        "lib32-alsa-plugins",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&pulse_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ PulseAudio installed"),
        _ => println!("❌ Failed to install PulseAudio"),
    }
}

fn setup_jack() {
    println!("🎚️  Setting up JACK");
    println!("💡 JACK is for professional audio, usually not needed for gaming");

    let jack_packages = ["jack2", "qjackctl"];
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&jack_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ JACK installed"),
        _ => println!("❌ Failed to install JACK"),
    }
}

fn audio_status() {
    println!("📋 Audio System Status");
    println!("======================");

    // Check audio systems
    let audio_systems = [
        ("PipeWire", "pipewire"),
        ("PulseAudio", "pulseaudio"),
        ("JACK", "jackd"),
    ];

    for (name, process) in &audio_systems {
        let running = Command::new("pgrep").arg(process).status();
        match running {
            Ok(s) if s.success() => println!("  🟢 {} running", name),
            _ => println!("  ⭕ {} not running", name),
        }
    }
}

fn display_environment() {
    println!("🖥️  Display Environment Setup");
    println!("=============================");

    println!("💡 Display server options:");
    println!("  • X11 (traditional, widely compatible)");
    println!("  • Wayland (modern, secure)");

    let display_options = [
        "🔍 Display Server Status",
        "🎮 Gaming Display Optimizations",
        "📊 Monitor Configuration",
        "🔧 Graphics Driver Status",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Display Environment")
        .items(&display_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => display_server_status(),
        1 => gaming_display_optimizations(),
        2 => monitor_configuration(),
        3 => graphics_driver_status(),
        _ => return,
    }
}

fn display_server_status() {
    println!("🔍 Display Server Status");
    println!("========================");

    if let Ok(display) = std::env::var("DISPLAY") {
        println!("📺 X11 Display: {}", display);
    }

    if let Ok(wayland) = std::env::var("WAYLAND_DISPLAY") {
        println!("🌊 Wayland Display: {}", wayland);
    }

    println!(
        "🖥️  Session Type: {}",
        std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string())
    );
}

fn gaming_display_optimizations() {
    println!("🎮 Gaming Display Optimizations");
    println!("===============================");

    println!("💡 Recommended optimizations:");
    println!("  • Disable compositor during gaming");
    println!("  • Use fullscreen exclusive mode");
    println!("  • Configure variable refresh rate (VRR)");
    println!("  • Set optimal resolution and refresh rate");

    println!("\n🔧 For GNOME/KDE:");
    println!("  • GNOME: Disable animations in settings");
    println!("  • KDE: System Settings > Display > Compositor");
}

fn monitor_configuration() {
    println!("📊 Monitor Configuration");
    println!("=======================");

    println!("🖥️  Detected displays:");
    let _ = Command::new("xrandr").arg("--listmonitors").status();

    println!("\n💡 Gaming monitor tips:");
    println!("  • Use native resolution");
    println!("  • Enable highest refresh rate");
    println!("  • Configure FreeSync/G-Sync if available");
}

fn graphics_driver_status() {
    println!("🔧 Graphics Driver Status");
    println!("=========================");

    let _ = Command::new("lspci")
        .args(&["-k", "|", "grep", "-A", "2", "-i", "VGA"])
        .status();

    println!("\n🎮 OpenGL info:");
    let _ = Command::new("glxinfo")
        .args(&["|", "grep", "OpenGL"])
        .status();

    println!("\n🌋 Vulkan info:");
    let _ = Command::new("vulkaninfo").args(&["--summary"]).status();
}

fn environment_status() {
    println!("📋 Gaming Environment Status");
    println!("============================");

    println!("🔧 System Environment:");
    system_environment_check();

    println!("\n🎮 Gaming Libraries:");
    gaming_libraries_check();

    println!("\n🔊 Audio Environment:");
    audio_environment_check();

    println!("\n🖥️  Display Environment:");
    display_environment_check();
}

fn system_environment_check() {
    // Check multilib
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();
    match multilib_check {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Multilib enabled"),
        _ => println!("  ❌ Multilib disabled"),
    }

    // Check gaming groups
    let groups_output = Command::new("groups").output();
    if let Ok(output) = groups_output {
        let groups = String::from_utf8_lossy(&output.stdout);
        if groups.contains("audio") {
            println!("  ✅ User in audio group");
        } else {
            println!("  ⚠️  User not in audio group");
        }

        if groups.contains("video") {
            println!("  ✅ User in video group");
        } else {
            println!("  ⚠️  User not in video group");
        }
    }
}

fn gaming_libraries_check() {
    let libraries = [
        ("Mesa", "glxinfo"),
        ("Vulkan", "vulkaninfo"),
        ("GameMode", "gamemoderun"),
        ("MangoHud", "mangohud"),
        ("DXVK", "dxvk-setup"),
    ];

    for (name, cmd) in &libraries {
        let status = Command::new("which").arg(cmd).status();
        match status {
            Ok(s) if s.success() => println!("  ✅ {} available", name),
            _ => println!("  ❌ {} missing", name),
        }
    }
}

fn audio_environment_check() {
    let audio_check = Command::new("pactl").arg("info").status();
    match audio_check {
        Ok(s) if s.success() => println!("  ✅ PulseAudio/PipeWire working"),
        _ => println!("  ❌ Audio system issues"),
    }
}

fn display_environment_check() {
    if std::env::var("DISPLAY").is_ok() {
        println!("  ✅ X11 available");
    }

    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        println!("  ✅ Wayland available");
    }

    let opengl_check = Command::new("glxinfo").arg("-B").status();
    match opengl_check {
        Ok(s) if s.success() => println!("  ✅ OpenGL working"),
        _ => println!("  ❌ OpenGL issues"),
    }
}
