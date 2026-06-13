//! `ghostctl ai` - local AI helper for Ollama and the Hermes agent.
//!
//! Ollama operations talk to the REST API configured under `[ai]` in the
//! ghostctl config (defaults to localhost:11434). Hermes operations shell out
//! to the installed `hermes` CLI.

pub mod config;
pub mod hermes;
pub mod ollama;
pub mod tune;

use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};
use config::AiConfig;
use ollama::{OllamaClient, human_bytes};
use std::process::Command as ProcCommand;

pub fn command() -> Command {
    Command::new("ai")
        .about("Local AI helper (Ollama models + Hermes agent)")
        .subcommand(
            Command::new("status").about("Ollama service health, GPU detection, and loaded models"),
        )
        .subcommand(Command::new("models").about("List installed Ollama models"))
        .subcommand(
            Command::new("pull")
                .about("Pull a model from the Ollama registry")
                .arg(
                    Arg::new("model")
                        .required(true)
                        .help("Model name, e.g. qwen3-coder:30b"),
                ),
        )
        .subcommand(
            Command::new("rm").about("Delete an installed model").arg(
                Arg::new("model")
                    .required(true)
                    .help("Model name to delete"),
            ),
        )
        .subcommand(
            Command::new("show")
                .about("Show a model's architecture and max context window")
                .arg(Arg::new("model").required(true).help("Model name")),
        )
        .subcommand(
            Command::new("ctx-check")
                .about("Verify a model's context window meets the configured minimum")
                .arg(Arg::new("model").help("Model name (defaults to [ai].default_model)")),
        )
        .subcommand(
            Command::new("run")
                .about("Run a one-shot prompt against a model (streams output)")
                .arg(Arg::new("model").required(true).help("Model name"))
                .arg(Arg::new("prompt").required(true).help("Prompt text"))
                .arg(
                    Arg::new("no-stream")
                        .long("no-stream")
                        .action(ArgAction::SetTrue)
                        .help("Wait for the full response instead of streaming"),
                )
                .arg(
                    Arg::new("ctx")
                        .long("ctx")
                        .value_parser(clap::value_parser!(u64))
                        .help("Context window size in tokens (options.num_ctx)"),
                )
                .arg(
                    Arg::new("temp")
                        .long("temp")
                        .value_parser(clap::value_parser!(f64))
                        .help("Sampling temperature (options.temperature)"),
                )
                .arg(
                    Arg::new("num-predict")
                        .long("num-predict")
                        .value_parser(clap::value_parser!(i64))
                        .help("Max tokens to generate, -1 for unlimited (options.num_predict)"),
                )
                .arg(
                    Arg::new("seed")
                        .long("seed")
                        .value_parser(clap::value_parser!(i64))
                        .help("RNG seed for reproducible output (options.seed)"),
                ),
        )
        .subcommand(Command::new("ps").about("Show currently loaded models and VRAM usage"))
        .subcommand(
            Command::new("tune")
                .about("Inspect or apply Ollama server tuning (systemd override env)")
                .subcommand(
                    Command::new("show").about("Show the current Ollama override environment"),
                )
                .subcommand(
                    Command::new("recommend")
                        .about("Print recommended tuning for the detected GPU VRAM"),
                )
                .subcommand(
                    Command::new("apply")
                        .about("Write the recommended override (sudo) and restart ollama"),
                ),
        )
        .subcommand(
            Command::new("hermes")
                .about("Pass through to the Hermes agent CLI")
                .arg(
                    Arg::new("args")
                        .num_args(0..)
                        .trailing_var_arg(true)
                        .allow_hyphen_values(true)
                        .help("Arguments forwarded to `hermes` (e.g. doctor, status, model)"),
                ),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = AiConfig::load();

    match matches.subcommand() {
        Some(("status", _)) => status(&cfg),
        Some(("models", _)) => models(&cfg),
        Some(("pull", m)) => {
            let model = m.get_one::<String>("model").unwrap();
            client(&cfg)?.pull_stream(model)
        }
        Some(("rm", m)) => {
            let model = m.get_one::<String>("model").unwrap();
            client(&cfg)?.delete(model)?;
            println!("✓ Deleted {model}");
            Ok(())
        }
        Some(("show", m)) => {
            let model = m.get_one::<String>("model").unwrap();
            show(&cfg, model)
        }
        Some(("ctx-check", m)) => {
            let model = m.get_one::<String>("model").map(String::as_str);
            ctx_check(&cfg, model)
        }
        Some(("run", m)) => {
            let model = m.get_one::<String>("model").unwrap();
            let prompt = m.get_one::<String>("prompt").unwrap();
            let stream = !m.get_flag("no-stream");
            let opts = build_gen_options(
                m.get_one::<u64>("ctx").copied(),
                m.get_one::<f64>("temp").copied(),
                m.get_one::<i64>("num-predict").copied(),
                m.get_one::<i64>("seed").copied(),
            );
            client(&cfg)?.generate_stream(model, prompt, stream, opts)
        }
        Some(("ps", _)) => ps(&cfg),
        Some(("tune", m)) => match m.subcommand() {
            Some(("show", _)) => tune::show(),
            Some(("recommend", _)) => tune::recommend(),
            Some(("apply", _)) => tune::apply(),
            _ => {
                println!("Use `ghostctl ai tune show|recommend|apply`.");
                Ok(())
            }
        },
        Some(("hermes", m)) => {
            let args: Vec<String> = m
                .get_many::<String>("args")
                .map(|vals| vals.cloned().collect())
                .unwrap_or_default();
            hermes::run(&cfg.hermes_bin, &args)
        }
        _ => {
            println!("Use `ghostctl ai --help` to see available subcommands.");
            Ok(())
        }
    }
}

fn client(cfg: &AiConfig) -> Result<OllamaClient> {
    OllamaClient::new(cfg.base(), cfg.timeout_secs)
}

/// Build an Ollama `options` object from per-request flags, including only the
/// keys the user actually set. Returns `None` when no flags were given so the
/// server keeps its own defaults.
fn build_gen_options(
    ctx: Option<u64>,
    temp: Option<f64>,
    num_predict: Option<i64>,
    seed: Option<i64>,
) -> Option<serde_json::Value> {
    let mut map = serde_json::Map::new();
    if let Some(c) = ctx {
        map.insert("num_ctx".into(), c.into());
    }
    if let Some(t) = temp {
        map.insert("temperature".into(), t.into());
    }
    if let Some(n) = num_predict {
        map.insert("num_predict".into(), n.into());
    }
    if let Some(s) = seed {
        map.insert("seed".into(), s.into());
    }
    if map.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(map))
    }
}

