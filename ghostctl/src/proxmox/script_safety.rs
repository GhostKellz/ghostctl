use crate::http_client::RobustHttpClient;
use crate::logging::GhostLogger;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Configuration for script execution safety
#[derive(Debug, Clone)]
pub struct ScriptSafetyConfig {
    /// Whether to show script preview before execution
    pub show_preview: bool,
    /// Whether to require checksum verification
    pub require_checksum: bool,
    /// Whether to cache scripts locally
    pub cache_scripts: bool,
    /// Whether to run in dry-run mode (no actual execution)
    pub dry_run: bool,
    /// Number of preview lines to show
    pub preview_lines: usize,
}

impl Default for ScriptSafetyConfig {
    fn default() -> Self {
        Self {
            show_preview: true,
            require_checksum: false,
            cache_scripts: true,
            dry_run: false,
            preview_lines: 15,
        }
    }
}

/// Cached script information
#[derive(Debug, Clone)]
pub struct CachedScript {
    pub url: String,
    pub content: String,
    pub sha256: String,
    pub cached_at: chrono::DateTime<chrono::Utc>,
}

/// Result of script verification
#[derive(Debug)]
pub struct ScriptVerification {
    pub url: String,
    pub sha256: String,
    pub line_count: usize,
    pub size_bytes: usize,
    pub has_sudo: bool,
    pub has_rm_rf: bool,
    pub has_curl_pipe: bool,
}

impl ScriptVerification {
    /// Returns a list of warnings based on script content analysis
    pub fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        if self.has_sudo {
            warnings.push("Script contains sudo commands (requires root privileges)".to_string());
        }
        if self.has_rm_rf {
            warnings.push("Script contains 'rm -rf' commands (destructive)".to_string());
        }
        if self.has_curl_pipe {
            warnings
                .push("Script downloads and executes additional scripts (curl|bash)".to_string());
        }

        warnings
    }
}

/// Safe script executor with verification and caching
pub struct SafeScriptExecutor {
    config: ScriptSafetyConfig,
    client: RobustHttpClient,
    cache_dir: PathBuf,
    known_checksums: HashMap<String, String>,
}

impl SafeScriptExecutor {
    pub fn new(config: ScriptSafetyConfig) -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("ghostctl")
            .join("scripts");

        fs::create_dir_all(&cache_dir).context("Failed to create script cache directory")?;

        let client = RobustHttpClient::new().context("Failed to create HTTP client")?;

