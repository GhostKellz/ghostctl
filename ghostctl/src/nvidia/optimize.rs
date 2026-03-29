//! NVIDIA GPU Optimization Module
//!
//! Provides comprehensive GPU optimization features:
//! - Power management (persistence mode, power limits)
//! - Memory clock adjustment
//! - Fan curve configuration
//! - Performance level presets (powersave, balanced, performance, max)
//! - GPU monitoring integration

use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

/// Performance preset levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformancePreset {
    Powersave,
    Balanced,
    Performance,
    Max,
}

impl std::fmt::Display for PerformancePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformancePreset::Powersave => write!(f, "Powersave (Low power, reduced clocks)"),
            PerformancePreset::Balanced => write!(f, "Balanced (Default settings)"),
            PerformancePreset::Performance => write!(f, "Performance (High clocks)"),
            PerformancePreset::Max => write!(f, "Max Performance (Maximum clocks, full power)"),
        }
    }
}

/// GPU information structure
#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub power_limit: Option<u32>,
    pub power_limit_min: Option<u32>,
    pub power_limit_max: Option<u32>,
    pub memory_clock: Option<u32>,
    pub graphics_clock: Option<u32>,
    pub temperature: Option<u32>,
    pub fan_speed: Option<u32>,
    pub persistence_mode: bool,
}

/// Main optimize function (called from mod.rs)
pub fn optimize() {
    println!("ghostctl :: NVIDIA Optimization");
    println!("================================\n");

    loop {
        let options = [
            "Performance Presets (Quick Setup)",
            "Power Management",
            "Clock Settings",
            "Fan Control",
            "GPU Monitoring",
            "Show Current Settings",
            "Reset to Defaults",
            "Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("NVIDIA Optimization")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => break,
        };

        match choice {
            0 => performance_preset_menu(),
            1 => power_management_menu(),
            2 => clock_settings_menu(),
            3 => fan_control_menu(),
            4 => gpu_monitoring(),
            5 => show_current_settings(),
            6 => reset_to_defaults(),
            _ => break,
        }
    }
}

/// Apply a performance preset
fn performance_preset_menu() {
    println!("\n=== Performance Presets ===\n");

    let presets = [
        PerformancePreset::Powersave,
        PerformancePreset::Balanced,
        PerformancePreset::Performance,
        PerformancePreset::Max,
    ];

    let preset_names: Vec<String> = presets.iter().map(|p| p.to_string()).collect();

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Performance Preset")
        .items(&preset_names)
        .default(1)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    apply_preset(presets[choice]);
}

/// Apply a specific performance preset
pub fn apply_preset(preset: PerformancePreset) {
    println!("\nApplying preset: {:?}", preset);

    match preset {
        PerformancePreset::Powersave => {
            // Enable persistence mode for faster wake
            set_persistence_mode(true);
            // Set power limit to minimum
            if let Some(gpu) = get_gpu_info(0)
                && let Some(min_power) = gpu.power_limit_min
            {
                set_power_limit(0, min_power);
            }
            // Force low performance level
            set_performance_level("0");
            println!("Powersave preset applied - GPU will use minimal power");
        }
        PerformancePreset::Balanced => {
            set_persistence_mode(true);
            // Set power limit to default (usually 100% TDP)
            if let Some(gpu) = get_gpu_info(0) {
                // Calculate balanced as 80% of max
                if let (Some(min), Some(max)) = (gpu.power_limit_min, gpu.power_limit_max) {
                    let balanced = min + (max - min) * 80 / 100;
                    set_power_limit(0, balanced);
                }
            }
            set_performance_level("auto");
            println!("Balanced preset applied - Normal operation");
        }
        PerformancePreset::Performance => {
            set_persistence_mode(true);
            // Set power limit to 90% of max
            if let Some(gpu) = get_gpu_info(0)
                && let Some(max_power) = gpu.power_limit_max
            {
                set_power_limit(0, max_power * 90 / 100);
            }
            set_performance_level("3");
            println!("Performance preset applied - Higher clocks enabled");
        }
        PerformancePreset::Max => {
            set_persistence_mode(true);
            // Set power limit to maximum
            if let Some(gpu) = get_gpu_info(0)
                && let Some(max_power) = gpu.power_limit_max
            {
                set_power_limit(0, max_power);
            }
            set_performance_level("3");
            // Enable max clocks
            enable_max_clocks();
            println!("Max Performance preset applied - Maximum power and clocks");
        }
    }
}