fn status(cfg: &AiConfig) -> Result<()> {
    println!("🤖 AI / Ollama status");
    println!("─────────────────────");
    let oc = client(cfg)?;
    let up = oc.is_up();
    println!("  Endpoint : {}", cfg.ollama_url);
    println!(
        "  Service  : {}",
        if up {
            "✓ reachable"
        } else {
            "✗ unreachable"
        }
    );

    if up {
        match oc.ps() {
            Ok(loaded) if !loaded.is_empty() => {
                println!("  Loaded models:");
                for m in &loaded {
                    let ctx = m
                        .context_length
                        .map(|c| format!(", ctx {c}"))
                        .unwrap_or_default();
                    println!(
                        "    • {:<28} VRAM {}{}",
                        m.name,
                        human_bytes(m.size_vram),
                        ctx
                    );
                }
            }
            Ok(_) => println!("  Loaded models: none (idle)"),
            Err(e) => println!("  Loaded models: error ({e})"),
        }
    }

    // GPU / CUDA detection from the ollama service journal (best-effort).
    print!("  GPU      : ");
    match gpu_detect() {
        Some(info) => println!("{info}"),
        None => println!("(no GPU line found in `journalctl -u ollama`)"),
    }
    Ok(())
}

/// Best-effort GPU detection by grepping the ollama service journal.
fn gpu_detect() -> Option<String> {
    let out = ProcCommand::new("journalctl")
        .args(["-u", "ollama", "--no-pager", "-n", "200"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    // Look for the most recent line mentioning a CUDA/GPU compute device.
    text.lines()
        .rev()
        .find(|l| {
            let ll = l.to_lowercase();
            (ll.contains("cuda") || ll.contains("gpu")) && ll.contains("compute")
        })
        .or_else(|| {
            text.lines()
                .rev()
                .find(|l| l.to_lowercase().contains("inference compute"))
        })
        .map(|l| {
            // Trim the journald timestamp/prefix for readability.
            l.split("level=").next().unwrap_or(l).trim().to_string()
        })
}

fn models(cfg: &AiConfig) -> Result<()> {
    let list = client(cfg)?.list_models()?;
    if list.is_empty() {
        println!("No models installed.");
        return Ok(());
    }
    println!("{:<30} {:>10}  {:<10} QUANT", "NAME", "SIZE", "PARAMS");
    for m in &list {
        println!(
            "{:<30} {:>10}  {:<10} {}",
            m.name,
            human_bytes(m.size_bytes),
            m.parameter_size,
            m.quantization
        );
    }
    Ok(())
}

fn show(cfg: &AiConfig, model: &str) -> Result<()> {
    let (arch, ctx) = client(cfg)?.show_context(model)?;
    println!("Model        : {model}");
    println!(
        "Architecture : {}",
        if arch.is_empty() { "unknown" } else { &arch }
    );
    match ctx {
        Some(c) => println!("Max context  : {c} tokens"),
        None => println!("Max context  : unknown"),
    }
    Ok(())
}

fn ctx_check(cfg: &AiConfig, model: Option<&str>) -> Result<()> {
    let model = match model
        .map(str::to_string)
        .or_else(|| cfg.default_model.clone())
    {
        Some(m) => m,
        None => {
            println!(
                "No model given and [ai].default_model is unset. Usage: ghostctl ai ctx-check <model>"
            );
            return Ok(());
        }
    };
    let (_, ctx) = client(cfg)?.show_context(&model)?;
    match ctx {
        Some(c) => {
            let ok = c >= cfg.min_context;
            println!(
                "{} {model}: max context {c} tokens (minimum {})",
                if ok { "✓" } else { "✗" },
                cfg.min_context
            );
            println!(
                "  Note: the *served* context is capped by OLLAMA_CONTEXT_LENGTH on the server."
            );
        }
        None => println!("? {model}: context window not reported by /api/show"),
    }
    Ok(())
}

fn ps(cfg: &AiConfig) -> Result<()> {
    let loaded = client(cfg)?.ps()?;
    if loaded.is_empty() {
        println!("No models currently loaded.");
        return Ok(());
    }
    println!("{:<30} {:>10} {:>10}  CONTEXT", "NAME", "SIZE", "VRAM");
    for m in &loaded {
        let ctx = m
            .context_length
            .map(|c| c.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{:<30} {:>10} {:>10}  {}",
            m.name,
            human_bytes(m.size_bytes),
            human_bytes(m.size_vram),
            ctx
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_gen_options_none_when_unset() {
        assert!(build_gen_options(None, None, None, None).is_none());
    }

    #[test]
    fn test_build_gen_options_only_set_keys() {
        let opts = build_gen_options(Some(32768), None, Some(-1), None).unwrap();
        let obj = opts.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("num_ctx").unwrap().as_u64(), Some(32768));
        assert_eq!(obj.get("num_predict").unwrap().as_i64(), Some(-1));
        assert!(obj.get("temperature").is_none());
        assert!(obj.get("seed").is_none());
    }

    #[test]
    fn test_build_gen_options_all_keys() {
        let opts = build_gen_options(Some(8192), Some(0.7), Some(256), Some(42)).unwrap();
        let obj = opts.as_object().unwrap();
        assert_eq!(obj.len(), 4);
        assert_eq!(obj.get("num_ctx").unwrap().as_u64(), Some(8192));
        assert_eq!(obj.get("temperature").unwrap().as_f64(), Some(0.7));
        assert_eq!(obj.get("seed").unwrap().as_i64(), Some(42));
    }
}