        Ok(Self {
            config,
            client,
            cache_dir,
            known_checksums: Self::load_known_checksums(),
        })
    }

    /// Load known good checksums for trusted scripts
    fn load_known_checksums() -> HashMap<String, String> {
        // Known checksums for popular community scripts can be added here
        // These would be verified against the community-scripts repository
        // For now, we'll compute and display checksums for user verification
        HashMap::new()
    }

    /// Fetch a script from URL and return its content
    pub fn fetch_script(&self, url: &str) -> Result<String> {
        // Check cache first
        if self.config.cache_scripts
            && let Some(cached) = self.get_cached_script(url) {
                println!("  Using cached script (cached at {})", cached.cached_at);
                GhostLogger::log_action(
                    "script_fetch",
                    true,
                    Some(&format!("cached: {} sha256:{}", url, cached.sha256)),
                );
                return Ok(cached.content);
            }

        println!("  Fetching script from: {}", url);

        // Use RobustHttpClient with automatic retry and fallback
        let content = self.client.fetch(url).context("Failed to fetch script")?;

        if content.trim().is_empty() {
            GhostLogger::log_action(
                "script_fetch",
                false,
                Some(&format!("empty script from {}", url)),
            );
            anyhow::bail!("Script is empty");
        }

        // Cache the script
        if self.config.cache_scripts {
            self.cache_script(url, &content)?;
        }

        let sha256 = Self::compute_sha256(&content);
        GhostLogger::log_action(
            "script_fetch",
            true,
            Some(&format!("fresh: {} sha256:{}", url, sha256)),
        );

        Ok(content)
    }

    /// Compute SHA256 hash of script content
    pub fn compute_sha256(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify a script and return verification details
    pub fn verify_script(&self, url: &str, content: &str) -> ScriptVerification {
        let sha256 = Self::compute_sha256(content);
        let lines: Vec<&str> = content.lines().collect();

        // Analyze script for potentially dangerous patterns
        let content_lower = content.to_lowercase();
        let has_sudo = content_lower.contains("sudo ");
        let has_rm_rf = content.contains("rm -rf") || content.contains("rm -fr");
        let has_curl_pipe = content.contains("curl") && content.contains("| bash")
            || content.contains("|bash")
            || content.contains("| sh")
            || content.contains("|sh");

        ScriptVerification {
            url: url.to_string(),
            sha256,
            line_count: lines.len(),
            size_bytes: content.len(),
            has_sudo,
            has_rm_rf,
            has_curl_pipe,
        }
    }

    /// Display script preview
    pub fn show_preview(&self, content: &str, verification: &ScriptVerification) {
        println!("\n  Script Verification");
        println!("  ════════════════════");
        println!("  SHA256: {}", verification.sha256);
        println!(
            "  Size: {} bytes ({} lines)",
            verification.size_bytes, verification.line_count
        );

        // Show warnings
        let warnings = verification.warnings();
        if !warnings.is_empty() {
            println!("\n  Warnings:");
            for warning in &warnings {
                println!("    - {}", warning);
            }
        }

        // Check against known checksums
        if let Some(known_hash) = self.known_checksums.get(&verification.url) {
            if known_hash == &verification.sha256 {
                println!("\n  Checksum verified against known good hash");
            } else {
                println!("\n  WARNING: Checksum does not match known good hash!");
                println!("  Expected: {}", known_hash);
                println!("  Got:      {}", verification.sha256);
            }
        }

        if self.config.show_preview {
            let lines: Vec<&str> = content.lines().collect();
            let preview_count = self.config.preview_lines.min(lines.len());

            println!("\n  Preview (first {} lines):", preview_count);
            println!("  ─────────────────────────");
            for line in lines.iter().take(preview_count) {
                println!("    {}", line);
            }
            if lines.len() > preview_count {
                println!("    ... ({} more lines)", lines.len() - preview_count);
            }
        }
    }

    /// Interactively confirm and execute a script
    pub fn confirm_and_execute(&self, name: &str, url: &str) -> Result<bool> {
        println!("\n  Script: {}", name);
        println!("  URL: {}", url);

        GhostLogger::log_action(
            "script_review_start",
            true,
            Some(&format!("name:{} url:{}", name, url)),
        );

        // Fetch the script
        let content = self.fetch_script(url)?;

        // Verify the script
        let verification = self.verify_script(url, &content);

        // Log verification warnings
        let warnings = verification.warnings();
        if !warnings.is_empty() {
            GhostLogger::log_action(
                "script_warnings",
                true,
                Some(&format!(
                    "name:{} sha256:{} warnings:{}",
                    name,
                    verification.sha256,
                    warnings.join("; ")
                )),
            );
        }

        // Show preview
        self.show_preview(&content, &verification);

        // Show action options
        let warnings = verification.warnings();
        let has_warnings = !warnings.is_empty();

        let options = if has_warnings {
            vec![
                "Execute script (with warnings)",
                "View full script",
                "Save script locally",
                "Cancel",
            ]
        } else {
            vec![
                "Execute script",
                "View full script",
                "Save script locally",
                "Cancel",
            ]
        };

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose action")
            .items(&options)
            .default(if has_warnings { 3 } else { 0 })
            .interact()
            .unwrap_or(3);

        match choice {
            0 => {
                // Execute
                if has_warnings {
                    let confirm = Confirm::new()
                        .with_prompt("Script has warnings. Are you sure you want to execute?")
                        .default(false)
                        .interact()
                        .unwrap_or(false);

                    if !confirm {
                        println!("  Execution cancelled");
                        GhostLogger::log_action(
                            "script_cancelled",
                            true,
                            Some(&format!(
                                "name:{} sha256:{} reason:user_declined_warnings",
                                name, verification.sha256
                            )),
                        );
                        return Ok(false);
                    }
                }

                if self.config.dry_run {
                    println!("  [DRY RUN] Would execute script: {}", name);
                    println!("  Script content SHA256: {}", verification.sha256);
                    GhostLogger::log_action(
                        "script_dry_run",
                        true,
                        Some(&format!("name:{} sha256:{}", name, verification.sha256)),
                    );
                    return Ok(true);
                }

                self.execute_script(&content, name)
            }
            1 => {
                // View full script
                println!("\n  Full Script Content:");
                println!("  ════════════════════");
                for (i, line) in content.lines().enumerate() {
                    println!("  {:4} | {}", i + 1, line);
                }

                // Ask again after viewing
                let execute = Confirm::new()
                    .with_prompt("Execute this script?")
                    .default(false)
                    .interact()
                    .unwrap_or(false);

                if execute {
                    if self.config.dry_run {
                        println!("  [DRY RUN] Would execute script: {}", name);
                        GhostLogger::log_action(
                            "script_dry_run",
                            true,
                            Some(&format!("name:{} sha256:{}", name, verification.sha256)),
                        );
                        return Ok(true);
                    }
                    self.execute_script(&content, name)
                } else {
                    println!("  Execution cancelled");
                    GhostLogger::log_action(
                        "script_cancelled",
                        true,
                        Some(&format!(
                            "name:{} sha256:{} reason:user_declined_after_review",
                            name, verification.sha256
                        )),
                    );
                    Ok(false)
                }
            }
            2 => {
                // Save locally
                let save_path = self.save_script_locally(name, &content)?;
                println!("  Script saved to: {}", save_path.display());
                println!(
                    "  You can review and execute it manually with: bash {}",
                    save_path.display()
                );
                GhostLogger::log_action(
                    "script_saved_locally",
                    true,
                    Some(&format!(
                        "name:{} sha256:{} path:{}",
                        name,
                        verification.sha256,
                        save_path.display()
                    )),
                );
                Ok(false)
            }
            _ => {
                println!("  Execution cancelled");
                GhostLogger::log_action(
                    "script_cancelled",
                    true,
                    Some(&format!(
                        "name:{} sha256:{} reason:user_cancelled",
                        name, verification.sha256
                    )),
                );
                Ok(false)
            }
        }
    }

    /// Execute a script
    fn execute_script(&self, content: &str, name: &str) -> Result<bool> {
        let sha256 = Self::compute_sha256(content);
        println!("\n  Executing script: {}...", name);

        GhostLogger::log_action(
            "script_execute_start",
            true,
            Some(&format!("name:{} sha256:{}", name, sha256)),
        );

        let status = Command::new("bash")
            .arg("-c")
            .arg(content)
            .status()
            .context("Failed to execute script")?;

        if status.success() {
            println!("  Script executed successfully");
            GhostLogger::log_action(
                "script_execute",
                true,
                Some(&format!("name:{} sha256:{} exit:0", name, sha256)),
            );
            Ok(true)
        } else {
            let code = status.code().unwrap_or(-1);
            println!("  Script failed with exit code: {}", code);
            GhostLogger::log_action(
                "script_execute",
                false,
                Some(&format!("name:{} sha256:{} exit:{}", name, sha256, code)),
            );
            Ok(false)
        }
    }

    /// Save script to local file
    fn save_script_locally(&self, name: &str, content: &str) -> Result<PathBuf> {
        let safe_name = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>();

        let path = self.cache_dir.join(format!("{}.sh", safe_name));
        fs::write(&path, content).context("Failed to save script")?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&path, perms)?;
        }

        Ok(path)
    }

    /// Get cached script if available
    fn get_cached_script(&self, url: &str) -> Option<CachedScript> {
        let cache_file = self.get_cache_path(url);

        if !cache_file.exists() {
            return None;
        }

        // Check cache age (expire after 1 hour)
        if let Ok(metadata) = fs::metadata(&cache_file)
            && let Ok(modified) = metadata.modified() {
                let age = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default();

                if age.as_secs() > 3600 {
                    // Cache expired
                    let _ = fs::remove_file(&cache_file);
                    return None;
                }
            }

        if let Ok(content) = fs::read_to_string(&cache_file) {
            let sha256 = Self::compute_sha256(&content);
            return Some(CachedScript {
                url: url.to_string(),
                content,
                sha256,
                cached_at: chrono::Utc::now(),
            });
        }

        None
    }

    /// Cache a script locally
    fn cache_script(&self, url: &str, content: &str) -> Result<()> {
        let cache_file = self.get_cache_path(url);
        fs::write(&cache_file, content).context("Failed to cache script")?;
        Ok(())
    }

    /// Get cache file path for a URL
    fn get_cache_path(&self, url: &str) -> PathBuf {
        let hash = Self::compute_sha256(url);
        self.cache_dir.join(format!("{}.sh", &hash[..16]))
    }
}

