//! NVIDIA Open GPU Kernel Modules Source Build Pipeline
//!
//! Provides functionality to build NVIDIA open-source kernel modules from source,
//! manage DKMS lifecycle, and offer rollback/cleanup tooling.
//!
//! Supports building from the `open-gpu-kernel-modules` repository.

use anyhow::{Context, Result};
use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::GhostConfig;
use crate::tui;
use crate::utils;

/// Build options for source compilation
#[derive(Debug, Clone)]
pub struct SourceBuildOptions {
    /// Build for all installed kernels (default: current kernel only)
    pub all_kernels: bool,
    /// Use DKMS-managed build instead of direct install
    pub use_dkms: bool,
    /// Auto-clean old DKMS entries without prompting
    pub auto_clean: bool,
    /// Dry run - log actions without executing
    pub dry_run: bool,
}

impl Default for SourceBuildOptions {
    fn default() -> Self {
        Self {
            all_kernels: false,
            use_dkms: true, // DKMS is the safer default
            auto_clean: false,
            dry_run: false,
        }
    }
}

/// Information about a detected source tree
#[derive(Debug, Clone)]
pub struct SourceTreeInfo {
    pub path: PathBuf,
    pub version: Option<String>,
    pub branch: Option<String>,
    pub is_valid: bool,
}

/// Build environment validation result
#[derive(Debug)]
pub struct BuildEnvStatus {
    pub kernel_headers: Vec<KernelHeaderStatus>,
    pub gcc_available: bool,
    pub make_available: bool,
    pub dkms_available: bool,
    pub pkg_config_available: bool,
    pub missing_tools: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KernelHeaderStatus {
    pub kernel_version: String,
    pub headers_path: PathBuf,
    pub available: bool,
}

/// Detect the NVIDIA open-gpu-kernel-modules source tree
///
/// Searches in order:
/// 1. Config-specified path (`nvidia.kernel_module_path`)
/// 2. Common locations: ~/src/open-gpu-kernel-modules, /usr/src/nvidia-*
/// 3. Environment variable NVIDIA_SRC
pub fn detect_source_tree() -> Result<SourceTreeInfo> {
    let config = GhostConfig::load();

    // Priority 1: Config-specified path
    if let Some(ref nvidia_config) = config.nvidia
        && let Some(ref path_str) = nvidia_config.kernel_module_path
    {
        let path = PathBuf::from(path_str);
        if validate_source_tree(&path) {
            log::info!("Using config-specified source tree: {:?}", path);
            return Ok(create_source_info(path));
        }
        log::warn!(
            "Config-specified path {:?} is not a valid source tree",
            path
        );
    }

    // Priority 2: Environment variable
    if let Ok(env_path) = std::env::var("NVIDIA_SRC") {
        let path = PathBuf::from(env_path);
        if validate_source_tree(&path) {
            log::info!("Using NVIDIA_SRC environment variable: {:?}", path);
            return Ok(create_source_info(path));
        }
    }

    // Priority 3: Common locations
    let common_locations = [
        dirs::home_dir()
            .unwrap_or_default()
            .join("src/open-gpu-kernel-modules"),
        dirs::home_dir()
            .unwrap_or_default()
            .join("nvidia/open-gpu-kernel-modules"),
        PathBuf::from("/usr/src/open-gpu-kernel-modules"),
        PathBuf::from("/opt/nvidia/open-gpu-kernel-modules"),
    ];

    for location in &common_locations {
        if validate_source_tree(location) {
            log::info!("Found source tree at common location: {:?}", location);
            return Ok(create_source_info(location.clone()));
        }
    }

    // Priority 4: Search /usr/src for nvidia-* directories
    if let Ok(entries) = std::fs::read_dir("/usr/src") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with("nvidia-") && validate_source_tree(&path) {
                    log::info!("Found source tree in /usr/src: {:?}", path);
                    return Ok(create_source_info(path));
                }
            }
        }
    }

    anyhow::bail!(
        "No valid NVIDIA source tree found.\n\
        Clone the open-gpu-kernel-modules repository:\n\
        git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git ~/src/open-gpu-kernel-modules\n\
        Or set nvidia.kernel_module_path in ~/.config/ghostctl/config.toml"
    )
}

