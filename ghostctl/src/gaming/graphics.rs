use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn graphics_menu() {
    loop {
        let options = [
            "🎨 Graphics Driver Management",
            "🌋 Vulkan Setup & Optimization",
            "🔧 OpenGL Configuration",
            "⚡ Graphics Performance Tuning",
            "🖥️  Multi-GPU Setup",
            "🎮 Game-specific Graphics Fixes",
            "📊 Graphics Diagnostics",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎨 Graphics & Compatibility")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => graphics_driver_management(),
            1 => vulkan_setup(),
            2 => opengl_configuration(),
            3 => graphics_performance_tuning(),
            4 => multi_gpu_setup(),
            5 => game_specific_fixes(),
            6 => graphics_diagnostics(),
            _ => break,
        }
    }
}

fn graphics_driver_management() {
    println!("🎨 Graphics Driver Management");
    println!("=============================");

    let driver_options = [
        "🟢 NVIDIA Driver Management",
        "🔴 AMD Driver Management", 
        "🔵 Intel Driver Management",
        "📦 Install Common Graphics Libraries",
        "🔧 Driver Status Check",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Graphics Driver Management")
        .items(&driver_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => nvidia_driver_management(),
        1 => amd_driver_management(),
        2 => intel_driver_management(),
        3 => install_graphics_libraries(),
        4 => driver_status_check(),
        _ => return,
    }
}

fn nvidia_driver_management() {
    println!("🟢 NVIDIA Driver Management");
    println!("===========================");
    
    let nvidia_options = [
        "📦 Install NVIDIA Drivers",
        "🐳 Install NVIDIA Container Toolkit",
        "⚡ Install NVIDIA Performance Tools",
        "🔧 NVIDIA Settings",
        "📊 NVIDIA Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("NVIDIA Management")
        .items(&nvidia_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_nvidia_drivers(),
        1 => install_nvidia_container_toolkit(),
        2 => install_nvidia_performance_tools(),
        3 => launch_nvidia_settings(),
        4 => nvidia_status(),
        _ => return,
    }
}

fn install_nvidia_drivers() {
    println!("📦 Installing NVIDIA Drivers");
    println!("============================");

    let confirm = Confirm::new()
        .with_prompt("Install NVIDIA drivers and utilities?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let nvidia_packages = [
            "nvidia",
            "nvidia-utils",
            "lib32-nvidia-utils",
            "nvidia-settings",
            "vulkan-tools",
            "lib32-vulkan-mesa-layers",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&nvidia_packages)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ NVIDIA drivers installed");
                println!("⚠️  Reboot required to load new drivers");
            }
            _ => println!("❌ Failed to install NVIDIA drivers"),
        }
    }
}

fn install_nvidia_container_toolkit() {
    println!("🐳 Installing NVIDIA Container Toolkit");
    
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "nvidia-container-toolkit"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("✅ NVIDIA Container Toolkit installed");
                        return;
                    }
                    _ => println!("❌ Failed to install with {}", helper),
                }
            }
        }
    }
    
    println!("❌ No AUR helper found. Install yay first.");
}

fn install_nvidia_performance_tools() {
    println!("⚡ Installing NVIDIA Performance Tools");
    
    let tools = ["nvtop", "nvidia-ml-py"];
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&tools)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ NVIDIA performance tools installed"),
        _ => println!("❌ Failed to install performance tools"),
    }
}

fn launch_nvidia_settings() {
    println!("🔧 Launching NVIDIA Settings");
    let _ = Command::new("nvidia-settings").spawn();
}

fn nvidia_status() {
    println!("📊 NVIDIA Status");
    println!("================");
    
    let nvidia_smi = Command::new("nvidia-smi").status();
    match nvidia_smi {
        Ok(s) if s.success() => {
            println!("✅ NVIDIA drivers working");
            let _ = Command::new("nvidia-smi").status();
        }
        _ => println!("❌ NVIDIA drivers not working"),
    }
}

