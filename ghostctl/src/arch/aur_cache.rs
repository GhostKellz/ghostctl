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
            && let Ok(cache) = serde_json::from_str::<AurCache>(&content)
        {
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
            .map(|d| d.as_secs())
            .unwrap_or(0);

        now - cached_at > self.ttl_seconds
    }

    fn get(&self, package: &str) -> Option<AurPackageInfo> {
        if let Some(info) = self.packages.get(package)
            && !self.is_expired(info.cached_at)
        {
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
        && let Some(result) = results.first()
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

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
            .map(|d| d.as_secs())
            .unwrap_or(0);

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

// ============= Utility functions for testing =============

/// Validate AUR package name format
pub fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 127 {
        return false;
    }
    // Must start with alphanumeric
    let first = match name.chars().next() {
        Some(c) => c,
        None => return false,
    };
    if !first.is_ascii_alphanumeric() {
        return false;
    }
    // Can contain alphanumeric, @, ., _, +, -
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '@' | '.' | '_' | '+' | '-'))
}

/// Parse AUR API response to extract package count
pub fn parse_result_count(json: &serde_json::Value) -> usize {
    json["resultcount"].as_u64().unwrap_or(0) as usize
}

/// Check if AUR API response indicates an error
pub fn is_api_error(json: &serde_json::Value) -> bool {
    json["type"].as_str() == Some("error")
}

/// Get error message from AUR API response
pub fn get_api_error_message(json: &serde_json::Value) -> Option<String> {
    json["error"].as_str().map(|s| s.to_string())
}

/// Validate version string format (basic check)
pub fn is_valid_version(version: &str) -> bool {
    if version.is_empty() {
        return false;
    }
    // Version should not contain spaces and should have at least one digit
    !version.contains(' ') && version.chars().any(|c| c.is_ascii_digit())
}

