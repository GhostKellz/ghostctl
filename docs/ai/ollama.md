# AI & Local Models

GhostCTL manages a local [Ollama](https://ollama.com) instance and integrates
with the Hermes agent CLI, so you can inspect models, check context windows,
and run prompts from the command line.

## Quick Commands

```bash
ghostctl ai status                 # Ollama health, GPU detection, loaded models
ghostctl ai models                 # List installed models
ghostctl ai pull llama3.1          # Pull a model from the registry
ghostctl ai rm llama3.1            # Delete an installed model
ghostctl ai show llama3.1          # Architecture + max context window
ghostctl ai ctx-check llama3.1     # Verify context meets configured minimum
ghostctl ai run llama3.1 "hello"   # One-shot prompt (streams output)
ghostctl ai ps                     # Loaded models and VRAM usage
ghostctl ai tune recommend         # Suggest Ollama server tuning for your GPU
ghostctl ai hermes ...             # Pass through to the Hermes agent CLI
```

## Per-Request Tuning (`ai run`)

`ai run` accepts optional flags that map to the Ollama `options` object, so you
can override context size and sampling per prompt without changing any config:

```bash
ghostctl ai run llama3.1 "summarize this" --ctx 16384   # num_ctx
ghostctl ai run llama3.1 "be creative"    --temp 0.9    # temperature
ghostctl ai run llama3.1 "draft" --num-predict 512      # cap output tokens
ghostctl ai run llama3.1 "reproducible"   --seed 42     # deterministic seed
```

Only the flags you pass are sent; unset flags fall back to the model/server
defaults.

## Server Tuning (`ai tune`)

`ai tune` manages the Ollama systemd drop-in
(`/etc/systemd/system/ollama.service.d/override.conf`) where server-wide
performance settings live (flash attention, KV cache type, context length,
keep-alive, parallelism, max loaded models).

```bash
ghostctl ai tune show        # Print the current override + effective OLLAMA_* env
ghostctl ai tune recommend   # Detect VRAM and print a ready-to-paste [Service] block
ghostctl ai tune apply       # Merge recommendations into the override and restart ollama
```

- **show** and **recommend** are read-only and never touch the system.
- **recommend** detects VRAM via `nvidia-smi` and scales the suggested values
  (e.g. >=20 GB enables flash attention, q8_0 KV cache, 32k context, 2 parallel
  requests, 2 loaded models).
- **apply** *merges* the recommended keys into the existing override, preserving
  any unmanaged keys you already set (such as `OLLAMA_MODELS` or `OLLAMA_HOST`),
  shows the proposed result, then writes via `sudo`, runs `systemctl
  daemon-reload`, and restarts the `ollama` service. It honors the global
  `--dry-run` (print only), `--yes` (skip confirmation), and `--plain` flags.

## Features

- Ollama service health with GPU detection
- Model inventory, pull, and removal
- Model architecture and maximum context-window inspection
- Context-window verification against a configured minimum
- One-shot streaming prompts with per-request tuning (`--ctx`, `--temp`, `--num-predict`, `--seed`)
- Loaded-model and VRAM usage reporting
- VRAM-aware Ollama server tuning (`ai tune show|recommend|apply`)
- Hermes agent CLI passthrough

## Configuration

Settings live under `[ai]` in `config.toml` (`~/.config/ghostctl/config.toml`):
the Ollama URL, an optional default model, the minimum acceptable context size,
and the Hermes binary path. Run `ghostctl config show` to see resolved values.