fn amd_driver_management() {
    println!("🔴 AMD Driver Management");
    println!("========================");
    
    let amd_options = [
        "📦 Install AMD Drivers",
        "🌋 Install AMD Vulkan Drivers",
        "⚡ Install AMD Performance Tools",
        "🔧 AMD GPU Configuration",
        "📊 AMD Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AMD Management")
        .items(&amd_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_amd_drivers(),
        1 => install_amd_vulkan(),
        2 => install_amd_performance_tools(),
        3 => amd_gpu_configuration(),
        4 => amd_status(),
        _ => return,
    }
}

fn install_amd_drivers() {
    println!("📦 Installing AMD Drivers");
    
    let amd_packages = [
        "mesa",
        "lib32-mesa",
        "vulkan-radeon",
        "lib32-vulkan-radeon",
        "libva-mesa-driver",
        "lib32-libva-mesa-driver",
        "mesa-vdpau",
        "lib32-mesa-vdpau",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&amd_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ AMD drivers installed"),
        _ => println!("❌ Failed to install AMD drivers"),
    }
}

fn install_amd_vulkan() {
    println!("🌋 Installing AMD Vulkan Drivers");
    
    let vulkan_packages = [
        "vulkan-radeon",
        "lib32-vulkan-radeon",
        "vulkan-mesa-layers",
        "lib32-vulkan-mesa-layers",
        "vulkan-tools",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&vulkan_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ AMD Vulkan drivers installed"),
        _ => println!("❌ Failed to install Vulkan drivers"),
    }
}

fn install_amd_performance_tools() {
    println!("⚡ Installing AMD Performance Tools");
    
    let _tools = ["radeontop", "corectrl"];
    
    // Install from repos
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm", "radeontop"])
        .status();

    match status {
        Ok(s) if s.success() => println!("  ✅ radeontop installed"),
        _ => println!("  ❌ Failed to install radeontop"),
    }

    // Try to install corectrl from AUR
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "corectrl"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("  ✅ corectrl installed");
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("  💡 corectrl available via AUR");
}

fn amd_gpu_configuration() {
    println!("🔧 AMD GPU Configuration");
    println!("========================");
    
    println!("💡 AMD GPU optimizations:");
    println!("  • Enable GPU scheduling: echo 'amdgpu.gpu_recovery=1' | sudo tee -a /etc/modprobe.d/amdgpu.conf");
    println!("  • Force performance mode: echo 'performance' | sudo tee /sys/class/drm/card0/device/power_dpm_force_performance_level");
    println!("  • Configure fan curves with corectrl");
    
    let apply_optimizations = Confirm::new()
        .with_prompt("Apply basic AMD optimizations?")
        .default(false)
        .interact()
        .unwrap();

    if apply_optimizations {
        apply_amd_optimizations();
    }
}

fn apply_amd_optimizations() {
    println!("🔧 Applying AMD optimizations...");
    
    // Create modprobe config
    let modprobe_config = "options amdgpu gpu_recovery=1\n";
    let config_path = "/etc/modprobe.d/amdgpu.conf";
    
    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo '{}' >> {}", modprobe_config, config_path))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ AMD GPU recovery enabled"),
        _ => println!("❌ Failed to configure AMD GPU"),
    }
}

fn amd_status() {
    println!("📊 AMD Status");
    println!("=============");
    
    let lspci_output = Command::new("lspci")
        .args(&["-k"])
        .output();
    
    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        if lspci.contains("AMD") || lspci.contains("Radeon") {
            println!("✅ AMD GPU detected");
            
            let glxinfo = Command::new("glxinfo").args(&["-B"]).status();
            match glxinfo {
                Ok(s) if s.success() => println!("✅ AMD graphics working"),
                _ => println!("❌ Graphics driver issues"),
            }
        } else {
            println!("❌ No AMD GPU detected");
        }
    }
}

fn intel_driver_management() {
    println!("🔵 Intel Driver Management");
    println!("==========================");
    
    let intel_packages = [
        "mesa",
        "lib32-mesa",
        "vulkan-intel",
        "lib32-vulkan-intel",
        "intel-media-driver",
        "libva-intel-driver",
    ];

    let confirm = Confirm::new()
        .with_prompt("Install Intel graphics drivers?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&intel_packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Intel drivers installed"),
            _ => println!("❌ Failed to install Intel drivers"),
        }
    }
}

