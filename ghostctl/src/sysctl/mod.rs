//! Sysctl/Kernel Parameter Browser Module
//!
//! Provides CLI and TUI interfaces for browsing and managing Linux kernel parameters.
//! Similar to systeroid - browses /proc/sys and provides documentation.

mod tui;

use crate::tui::{confirm, error, header, icons, info, input, select_with_back, success, warn};
use crate::utils::sudo_write_file;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub use tui::sysctl_tui;

/// Kernel parameter with metadata
#[derive(Debug, Clone)]
pub struct KernelParam {
    pub name: String,
    pub path: String,
    pub value: String,
    pub description: Option<String>,
    pub category: String,
}

/// Main sysctl management menu
pub fn sysctl_menu() {
    loop {
        header("Kernel Parameter Browser");

        let options = [
            "Launch TUI (Interactive Browser)",
            "List All Parameters",
            "Search Parameters",
            "View Parameter",
            "Set Parameter",
            "Show Categories",
            "Export Configuration",
            "Back",
        ];

        match select_with_back("Choose an option", &options, 0) {
            Some(0) => {
                if let Err(e) = sysctl_tui() {
                    error(&format!("Failed to launch Sysctl TUI: {}", e));
                }
            }
            Some(1) => list_all_parameters(),
            Some(2) => search_parameters(),
            Some(3) => view_parameter(),
            Some(4) => set_parameter(),
            Some(5) => show_categories(),
            Some(6) => export_configuration(),
            _ => break,
        }
    }
}

/// Get all kernel parameters from /proc/sys
pub fn get_all_parameters() -> Vec<KernelParam> {
    let mut params = Vec::new();
    collect_parameters(Path::new("/proc/sys"), "", &mut params);
    params.sort_by(|a, b| a.name.cmp(&b.name));
    params
}

/// Recursively collect parameters from a directory
fn collect_parameters(dir: &Path, prefix: &str, params: &mut Vec<KernelParam>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            let full_name = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", prefix, name)
            };

            if path.is_dir() {
                collect_parameters(&path, &full_name, params);
            } else if path.is_file() {
                // Read the value
                let value = fs::read_to_string(&path)
                    .map(|v| v.trim().to_string())
                    .unwrap_or_else(|_| "<unreadable>".to_string());

                // Determine category from the path
                let category = prefix.split('.').next().unwrap_or("other").to_string();

                params.push(KernelParam {
                    name: full_name,
                    path: path.to_string_lossy().to_string(),
                    value,
                    description: get_param_description(&name),
                    category,
                });
            }
        }
    }
}

/// Get description for common kernel parameters
fn get_param_description(name: &str) -> Option<String> {
    let descriptions: HashMap<&str, &str> = [
        ("swappiness", "How aggressively the kernel swaps out anonymous memory (0-200)"),
        ("dirty_ratio", "Percentage of total RAM that can be filled with dirty pages before processes are forced to write dirty buffers"),
        ("dirty_background_ratio", "Percentage of total RAM for background writeout of dirty data"),
        ("vfs_cache_pressure", "Controls the tendency to reclaim memory used for VFS caches"),
        ("min_free_kbytes", "Minimum free memory reserved for the kernel"),
        ("overcommit_memory", "Memory overcommit mode (0=heuristic, 1=always, 2=never)"),
        ("overcommit_ratio", "Percentage of RAM to allow overcommit when overcommit_memory=2"),
        ("panic", "Seconds to wait before rebooting on kernel panic (0=never reboot)"),
        ("panic_on_oops", "Whether to panic on kernel oops"),
        ("sysrq", "Enables SysRq key functions"),
        ("ip_forward", "Enable IP forwarding between interfaces"),
        ("tcp_syncookies", "Enable TCP SYN cookies for SYN flood protection"),
        ("tcp_timestamps", "Enable TCP timestamps"),
        ("tcp_keepalive_time", "Time in seconds before TCP keepalive probes start"),
        ("tcp_keepalive_intvl", "Interval between TCP keepalive probes"),
        ("tcp_keepalive_probes", "Number of TCP keepalive probes before declaring connection dead"),
        ("tcp_fin_timeout", "Time to hold socket in FIN-WAIT-2 state"),
        ("tcp_tw_reuse", "Allow reuse of TIME-WAIT sockets for new connections"),
        ("somaxconn", "Maximum listen queue backlog"),
        ("max_map_count", "Maximum number of memory map areas a process may have"),
        ("file-max", "Maximum number of file handles"),
        ("inotify/max_user_watches", "Maximum number of inotify watches per user"),
        ("inotify/max_user_instances", "Maximum number of inotify instances per user"),
        ("pid_max", "Maximum PID value"),
        ("threads-max", "Maximum number of threads"),
        ("core_pattern", "Pattern for core dump file names"),
        ("dmesg_restrict", "Restrict dmesg access to root"),
        ("kptr_restrict", "Restrict kernel pointer exposure"),
        ("randomize_va_space", "Address space layout randomization (ASLR)"),
        ("ptrace_scope", "Ptrace scope for process tracing"),
    ].iter().cloned().collect();

    descriptions.get(name).map(|s| s.to_string())
}