/// Simple wrapper for basic script execution with safety checks
pub fn safe_run_script(name: &str, url: &str) -> Result<bool> {
    let config = ScriptSafetyConfig::default();
    let executor = SafeScriptExecutor::new(config)?;
    executor.confirm_and_execute(name, url)
}

/// Run script with custom configuration
pub fn safe_run_script_with_config(
    name: &str,
    url: &str,
    config: ScriptSafetyConfig,
) -> Result<bool> {
    let executor = SafeScriptExecutor::new(config)?;
    executor.confirm_and_execute(name, url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sha256() {
        let content = "echo 'hello world'";
        let hash = SafeScriptExecutor::compute_sha256(content);
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex chars
    }

    #[test]
    fn test_verify_script_detects_sudo() {
        let content = "sudo apt-get update";
        let config = ScriptSafetyConfig::default();
        let executor = SafeScriptExecutor::new(config).unwrap();
        let verification = executor.verify_script("test://url", content);
        assert!(verification.has_sudo);
    }

    #[test]
    fn test_verify_script_detects_rm_rf() {
        let content = "rm -rf /tmp/test";
        let config = ScriptSafetyConfig::default();
        let executor = SafeScriptExecutor::new(config).unwrap();
        let verification = executor.verify_script("test://url", content);
        assert!(verification.has_rm_rf);
    }
}