fn install_graphics_libraries() {
    println!("📦 Installing Common Graphics Libraries");
    println!("=======================================");
    
    let graphics_libs = [
        "mesa",
        "lib32-mesa",
        "vulkan-tools",
        "vulkan-mesa-layers",
        "lib32-vulkan-mesa-layers",
        "glxinfo",
        "vulkaninfo",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&graphics_libs)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Graphics libraries installed"),
        _ => println!("❌ Failed to install graphics libraries"),
    }
}

fn driver_status_check() {
    println!("🔧 Graphics Driver Status");
    println!("=========================");
    
    println!("🖥️  Hardware detected:");
    let _ = Command::new("lspci").args(&["-k", "|", "grep", "-A", "2", "-E", "(VGA|3D)"]).status();
    
    println!("\n🎮 OpenGL status:");
    let _ = Command::new("glxinfo").args(&["-B"]).status();
    
    println!("\n🌋 Vulkan status:");
    let _ = Command::new("vulkaninfo").args(&["--summary"]).status();
}

fn vulkan_setup() {
    println!("🌋 Vulkan Setup & Optimization");
    println!("==============================");

    let vulkan_options = [
        "📦 Install Vulkan Drivers",
        "🔧 Vulkan Layer Configuration",
        "📊 Vulkan Validation Layers",
        "⚡ Vulkan Performance Tuning",
        "🧪 Vulkan Testing Tools",
        "📋 Vulkan Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Vulkan Setup")
        .items(&vulkan_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_vulkan_drivers(),
        1 => vulkan_layer_config(),
        2 => vulkan_validation_layers(),
        3 => vulkan_performance_tuning(),
        4 => vulkan_testing_tools(),
        5 => vulkan_status(),
        _ => return,
    }
}

fn install_vulkan_drivers() {
    println!("📦 Installing Vulkan Drivers");
    
    let vulkan_packages = [
        "vulkan-tools",
        "vulkan-mesa-layers",
        "lib32-vulkan-mesa-layers",
        "vulkan-validation-layers",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&vulkan_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Vulkan drivers installed"),
        _ => println!("❌ Failed to install Vulkan drivers"),
    }
}

fn vulkan_layer_config() {
    println!("🔧 Vulkan Layer Configuration");
    println!("=============================");
    
    println!("💡 Vulkan layers for gaming:");
    println!("  • VK_LAYER_MESA_overlay - Performance overlay");
    println!("  • VK_LAYER_KHRONOS_validation - Debugging");
    println!("  • VK_LAYER_LUNARG_monitor - Frame rate limiting");
    
    println!("\n🔧 Environment variables:");
    println!("  export VK_LAYER_PATH=/usr/share/vulkan/explicit_layer.d");
    println!("  export VK_INSTANCE_LAYERS=VK_LAYER_MESA_overlay");
}

fn vulkan_validation_layers() {
    println!("📊 Installing Vulkan Validation Layers");
    
    let validation_packages = [
        "vulkan-validation-layers",
        "vulkan-extra-layers",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&validation_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Validation layers installed"),
        _ => println!("❌ Failed to install validation layers"),
    }
}

fn vulkan_performance_tuning() {
    println!("⚡ Vulkan Performance Tuning");
    println!("============================");
    
    println!("💡 Vulkan optimization environment variables:");
    println!("  export VK_LAYER_MESA_OVERLAY_CONFIG=fps");
    println!("  export RADV_PERFTEST=aco");  // AMD specific
    println!("  export ACO_DEBUG=validateir,validatera");
    println!("  export MESA_VK_WSI_PRESENT_MODE=fifo");  // VSync
    
    let apply_config = Confirm::new()
        .with_prompt("Add Vulkan optimizations to ~/.profile?")
        .default(false)
        .interact()
        .unwrap();

    if apply_config {
        setup_vulkan_optimizations();
    }
}