/// List all parameters
fn list_all_parameters() {
    info("Fetching kernel parameters...");

    let params = get_all_parameters();
    println!("\n{} Found {} parameters\n", icons::info(), params.len());

    // Group by category
    let mut categories: HashMap<String, Vec<&KernelParam>> = HashMap::new();
    for param in &params {
        categories
            .entry(param.category.clone())
            .or_default()
            .push(param);
    }

    let cat_names: Vec<&String> = categories.keys().collect();
    let cat_strs: Vec<&str> = cat_names.iter().map(|s| s.as_str()).collect();

    if let Some(idx) = select_with_back("Select category to view", &cat_strs, 0) {
        let category = cat_names[idx];
        println!("\n{} Parameters in '{}':", icons::folder(), category);
        println!("{}", "=".repeat(60));

        if let Some(params) = categories.get(category) {
            for param in params.iter().take(50) {
                // Limit display
                let desc = param
                    .description
                    .as_ref()
                    .map(|d| format!(" - {}", d))
                    .unwrap_or_default();
                println!(
                    "  {} = {}{}",
                    param.name,
                    truncate_value(&param.value, 40),
                    desc
                );
            }
            if params.len() > 50 {
                println!("  ... and {} more", params.len() - 50);
            }
        }
    }
}

/// Truncate a value for display
fn truncate_value(value: &str, max_len: usize) -> String {
    if value.len() > max_len {
        format!("{}...", &value[..max_len])
    } else {
        value.to_string()
    }
}

/// Search for parameters
fn search_parameters() {
    let query = match input("Enter search term", None) {
        Some(q) if !q.is_empty() => q.to_lowercase(),
        _ => {
            warn("No search term provided");
            return;
        }
    };

    info(&format!("Searching for '{}'...", query));

    let params = get_all_parameters();
    let matches: Vec<&KernelParam> = params
        .iter()
        .filter(|p| {
            p.name.to_lowercase().contains(&query)
                || p.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query))
                    .unwrap_or(false)
        })
        .collect();

    if matches.is_empty() {
        warn("No matching parameters found");
        return;
    }

    println!("\n{} Found {} matches:\n", icons::success(), matches.len());

    for param in matches.iter().take(30) {
        let desc = param
            .description
            .as_ref()
            .map(|d| format!("\n    {}", d))
            .unwrap_or_default();
        println!(
            "  {} {} = {}{}",
            icons::config(),
            param.name,
            truncate_value(&param.value, 50),
            desc
        );
    }

    if matches.len() > 30 {
        println!("\n  ... and {} more matches", matches.len() - 30);
    }
}

/// View a specific parameter
fn view_parameter() {
    let name = match input("Enter parameter name (e.g., vm.swappiness)", None) {
        Some(n) if !n.is_empty() => n,
        _ => {
            warn("No parameter name provided");
            return;
        }
    };

    // Convert dot notation to path
    let path = format!("/proc/sys/{}", name.replace('.', "/"));

    if !Path::new(&path).exists() {
        error(&format!("Parameter '{}' not found at {}", name, path));
        return;
    }

    match fs::read_to_string(&path) {
        Ok(value) => {
            println!("\n{} Parameter: {}", icons::config(), name);
            println!("  Path: {}", path);
            println!("  Value: {}", value.trim());

            // Try to get description
            let param_name = name.split('.').next_back().unwrap_or(&name);
            if let Some(desc) = get_param_description(param_name) {
                println!("  Description: {}", desc);
            }
        }
        Err(e) => {
            error(&format!("Failed to read parameter: {}", e));
        }
    }
}

