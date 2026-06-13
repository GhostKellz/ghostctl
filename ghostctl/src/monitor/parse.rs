//! Pure response-parsing functions for the monitoring APIs.
//!
//! Kept separate from the HTTP client so they can be unit-tested against
//! captured JSON fixtures without any network access.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::BTreeMap;

// ---- Prometheus /api/v1/targets ----

#[derive(Debug, Clone)]
pub struct Target {
    pub job: String,
    pub instance: String,
    pub health: String,
    pub scrape_url: String,
    pub last_error: String,
}

#[derive(Deserialize)]
struct TargetsEnvelope {
    data: TargetsData,
}

#[derive(Deserialize)]
struct TargetsData {
    #[serde(rename = "activeTargets", default)]
    active_targets: Vec<ActiveTarget>,
}

#[derive(Deserialize)]
struct ActiveTarget {
    #[serde(default)]
    labels: BTreeMap<String, String>,
    #[serde(rename = "scrapeUrl", default)]
    scrape_url: String,
    #[serde(default)]
    health: String,
    #[serde(rename = "lastError", default)]
    last_error: String,
}

pub fn parse_targets(json: &str) -> Result<Vec<Target>> {
    let env: TargetsEnvelope =
        serde_json::from_str(json).context("failed to parse Prometheus targets response")?;
    Ok(env
        .data
        .active_targets
        .into_iter()
        .map(|t| Target {
            job: t.labels.get("job").cloned().unwrap_or_default(),
            instance: t.labels.get("instance").cloned().unwrap_or_default(),
            health: t.health,
            scrape_url: t.scrape_url,
            last_error: t.last_error,
        })
        .collect())
}

// ---- Prometheus /api/v1/query (instant vector) ----

#[derive(Debug, Clone)]
pub struct InstantSample {
    pub labels: BTreeMap<String, String>,
    pub value: String,
}

#[derive(Deserialize)]
struct QueryEnvelope {
    data: QueryData,
}

#[derive(Deserialize)]
struct QueryData {
    #[serde(default)]
    result: Vec<QueryResult>,
}

#[derive(Deserialize)]
struct QueryResult {
    #[serde(default)]
    metric: BTreeMap<String, String>,
    /// [ <unix_ts: number>, "<value: string>" ]
    #[serde(default)]
    value: Vec<serde_json::Value>,
}

pub fn parse_instant_query(json: &str) -> Result<Vec<InstantSample>> {
    let env: QueryEnvelope =
        serde_json::from_str(json).context("failed to parse Prometheus query response")?;
    Ok(env
        .data
        .result
        .into_iter()
        .map(|r| {
            let value = r
                .value
                .get(1)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            InstantSample {
                labels: r.metric,
                value,
            }
        })
        .collect())
}

// ---- Alertmanager /api/v2/alerts ----

#[derive(Debug, Clone)]
pub struct Alert {
    pub name: String,
    pub severity: String,
    pub state: String,
    pub summary: String,
}

#[derive(Deserialize)]
struct V2Alert {
    #[serde(default)]
    labels: BTreeMap<String, String>,
    #[serde(default)]
    annotations: BTreeMap<String, String>,
    #[serde(default)]
    status: V2Status,
}

#[derive(Deserialize, Default)]
struct V2Status {
    #[serde(default)]
    state: String,
}

pub fn parse_alerts(json: &str) -> Result<Vec<Alert>> {
    let alerts: Vec<V2Alert> =
        serde_json::from_str(json).context("failed to parse Alertmanager alerts response")?;
    Ok(alerts
        .into_iter()
        .map(|a| Alert {
            name: a.labels.get("alertname").cloned().unwrap_or_default(),
            severity: a.labels.get("severity").cloned().unwrap_or_default(),
            state: a.status.state,
            summary: a
                .annotations
                .get("summary")
                .or_else(|| a.annotations.get("description"))
                .cloned()
                .unwrap_or_default(),
        })
        .collect())
}

// ---- Loki /loki/api/v1/query_range ----

#[derive(Debug, Clone)]
pub struct LogLine {
    /// Nanosecond unix timestamp as reported by Loki.
    pub timestamp_ns: u64,
    pub line: String,
}

#[derive(Deserialize)]
struct LokiEnvelope {
    data: LokiData,
}

#[derive(Deserialize)]
struct LokiData {
    #[serde(rename = "resultType", default)]
    result_type: String,
    #[serde(default)]
    result: Vec<LokiStream>,
}