/// Power management menu
fn power_management_menu() {
    println!("\n=== Power Management ===\n");

    let options = [
        "Enable Persistence Mode",
        "Disable Persistence Mode",
        "Set Power Limit",
        "Show Power Info",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Power Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            set_persistence_mode(true);
            println!("Persistence mode enabled");
        }
        1 => {
            set_persistence_mode(false);
            println!("Persistence mode disabled");
        }
        2 => set_power_limit_interactive(),
        3 => show_power_info(),
        _ => {}
    }
}

/// Enable or disable persistence mode
pub fn set_persistence_mode(enabled: bool) {
    let mode = if enabled { "1" } else { "0" };
    let status = Command::new("sudo")
        .args(["nvidia-smi", "-pm", mode])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!(
                "Persistence mode {}",
                if enabled { "enabled" } else { "disabled" }
            );
        }
        _ => println!("Failed to set persistence mode"),
    }
}

/// Set power limit for a GPU
pub fn set_power_limit(gpu_index: u32, watts: u32) {
    let gpu_arg = format!("-i {}", gpu_index);
    let power_arg = format!("-pl {}", watts);

    let status = Command::new("sudo")
        .args(["nvidia-smi", &gpu_arg, &power_arg])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Power limit set to {}W for GPU {}", watts, gpu_index);
        }
        _ => println!("Failed to set power limit"),
    }
}

/// Interactive power limit setting
fn set_power_limit_interactive() {
    if let Some(gpu) = get_gpu_info(0) {
        println!("Current power limit: {:?}W", gpu.power_limit);
        println!(
            "Range: {:?}W - {:?}W",
            gpu.power_limit_min, gpu.power_limit_max
        );

        let min = gpu.power_limit_min.unwrap_or(100);
        let max = gpu.power_limit_max.unwrap_or(500);

        let input: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter power limit ({}-{}W)", min, max))
            .interact()
        {
            Ok(i) => i,
            Err(_) => return,
        };

        if let Ok(watts) = input.parse::<u32>() {
            if watts >= min && watts <= max {
                set_power_limit(0, watts);
            } else {
                println!("Value out of range");
            }
        } else {
            println!("Invalid input");
        }
    } else {
        println!("Could not get GPU info");
    }
}

/// Show power information
fn show_power_info() {
    println!("\n=== Power Information ===\n");
    let _ = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,power.draw,power.limit,power.min_limit,power.max_limit,enforced.power.limit",
            "--format=csv",
        ])
        .status();
}

/// Clock settings menu
fn clock_settings_menu() {
    println!("\n=== Clock Settings ===\n");

    let options = [
        "Set Memory Clock Offset",
        "Set Graphics Clock Offset",
        "Lock Clocks to Maximum",
        "Unlock Clocks (Auto)",
        "Show Clock Info",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Clock Settings")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => set_memory_clock_offset(),
        1 => set_graphics_clock_offset(),
        2 => enable_max_clocks(),
        3 => reset_clocks(),
        4 => show_clock_info(),
        _ => {}
    }
}

/// Set memory clock offset
fn set_memory_clock_offset() {
    println!("Note: Memory clock offset requires X server with nvidia-settings");
    println!("Typical range: -500 to +1500 MHz offset\n");

    let input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter memory clock offset (MHz)")
        .default("0".to_string())
        .interact()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if let Ok(offset) = input.parse::<i32>() {
        let attr = format!("[gpu:0]/GPUMemoryTransferRateOffset[3]={}", offset);
        let status = Command::new("nvidia-settings").args(["-a", &attr]).status();

        match status {
            Ok(s) if s.success() => {
                println!("Memory clock offset set to {} MHz", offset);
            }
            _ => {
                println!("Failed to set memory clock offset");
                println!("Make sure X server is running and nvidia-settings is available");
            }
        }
    }
}

