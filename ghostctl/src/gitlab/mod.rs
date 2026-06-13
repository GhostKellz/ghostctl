//! `ghostctl gitlab` - lightweight client for self-hosted GitLab.
//!
//! Talks to the GitLab REST API (`/api/v4`) for the day-to-day checks that pair
//! with the rest of ghostctl's dev/CI tooling: confirm connectivity and auth
//! (`status`), validate a `.gitlab-ci.yml` against the CI Lint API (`ci-lint`),
//! inspect pipelines and jobs (`pipelines`, `pipeline`, `trace`), list merge
//! requests, runners, and projects, and trigger/retry/cancel pipelines. The
//! read-only checks are always safe; the write actions (`run`/`retry`/`cancel`)
//! honor the global `--dry-run` and `--yes` flags. The access token is never
//! logged.

pub mod config;

use anyhow::{Context, Result, anyhow, bail};
use clap::{Arg, ArgMatches, Command};
use config::GitlabConfig;
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::time::Duration;

pub fn command() -> Command {
    Command::new("gitlab")
        .about("Self-hosted GitLab: connectivity, CI lint, pipelines, MRs, and runners")
        .subcommand(
            Command::new("status")
                .about("Verify connectivity and authentication to the configured instance"),
        )
        .subcommand(
            Command::new("ci-lint")
                .about("Validate a GitLab CI file via the CI Lint API")
                .arg(
                    Arg::new("file")
                        .help("CI file to validate (default: .gitlab-ci.yml)")
                        .default_value(".gitlab-ci.yml"),
                ),
        )
        .subcommand(
            Command::new("pipelines").about("List recent pipelines for the configured project"),
        )
        .subcommand(
            Command::new("pipeline")
                .about("Show one pipeline and its jobs, grouped by stage")
                .arg(
                    Arg::new("id")
                        .required(true)
                        .help("Pipeline id (see `gitlab pipelines`)"),
                ),
        )
        .subcommand(
            Command::new("trace")
                .about("Print a job's log (useful for debugging a failed CI job)")
                .arg(Arg::new("job").required(true).help("Job id")),
        )
        .subcommand(
            Command::new("runners")
                .about("List CI runners available to the project (online/status)"),
        )
        .subcommand(
            Command::new("mrs")
                .alias("mr")
                .about("List open merge requests for the project"),
        )
        .subcommand(
            Command::new("projects")
                .about("List projects you are a member of (with their ids and paths)"),
        )
        .subcommand(
            Command::new("run")
                .about("Trigger a new pipeline (write; honors --dry-run/--yes)")
                .arg(
                    Arg::new("ref")
                        .help("Branch or tag to run (default: the project's default branch)"),
                ),
        )
        .subcommand(
            Command::new("retry")
                .about("Retry a pipeline (write; honors --dry-run/--yes)")
                .arg(Arg::new("id").required(true).help("Pipeline id")),
        )
        .subcommand(
            Command::new("cancel")
                .about("Cancel a pipeline (write; honors --dry-run/--yes)")
                .arg(Arg::new("id").required(true).help("Pipeline id")),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = GitlabConfig::load();
    match matches.subcommand() {
        Some(("status", _)) => status(&cfg),
        Some(("ci-lint", m)) => ci_lint(&cfg, m.get_one::<String>("file").unwrap()),
        Some(("pipelines", _)) => pipelines(&cfg),
        Some(("pipeline", m)) => pipeline_detail(&cfg, m.get_one::<String>("id").unwrap()),
        Some(("trace", m)) => trace(&cfg, m.get_one::<String>("job").unwrap()),
        Some(("runners", _)) => runners(&cfg),
        Some(("mrs", _)) => merge_requests(&cfg),
        Some(("projects", _)) => projects(&cfg),
        Some(("run", m)) => run_pipeline(&cfg, m.get_one::<String>("ref").map(String::as_str)),
        Some(("retry", m)) => retry_pipeline(&cfg, m.get_one::<String>("id").unwrap()),
        Some(("cancel", m)) => cancel_pipeline(&cfg, m.get_one::<String>("id").unwrap()),
        _ => {
            println!("Use `ghostctl gitlab --help` to see available subcommands.");
            Ok(())
        }
    }
}

fn status(cfg: &GitlabConfig) -> Result<()> {
    let token = require_token(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();

    let version = get_json(&client, &format!("{base}/api/v4/version"), &token)?;
    let user = get_json(&client, &format!("{base}/api/v4/user"), &token)?;

    println!("GitLab instance: {base}");
    println!(
        "  version: {} (revision {})",
        version
            .get("version")
            .and_then(Value::as_str)
            .unwrap_or("?"),
        version
            .get("revision")
            .and_then(Value::as_str)
            .unwrap_or("?"),
    );
    println!(
        "  authenticated as: {} (@{})",
        user.get("name").and_then(Value::as_str).unwrap_or("?"),
        user.get("username").and_then(Value::as_str).unwrap_or("?"),
    );
    match &cfg.project {
        Some(p) => println!("  default project: {p}"),
        None => println!("  default project: (none — set [gitlab].project)"),
    }
    Ok(())
}

fn ci_lint(cfg: &GitlabConfig, file: &str) -> Result<()> {
    let token = require_token(cfg)?;
    let content =
        std::fs::read_to_string(file).with_context(|| format!("failed to read {file}"))?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();

    // A project-scoped lint resolves includes/variables; the global endpoint is
    // a standalone syntax check.
    let url = match &cfg.project {
        Some(p) => format!("{base}/api/v4/projects/{}/ci/lint", encode_project(p)),
        None => format!("{base}/api/v4/ci/lint"),
    };

    let resp = client
        .post(&url)
        .header("PRIVATE-TOKEN", &token)
        .json(&json!({ "content": content }))
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    let http = resp.status();
    if http.as_u16() == 401 {
        bail!("GitLab returned 401 Unauthorized — check the token and its scopes (api).");
    }
    if !http.is_success() {
        bail!("GitLab CI lint returned HTTP {}", http.as_u16());
    }
    let body: Value = resp.json().context("invalid JSON from CI lint API")?;
    let (valid, errors, warnings) = parse_lint_result(&body);

    if valid {
        println!("✅ {file} is valid.");
    } else {
        println!("❌ {file} is invalid.");
    }
    for w in &warnings {
        println!("  ⚠ warning: {w}");
    }
    for e in &errors {
        println!("  ✗ error: {e}");
    }
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn pipelines(cfg: &GitlabConfig) -> Result<()> {
    let token = require_token(cfg)?;
    let project = cfg.project.clone().ok_or_else(|| {
        anyhow!("no project configured — set [gitlab].project (numeric id or group/repo path)")
    })?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let url = format!(
        "{base}/api/v4/projects/{}/pipelines?per_page=20",
        encode_project(&project)
    );

    let body = get_json(&client, &url, &token)?;
    let Some(items) = body.as_array() else {
        bail!("unexpected pipelines response (not a list)");
    };
    if items.is_empty() {
        println!("No pipelines found for project {project}.");
        return Ok(());
    }

    println!("Recent pipelines for {project}:\n");
    println!("{:<8} {:<10} {:<16} STATUS", "ID", "STATUS", "REF");
    for p in items {
        let id = p.get("id").and_then(Value::as_i64).unwrap_or(0);
        let st = p.get("status").and_then(Value::as_str).unwrap_or("?");
        let r = p.get("ref").and_then(Value::as_str).unwrap_or("?");
        let url = p.get("web_url").and_then(Value::as_str).unwrap_or("");
        println!("{id:<8} {st:<10} {r:<16} {url}");
    }
    Ok(())
}

fn pipeline_detail(cfg: &GitlabConfig, id: &str) -> Result<()> {
    let token = require_token(cfg)?;
    let project = require_project(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let enc = encode_project(&project);

    let pipe = get_json(
        &client,
        &format!("{base}/api/v4/projects/{enc}/pipelines/{id}"),
        &token,
    )?;
    let jobs = get_json(
        &client,
        &format!("{base}/api/v4/projects/{enc}/pipelines/{id}/jobs?per_page=100"),
        &token,
    )?;

    println!(
        "Pipeline #{} on {} — {}",
        pipe.get("id").and_then(Value::as_i64).unwrap_or(0),
        pipe.get("ref").and_then(Value::as_str).unwrap_or("?"),
        pipe.get("status").and_then(Value::as_str).unwrap_or("?"),
    );
    if let Some(url) = pipe.get("web_url").and_then(Value::as_str) {
        println!("  {url}");
    }

    let Some(items) = jobs.as_array() else {
        bail!("unexpected jobs response (not a list)");
    };
    if items.is_empty() {
        println!("\nNo jobs found for pipeline {id}.");
        return Ok(());
    }

    println!("\n{:<10} {:<24} {:<10} JOB ID", "STAGE", "JOB", "STATUS");
    // GitLab returns jobs newest-first; show them grouped by stage in API order.
    for j in items {
        let stage = j.get("stage").and_then(Value::as_str).unwrap_or("?");
        let name = j.get("name").and_then(Value::as_str).unwrap_or("?");
        let st = j.get("status").and_then(Value::as_str).unwrap_or("?");
        let jid = j.get("id").and_then(Value::as_i64).unwrap_or(0);
        println!("{stage:<10} {name:<24} {st:<10} {jid}");
    }
    println!("\nView a job's log with `ghostctl gitlab trace <JOB ID>`.");
    Ok(())
}

fn trace(cfg: &GitlabConfig, job: &str) -> Result<()> {
    let token = require_token(cfg)?;
    let project = require_project(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let url = format!(
        "{base}/api/v4/projects/{}/jobs/{job}/trace",
        encode_project(&project)
    );
    let log = get_text(&client, &url, &token)?;
    if log.trim().is_empty() {
        println!("(job {job} has no log output yet)");
    } else {
        print!("{log}");
        if !log.ends_with('\n') {
            println!();
        }
    }
    Ok(())
}

fn runners(cfg: &GitlabConfig) -> Result<()> {
    let token = require_token(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();

    // Prefer project-scoped runners; fall back to the runners the token can see.
    let (url, scope) = match &cfg.project {
        Some(p) => (
            format!("{base}/api/v4/projects/{}/runners", encode_project(p)),
            format!("project {p}"),
        ),
        None => (format!("{base}/api/v4/runners"), "your account".to_string()),
    };

    let body = get_json(&client, &url, &token)?;
    let Some(items) = body.as_array() else {
        bail!("unexpected runners response (not a list)");
    };
    if items.is_empty() {
        println!("No runners available to {scope}.");
        return Ok(());
    }

    println!("Runners for {scope}:\n");
    println!("{:<8} {:<10} {:<8} DESCRIPTION", "ID", "STATUS", "ONLINE");
    for r in items {
        let id = r.get("id").and_then(Value::as_i64).unwrap_or(0);
        let st = r.get("status").and_then(Value::as_str).unwrap_or("?");
        let online = match r.get("online").and_then(Value::as_bool) {
            Some(true) => "yes",
            Some(false) => "no",
            None => "?",
        };
        let desc = r.get("description").and_then(Value::as_str).unwrap_or("");
        println!("{id:<8} {st:<10} {online:<8} {desc}");
    }
    Ok(())
}

fn merge_requests(cfg: &GitlabConfig) -> Result<()> {
    let token = require_token(cfg)?;
    let project = require_project(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let url = format!(
        "{base}/api/v4/projects/{}/merge_requests?state=opened&per_page=20&order_by=updated_at",
        encode_project(&project)
    );

    let body = get_json(&client, &url, &token)?;
    let Some(items) = body.as_array() else {
        bail!("unexpected merge_requests response (not a list)");
    };
    if items.is_empty() {
        println!("No open merge requests for project {project}.");
        return Ok(());
    }

    println!("Open merge requests for {project}:\n");
    for m in items {
        let iid = m.get("iid").and_then(Value::as_i64).unwrap_or(0);
        let title = m.get("title").and_then(Value::as_str).unwrap_or("?");
        let author = m
            .get("author")
            .and_then(|a| a.get("username"))
            .and_then(Value::as_str)
            .unwrap_or("?");
        let src = m
            .get("source_branch")
            .and_then(Value::as_str)
            .unwrap_or("?");
        let tgt = m
            .get("target_branch")
            .and_then(Value::as_str)
            .unwrap_or("?");
        let draft = m.get("draft").and_then(Value::as_bool).unwrap_or(false);
        let flag = if draft { " [draft]" } else { "" };
        println!("!{iid}{flag} {title}");
        println!("    @{author}  {src} → {tgt}");
    }
    Ok(())
}

fn projects(cfg: &GitlabConfig) -> Result<()> {
    let token = require_token(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let url = format!(
        "{base}/api/v4/projects?membership=true&simple=true&order_by=last_activity_at&per_page=30"
    );

    let body = get_json(&client, &url, &token)?;
    let Some(items) = body.as_array() else {
        bail!("unexpected projects response (not a list)");
    };
    if items.is_empty() {
        println!("You are not a member of any projects on {base}.");
        return Ok(());
    }

    println!("Your projects on {base}:\n");
    println!("{:<8} {:<12} PATH", "ID", "VISIBILITY");
    for p in items {
        let id = p.get("id").and_then(Value::as_i64).unwrap_or(0);
        let vis = p.get("visibility").and_then(Value::as_str).unwrap_or("?");
        let path = p
            .get("path_with_namespace")
            .and_then(Value::as_str)
            .unwrap_or("?");
        println!("{id:<8} {vis:<12} {path}");
    }
    println!("\nSet `[gitlab].project` to an id or path above to target pipelines/MRs.");
    Ok(())
}

fn run_pipeline(cfg: &GitlabConfig, gitref: Option<&str>) -> Result<()> {
    let token = require_token(cfg)?;
    let project = require_project(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let enc = encode_project(&project);

    // Under --dry-run, avoid the lookup call: describe the intended ref instead.
    if crate::utils::is_dry_run() {
        let shown = gitref.unwrap_or("the project's default branch");
        println!("[dry-run] would trigger a pipeline on {project}@{shown} (no request sent).");
        return Ok(());
    }

    // Resolve the ref: use the argument, else the project's default branch.
    let gitref = match gitref {
        Some(r) => r.to_string(),
        None => {
            let proj = get_json(&client, &format!("{base}/api/v4/projects/{enc}"), &token)?;
            proj.get("default_branch")
                .and_then(Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| {
                    anyhow!("could not determine default branch — pass a ref explicitly")
                })?
        }
    };

    if !confirm_write(&format!("trigger a pipeline on {project}@{gitref}")) {
        return Ok(());
    }

    let body = post_json(
        &client,
        &format!("{base}/api/v4/projects/{enc}/pipeline"),
        &token,
        Some(json!({ "ref": gitref })),
    )?;
    report_pipeline_action("Triggered", &body);
    Ok(())
}

fn retry_pipeline(cfg: &GitlabConfig, id: &str) -> Result<()> {
    pipeline_write(cfg, id, "retry", "Retried")
}

fn cancel_pipeline(cfg: &GitlabConfig, id: &str) -> Result<()> {
    pipeline_write(cfg, id, "cancel", "Cancelled")
}

/// Shared body for the `retry`/`cancel` pipeline actions (same shape, different
/// verb in the URL).
fn pipeline_write(cfg: &GitlabConfig, id: &str, action: &str, past_tense: &str) -> Result<()> {
    let token = require_token(cfg)?;
    let project = require_project(cfg)?;
    let client = build_client(cfg.timeout_secs)?;
    let base = cfg.base();
    let enc = encode_project(&project);

    if !confirm_write(&format!("{action} pipeline #{id} on {project}")) {
        return Ok(());
    }

    let body = post_json(
        &client,
        &format!("{base}/api/v4/projects/{enc}/pipelines/{id}/{action}"),
        &token,
        None,
    )?;
    report_pipeline_action(past_tense, &body);
    Ok(())
}

/// Print the result of a pipeline write (id/status/url), tolerating partial JSON.
fn report_pipeline_action(past_tense: &str, body: &Value) {
    let id = body.get("id").and_then(Value::as_i64).unwrap_or(0);
    let st = body.get("status").and_then(Value::as_str).unwrap_or("?");
    println!("✅ {past_tense} pipeline #{id} ({st}).");
    if let Some(url) = body.get("web_url").and_then(Value::as_str) {
        println!("   {url}");
    }
}

/// Gate a write action behind `--dry-run` (print intent, do nothing) and a
/// confirmation prompt that is auto-accepted under `--yes`/headless/CI.
fn confirm_write(action: &str) -> bool {
    if crate::utils::is_dry_run() {
        println!("[dry-run] would {action} (no request sent).");
        return false;
    }
    if crate::utils::is_headless() || std::env::var("GHOSTCTL_YES").is_ok() {
        return true;
    }
    use dialoguer::{Confirm, theme::ColorfulTheme};
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("About to {action}. Continue?"))
        .default(false)
        .interact()
        .unwrap_or(false)
}

/// Extract `(valid, errors, warnings)` from either the project-scoped lint
/// response (`{valid, errors, warnings}`) or the global one
/// (`{status: "valid"|"invalid", errors, warnings}`).
fn parse_lint_result(body: &Value) -> (bool, Vec<String>, Vec<String>) {
    let valid = body
        .get("valid")
        .and_then(Value::as_bool)
        .or_else(|| {
            body.get("status")
                .and_then(Value::as_str)
                .map(|s| s.eq_ignore_ascii_case("valid"))
        })
        .unwrap_or(false);
    (
        valid,
        string_list(body, "errors"),
        string_list(body, "warnings"),
    )
}

fn string_list(body: &Value, key: &str) -> Vec<String> {
    body.get(key)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn require_token(cfg: &GitlabConfig) -> Result<String> {
    cfg.resolve_token().ok_or_else(|| {
        anyhow!(
            "no GitLab token found — set GITLAB_TOKEN (or GHOSTCTL_GITLAB_TOKEN), \
             or [gitlab].token in config.toml"
        )
    })
}

fn require_project(cfg: &GitlabConfig) -> Result<String> {
    cfg.project.clone().ok_or_else(|| {
        anyhow!("no project configured — set [gitlab].project (numeric id or group/repo path)")
    })
}

fn build_client(timeout_secs: u64) -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("ghostctl")
        .build()
        .context("failed to build HTTP client")
}

fn get_json(client: &Client, url: &str, token: &str) -> Result<Value> {
    let resp = client
        .get(url)
        .header("PRIVATE-TOKEN", token)
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    let status = resp.status();
    if status.as_u16() == 401 {
        bail!("GitLab returned 401 Unauthorized — check the token and its scopes (api/read_api).");
    }
    if !status.is_success() {
        bail!("GitLab API {url} returned HTTP {}", status.as_u16());
    }
    resp.json().context("invalid JSON from GitLab API")
}

/// GET an endpoint that returns plain text (e.g. a job trace) rather than JSON.
fn get_text(client: &Client, url: &str, token: &str) -> Result<String> {
    let resp = client
        .get(url)
        .header("PRIVATE-TOKEN", token)
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    let status = resp.status();
    if status.as_u16() == 401 {
        bail!("GitLab returned 401 Unauthorized — check the token and its scopes (api/read_api).");
    }
    if status.as_u16() == 404 {
        bail!("GitLab API {url} returned HTTP 404 — no such job, or no log for it.");
    }
    if !status.is_success() {
        bail!("GitLab API {url} returned HTTP {}", status.as_u16());
    }
    resp.text().context("failed to read response body")
}

/// POST to an endpoint, optionally with a JSON body, returning the parsed JSON
/// response. Used by the pipeline write actions.
fn post_json(client: &Client, url: &str, token: &str, body: Option<Value>) -> Result<Value> {
    let mut req = client.post(url).header("PRIVATE-TOKEN", token);
    if let Some(b) = body {
        req = req.json(&b);
    }
    let resp = req
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    let status = resp.status();
    if status.as_u16() == 401 {
        bail!("GitLab returned 401 Unauthorized — check the token and its scopes (api).");
    }
    if status.as_u16() == 403 {
        bail!("GitLab returned 403 Forbidden — the token lacks permission for this action.");
    }
    if !status.is_success() {
        bail!("GitLab API {url} returned HTTP {}", status.as_u16());
    }
    resp.json().context("invalid JSON from GitLab API")
}

/// Encode a project identifier for use in an API path. Numeric ids pass through;
/// `group/repo` paths are percent-encoded (so `/` becomes `%2F`).
fn encode_project(project: &str) -> String {
    if !project.is_empty() && project.chars().all(|c| c.is_ascii_digit()) {
        return project.to_string();
    }
    let mut out = String::with_capacity(project.len());
    for b in project.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encode_project() {
        assert_eq!(encode_project("123"), "123");
        assert_eq!(encode_project("group/repo"), "group%2Frepo");
        assert_eq!(encode_project("group/sub/repo"), "group%2Fsub%2Frepo");
        assert_eq!(encode_project("my-repo.git"), "my-repo.git");
    }

    #[test]
    fn test_parse_lint_project_form() {
        let body = json!({
            "valid": false,
            "errors": ["jobs:build script can't be blank"],
            "warnings": ["unused key"]
        });
        let (valid, errors, warnings) = parse_lint_result(&body);
        assert!(!valid);
        assert_eq!(errors.len(), 1);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_parse_lint_global_form() {
        let body = json!({ "status": "valid", "errors": [], "warnings": [] });
        let (valid, errors, warnings) = parse_lint_result(&body);
        assert!(valid);
        assert!(errors.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_command_tree_is_valid() {
        // Catches duplicate args/subcommands and malformed definitions.
        command().debug_assert();
    }

    #[test]
    fn test_subcommands_parse() {
        let m = command().get_matches_from(["gitlab", "pipeline", "42"]);
        let (name, sub) = m.subcommand().unwrap();
        assert_eq!(name, "pipeline");
        assert_eq!(sub.get_one::<String>("id").map(String::as_str), Some("42"));

        // `mr` is an alias for `mrs`.
        let m = command().get_matches_from(["gitlab", "mr"]);
        assert_eq!(m.subcommand().unwrap().0, "mrs");

        // `run` takes an optional ref.
        let m = command().get_matches_from(["gitlab", "run"]);
        let (_, sub) = m.subcommand().unwrap();
        assert!(sub.get_one::<String>("ref").is_none());
    }
}
