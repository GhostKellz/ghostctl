//! Blocking Ollama REST client plus pure parsing helpers.
//!
//! Streaming endpoints (`/api/generate`, `/api/pull`) read newline-delimited
//! JSON directly off the blocking response body, so no async runtime is needed.

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

pub struct OllamaClient {
    client: Client,
    base: String,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub size_bytes: u64,
    pub family: String,
    pub parameter_size: String,
    pub quantization: String,
}

#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub name: String,
    pub size_bytes: u64,
    pub size_vram: u64,
    pub context_length: Option<u64>,
}

impl OllamaClient {
    pub fn new(base: &str, timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("ghostctl")
            .build()
            .context("failed to build HTTP client")?;
        Ok(Self {
            client,
            base: base.trim_end_matches('/').to_string(),
        })
    }

    pub fn is_up(&self) -> bool {
        self.client
            .get(format!("{}/api/version", self.base))
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let body = self.get_text(&format!("{}/api/tags", self.base))?;
        parse_tags(&body)
    }

    pub fn ps(&self) -> Result<Vec<LoadedModel>> {
        let body = self.get_text(&format!("{}/api/ps", self.base))?;
        parse_ps(&body)
    }

    /// Returns (architecture, max_context) for a model via /api/show.
    pub fn show_context(&self, model: &str) -> Result<(String, Option<u64>)> {
        let resp = self
            .client
            .post(format!("{}/api/show", self.base))
            .json(&json!({ "name": model }))
            .send()
            .context("request to /api/show failed")?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if !status.is_success() {
            bail!("HTTP {} from /api/show: {}", status.as_u16(), body.trim());
        }
        parse_show_context(&body)
    }

    pub fn delete(&self, model: &str) -> Result<()> {
        let resp = self
            .client
            .delete(format!("{}/api/delete", self.base))
            .json(&json!({ "name": model }))
            .send()
            .context("request to /api/delete failed")?;
        if !resp.status().is_success() {
            bail!("HTTP {} deleting model {}", resp.status().as_u16(), model);
        }
        Ok(())
    }

    /// Stream a one-shot completion, writing response chunks to stdout.
    ///
    /// `options` is an optional Ollama `options` object (num_ctx, temperature, …);
    /// when present it is attached to the request body.
    pub fn generate_stream(
        &self,
        model: &str,
        prompt: &str,
        stream: bool,
        options: Option<serde_json::Value>,
    ) -> Result<()> {
        let mut body = json!({ "model": model, "prompt": prompt, "stream": stream });
        if let Some(opts) = options {
            body["options"] = opts;
        }
        let resp = self
            .client
            .post(format!("{}/api/generate", self.base))
            .json(&body)
            .send()
            .context("request to /api/generate failed")?;
        if !resp.status().is_success() {
            bail!("HTTP {} from /api/generate", resp.status().as_u16());
        }
        let reader = BufReader::new(resp);
        let mut out = std::io::stdout();
        for line in reader.lines() {
            let line = line.context("error reading stream")?;
            if line.trim().is_empty() {
                continue;
            }
            #[derive(Deserialize)]
            struct Chunk {
                #[serde(default)]
                response: String,
                #[serde(default)]
                done: bool,
                #[serde(default)]
                error: Option<String>,
            }
            let chunk: Chunk =
                serde_json::from_str(&line).with_context(|| format!("bad NDJSON chunk: {line}"))?;
            if let Some(err) = chunk.error {
                bail!("ollama error: {err}");
            }
            print!("{}", chunk.response);
            let _ = out.flush();
            if chunk.done {
                println!();
                break;
            }
        }
        Ok(())
    }

    /// Stream a model pull, printing status lines as they arrive.
    pub fn pull_stream(&self, model: &str) -> Result<()> {
        let resp = self
            .client
            .post(format!("{}/api/pull", self.base))
            .json(&json!({ "name": model, "stream": true }))
            .send()
            .context("request to /api/pull failed")?;
        if !resp.status().is_success() {
            bail!("HTTP {} from /api/pull", resp.status().as_u16());
        }
        let reader = BufReader::new(resp);
        let mut last_status = String::new();
        for line in reader.lines() {
            let line = line.context("error reading pull stream")?;
            if line.trim().is_empty() {
                continue;
            }
            #[derive(Deserialize)]
            struct PullChunk {
                #[serde(default)]
                status: String,
                #[serde(default)]
                completed: Option<u64>,
                #[serde(default)]
                total: Option<u64>,
                #[serde(default)]
                error: Option<String>,
            }
            let chunk: PullChunk =
                serde_json::from_str(&line).with_context(|| format!("bad NDJSON chunk: {line}"))?;
            if let Some(err) = chunk.error {
                bail!("ollama error: {err}");
            }
            match (chunk.completed, chunk.total) {
                (Some(c), Some(t)) if t > 0 => {
                    let pct = (c as f64 / t as f64) * 100.0;
                    print!("\r{:<40} {:>5.1}%", chunk.status, pct);
                    let _ = std::io::stdout().flush();
                }
                _ => {
                    if chunk.status != last_status {
                        println!("\r{}", chunk.status);
                        last_status = chunk.status;
                    }
                }
            }
        }
        println!();
        Ok(())
    }