/// Set graphics clock offset
fn set_graphics_clock_offset() {
    println!("Note: Graphics clock offset requires X server with nvidia-settings");
    println!("Typical range: -200 to +200 MHz offset\n");

    let input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter graphics clock offset (MHz)")
        .default("0".to_string())
        .interact()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if let Ok(offset) = input.parse::<i32>() {
        let attr = format!("[gpu:0]/GPUGraphicsClockOffset[3]={}", offset);
        let status = Command::new("nvidia-settings").args(["-a", &attr]).status();

        match status {
            Ok(s) if s.success() => {
                println!("Graphics clock offset set to {} MHz", offset);
            }
            _ => {
                println!("Failed to set graphics clock offset");
                println!("Make sure X server is running and nvidia-settings is available");
            }
        }
    }
}

/// Enable maximum clocks
fn enable_max_clocks() {
    // Get max supported clocks
    let output = Command::new("nvidia-smi")
        .args([
            "--query-supported-clocks=memory,graphics",
            "--format=csv,noheader",
        ])
        .output();

    if let Ok(output) = output {
        let clocks = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = clocks.lines().collect();

        if let Some(first_line) = lines.first() {
            let parts: Vec<&str> = first_line.split(',').collect();
            if parts.len() >= 2 {
                let mem_clock = parts[0].trim().replace(" MHz", "");
                let gfx_clock = parts[1].trim().replace(" MHz", "");

                println!(
                    "Setting clocks to max: Memory {}MHz, Graphics {}MHz",
                    mem_clock, gfx_clock
                );

                let status = Command::new("sudo")
                    .args(["nvidia-smi", "-lgc", &gfx_clock, "-lmc", &mem_clock])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("Maximum clocks locked");
                    }
                    _ => println!("Failed to lock clocks"),
                }
            }
        }
    }
}

/// Reset clocks to auto
fn reset_clocks() {
    let status = Command::new("sudo").args(["nvidia-smi", "-rgc"]).status();

    match status {
        Ok(s) if s.success() => {
            println!("Graphics clocks reset to auto");
        }
        _ => println!("Failed to reset graphics clocks"),
    }

    let status = Command::new("sudo").args(["nvidia-smi", "-rmc"]).status();

    match status {
        Ok(s) if s.success() => {
            println!("Memory clocks reset to auto");
        }
        _ => println!("Failed to reset memory clocks"),
    }
}

/// Show clock information
fn show_clock_info() {
    println!("\n=== Clock Information ===\n");
    let _ = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,clocks.current.memory,clocks.current.graphics,clocks.max.memory,clocks.max.graphics",
            "--format=csv",
        ])
        .status();
}

/// Set performance level
fn set_performance_level(level: &str) {
    // Performance levels: 0=max power savings, 1-2=balanced, 3=max performance
    // "auto" lets the driver decide
    let attr = format!("[gpu:0]/GpuPowerMizerMode={}", level);
    let _ = Command::new("nvidia-settings").args(["-a", &attr]).status();
}

/// Fan control menu
fn fan_control_menu() {
    println!("\n=== Fan Control ===\n");

    let options = [
        "Enable Manual Fan Control",
        "Set Fan Speed",
        "Apply Fan Curve",
        "Return to Automatic Control",
        "Show Fan Status",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Fan Control")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => enable_manual_fan_control(),
        1 => set_fan_speed_interactive(),
        2 => apply_fan_curve(),
        3 => disable_manual_fan_control(),
        4 => show_fan_status(),
        _ => {}
    }
}

/// Enable manual fan control
fn enable_manual_fan_control() {
    let attr = "[gpu:0]/GPUFanControlState=1";
    let status = Command::new("nvidia-settings").args(["-a", attr]).status();

    match status {
        Ok(s) if s.success() => {
            println!("Manual fan control enabled");
        }
        _ => {
            println!("Failed to enable manual fan control");
            println!("Make sure X server is running and nvidia-settings is available");
        }
    }
}