fn setup_vulkan_optimizations() {
    let vulkan_env = r#"
# Vulkan Performance Optimizations
export VK_LAYER_MESA_OVERLAY_CONFIG=fps
export RADV_PERFTEST=aco
export MESA_VK_WSI_PRESENT_MODE=fifo
"#;

    let profile_path = std::env::home_dir()
        .map(|h| h.join(".profile"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.profile"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(&profile_path) {
        if let Err(_) = writeln!(file, "{}", vulkan_env) {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ Vulkan optimizations added to ~/.profile");
        }
    }
}

fn vulkan_testing_tools() {
    println!("🧪 Installing Vulkan Testing Tools");
    
    let test_tools = ["vkcube", "vulkan-tools"];
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&test_tools)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Vulkan testing tools installed");
            println!("🧪 Test with: vkcube");
        }
        _ => println!("❌ Failed to install testing tools"),
    }
}

fn vulkan_status() {
    println!("📋 Vulkan Status");
    println!("================");
    
    let vulkaninfo = Command::new("vulkaninfo").arg("--summary").status();
    match vulkaninfo {
        Ok(s) if s.success() => println!("✅ Vulkan working"),
        _ => println!("❌ Vulkan not working"),
    }
    
    println!("\n🔧 Available Vulkan devices:");
    let _ = Command::new("vulkaninfo").args(&["--summary"]).status();
}

fn opengl_configuration() {
    println!("🔧 OpenGL Configuration");
    println!("=======================");

    let opengl_options = [
        "📊 OpenGL Status & Info",
        "⚡ OpenGL Performance Settings",
        "🔧 Mesa Configuration",
        "🧪 OpenGL Testing",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("OpenGL Configuration")
        .items(&opengl_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => opengl_status(),
        1 => opengl_performance_settings(),
        2 => mesa_configuration(),
        3 => opengl_testing(),
        _ => return,
    }
}

fn opengl_status() {
    println!("📊 OpenGL Status & Information");
    println!("==============================");
    
    let glxinfo = Command::new("glxinfo").status();
    match glxinfo {
        Ok(s) if s.success() => {
            println!("✅ OpenGL working");
            let _ = Command::new("glxinfo").args(&["-B"]).status();
        }
        _ => println!("❌ OpenGL not working"),
    }
}

fn opengl_performance_settings() {
    println!("⚡ OpenGL Performance Settings");
    println!("==============================");
    
    println!("💡 OpenGL optimization environment variables:");
    println!("  export __GL_THREADED_OPTIMIZATIONS=1");
    println!("  export __GL_SHADER_DISK_CACHE=1");
    println!("  export __GL_SHADER_DISK_CACHE_PATH=~/.cache/gl_shader");
    println!("  export MESA_GL_VERSION_OVERRIDE=4.6");
    
    let apply_config = Confirm::new()
        .with_prompt("Add OpenGL optimizations to ~/.profile?")
        .default(false)
        .interact()
        .unwrap();

    if apply_config {
        setup_opengl_optimizations();
    }
}

fn setup_opengl_optimizations() {
    let opengl_env = r#"
# OpenGL Performance Optimizations
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SHADER_DISK_CACHE=1
export __GL_SHADER_DISK_CACHE_PATH=~/.cache/gl_shader
"#;

    let profile_path = std::env::home_dir()
        .map(|h| h.join(".profile"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.profile"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(&profile_path) {
        if let Err(_) = writeln!(file, "{}", opengl_env) {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ OpenGL optimizations added to ~/.profile");
        }
    }
}

fn mesa_configuration() {
    println!("🔧 Mesa Configuration");
    println!("=====================");
    
    println!("💡 Mesa environment variables:");
    println!("  export MESA_GL_VERSION_OVERRIDE=4.6");
    println!("  export MESA_GLSL_VERSION_OVERRIDE=460");
    println!("  export RADV_PERFTEST=aco");  // AMD specific
    println!("  export MESA_VK_WSI_PRESENT_MODE=fifo");
}

fn opengl_testing() {
    println!("🧪 OpenGL Testing");
    println!("=================");
    
    println!("🧪 OpenGL test commands:");
    println!("  glxgears         # Simple OpenGL test");
    println!("  glxinfo -B       # OpenGL information");
    println!("  mesa-demos       # Mesa demo applications");
    
    let install_demos = Confirm::new()
        .with_prompt("Install mesa-demos for testing?")
        .default(false)
        .interact()
        .unwrap();

    if install_demos {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "mesa-demos"])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ mesa-demos installed");
                println!("🧪 Test with: glxgears");
            }
            _ => println!("❌ Failed to install mesa-demos"),
        }
    }
}