/// Validate that a path contains a valid NVIDIA kernel module source tree
fn validate_source_tree(path: &Path) -> bool {
    if !path.exists() || !path.is_dir() {
        return false;
    }

    // Check for key files that indicate a valid source tree
    let required_files = ["Makefile", "kernel-open"];
    let optional_markers = ["README.md", "NVIDIA-kernel.spec"];

    let has_required = required_files.iter().all(|f| path.join(f).exists());
    let has_some_markers = optional_markers.iter().any(|f| path.join(f).exists());

    has_required || (path.join("Makefile").exists() && has_some_markers)
}

/// Create SourceTreeInfo from a validated path
fn create_source_info(path: PathBuf) -> SourceTreeInfo {
    let version = extract_version(&path);
    let branch = extract_git_branch(&path);

    SourceTreeInfo {
        path,
        version,
        branch,
        is_valid: true,
    }
}

/// Extract version from source tree (from version.mk or git tag)
fn extract_version(path: &Path) -> Option<String> {
    // Try version.mk
    let version_mk = path.join("version.mk");
    if version_mk.exists()
        && let Ok(content) = std::fs::read_to_string(&version_mk)
    {
        for line in content.lines() {
            if (line.starts_with("NVIDIA_VERSION") || line.starts_with("VERSION"))
                && let Some(ver) = line.split('=').nth(1)
            {
                return Some(ver.trim().to_string());
            }
        }
    }

    // Try git describe
    let output = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .current_dir(path)
        .output()
        .ok()?;

    if output.status.success() {
        return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }

    None
}

/// Extract current git branch
fn extract_git_branch(path: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(path)
        .output()
        .ok()?;

    if output.status.success() {
        return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }

    None
}

/// Validate the build environment and return detailed status
pub fn validate_build_env(target_kernels: &[String]) -> BuildEnvStatus {
    let gcc_available = check_tool_available("gcc");
    let make_available = check_tool_available("make");
    let dkms_available = check_tool_available("dkms");
    let pkg_config_available = check_tool_available("pkg-config");

    let mut missing_tools = Vec::new();
    if !gcc_available {
        missing_tools.push("gcc".to_string());
    }
    if !make_available {
        missing_tools.push("make".to_string());
    }
    if !dkms_available {
        missing_tools.push("dkms".to_string());
    }
    if !pkg_config_available {
        missing_tools.push("pkg-config".to_string());
    }

    let kernel_headers: Vec<KernelHeaderStatus> = target_kernels
        .iter()
        .map(|k| {
            let headers_path = PathBuf::from(format!("/lib/modules/{}/build", k));
            let available = headers_path.exists() && headers_path.join("Makefile").exists();
            KernelHeaderStatus {
                kernel_version: k.clone(),
                headers_path,
                available,
            }
        })
        .collect();

    BuildEnvStatus {
        kernel_headers,
        gcc_available,
        make_available,
        dkms_available,
        pkg_config_available,
        missing_tools,
    }
}