/// Disable manual fan control (return to automatic)
fn disable_manual_fan_control() {
    let attr = "[gpu:0]/GPUFanControlState=0";
    let status = Command::new("nvidia-settings").args(["-a", attr]).status();

    match status {
        Ok(s) if s.success() => {
            println!("Automatic fan control restored");
        }
        _ => println!("Failed to restore automatic fan control"),
    }
}

/// Set fan speed interactively
fn set_fan_speed_interactive() {
    println!("Note: Manual fan control must be enabled first");

    let input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter fan speed (30-100%)")
        .default("60".to_string())
        .interact()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if let Ok(speed) = input.parse::<u32>() {
        if (30..=100).contains(&speed) {
            set_fan_speed(0, speed);
        } else {
            println!("Speed must be between 30-100%");
        }
    }
}

/// Set fan speed for a GPU
pub fn set_fan_speed(gpu_index: u32, speed_percent: u32) {
    let attr = format!("[fan:{}]/GPUTargetFanSpeed={}", gpu_index, speed_percent);
    let status = Command::new("nvidia-settings").args(["-a", &attr]).status();

    match status {
        Ok(s) if s.success() => {
            println!("Fan speed set to {}%", speed_percent);
        }
        _ => println!("Failed to set fan speed"),
    }
}

/// Apply a predefined fan curve
fn apply_fan_curve() {
    println!("\n=== Fan Curve Options ===\n");

    let curves = [
        "Silent (Lower temps, higher fan speeds delayed)",
        "Balanced (Default-like behavior)",
        "Aggressive (Lower temps, more fan noise)",
        "Custom curve",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Fan Curve")
        .items(&curves)
        .default(1)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    // Enable manual control first
    enable_manual_fan_control();

    match choice {
        0 => {
            // Silent curve - higher temps before fans spin up
            println!("Applying silent fan curve...");
            println!(
                "Note: This sets a fixed 40% fan speed. For dynamic curves, use nvfancontrol."
            );
            set_fan_speed(0, 40);
        }
        1 => {
            // Balanced - reasonable cooling
            println!("Applying balanced fan curve...");
            set_fan_speed(0, 50);
        }
        2 => {
            // Aggressive - maximum cooling
            println!("Applying aggressive fan curve...");
            set_fan_speed(0, 70);
        }
        3 => {
            // Custom
            println!("\nFor advanced fan curves, consider using:");
            println!("  - nvfancontrol: https://github.com/foucault/nvfancontrol");
            println!("  - GreenWithEnvy (GWE): GTK app for NVIDIA control");
            println!("  - CoreCtrl: General GPU control app\n");

            set_fan_speed_interactive();
        }
        _ => {}
    }
}

/// Show fan status
fn show_fan_status() {
    println!("\n=== Fan Status ===\n");
    let _ = Command::new("nvidia-smi")
        .args(["--query-gpu=name,fan.speed,temperature.gpu", "--format=csv"])
        .status();
}

/// GPU monitoring
fn gpu_monitoring() {
    println!("\n=== GPU Monitoring ===\n");

    let options = [
        "Quick Status",
        "Detailed Stats",
        "Live Monitoring (watch mode)",
        "Process List (GPU usage)",
        "Power & Thermal",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GPU Monitoring")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            let _ = Command::new("nvidia-smi").status();
        }
        1 => {
            let _ = Command::new("nvidia-smi")
                .args(["-q", "-d", "PERFORMANCE,POWER,CLOCK,TEMPERATURE"])
                .status();
        }
        2 => {
            println!("Starting live monitoring (Ctrl+C to exit)...\n");
            let _ = Command::new("nvidia-smi")
                .args(["--loop=1", "--format=csv", "--query-gpu=timestamp,name,utilization.gpu,utilization.memory,memory.used,memory.total,temperature.gpu,power.draw,clocks.current.graphics,clocks.current.memory"])
                .status();
        }
        3 => {
            let _ = Command::new("nvidia-smi")
                .args(["pmon", "-s", "um", "-c", "10"])
                .status();
        }
        4 => {
            let _ = Command::new("nvidia-smi")
                .args([
                    "--query-gpu=name,power.draw,power.limit,temperature.gpu,temperature.memory,fan.speed",
                    "--format=csv",
                ])
                .status();
        }
        _ => {}
    }
}

