//! `ghostctl ai tune` - inspect and apply Ollama server tuning.
//!
//! The performance-relevant Ollama settings (flash attention, KV cache type,
//! served context length, keep-alive, parallelism) live in the systemd drop-in
//! at `/etc/systemd/system/ollama.service.d/override.conf`, not in any per-request
//! API call. This command reads that drop-in, recommends values for the detected
//! GPU, and can write the recommendation (with sudo) and restart the service.

use anyhow::{Context, Result, bail};
use std::io::Write;
use std::process::Command;

use crate::utils::{is_dry_run, sudo_run};

/// systemd drop-in that overrides the packaged ollama unit.
const OVERRIDE_PATH: &str = "/etc/systemd/system/ollama.service.d/override.conf";

/// Show the current Ollama override env (drop-in file + effective env).
pub fn show() -> Result<()> {
    println!("Ollama tuning ({OVERRIDE_PATH})");
    println!("────────────────────────────────────────");

    match std::fs::read_to_string(OVERRIDE_PATH) {
        Ok(content) => {
            let env = parse_environment_lines(&content);
            if env.is_empty() {
                println!("  drop-in present but defines no Environment= keys");
            } else {
                println!("  Drop-in environment:");
                for (k, v) in &env {
                    println!("    {k}={v}");
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("  No drop-in at {OVERRIDE_PATH} (ollama is using packaged defaults).");
            println!("  Run `ghostctl ai tune recommend` for suggested settings.");
        }
        Err(e) => println!("  Could not read drop-in: {e}"),
    }

    // Effective env as systemd sees it (best-effort).
    if let Ok(out) = Command::new("systemctl")
        .args(["show", "ollama", "-p", "Environment", "--no-pager"])
        .output()
        && out.status.success()
    {
        let text = String::from_utf8_lossy(&out.stdout);
        let effective: Vec<&str> = text
            .trim()
            .trim_start_matches("Environment=")
            .split_whitespace()
            .filter(|kv| kv.starts_with("OLLAMA_"))
            .collect();
        if !effective.is_empty() {
            println!("  Effective OLLAMA_* env (systemctl show):");
            for kv in effective {
                println!("    {kv}");
            }
        }
    }
    Ok(())
}

/// Detect GPU VRAM and print a recommended override block.
pub fn recommend() -> Result<()> {
    let (recs, label) = match detect_vram() {
        Some((mb, name)) => (
            recommend_for_vram(mb),
            format!("{name} ({} GB VRAM)", mb / 1024),
        ),
        None => (
            recommend_for_vram(0),
            "no NVIDIA GPU detected — conservative defaults".to_string(),
        ),
    };

    println!("Recommended Ollama tuning for: {label}");
    println!("Note: q8_0 KV cache requires flash attention (both are set together).");
    println!();
    print!("{}", render_override_block(&recs));
    println!();
    println!("Apply with: ghostctl ai tune apply   (writes {OVERRIDE_PATH}, needs sudo)");
    Ok(())
}

/// Write the recommended override (sudo), reload systemd, and restart ollama.
pub fn apply() -> Result<()> {
    let (recs, label) = match detect_vram() {
        Some((mb, name)) => (recommend_for_vram(mb), format!("{name} ({} GB)", mb / 1024)),
        None => bail!(
            "no NVIDIA GPU detected via nvidia-smi; refusing to auto-apply. \
             Run `ghostctl ai tune recommend` and edit {OVERRIDE_PATH} by hand."
        ),
    };

    // Preserve unmanaged keys already in the drop-in (e.g. OLLAMA_MODELS,
    // OLLAMA_HOST) and only update/append the performance keys.
    let existing = std::fs::read_to_string(OVERRIDE_PATH)
        .ok()
        .map(|c| parse_environment_lines(&c))
        .unwrap_or_default();
    let merged = merge_env(&existing, &recs);
    let block = render_override_block(&merged);

    println!("Target GPU : {label}");
    if existing.is_empty() {
        println!("Current {OVERRIDE_PATH}: (none)");
    } else {
        println!("Current {OVERRIDE_PATH}:");
        for (k, v) in &existing {
            println!("  {k}={v}");
        }
    }
    println!("Proposed (existing keys preserved):");
    for (k, v) in &merged {
        println!("  {k}={v}");
    }
    println!();

    if is_dry_run() {
        println!("[DRY RUN] Would write {OVERRIDE_PATH}, daemon-reload, and restart ollama.");
        return Ok(());
    }

    if !confirm("Write this override and restart ollama?") {
        println!("Aborted.");
        return Ok(());
    }

    // Stage the content in a temp file, then install it into /etc with sudo.
    let tmp = std::env::temp_dir().join(format!("ghostctl-ollama-override.{}.conf", proc_id()));
    std::fs::write(&tmp, &block).with_context(|| format!("failed to stage {}", tmp.display()))?;

    let tmp_str = tmp.to_string_lossy().to_string();
    let install = sudo_run("install", &["-Dm644", &tmp_str, OVERRIDE_PATH])
        .context("failed to run install")?;
    let _ = std::fs::remove_file(&tmp);
    if !install.success {
        bail!(
            "install to {OVERRIDE_PATH} failed: {}",
            install.stderr.trim()
        );
    }

    let reload = sudo_run("systemctl", &["daemon-reload"]).context("daemon-reload failed")?;
    if !reload.success {
        bail!("systemctl daemon-reload failed: {}", reload.stderr.trim());
    }
    let restart = sudo_run("systemctl", &["restart", "ollama"]).context("restart failed")?;
    if !restart.success {
        bail!("systemctl restart ollama failed: {}", restart.stderr.trim());
    }

    println!("✓ Wrote {OVERRIDE_PATH} and restarted ollama.");
    Ok(())
}

fn proc_id() -> u32 {
    std::process::id()
}

fn confirm(prompt: &str) -> bool {
    if std::env::var("GHOSTCTL_YES").is_ok() || crate::utils::is_headless() {
        return true;
    }
    print!("{prompt} [y/N] ");
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Detect total VRAM (MB) and the GPU name from nvidia-smi, if present.
fn detect_vram() -> Option<(u64, String)> {
    if which::which("nvidia-smi").is_err() {
        return None;
    }
    let out = Command::new("nvidia-smi")
        .args([
            "--query-gpu=memory.total,name",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().next()?;
    let (mb, name) = line.split_once(',')?;
    let mb: u64 = mb.trim().parse().ok()?;
    Some((mb, name.trim().to_string()))
}

// ---- Pure helpers (unit-testable) ----

/// Recommend OLLAMA_* tuning values for a given VRAM size (MB). A VRAM of 0
/// means "unknown / no GPU" and yields conservative defaults. q8_0 KV cache is
/// always paired with flash attention, which it requires.
pub fn recommend_for_vram(vram_mb: u64) -> Vec<(String, String)> {
    let (keep_alive, num_parallel, max_loaded, context_length) = if vram_mb >= 20_000 {
        ("30m", 2, 2, 32_768)
    } else if vram_mb >= 12_000 {
        ("15m", 1, 1, 16_384)
    } else if vram_mb >= 6_000 {
        ("5m", 1, 1, 8_192)
    } else {
        ("5m", 1, 1, 4_096)
    };
    [
        ("OLLAMA_FLASH_ATTENTION", "1".to_string()),
        ("OLLAMA_KV_CACHE_TYPE", "q8_0".to_string()),
        ("OLLAMA_CONTEXT_LENGTH", context_length.to_string()),
        ("OLLAMA_KEEP_ALIVE", keep_alive.to_string()),
        ("OLLAMA_NUM_PARALLEL", num_parallel.to_string()),
        ("OLLAMA_MAX_LOADED_MODELS", max_loaded.to_string()),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

/// Merge recommended keys into the existing drop-in env: unmanaged keys
/// (e.g. OLLAMA_MODELS, OLLAMA_HOST) are preserved in place, managed keys are
/// updated where present and appended otherwise.
pub fn merge_env(
    existing: &[(String, String)],
    recs: &[(String, String)],
) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = existing.to_vec();
    for (k, v) in recs {
        if let Some(slot) = out.iter_mut().find(|(ek, _)| ek == k) {
            slot.1 = v.clone();
        } else {
            out.push((k.clone(), v.clone()));
        }
    }
    out
}

/// Render a systemd drop-in `[Service]` block from key/value pairs.
pub fn render_override_block(recs: &[(String, String)]) -> String {
    let mut s = String::from("[Service]\n");
    for (k, v) in recs {
        s.push_str(&format!("Environment=\"{k}={v}\"\n"));
    }
    s
}

/// Parse `Environment="KEY=VALUE"` lines out of a drop-in file.
pub fn parse_environment_lines(content: &str) -> Vec<(String, String)> {
    content
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("Environment="))
        .map(|rest| rest.trim_matches('"'))
        .filter_map(|kv| kv.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommend_high_vram() {
        let r = recommend_for_vram(32_768);
        let map: std::collections::HashMap<_, _> = r.iter().cloned().collect();
        assert_eq!(map["OLLAMA_FLASH_ATTENTION"], "1");
        assert_eq!(map["OLLAMA_KV_CACHE_TYPE"], "q8_0");
        assert_eq!(map["OLLAMA_CONTEXT_LENGTH"], "32768");
        assert_eq!(map["OLLAMA_NUM_PARALLEL"], "2");
        assert_eq!(map["OLLAMA_MAX_LOADED_MODELS"], "2");
    }

    #[test]
    fn test_recommend_low_vram_conservative() {
        let r = recommend_for_vram(4_096);
        let map: std::collections::HashMap<_, _> = r.iter().cloned().collect();
        assert_eq!(map["OLLAMA_CONTEXT_LENGTH"], "4096");
        assert_eq!(map["OLLAMA_NUM_PARALLEL"], "1");
        assert_eq!(map["OLLAMA_MAX_LOADED_MODELS"], "1");
    }

    #[test]
    fn test_recommend_unknown_gpu() {
        // 0 MB (no GPU) must still pair flash attention with the q8_0 KV cache.
        let r = recommend_for_vram(0);
        let map: std::collections::HashMap<_, _> = r.iter().cloned().collect();
        assert_eq!(map["OLLAMA_FLASH_ATTENTION"], "1");
        assert_eq!(map["OLLAMA_KV_CACHE_TYPE"], "q8_0");
    }

    #[test]
    fn test_render_override_block() {
        let recs = vec![
            ("OLLAMA_FLASH_ATTENTION".to_string(), "1".to_string()),
            ("OLLAMA_KV_CACHE_TYPE".to_string(), "q8_0".to_string()),
        ];
        let block = render_override_block(&recs);
        assert!(block.starts_with("[Service]\n"));
        assert!(block.contains("Environment=\"OLLAMA_FLASH_ATTENTION=1\"\n"));
        assert!(block.contains("Environment=\"OLLAMA_KV_CACHE_TYPE=q8_0\"\n"));
    }

    #[test]
    fn test_merge_env_preserves_unmanaged_and_updates_managed() {
        let existing = vec![
            ("OLLAMA_MODELS".to_string(), "/data/ollama".to_string()),
            ("OLLAMA_HOST".to_string(), "127.0.0.1:11434".to_string()),
            ("OLLAMA_CONTEXT_LENGTH".to_string(), "65536".to_string()),
        ];
        let recs = recommend_for_vram(32_768);
        let merged = merge_env(&existing, &recs);
        let map: std::collections::HashMap<_, _> = merged.iter().cloned().collect();
        // Unmanaged keys survive untouched.
        assert_eq!(map["OLLAMA_MODELS"], "/data/ollama");
        assert_eq!(map["OLLAMA_HOST"], "127.0.0.1:11434");
        // Managed key is updated in place (not duplicated).
        assert_eq!(map["OLLAMA_CONTEXT_LENGTH"], "32768");
        assert_eq!(
            merged
                .iter()
                .filter(|(k, _)| k == "OLLAMA_CONTEXT_LENGTH")
                .count(),
            1
        );
        // New managed keys are appended.
        assert_eq!(map["OLLAMA_FLASH_ATTENTION"], "1");
    }

    #[test]
    fn test_parse_environment_lines() {
        let content = "[Service]\n\
            Environment=\"OLLAMA_MODELS=/data/ollama\"\n\
            Environment=\"OLLAMA_FLASH_ATTENTION=1\"\n\
            # comment\n\
            Environment=\"OLLAMA_KEEP_ALIVE=30m\"\n";
        let env = parse_environment_lines(content);
        assert_eq!(env.len(), 3);
        assert_eq!(env[0], ("OLLAMA_MODELS".into(), "/data/ollama".into()));
        assert_eq!(env[1], ("OLLAMA_FLASH_ATTENTION".into(), "1".into()));
        assert_eq!(env[2], ("OLLAMA_KEEP_ALIVE".into(), "30m".into()));
    }

    #[test]
    fn test_parse_environment_round_trips_render() {
        let recs = recommend_for_vram(32_768);
        let block = render_override_block(&recs);
        let parsed = parse_environment_lines(&block);
        assert_eq!(parsed.len(), recs.len());
        assert_eq!(parsed[0].0, "OLLAMA_FLASH_ATTENTION");
    }
}