fn graphics_performance_tuning() {
    println!("⚡ Graphics Performance Tuning");
    println!("==============================");

    let perf_options = [
        "🚀 GPU Frequency Scaling",
        "🌡️  Temperature Management",
        "⚡ Power Management",
        "🔧 Gaming-specific Tweaks",
        "📊 Performance Monitoring",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Tuning")
        .items(&perf_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => gpu_frequency_scaling(),
        1 => temperature_management(),
        2 => power_management(),
        3 => gaming_specific_tweaks(),
        4 => performance_monitoring(),
        _ => return,
    }
}

fn gpu_frequency_scaling() {
    println!("🚀 GPU Frequency Scaling");
    println!("========================");
    
    println!("💡 GPU frequency control:");
    println!("  • NVIDIA: nvidia-settings, nvidia-smi");
    println!("  • AMD: corectrl, amdgpu-clocks");
    println!("  • Intel: intel-gpu-tools");
    
    println!("\n⚠️  GPU overclocking can damage hardware!");
    println!("Always monitor temperatures and start conservative.");
}

fn temperature_management() {
    println!("🌡️  Temperature Management");
    println!("==========================");
    
    println!("💡 GPU temperature monitoring:");
    println!("  • sensors - System temperatures");
    println!("  • nvidia-smi - NVIDIA GPU temp");
    println!("  • radeontop - AMD GPU monitoring");
    
    let install_sensors = Confirm::new()
        .with_prompt("Install temperature monitoring tools?")
        .default(true)
        .interact()
        .unwrap();

    if install_sensors {
        let tools = ["lm_sensors", "nvtop", "radeontop"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&tools)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Temperature monitoring tools installed"),
            _ => println!("❌ Failed to install monitoring tools"),
        }
    }
}

fn power_management() {
    println!("⚡ Graphics Power Management");
    println!("============================");
    
    println!("💡 Power management modes:");
    println!("  • Performance - Maximum performance");
    println!("  • Balanced - Balance power and performance");
    println!("  • Power saving - Minimum power consumption");
    
    println!("\n🔧 For gaming, use performance mode");
}

fn gaming_specific_tweaks() {
    println!("🔧 Gaming-specific Graphics Tweaks");
    println!("===================================");
    
    println!("💡 Gaming optimizations:");
    println!("  • Disable compositor during gaming");
    println!("  • Force discrete GPU for games");
    println!("  • Enable GPU scheduling");
    println!("  • Configure shader cache");
    
    let gaming_tweaks = [
        "🎮 Setup Gaming GPU Profile",
        "🔧 Configure Shader Cache",
        "⚡ GPU Scheduling Tweaks",
        "🖥️  Compositor Settings",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Gaming Tweaks")
        .items(&gaming_tweaks)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_gaming_gpu_profile(),
        1 => configure_shader_cache(),
        2 => gpu_scheduling_tweaks(),
        3 => compositor_settings(),
        _ => return,
    }
}

fn setup_gaming_gpu_profile() {
    println!("🎮 Setting up Gaming GPU Profile");
    println!("================================");
    
    println!("💡 Creating performance profile for gaming...");
    println!("This will optimize GPU settings for maximum performance");
}