fn check_tool_available(tool: &str) -> bool {
    Command::new("which")
        .arg(tool)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get list of installed kernels
pub fn get_installed_kernels(all_kernels: bool) -> Vec<String> {
    if !all_kernels {
        // Return only the running kernel
        if let Ok(output) = Command::new("uname").arg("-r").output()
            && output.status.success()
        {
            let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return vec![kernel];
        }
        return Vec::new();
    }

    // Return all installed kernels
    let mut kernels = Vec::new();

    if let Ok(entries) = std::fs::read_dir("/lib/modules") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Check if this looks like a valid kernel directory
                if (path.join("modules.dep").exists() || path.join("build").exists())
                    && let Some(name) = path.file_name()
                {
                    kernels.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    // Sort with newest first (by version comparison)
    kernels.sort_by(|a, b| b.cmp(a));
    kernels
}

/// Build kernel modules from source
pub fn build_modules(source_path: &Path, target_kernel: &str, dry_run: bool) -> Result<()> {
    log::info!(
        "Building modules for kernel {} from {:?}",
        target_kernel,
        source_path
    );

    let nproc = get_nproc();
    let make_cmd = format!("make -j{} KERNEL_UNAME={} modules", nproc, target_kernel);

    if dry_run {
        println!("[DRY RUN] Would execute: {}", make_cmd);
        println!("[DRY RUN] In directory: {:?}", source_path);
        return Ok(());
    }

    println!("Building NVIDIA modules for kernel {}...", target_kernel);
    println!("This may take several minutes...");

    let status = Command::new("make")
        .args(["-j", &nproc.to_string()])
        .arg(format!("KERNEL_UNAME={}", target_kernel))
        .arg("modules")
        .current_dir(source_path)
        .status()
        .context("Failed to execute make")?;

    if !status.success() {
        anyhow::bail!("Build failed for kernel {}", target_kernel);
    }

    log::info!("Build completed successfully for kernel {}", target_kernel);
    println!("Build completed successfully!");
    Ok(())
}

/// Get number of processors for parallel build
fn get_nproc() -> usize {
    Command::new("nproc")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(4)
}

/// Install modules using direct make install or DKMS
pub fn install_modules(
    source_path: &Path,
    target_kernel: &str,
    use_dkms: bool,
    dry_run: bool,
) -> Result<()> {
    if use_dkms {
        install_via_dkms(source_path, target_kernel, dry_run)
    } else {
        install_direct(source_path, target_kernel, dry_run)
    }
}

/// Install modules directly using make modules_install
fn install_direct(source_path: &Path, target_kernel: &str, dry_run: bool) -> Result<()> {
    let cmd = format!("sudo make KERNEL_UNAME={} modules_install", target_kernel);

    if dry_run {
        println!("[DRY RUN] Would execute: {}", cmd);
        return Ok(());
    }

    println!("Installing modules for kernel {}...", target_kernel);

    // Use shell command to run make with current_dir
    let shell_cmd = format!(
        "cd {:?} && make KERNEL_UNAME={} modules_install",
        source_path, target_kernel
    );
    let result = utils::sudo_shell(&shell_cmd).context("Failed to execute make modules_install")?;

    if !result.success {
        anyhow::bail!(
            "Installation failed for kernel {}: {}",
            target_kernel,
            result.stderr
        );
    }

    // Update module dependencies
    println!("Updating module dependencies...");
    let _ = utils::sudo_run("depmod", &[target_kernel]);

    log::info!(
        "Modules installed successfully for kernel {}",
        target_kernel
    );
    Ok(())
}

/// Install modules via DKMS
fn install_via_dkms(source_path: &Path, target_kernel: &str, dry_run: bool) -> Result<()> {
    // Get version for DKMS
    let version = extract_version(source_path).unwrap_or_else(|| "0.0.0".to_string());
    let module_name = "nvidia";

    if dry_run {
        println!("[DRY RUN] Would execute DKMS workflow:");
        println!("[DRY RUN]   dkms add -m {} -v {}", module_name, version);
        println!(
            "[DRY RUN]   dkms build -m {} -v {} -k {}",
            module_name, version, target_kernel
        );
        println!(
            "[DRY RUN]   dkms install -m {} -v {} -k {}",
            module_name, version, target_kernel
        );
        return Ok(());
    }

    println!("Setting up DKMS for nvidia v{}...", version);

    // Create DKMS source symlink if needed
    let dkms_src = PathBuf::from(format!("/usr/src/{}-{}", module_name, version));
    if !dkms_src.exists() {
        println!("Creating DKMS source link at {:?}", dkms_src);
        let source_str = source_path.to_string_lossy().to_string();
        let dkms_str = dkms_src.to_string_lossy().to_string();
        let _ = utils::sudo_run("ln", &["-sf", &source_str, &dkms_str]);
    }

    // DKMS add
    println!("Adding module to DKMS...");
    let result = utils::sudo_run("dkms", &["add", "-m", module_name, "-v", &version]);

    // Ignore "already added" errors
    if let Ok(r) = result
        && !r.success
    {
        log::warn!("DKMS add returned non-zero (module may already be added)");
    }

    // DKMS build
    println!("Building via DKMS for kernel {}...", target_kernel);
    let result = utils::sudo_run_interactive(
        "dkms",
        &[
            "build",
            "-m",
            module_name,
            "-v",
            &version,
            "-k",
            target_kernel,
        ],
    )
    .context("DKMS build failed")?;

    if !result.success {
        anyhow::bail!(
            "DKMS build failed for kernel {}: {}",
            target_kernel,
            result.stderr
        );
    }

    // DKMS install
    println!("Installing via DKMS...");
    let result = utils::sudo_run(
        "dkms",
        &[
            "install",
            "-m",
            module_name,
            "-v",
            &version,
            "-k",
            target_kernel,
        ],
    )
    .context("DKMS install failed")?;

    if !result.success {
        anyhow::bail!(
            "DKMS install failed for kernel {}: {}",
            target_kernel,
            result.stderr
        );
    }

    log::info!(
        "DKMS installation completed for {} v{} on kernel {}",
        module_name,
        version,
        target_kernel
    );
    println!("DKMS installation completed!");
    Ok(())
}

/// Get list of old DKMS entries for cleanup
pub fn get_old_dkms_entries() -> Vec<(String, String, String)> {
    let mut entries = Vec::new();

    let output = Command::new("dkms").arg("status").output();
    if let Ok(output) = output {
        let status_str = String::from_utf8_lossy(&output.stdout);
        for line in status_str.lines() {
            // Parse: module/version, kernel_version, arch: status
            if line.contains("nvidia") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let module_ver = parts[0].trim();
                    let kernel_ver = parts[1].trim();
                    let status = parts.get(2).map(|s| s.trim()).unwrap_or("unknown");
                    entries.push((
                        module_ver.to_string(),
                        kernel_ver.to_string(),
                        status.to_string(),
                    ));
                }
            }
        }
    }

    entries
}

