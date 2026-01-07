use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AurPackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub maintainer: Option<String>,
    pub url: Option<String>,
    pub votes: i32,
    pub popularity: f64,
    pub out_of_date: Option<i64>,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AurCache {
    packages: HashMap<String, AurPackageInfo>,
    ttl_seconds: u64,
}

impl AurCache {
    fn new(ttl_seconds: u64) -> Self {
        Self {
            packages: HashMap::new(),
            ttl_seconds,
        }
    }

    fn get_cache_path() -> PathBuf {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("ghostctl")
            .join("aur");

        fs::create_dir_all(&cache_dir).ok();
        cache_dir.join("metadata.json")
    }

    fn load() -> Self {
        let cache_path = Self::get_cache_path();

        if let Ok(content) = fs::read_to_string(&cache_path)
            && let Ok(cache) = serde_json::from_str::<AurCache>(&content) {
                return cache;
            }

        // Default TTL: 1 hour
        Self::new(3600)
    }

    fn save(&self) {
        let cache_path = Self::get_cache_path();
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(&cache_path, json);
        }
    }

    fn is_expired(&self, cached_at: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - cached_at > self.ttl_seconds
    }

    fn get(&self, package: &str) -> Option<AurPackageInfo> {
        if let Some(info) = self.packages.get(package)
            && !self.is_expired(info.cached_at) {
                return Some(info.clone());
            }
        None
    }

    fn insert(&mut self, info: AurPackageInfo) {
        self.packages.insert(info.name.clone(), info);
    }

    fn clear_expired(&mut self) {
        let expired_keys: Vec<_> = self
            .packages
            .iter()
            .filter(|(_, info)| self.is_expired(info.cached_at))
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            self.packages.remove(&key);
        }
    }
}

/// Get AUR package info with caching
pub fn get_package_info(package: &str) -> Option<AurPackageInfo> {
    let mut cache = AurCache::load();

    // Check cache first
    if let Some(info) = cache.get(package) {
        return Some(info);
    }

    // Fetch from AUR
    if let Some(info) = fetch_from_aur(package) {
        cache.insert(info.clone());
        cache.save();
        return Some(info);
    }

    None
}

/// Fetch package info from AUR API
fn fetch_from_aur(package: &str) -> Option<AurPackageInfo> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
        package
    );

    let response = reqwest::blocking::get(&url).ok()?;
    let json: serde_json::Value = response.json().ok()?;

    if let Some(results) = json["results"].as_array()
        && let Some(result) = results.first() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            return Some(AurPackageInfo {
                name: result["Name"].as_str()?.to_string(),
                version: result["Version"].as_str()?.to_string(),
                description: result["Description"].as_str().map(|s| s.to_string()),
                maintainer: result["Maintainer"].as_str().map(|s| s.to_string()),
                url: result["URL"].as_str().map(|s| s.to_string()),
                votes: result["NumVotes"].as_i64().unwrap_or(0) as i32,
                popularity: result["Popularity"].as_f64().unwrap_or(0.0),
                out_of_date: result["OutOfDate"].as_i64(),
                cached_at: now,
            });
        }

    None
}

/// Search AUR packages (with optional caching for popular queries)
pub fn search_packages(query: &str) -> Option<Vec<AurPackageInfo>> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",
        query
    );

    let response = reqwest::blocking::get(&url).ok()?;
    let json: serde_json::Value = response.json().ok()?;

    if let Some(results) = json["results"].as_array() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let packages: Vec<_> = results
            .iter()
            .filter_map(|result| {
                Some(AurPackageInfo {
                    name: result["Name"].as_str()?.to_string(),
                    version: result["Version"].as_str()?.to_string(),
                    description: result["Description"].as_str().map(|s| s.to_string()),
                    maintainer: result["Maintainer"].as_str().map(|s| s.to_string()),
                    url: result["URL"].as_str().map(|s| s.to_string()),
                    votes: result["NumVotes"].as_i64().unwrap_or(0) as i32,
                    popularity: result["Popularity"].as_f64().unwrap_or(0.0),
                    out_of_date: result["OutOfDate"].as_i64(),
                    cached_at: now,
                })
            })
            .collect();

        return Some(packages);
    }

    None
}

/// Clear all cached AUR metadata
pub fn clear_cache() {
    let cache_path = AurCache::get_cache_path();
    let _ = fs::remove_file(&cache_path);
    println!("✅ AUR cache cleared");
}

/// Clear only expired entries
pub fn clear_expired() {
    let mut cache = AurCache::load();
    let before = cache.packages.len();
    cache.clear_expired();
    let after = cache.packages.len();
    cache.save();

    println!("✅ Removed {} expired cache entries", before - after);
}

/// Get cache statistics
pub fn cache_stats() {
    let cache = AurCache::load();
    let total = cache.packages.len();
    let expired = cache
        .packages
        .values()
        .filter(|info| cache.is_expired(info.cached_at))
        .count();

    println!("📊 AUR Cache Statistics");
    println!("  Total entries: {}", total);
    println!("  Valid entries: {}", total - expired);
    println!("  Expired entries: {}", expired);
    println!(
        "  TTL: {} seconds ({} minutes)",
        cache.ttl_seconds,
        cache.ttl_seconds / 60
    );
}