fn configure_shader_cache() {
    println!("🔧 Configuring Shader Cache");
    println!("===========================");
    
    let cache_dir = std::env::home_dir()
        .map(|h| h.join(".cache/gl_shader"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.cache/gl_shader"));

    if !cache_dir.exists() {
        if let Err(_) = std::fs::create_dir_all(&cache_dir) {
            println!("❌ Failed to create shader cache directory");
            return;
        }
    }

    println!("✅ Shader cache directory created: {}", cache_dir.display());
    
    let shader_env = format!("export __GL_SHADER_DISK_CACHE_PATH={}", cache_dir.display());
    println!("💡 Add to ~/.profile: {}", shader_env);
}

fn gpu_scheduling_tweaks() {
    println!("⚡ GPU Scheduling Tweaks");
    println!("========================");
    
    println!("💡 GPU scheduling optimizations:");
    println!("  • Enable GPU preemption");
    println!("  • Configure GPU priorities");
    println!("  • Optimize GPU memory management");
}

fn compositor_settings() {
    println!("🖥️  Compositor Settings for Gaming");
    println!("==================================");
    
    println!("💡 Desktop environment compositor settings:");
    println!("  • GNOME: Disable animations in Tweaks");
    println!("  • KDE: System Settings > Display > Compositor");
    println!("  • i3/Sway: Disable compositor during gaming");
    
    println!("\n🎮 For best gaming performance:");
    println!("  • Disable compositor");
    println!("  • Use fullscreen exclusive mode");
    println!("  • Disable desktop effects");
}

fn performance_monitoring() {
    println!("📊 Graphics Performance Monitoring");
    println!("==================================");
    
    let monitoring_tools = [
        "📊 Install MangoHud",
        "🔍 Install GPU Monitoring Tools",
        "📈 Setup Performance Logging", 
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Monitoring")
        .items(&monitoring_tools)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_mangohud(),
        1 => install_gpu_monitoring_tools(),
        2 => setup_performance_logging(),
        _ => return,
    }
}

fn install_mangohud() {
    println!("📊 Installing MangoHud");
    
    let mangohud_packages = ["mangohud", "lib32-mangohud"];
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&mangohud_packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ MangoHud installed");
            println!("🎮 Use with: mangohud <game_command>");
            println!("⚙️  Configure in: ~/.config/MangoHud/MangoHud.conf");
        }
        _ => println!("❌ Failed to install MangoHud"),
    }
}

fn install_gpu_monitoring_tools() {
    println!("🔍 Installing GPU Monitoring Tools");
    
    let tools = ["nvtop", "radeontop", "intel-gpu-tools"];
    for tool in &tools {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", tool])
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", tool),
            _ => println!("  💡 {} not available in repos", tool),
        }
    }
}

fn setup_performance_logging() {
    println!("📈 Setting up Performance Logging");
    println!("=================================");
    
    println!("💡 Performance logging options:");
    println!("  • MangoHud logging to file");
    println!("  • Custom performance scripts");
    println!("  • System monitoring with collectd");
}

fn multi_gpu_setup() {
    println!("🖥️  Multi-GPU Setup");
    println!("===================");

    let multi_gpu_options = [
        "🔍 Detect Multiple GPUs",
        "🎮 Configure Gaming GPU Priority", 
        "⚡ NVIDIA/AMD Hybrid Setup",
        "🔧 GPU Switching Configuration",
        "📊 Multi-GPU Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multi-GPU Setup")
        .items(&multi_gpu_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => detect_multiple_gpus(),
        1 => configure_gaming_gpu_priority(),
        2 => nvidia_amd_hybrid_setup(),
        3 => gpu_switching_configuration(),
        4 => multi_gpu_status(),
        _ => return,
    }
}

fn detect_multiple_gpus() {
    println!("🔍 Detecting Multiple GPUs");
    println!("==========================");
    
    println!("🖥️  Detected graphics devices:");
    let _ = Command::new("lspci").args(&["-k", "|", "grep", "-A", "2", "-E", "(VGA|3D)"]).status();
    
    println!("\n🌋 Vulkan devices:");
    let _ = Command::new("vulkaninfo").args(&["--summary"]).status();
}

fn configure_gaming_gpu_priority() {
    println!("🎮 Configuring Gaming GPU Priority");
    println!("===================================");
    
    println!("💡 GPU priority configuration:");
    println!("  • Set discrete GPU as default for games");
    println!("  • Configure GPU environment variables");
    println!("  • Setup per-application GPU selection");
}