/// Set a kernel parameter
fn set_parameter() {
    let name = match input("Enter parameter name (e.g., vm.swappiness)", None) {
        Some(n) if !n.is_empty() => n,
        _ => {
            warn("No parameter name provided");
            return;
        }
    };

    // Convert dot notation to path
    let path = format!("/proc/sys/{}", name.replace('.', "/"));

    if !Path::new(&path).exists() {
        error(&format!("Parameter '{}' not found", name));
        return;
    }

    // Show current value
    let current = fs::read_to_string(&path)
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|_| "<unknown>".to_string());

    info(&format!("Current value of {}: {}", name, current));

    let new_value = match input("Enter new value", Some(&current)) {
        Some(v) if !v.is_empty() => v,
        _ => {
            warn("No value provided");
            return;
        }
    };

    if !confirm(&format!("Set {} = {}?", name, new_value), false) {
        return;
    }

    // Write using sudo if needed
    match sudo_write_file(&path, &format!("{}\n", new_value)) {
        Ok(_) => {
            success(&format!("Set {} = {}", name, new_value));
            info("Note: This change is temporary. To persist, add to /etc/sysctl.d/");
        }
        Err(e) => {
            error(&format!("Failed to set parameter: {}", e));
        }
    }
}

/// Show parameter categories
fn show_categories() {
    let params = get_all_parameters();

    let mut categories: HashMap<String, usize> = HashMap::new();
    for param in &params {
        *categories.entry(param.category.clone()).or_insert(0) += 1;
    }

    println!("\n{} Kernel Parameter Categories:\n", icons::folder());

    let mut cats: Vec<(&String, &usize)> = categories.iter().collect();
    cats.sort_by(|a, b| b.1.cmp(a.1));

    for (category, count) in cats {
        let desc = get_category_description(category);
        println!("  {:20} {:5} params  {}", category, count, desc);
    }
}

/// Get description for a category
fn get_category_description(category: &str) -> &'static str {
    match category {
        "kernel" => "Core kernel parameters",
        "vm" => "Virtual memory management",
        "net" => "Network stack configuration",
        "fs" => "Filesystem settings",
        "dev" => "Device parameters",
        "debug" => "Debugging options",
        "abi" => "Application Binary Interface",
        "crypto" => "Cryptographic subsystem",
        "user" => "User namespace limits",
        _ => "",
    }
}

/// Export current configuration
fn export_configuration() {
    let filename = match input("Export filename", Some("sysctl.conf")) {
        Some(f) if !f.is_empty() => f,
        _ => "sysctl.conf".to_string(),
    };

    info("Exporting kernel parameters...");

    let params = get_all_parameters();
    let mut output = String::from("# GhostCTL sysctl configuration export\n");
    output.push_str(&format!(
        "# Generated: {}\n\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    // Export only writable parameters with non-default values
    // For simplicity, we'll export common tunable parameters
    let tunable = [
        "vm.swappiness",
        "vm.dirty_ratio",
        "vm.dirty_background_ratio",
        "vm.vfs_cache_pressure",
        "vm.min_free_kbytes",
        "vm.overcommit_memory",
        "kernel.panic",
        "kernel.sysrq",
        "net.core.somaxconn",
        "net.ipv4.tcp_syncookies",
        "net.ipv4.ip_forward",
    ];

    for param in &params {
        if tunable.iter().any(|t| param.name == *t) {
            if let Some(desc) = &param.description {
                output.push_str(&format!("# {}\n", desc));
            }
            output.push_str(&format!("{} = {}\n\n", param.name, param.value));
        }
    }

    match fs::write(&filename, &output) {
        Ok(_) => success(&format!("Exported to {}", filename)),
        Err(e) => error(&format!("Failed to export: {}", e)),
    }
}