#[derive(Deserialize)]
struct LokiStream {
    /// each value is ["<ns ts string>", "<log line>"]
    #[serde(default)]
    values: Vec<Vec<String>>,
}

pub fn parse_loki_lines(json: &str) -> Result<Vec<LogLine>> {
    let env: LokiEnvelope =
        serde_json::from_str(json).context("failed to parse Loki query response")?;
    if !env.data.result_type.is_empty() && env.data.result_type != "streams" {
        bail!(
            "unexpected Loki resultType '{}' (expected streams)",
            env.data.result_type
        );
    }
    let mut lines = Vec::new();
    for stream in env.data.result {
        for pair in stream.values {
            if pair.len() >= 2 {
                let ts = pair[0].parse::<u64>().unwrap_or(0);
                lines.push(LogLine {
                    timestamp_ns: ts,
                    line: pair[1].clone(),
                });
            }
        }
    }
    // Loki returns newest-first per stream; sort ascending for readable output.
    lines.sort_by_key(|l| l.timestamp_ns);
    Ok(lines)
}

// ---- Grafana /api/datasources/uid/<uid>/health ----

#[derive(Deserialize)]
struct DatasourceHealth {
    #[serde(default)]
    status: String,
    #[serde(default)]
    message: String,
}

pub fn parse_datasource_health(json: &str) -> Result<(String, String)> {
    let h: DatasourceHealth =
        serde_json::from_str(json).context("failed to parse Grafana datasource health")?;
    Ok((h.status, h.message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_targets() {
        let json = r#"{
          "status":"success",
          "data":{"activeTargets":[
            {"labels":{"job":"node","instance":"10.0.0.10:9100"},
             "scrapeUrl":"http://10.0.0.10:9100/metrics","health":"up","lastError":""},
            {"labels":{"job":"crowdsec","instance":"10.0.0.23:6060"},
             "scrapeUrl":"http://10.0.0.23:6060/metrics","health":"down",
             "lastError":"connection refused"}
          ],"droppedTargets":[]}
        }"#;
        let targets = parse_targets(json).unwrap();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].job, "node");
        assert_eq!(targets[0].health, "up");
        assert_eq!(targets[1].health, "down");
        assert_eq!(targets[1].last_error, "connection refused");
    }

    #[test]
    fn test_parse_instant_query() {
        let json = r#"{
          "status":"success",
          "data":{"resultType":"vector","result":[
            {"metric":{"instance":"10.0.0.10:9100"},"value":[1700000000,"42.5"]}
          ]}
        }"#;
        let samples = parse_instant_query(json).unwrap();
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].value, "42.5");
        assert_eq!(
            samples[0].labels.get("instance").map(String::as_str),
            Some("10.0.0.10:9100")
        );
    }

    #[test]
    fn test_parse_alerts() {
        let json = r#"[
          {"labels":{"alertname":"NodeHighCpu","severity":"warning"},
           "annotations":{"summary":"CPU > 90%"},"status":{"state":"active"}},
          {"labels":{"alertname":"TargetDown"},
           "annotations":{"description":"target down"},"status":{"state":"suppressed"}}
        ]"#;
        let alerts = parse_alerts(json).unwrap();
        assert_eq!(alerts.len(), 2);
        assert_eq!(alerts[0].name, "NodeHighCpu");
        assert_eq!(alerts[0].severity, "warning");
        assert_eq!(alerts[0].summary, "CPU > 90%");
        assert_eq!(alerts[1].name, "TargetDown");
        // falls back to description when summary missing
        assert_eq!(alerts[1].summary, "target down");
    }

    #[test]
    fn test_parse_loki_lines_sorted() {
        let json = r#"{
          "status":"success",
          "data":{"resultType":"streams","result":[
            {"stream":{"app":"sshd"},"values":[
              ["1700000002000000000","second"],
              ["1700000001000000000","first"]
            ]}
          ]}
        }"#;
        let lines = parse_loki_lines(json).unwrap();
        assert_eq!(lines.len(), 2);
        // sorted ascending by timestamp
        assert_eq!(lines[0].line, "first");
        assert_eq!(lines[1].line, "second");
    }

    #[test]
    fn test_parse_loki_rejects_non_streams() {
        let json = r#"{"data":{"resultType":"matrix","result":[]}}"#;
        assert!(parse_loki_lines(json).is_err());
    }

    #[test]
    fn test_parse_datasource_health() {
        let json = r#"{"status":"OK","message":"Data source is working"}"#;
        let (status, msg) = parse_datasource_health(json).unwrap();
        assert_eq!(status, "OK");
        assert_eq!(msg, "Data source is working");
    }
}