/// Cleanup old DKMS versions
pub fn cleanup_old_versions(auto_clean: bool, dry_run: bool) -> Result<()> {
    let entries = get_old_dkms_entries();

    if entries.is_empty() {
        println!("No NVIDIA DKMS entries found.");
        return Ok(());
    }

    // Get current kernel
    let current_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Find entries that are not for the current kernel
    let old_entries: Vec<_> = entries
        .iter()
        .filter(|(_, kernel, _)| !kernel.contains(&current_kernel))
        .collect();

    if old_entries.is_empty() {
        println!("No old DKMS entries to clean up.");
        return Ok(());
    }

    println!("Found {} DKMS entries for removal:", old_entries.len());
    for (module, kernel, status) in &old_entries {
        println!("  {} - kernel {} ({})", module, kernel, status);
    }

    if !auto_clean {
        let confirm = Confirm::new()
            .with_prompt("Remove these old DKMS entries?")
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirm {
            println!("Cleanup cancelled.");
            return Ok(());
        }
    }

    for (module_ver, kernel_ver, _) in old_entries {
        // Parse module/version
        let parts: Vec<&str> = module_ver.split('/').collect();
        if parts.len() != 2 {
            continue;
        }
        let (module, version) = (parts[0], parts[1]);

        if dry_run {
            println!(
                "[DRY RUN] Would remove: dkms remove -m {} -v {} -k {}",
                module, version, kernel_ver
            );
        } else {
            println!(
                "Removing {} v{} for kernel {}...",
                module, version, kernel_ver
            );
            let _ = utils::sudo_run(
                "dkms",
                &["remove", "-m", module, "-v", version, "-k", kernel_ver],
            );
        }
    }

    log::info!("DKMS cleanup completed");
    println!("Cleanup completed.");
    Ok(())
}