/// Show current settings
fn show_current_settings() {
    println!("\n=== Current GPU Settings ===\n");

    if let Some(gpu) = get_gpu_info(0) {
        println!("GPU: {}", gpu.name);
        println!(
            "Persistence Mode: {}",
            if gpu.persistence_mode {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!("Power Limit: {:?}W", gpu.power_limit);
        println!(
            "Power Range: {:?}W - {:?}W",
            gpu.power_limit_min, gpu.power_limit_max
        );
        println!("Graphics Clock: {:?} MHz", gpu.graphics_clock);
        println!("Memory Clock: {:?} MHz", gpu.memory_clock);
        println!("Temperature: {:?}C", gpu.temperature);
        println!("Fan Speed: {:?}%", gpu.fan_speed);
    } else {
        // Fallback to nvidia-smi output
        let _ = Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,persistence_mode,power.limit,power.min_limit,power.max_limit,clocks.current.graphics,clocks.current.memory,temperature.gpu,fan.speed",
                "--format=csv",
            ])
            .status();
    }
}

/// Reset GPU to default settings
fn reset_to_defaults() {
    println!("\nResetting GPU to defaults...\n");

    // Reset clocks
    reset_clocks();

    // Reset power limit to default
    let status = Command::new("sudo")
        .args(["nvidia-smi", "-i", "0", "-pl", "default"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Power limit reset to default");
        }
        _ => println!("Note: Could not reset power limit"),
    }

    // Reset fan control to automatic
    disable_manual_fan_control();

    // Enable persistence mode by default (recommended)
    set_persistence_mode(true);

    println!("\nGPU reset to defaults complete");
}

/// Get GPU information
pub fn get_gpu_info(gpu_index: u32) -> Option<GpuInfo> {
    let output = Command::new("nvidia-smi")
        .args([
            &format!("-i {}", gpu_index),
            "--query-gpu=name,persistence_mode,power.limit,power.min_limit,power.max_limit,clocks.current.memory,clocks.current.graphics,temperature.gpu,fan.speed",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    let info_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = info_str.trim().split(',').collect();

    if parts.len() < 9 {
        return None;
    }

    Some(GpuInfo {
        index: gpu_index,
        name: parts[0].trim().to_string(),
        persistence_mode: parts[1].trim() == "Enabled",
        power_limit: parts[2].trim().parse().ok(),
        power_limit_min: parts[3].trim().parse().ok(),
        power_limit_max: parts[4].trim().parse().ok(),
        memory_clock: parts[5].trim().parse().ok(),
        graphics_clock: parts[6].trim().parse().ok(),
        temperature: parts[7].trim().parse().ok(),
        fan_speed: parts[8].trim().parse().ok(),
    })
}

/// Quick optimization for gaming
pub fn optimize_for_gaming() {
    println!("Optimizing GPU for gaming...");
    apply_preset(PerformancePreset::Performance);
}

/// Quick optimization for power saving
pub fn optimize_for_power_saving() {
    println!("Optimizing GPU for power saving...");
    apply_preset(PerformancePreset::Powersave);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_preset_display() {
        assert!(
            PerformancePreset::Powersave
                .to_string()
                .contains("Low power")
        );
        assert!(PerformancePreset::Balanced.to_string().contains("Default"));
        assert!(PerformancePreset::Performance.to_string().contains("High"));
        assert!(PerformancePreset::Max.to_string().contains("Maximum"));
    }

    #[test]
    fn test_gpu_info_struct() {
        let info = GpuInfo {
            index: 0,
            name: "Test GPU".to_string(),
            power_limit: Some(350),
            power_limit_min: Some(200),
            power_limit_max: Some(450),
            memory_clock: Some(2000),
            graphics_clock: Some(1500),
            temperature: Some(65),
            fan_speed: Some(45),
            persistence_mode: true,
        };

        assert_eq!(info.index, 0);
        assert_eq!(info.name, "Test GPU");
        assert!(info.persistence_mode);
    }
}