fn nvidia_amd_hybrid_setup() {
    println!("⚡ NVIDIA/AMD Hybrid Setup");
    println!("==========================");
    
    println!("💡 Hybrid GPU configuration:");
    println!("  • PRIME configuration for NVIDIA/Intel");
    println!("  • DRI_PRIME for AMD/Intel");
    println!("  • Optimus/PRIME offloading");
}

fn gpu_switching_configuration() {
    println!("🔧 GPU Switching Configuration");
    println!("==============================");
    
    println!("💡 GPU switching methods:");
    println!("  • Runtime GPU switching");
    println!("  • Application-specific GPU selection");
    println!("  • Power management integration");
}

fn multi_gpu_status() {
    println!("📊 Multi-GPU Status");
    println!("===================");
    
    println!("🖥️  GPU Hardware:");
    let _ = Command::new("lspci").args(&["|", "grep", "-i", "vga"]).status();
    
    println!("\n⚡ GPU Power Status:");
    // Check GPU power states if available
    let _ = Command::new("cat").arg("/sys/class/drm/card*/device/power_state").status();
}

fn game_specific_fixes() {
    println!("🎮 Game-specific Graphics Fixes");
    println!("===============================");

    let fix_categories = [
        "🎯 DirectX/DXVK Fixes",
        "🌋 Vulkan Game Fixes",
        "🔧 OpenGL Game Fixes",
        "📱 Specific Game Workarounds",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game Fixes")
        .items(&fix_categories)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => directx_dxvk_fixes(),
        1 => vulkan_game_fixes(),
        2 => opengl_game_fixes(),
        3 => specific_game_workarounds(),
        _ => return,
    }
}

fn directx_dxvk_fixes() {
    println!("🎯 DirectX/DXVK Fixes");
    println!("=====================");
    
    println!("💡 Common DXVK fixes:");
    println!("  • DXVK_HUD=fps - Show performance overlay");
    println!("  • DXVK_LOG_LEVEL=info - Enable logging");
    println!("  • WINEDEBUG=-all - Disable Wine debug output");
    println!("  • DXVK_CONFIG_FILE=~/.config/dxvk.conf");
}

fn vulkan_game_fixes() {
    println!("🌋 Vulkan Game Fixes");
    println!("===================");
    
    println!("💡 Vulkan game workarounds:");
    println!("  • VK_INSTANCE_LAYERS=VK_LAYER_MESA_overlay");
    println!("  • RADV_PERFTEST=aco (AMD)");
    println!("  • ANV_ENABLE_PIPELINE_CACHE=1 (Intel)");
}

fn opengl_game_fixes() {
    println!("🔧 OpenGL Game Fixes");
    println!("===================");
    
    println!("💡 OpenGL compatibility fixes:");
    println!("  • MESA_GL_VERSION_OVERRIDE=4.6");
    println!("  • MESA_GLSL_VERSION_OVERRIDE=460");
    println!("  • __GL_THREADED_OPTIMIZATIONS=1");
}

fn specific_game_workarounds() {
    println!("📱 Specific Game Workarounds");
    println!("============================");
    
    println!("💡 Game-specific environment variables:");
    println!("  • Cyberpunk 2077: PROTON_NO_ESYNC=1");
    println!("  • GTA V: WINEDEBUG=-all,+wgl");
    println!("  • Witcher 3: DXVK_HUD=fps");
    println!("  • Elden Ring: PROTON_USE_WINED3D=1");
}