/// Print remediation hints for missing build prerequisites
pub fn print_remediation_hints(env_status: &BuildEnvStatus) {
    if !env_status.missing_tools.is_empty() {
        println!("\n Missing build tools:");
        println!(
            "  Install with: sudo pacman -S {}",
            env_status.missing_tools.join(" ")
        );
        if !env_status.dkms_available {
            println!("  For DKMS: sudo pacman -S dkms");
        }
    }

    for header in &env_status.kernel_headers {
        if !header.available {
            println!("\n Missing headers for kernel {}:", header.kernel_version);
            // Try to suggest the right package
            if header.kernel_version.contains("lts") {
                println!("  Install with: sudo pacman -S linux-lts-headers");
            } else if header.kernel_version.contains("zen") {
                println!("  Install with: sudo pacman -S linux-zen-headers");
            } else if header.kernel_version.contains("hardened") {
                println!("  Install with: sudo pacman -S linux-hardened-headers");
            } else {
                println!("  Install with: sudo pacman -S linux-headers");
            }
        }
    }
}

/// Main source build workflow
pub fn source_build_workflow(opts: &SourceBuildOptions) -> Result<()> {
    tui::header("NVIDIA Source Build Pipeline");

    // Step 1: Detect source tree
    println!("Detecting source tree...");
    let source_info = detect_source_tree()?;
    println!("  Found: {:?}", source_info.path);
    if let Some(ref ver) = source_info.version {
        println!("  Version: {}", ver);
    }
    if let Some(ref branch) = source_info.branch {
        println!("  Branch: {}", branch);
    }

    // Step 2: Get target kernels
    let kernels = get_installed_kernels(opts.all_kernels);
    if kernels.is_empty() {
        anyhow::bail!("No target kernels found");
    }
    println!("\nTarget kernels: {}", kernels.join(", "));

    // Step 3: Validate build environment
    println!("\nValidating build environment...");
    let env_status = validate_build_env(&kernels);

    if !env_status.missing_tools.is_empty() {
        println!(" Missing tools: {}", env_status.missing_tools.join(", "));
        print_remediation_hints(&env_status);
        anyhow::bail!("Build environment incomplete. Install missing tools first.");
    }

    let missing_headers: Vec<_> = env_status
        .kernel_headers
        .iter()
        .filter(|h| !h.available)
        .collect();

    if !missing_headers.is_empty() {
        println!(" Missing kernel headers:");
        for h in &missing_headers {
            println!(
                "  - {} (expected at {:?})",
                h.kernel_version, h.headers_path
            );
        }
        print_remediation_hints(&env_status);
        anyhow::bail!("Missing kernel headers. Install them first.");
    }

    println!(" Build environment OK");

    // Step 4: Build and install for each kernel
    for kernel in &kernels {
        println!("\n{}", "=".repeat(60));
        println!("Processing kernel: {}", kernel);
        println!("{}", "=".repeat(60));

        // Build
        build_modules(&source_info.path, kernel, opts.dry_run)?;

        // Install
        install_modules(&source_info.path, kernel, opts.use_dkms, opts.dry_run)?;
    }

    // Step 5: Cleanup old versions if requested
    if opts.auto_clean || opts.use_dkms {
        println!("\nChecking for old DKMS entries...");
        cleanup_old_versions(opts.auto_clean, opts.dry_run)?;
    }

    // Step 6: Final recommendations
    println!("\n Build complete!");
    println!("Recommendations:");
    println!("  1. Run 'sudo mkinitcpio -P' to regenerate initramfs");
    println!("  2. Reboot to load the new modules");
    println!("  3. Verify with 'nvidia-smi' after reboot");

    Ok(())
}