/// Compare two version strings (simplified, not pkgver comparison)
/// Returns: -1 if v1 < v2, 0 if equal, 1 if v1 > v2
pub fn compare_versions_simple(v1: &str, v2: &str) -> i32 {
    // Simple string comparison - not a full version comparison
    match v1.cmp(v2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Calculate cache freshness percentage
pub fn cache_freshness(total: usize, expired: usize) -> f64 {
    if total == 0 {
        return 100.0;
    }
    ((total - expired) as f64 / total as f64) * 100.0
}

impl AurPackageInfo {
    /// Check if package is marked as out of date
    pub fn is_out_of_date(&self) -> bool {
        self.out_of_date.is_some()
    }

    /// Check if package has a maintainer
    pub fn has_maintainer(&self) -> bool {
        self.maintainer.as_ref().is_some_and(|m| !m.is_empty())
    }

    /// Check if package is popular (>= threshold votes)
    pub fn is_popular(&self, threshold: i32) -> bool {
        self.votes >= threshold
    }

    /// Get age of cache entry in seconds
    pub fn cache_age(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now.saturating_sub(self.cached_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package() -> AurPackageInfo {
        AurPackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0-1".to_string(),
            description: Some("A test package".to_string()),
            maintainer: Some("testuser".to_string()),
            url: Some("https://example.com".to_string()),
            votes: 100,
            popularity: 1.5,
            out_of_date: None,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    #[test]
    fn test_is_valid_package_name_valid() {
        assert!(is_valid_package_name("firefox"));
        assert!(is_valid_package_name("linux-zen"));
        assert!(is_valid_package_name("python-pip"));
        assert!(is_valid_package_name("nvidia-dkms"));
        assert!(is_valid_package_name("qt5-base"));
        assert!(is_valid_package_name("yay-bin"));
        assert!(is_valid_package_name("r8168-dkms"));
    }

    #[test]
    fn test_is_valid_package_name_with_special_chars() {
        assert!(is_valid_package_name("package@git"));
        assert!(is_valid_package_name("lib32+extra"));
        assert!(is_valid_package_name("some.package"));
        assert!(is_valid_package_name("pkg_name"));
    }

    #[test]
    fn test_is_valid_package_name_invalid() {
        assert!(!is_valid_package_name(""));
        assert!(!is_valid_package_name("-startswith-dash"));
        assert!(!is_valid_package_name(".startswith-dot"));
        assert!(!is_valid_package_name("has space"));
        assert!(!is_valid_package_name("has/slash"));
        assert!(!is_valid_package_name("has:colon"));
    }

    #[test]
    fn test_is_valid_package_name_too_long() {
        let long_name = "a".repeat(128);
        assert!(!is_valid_package_name(&long_name));
        let ok_name = "a".repeat(127);
        assert!(is_valid_package_name(&ok_name));
    }

    #[test]
    fn test_parse_result_count() {
        let json: serde_json::Value =
            match serde_json::from_str(r#"{"resultcount": 5, "results": []}"#) {
                Ok(v) => v,
                Err(_) => return,
            };
        assert_eq!(parse_result_count(&json), 5);
    }

    #[test]
    fn test_parse_result_count_zero() {
        let json: serde_json::Value =
            match serde_json::from_str(r#"{"resultcount": 0, "results": []}"#) {
                Ok(v) => v,
                Err(_) => return,
            };
        assert_eq!(parse_result_count(&json), 0);
    }

    #[test]
    fn test_parse_result_count_missing() {
        let json: serde_json::Value = match serde_json::from_str(r#"{"results": []}"#) {
            Ok(v) => v,
            Err(_) => return,
        };
        assert_eq!(parse_result_count(&json), 0);
    }

    #[test]
    fn test_is_api_error_true() {
        let json: serde_json::Value =
            match serde_json::from_str(r#"{"type": "error", "error": "some error"}"#) {
                Ok(v) => v,
                Err(_) => return,
            };
        assert!(is_api_error(&json));
    }

    #[test]
    fn test_is_api_error_false() {
        let json: serde_json::Value =
            match serde_json::from_str(r#"{"type": "multiinfo", "results": []}"#) {
                Ok(v) => v,
                Err(_) => return,
            };
        assert!(!is_api_error(&json));
    }

    #[test]
    fn test_get_api_error_message() {
        let json: serde_json::Value =
            match serde_json::from_str(r#"{"type": "error", "error": "Package not found"}"#) {
                Ok(v) => v,
                Err(_) => return,
            };
        assert_eq!(
            get_api_error_message(&json),
            Some("Package not found".to_string())
        );
    }

    #[test]
    fn test_get_api_error_message_missing() {
        let json: serde_json::Value = match serde_json::from_str(r#"{"type": "error"}"#) {
            Ok(v) => v,
            Err(_) => return,
        };
        assert_eq!(get_api_error_message(&json), None);
    }

    #[test]
    fn test_is_valid_version() {
        assert!(is_valid_version("1.0.0"));
        assert!(is_valid_version("1.0.0-1"));
        assert!(is_valid_version("2.3.4.r5.abc123-1"));
        assert!(is_valid_version("r123.abc456"));
    }

    #[test]
    fn test_is_valid_version_invalid() {
        assert!(!is_valid_version(""));
        assert!(!is_valid_version("no digits here"));
        assert!(!is_valid_version("1.0 beta")); // has space
    }

    #[test]
    fn test_compare_versions_simple() {
        assert_eq!(compare_versions_simple("1.0.0", "1.0.0"), 0);
        assert_eq!(compare_versions_simple("1.0.0", "2.0.0"), -1);
        assert_eq!(compare_versions_simple("2.0.0", "1.0.0"), 1);
    }

    #[test]
    fn test_cache_freshness_all_fresh() {
        assert!((cache_freshness(10, 0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_cache_freshness_half_expired() {
        assert!((cache_freshness(10, 5) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_cache_freshness_all_expired() {
        assert!((cache_freshness(10, 10) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_cache_freshness_empty() {
        assert!((cache_freshness(0, 0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_aur_package_info_is_out_of_date() {
        let mut pkg = create_test_package();
        assert!(!pkg.is_out_of_date());

        pkg.out_of_date = Some(1704067200);
        assert!(pkg.is_out_of_date());
    }

    #[test]
    fn test_aur_package_info_has_maintainer() {
        let mut pkg = create_test_package();
        assert!(pkg.has_maintainer());

        pkg.maintainer = None;
        assert!(!pkg.has_maintainer());

        pkg.maintainer = Some("".to_string());
        assert!(!pkg.has_maintainer());
    }

    #[test]
    fn test_aur_package_info_is_popular() {
        let pkg = create_test_package(); // votes = 100
        assert!(pkg.is_popular(50));
        assert!(pkg.is_popular(100));
        assert!(!pkg.is_popular(101));
    }

    #[test]
    fn test_aur_package_info_cache_age() {
        let pkg = create_test_package();
        // Cache age should be very small (just created)
        assert!(pkg.cache_age() < 5);
    }

    #[test]
    fn test_aur_cache_new() {
        let cache = AurCache::new(3600);
        assert_eq!(cache.ttl_seconds, 3600);
        assert!(cache.packages.is_empty());
    }

    #[test]
    fn test_aur_cache_is_expired() {
        let cache = AurCache::new(60); // 60 second TTL
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Fresh entry
        assert!(!cache.is_expired(now));

        // Expired entry (from 2 minutes ago)
        assert!(cache.is_expired(now - 120));
    }

    #[test]
    fn test_aur_cache_insert_and_get() {
        let mut cache = AurCache::new(3600);
        let pkg = create_test_package();

        cache.insert(pkg.clone());
        let retrieved = cache.get("test-package");

        assert!(retrieved.is_some());
        let retrieved = match retrieved {
            Some(r) => r,
            None => return,
        };
        assert_eq!(retrieved.name, "test-package");
    }

    #[test]
    fn test_aur_cache_get_expired() {
        let mut cache = AurCache::new(1); // 1 second TTL
        let mut pkg = create_test_package();

        // Set cached_at to 10 seconds ago
        pkg.cached_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            - 10;

        cache.insert(pkg);
        let retrieved = cache.get("test-package");

        assert!(retrieved.is_none()); // Should be expired
    }

    #[test]
    fn test_aur_cache_clear_expired() {
        let mut cache = AurCache::new(1); // 1 second TTL

        // Add fresh package
        let fresh_pkg = create_test_package();
        cache.insert(fresh_pkg);

        // Add expired package
        let expired_pkg = AurPackageInfo {
            name: "expired-package".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            maintainer: None,
            url: None,
            votes: 0,
            popularity: 0.0,
            out_of_date: None,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
                - 10,
        };
        cache
            .packages
            .insert("expired-package".to_string(), expired_pkg);

        assert_eq!(cache.packages.len(), 2);
        cache.clear_expired();
        assert_eq!(cache.packages.len(), 1);
        assert!(cache.packages.contains_key("test-package"));
    }
}