    fn get_text(&self, url: &str) -> Result<String> {
        let resp = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if !status.is_success() {
            bail!("HTTP {} from {}", status.as_u16(), url);
        }
        Ok(body)
    }
}

// ---- Pure parsers (unit-testable) ----

#[derive(Deserialize)]
struct TagsEnvelope {
    #[serde(default)]
    models: Vec<TagModel>,
}

#[derive(Deserialize)]
struct TagModel {
    #[serde(default)]
    name: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    details: TagDetails,
}

#[derive(Deserialize, Default)]
struct TagDetails {
    #[serde(default)]
    family: String,
    #[serde(default)]
    parameter_size: String,
    #[serde(default)]
    quantization_level: String,
}

pub fn parse_tags(json: &str) -> Result<Vec<ModelInfo>> {
    let env: TagsEnvelope =
        serde_json::from_str(json).context("failed to parse /api/tags response")?;
    Ok(env
        .models
        .into_iter()
        .map(|m| ModelInfo {
            name: m.name,
            size_bytes: m.size,
            family: m.details.family,
            parameter_size: m.details.parameter_size,
            quantization: m.details.quantization_level,
        })
        .collect())
}

#[derive(Deserialize)]
struct PsEnvelope {
    #[serde(default)]
    models: Vec<PsModel>,
}

#[derive(Deserialize)]
struct PsModel {
    #[serde(default)]
    name: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    size_vram: u64,
    #[serde(default)]
    context_length: Option<u64>,
}

pub fn parse_ps(json: &str) -> Result<Vec<LoadedModel>> {
    let env: PsEnvelope = serde_json::from_str(json).context("failed to parse /api/ps response")?;
    Ok(env
        .models
        .into_iter()
        .map(|m| LoadedModel {
            name: m.name,
            size_bytes: m.size,
            size_vram: m.size_vram,
            context_length: m.context_length,
        })
        .collect())
}

/// Pull the architecture and `<arch>.context_length` out of an /api/show response.
pub fn parse_show_context(json: &str) -> Result<(String, Option<u64>)> {
    let v: serde_json::Value =
        serde_json::from_str(json).context("failed to parse /api/show response")?;
    let model_info = v.get("model_info").and_then(|m| m.as_object());
    let arch = model_info
        .and_then(|m| m.get("general.architecture"))
        .and_then(|a| a.as_str())
        .unwrap_or("")
        .to_string();
    let ctx = model_info.and_then(|m| {
        m.iter()
            .find(|(k, _)| k.ends_with(".context_length"))
            .and_then(|(_, val)| val.as_u64())
    });
    Ok((arch, ctx))
}

/// Human-friendly byte formatting (e.g. 18.6 GB).
pub fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tags() {
        let json = r#"{"models":[
          {"name":"qwen3-coder:30b","size":18601631252,
           "details":{"family":"qwen3","parameter_size":"30.5B","quantization_level":"Q4_K_M"}},
          {"name":"nomic-embed-text:latest","size":274302450,
           "details":{"family":"nomic-bert","parameter_size":"137M","quantization_level":"F16"}}
        ]}"#;
        let models = parse_tags(json).unwrap();
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].name, "qwen3-coder:30b");
        assert_eq!(models[0].family, "qwen3");
        assert_eq!(models[0].parameter_size, "30.5B");
        assert_eq!(models[1].quantization, "F16");
    }

    #[test]
    fn test_parse_ps() {
        let json = r#"{"models":[
          {"name":"qwen3-coder:30b","size":20000000000,"size_vram":20000000000,"context_length":65536}
        ]}"#;
        let loaded = parse_ps(json).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].size_vram, 20000000000);
        assert_eq!(loaded[0].context_length, Some(65536));
    }

    #[test]
    fn test_parse_ps_missing_context() {
        let json = r#"{"models":[{"name":"m","size":1,"size_vram":1}]}"#;
        let loaded = parse_ps(json).unwrap();
        assert_eq!(loaded[0].context_length, None);
    }

    #[test]
    fn test_parse_show_context() {
        let json = r#"{
          "details":{"family":"qwen3"},
          "model_info":{"general.architecture":"qwen3","qwen3.context_length":262144,
                        "qwen3.embedding_length":5120}
        }"#;
        let (arch, ctx) = parse_show_context(json).unwrap();
        assert_eq!(arch, "qwen3");
        assert_eq!(ctx, Some(262144));
    }

    #[test]
    fn test_parse_show_context_absent() {
        let json = r#"{"model_info":{"general.architecture":"x"}}"#;
        let (arch, ctx) = parse_show_context(json).unwrap();
        assert_eq!(arch, "x");
        assert_eq!(ctx, None);
    }

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(512), "512 B");
        assert_eq!(human_bytes(1536), "1.5 KB");
        assert_eq!(human_bytes(18_601_631_252), "17.3 GB");
    }
}