/// Interactive TUI menu for source builds
pub fn source_build_menu() {
    tui::header("NVIDIA Source Build");

    let options = [
        "Build from source (current kernel)",
        "Build from source (all kernels)",
        "Validate build environment",
        "Detect source tree",
        "Cleanup old DKMS entries",
        "Show DKMS status",
        "Back",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Source Build Options")
            .items(&options)
            .default(0)
            .interact();

        match choice {
            Ok(0) => {
                let opts = SourceBuildOptions {
                    all_kernels: false,
                    use_dkms: true,
                    auto_clean: false,
                    dry_run: utils::is_dry_run(),
                };
                if let Err(e) = source_build_workflow(&opts) {
                    tui::error(&format!("Build failed: {}", e));
                }
            }
            Ok(1) => {
                let opts = SourceBuildOptions {
                    all_kernels: true,
                    use_dkms: true,
                    auto_clean: false,
                    dry_run: utils::is_dry_run(),
                };
                if let Err(e) = source_build_workflow(&opts) {
                    tui::error(&format!("Build failed: {}", e));
                }
            }
            Ok(2) => {
                let kernels = get_installed_kernels(true);
                let env_status = validate_build_env(&kernels);
                display_env_status(&env_status);
            }
            Ok(3) => match detect_source_tree() {
                Ok(info) => {
                    println!("Source tree found:");
                    println!("  Path: {:?}", info.path);
                    println!(
                        "  Version: {}",
                        info.version.as_deref().unwrap_or("unknown")
                    );
                    println!("  Branch: {}", info.branch.as_deref().unwrap_or("unknown"));
                }
                Err(e) => tui::error(&format!("Not found: {}", e)),
            },
            Ok(4) => {
                if let Err(e) = cleanup_old_versions(false, utils::is_dry_run()) {
                    tui::error(&format!("Cleanup failed: {}", e));
                }
            }
            Ok(5) => show_dkms_status(),
            _ => break,
        }
        println!();
    }
}

fn display_env_status(status: &BuildEnvStatus) {
    println!("\nBuild Environment Status:");
    println!(
        "  GCC:        {}",
        if status.gcc_available {
            "[OK]"
        } else {
            "[MISSING]"
        }
    );
    println!(
        "  Make:       {}",
        if status.make_available {
            "[OK]"
        } else {
            "[MISSING]"
        }
    );
    println!(
        "  DKMS:       {}",
        if status.dkms_available {
            "[OK]"
        } else {
            "[MISSING]"
        }
    );
    println!(
        "  pkg-config: {}",
        if status.pkg_config_available {
            "[OK]"
        } else {
            "[MISSING]"
        }
    );

    println!("\nKernel Headers:");
    for h in &status.kernel_headers {
        println!(
            "  {} {}: {:?}",
            if h.available { "[OK]" } else { "[MISSING]" },
            h.kernel_version,
            h.headers_path
        );
    }

    if !status.missing_tools.is_empty() {
        print_remediation_hints(status);
    }
}

fn show_dkms_status() {
    println!("\n DKMS Status:");
    let output = Command::new("dkms").arg("status").output();
    match output {
        Ok(out) => {
            let status = String::from_utf8_lossy(&out.stdout);
            if status.trim().is_empty() {
                println!("  No DKMS modules installed.");
            } else {
                for line in status.lines() {
                    println!("  {}", line);
                }
            }
        }
        Err(_) => println!("  Could not get DKMS status. Is dkms installed?"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_build_options_default() {
        let opts = SourceBuildOptions::default();
        assert!(!opts.all_kernels);
        assert!(opts.use_dkms);
        assert!(!opts.auto_clean);
        assert!(!opts.dry_run);
    }

    #[test]
    fn test_get_installed_kernels_current() {
        let kernels = get_installed_kernels(false);
        // Should return at least the current kernel
        assert!(!kernels.is_empty() || cfg!(not(target_os = "linux")));
    }
}