fn graphics_diagnostics() {
    println!("📊 Graphics Diagnostics");
    println!("=======================");

    let diagnostic_options = [
        "🔍 Hardware Information",
        "🎮 Graphics Driver Status",
        "🌋 Vulkan Diagnostics",
        "🔧 OpenGL Diagnostics",
        "⚡ Performance Analysis",
        "🧪 Graphics Tests",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Graphics Diagnostics")
        .items(&diagnostic_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => hardware_information(),
        1 => graphics_driver_diagnostics(),
        2 => vulkan_diagnostics(),
        3 => opengl_diagnostics(),
        4 => performance_analysis(),
        5 => graphics_tests(),
        _ => return,
    }
}

fn hardware_information() {
    println!("🔍 Graphics Hardware Information");
    println!("===============================");
    
    println!("🖥️  PCI Graphics Devices:");
    let _ = Command::new("lspci").args(&["-vnn", "|", "grep", "-A", "10", "-E", "(VGA|3D)"]).status();
    
    println!("\n💾 Graphics Memory:");
    let _ = Command::new("cat").arg("/proc/meminfo").args(&["|", "grep", "-i", "vmalloc"]).status();
}

fn graphics_driver_diagnostics() {
    println!("🎮 Graphics Driver Diagnostics");
    println!("==============================");
    
    println!("🔧 Loaded kernel modules:");
    let _ = Command::new("lsmod").args(&["|", "grep", "-E", "(nvidia|radeon|amdgpu|i915)"]).status();
    
    println!("\n📦 Installed graphics packages:");
    let _ = Command::new("pacman").args(&["-Q", "|", "grep", "-E", "(mesa|nvidia|vulkan)"]).status();
}

fn vulkan_diagnostics() {
    println!("🌋 Vulkan Diagnostics");
    println!("=====================");
    
    let vulkaninfo = Command::new("vulkaninfo").status();
    match vulkaninfo {
        Ok(s) if s.success() => {
            println!("✅ Vulkan working");
            let _ = Command::new("vulkaninfo").args(&["--summary"]).status();
        }
        _ => {
            println!("❌ Vulkan not working");
            println!("💡 Check Vulkan driver installation");
        }
    }
}

fn opengl_diagnostics() {
    println!("🔧 OpenGL Diagnostics");
    println!("=====================");
    
    let glxinfo = Command::new("glxinfo").status();
    match glxinfo {
        Ok(s) if s.success() => {
            println!("✅ OpenGL working");
            let _ = Command::new("glxinfo").args(&["-B"]).status();
        }
        _ => {
            println!("❌ OpenGL not working");
            println!("💡 Check graphics driver installation");
        }
    }
}

fn performance_analysis() {
    println!("⚡ Graphics Performance Analysis");
    println!("===============================");
    
    println!("📊 Performance monitoring tools:");
    println!("  • MangoHud - In-game overlay");
    println!("  • nvtop/radeontop - GPU monitoring");
    println!("  • glxgears - Basic OpenGL test");
    println!("  • vkcube - Vulkan test");
}

fn graphics_tests() {
    println!("🧪 Graphics Tests");
    println!("=================");
    
    let test_options = [
        "🔧 OpenGL Test (glxgears)",
        "🌋 Vulkan Test (vkcube)",
        "📊 Benchmark Test",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Graphics Tests")
        .items(&test_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => run_opengl_test(),
        1 => run_vulkan_test(),
        2 => run_benchmark_test(),
        _ => return,
    }
}

fn run_opengl_test() {
    println!("🔧 Running OpenGL Test");
    println!("======================");
    
    let glxgears = Command::new("glxgears").status();
    match glxgears {
        Ok(s) if s.success() => println!("✅ OpenGL test completed"),
        _ => println!("❌ OpenGL test failed"),
    }
}

fn run_vulkan_test() {
    println!("🌋 Running Vulkan Test");
    println!("======================");
    
    let vkcube = Command::new("vkcube").status();
    match vkcube {
        Ok(s) if s.success() => println!("✅ Vulkan test completed"),
        _ => println!("❌ Vulkan test failed"),
    }
}

fn run_benchmark_test() {
    println!("📊 Running Benchmark Test");
    println!("=========================");
    
    println!("💡 Available benchmarks:");
    println!("  • glmark2 - OpenGL benchmark");
    println!("  • vkmark - Vulkan benchmark");
    println!("  • Unigine Heaven/Valley");
    
    let install_glmark2 = Confirm::new()
        .with_prompt("Install and run glmark2 benchmark?")
        .default(false)
        .interact()
        .unwrap();

    if install_glmark2 {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "glmark2"])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ glmark2 installed");
                let _ = Command::new("glmark2").status();
            }
            _ => println!("❌ Failed to install glmark2"),
        }
    }
}