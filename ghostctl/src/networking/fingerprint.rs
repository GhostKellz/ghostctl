//! OS Fingerprinting Module
//!
//! Provides passive and semi-passive OS detection based on TCP/IP stack characteristics:
//! - TTL (Time To Live) analysis
//! - TCP window size patterns
//! - TCP options ordering (MSS, Window Scale, SACK, Timestamps)
//! - Known OS signature database
//!
//! This module enables network scanners to identify remote operating systems
//! without requiring special privileges beyond what's needed for TCP connections.

use std::collections::HashMap;
use std::sync::LazyLock;

/// OS fingerprint result
#[derive(Debug, Clone)]
pub struct OsFingerprint {
    /// Detected OS family (e.g., "Windows", "Linux", "macOS", "BSD")
    pub os_family: String,
    /// Specific OS guess (e.g., "Windows 10/11", "Linux 5.x")
    pub os_guess: String,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f32,
    /// Detected TTL value
    pub ttl: Option<u8>,
    /// Detected TCP window size (u32 to support scaled windows)
    pub window_size: Option<u32>,
    /// Detected TCP options
    pub tcp_options: Option<TcpOptionsInfo>,
    /// Additional notes about the detection
    pub notes: Vec<String>,
}

/// TCP options information for fingerprinting
#[derive(Debug, Clone)]
pub struct TcpOptionsInfo {
    /// Maximum Segment Size
    pub mss: Option<u16>,
    /// Window scale factor
    pub window_scale: Option<u8>,
    /// SACK permitted
    pub sack_permitted: bool,
    /// Timestamps enabled
    pub timestamps: bool,
    /// Options order signature
    pub options_order: String,
}

/// Known OS signature for matching
#[derive(Debug, Clone)]
pub struct OsSignature {
    /// OS family
    pub family: &'static str,
    /// Specific OS version
    pub version: &'static str,
    /// Expected TTL range (min, max)
    pub ttl_range: (u8, u8),
    /// Expected window sizes (u32 to support window scaling)
    pub window_sizes: &'static [u32],
    /// Expected TCP options order pattern
    pub options_pattern: Option<&'static str>,
    /// Typical MSS values
    pub mss_values: &'static [u16],
}

/// OS signature database
pub static OS_SIGNATURES: LazyLock<Vec<OsSignature>> = LazyLock::new(|| {
    vec![
        // Windows signatures
        OsSignature {
            family: "Windows",
            version: "Windows 10/11/Server 2016+",
            ttl_range: (128, 128),
            window_sizes: &[65535, 64240, 65535],
            options_pattern: Some("MSS,NOP,WS,NOP,NOP,SACK"),
            mss_values: &[1460, 1380, 1400],
        },
        OsSignature {
            family: "Windows",
            version: "Windows 7/8/Server 2008-2012",
            ttl_range: (128, 128),
            window_sizes: &[8192, 16384, 65535],
            options_pattern: Some("MSS,NOP,WS,SACK,TS"),
            mss_values: &[1460],
        },
        OsSignature {
            family: "Windows",
            version: "Windows XP/Server 2003",
            ttl_range: (128, 128),
            window_sizes: &[65535, 16384],
            options_pattern: Some("MSS,NOP,NOP,SACK"),
            mss_values: &[1460],
        },
        // Linux signatures
        OsSignature {
            family: "Linux",
            version: "Linux 5.x/6.x (Modern)",
            ttl_range: (64, 64),
            window_sizes: &[65535, 64240, 29200],
            options_pattern: Some("MSS,SACK,TS,NOP,WS"),
            mss_values: &[1460, 1380, 1400],
        },
        OsSignature {
            family: "Linux",
            version: "Linux 4.x",
            ttl_range: (64, 64),
            window_sizes: &[29200, 28960, 65535],
            options_pattern: Some("MSS,SACK,TS,NOP,WS"),
            mss_values: &[1460],
        },
        OsSignature {
            family: "Linux",
            version: "Linux 3.x",
            ttl_range: (64, 64),
            window_sizes: &[14600, 29200, 5840],
            options_pattern: Some("MSS,SACK,TS,NOP,WS"),
            mss_values: &[1460],
        },
        OsSignature {
            family: "Linux",
            version: "Linux 2.6",
            ttl_range: (64, 64),
            window_sizes: &[5840, 5720, 16384],
            options_pattern: Some("MSS,SACK,TS,NOP,WS"),
            mss_values: &[1460],
        },
        // macOS/iOS signatures
        OsSignature {
            family: "macOS",
            version: "macOS 10.x/11.x/12.x+",
            ttl_range: (64, 64),
            window_sizes: &[65535, 131072, 262144],
            options_pattern: Some("MSS,NOP,WS,NOP,NOP,TS,SACK,EOL"),
            mss_values: &[1460, 1380],
        },
        OsSignature {
            family: "iOS",
            version: "iOS/iPadOS",
            ttl_range: (64, 64),
            window_sizes: &[65535, 131072],
            options_pattern: Some("MSS,NOP,WS,NOP,NOP,TS,SACK"),
            mss_values: &[1460],
        },
        // BSD signatures
        OsSignature {
            family: "FreeBSD",
            version: "FreeBSD 12.x/13.x/14.x",
            ttl_range: (64, 64),
            window_sizes: &[65535, 65535],
            options_pattern: Some("MSS,NOP,WS,SACK,TS"),
            mss_values: &[1460],
        },
        OsSignature {
            family: "OpenBSD",
            version: "OpenBSD 6.x/7.x",
            ttl_range: (64, 64),
            window_sizes: &[16384, 65535],
            options_pattern: Some("MSS,NOP,NOP,SACK,NOP,WS"),
            mss_values: &[1460],
        },
        // Network devices
        OsSignature {
            family: "Cisco",
            version: "Cisco IOS",
            ttl_range: (255, 255),
            window_sizes: &[4128, 16384],
            options_pattern: Some("MSS"),
            mss_values: &[536, 1460],
        },
        OsSignature {
            family: "Juniper",
            version: "Juniper JunOS",
            ttl_range: (64, 64),
            window_sizes: &[65535],
            options_pattern: Some("MSS,NOP,WS,NOP,NOP,TS"),
            mss_values: &[1460],
        },
        // Embedded/IoT
        OsSignature {
            family: "Embedded",
            version: "Embedded Linux/BusyBox",
            ttl_range: (64, 64),
            window_sizes: &[5840, 14600],
            options_pattern: Some("MSS,SACK,TS,NOP,WS"),
            mss_values: &[1460, 536],
        },
        // Solaris
        OsSignature {
            family: "Solaris",
            version: "Solaris 10/11",
            ttl_range: (255, 255),
            window_sizes: &[49232, 32850],
            options_pattern: Some("MSS,NOP,WS,TS,SACK"),
            mss_values: &[1460],
        },
        // AIX
        OsSignature {
            family: "AIX",
            version: "IBM AIX",
            ttl_range: (60, 60),
            window_sizes: &[16384, 65535],
            options_pattern: Some("MSS,NOP,NOP,TS,NOP,WS,SACK"),
            mss_values: &[1460],
        },
    ]
});

/// TTL to OS family mapping for quick lookups
pub static TTL_OS_MAP: LazyLock<HashMap<u8, Vec<&'static str>>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // Common initial TTL values by OS
    m.insert(
        64,
        vec!["Linux", "macOS", "iOS", "FreeBSD", "OpenBSD", "Android"],
    );
    m.insert(128, vec!["Windows"]);
    m.insert(255, vec!["Cisco IOS", "Solaris", "Network Device"]);
    m.insert(60, vec!["AIX"]);
    m.insert(30, vec!["Embedded Device"]);

    // Nearby values (TTL decremented by hops)
    for i in 57..=63 {
        m.insert(i, vec!["Linux", "macOS", "BSD"]);
    }
    for i in 121..=127 {
        m.insert(i, vec!["Windows"]);
    }
    for i in 248..=254 {
        m.insert(i, vec!["Cisco IOS", "Solaris"]);
    }

    m
});

/// Fingerprint an OS based on observed TTL value
pub fn fingerprint_by_ttl(observed_ttl: u8) -> OsFingerprint {
    // Calculate likely initial TTL
    let initial_ttl = estimate_initial_ttl(observed_ttl);
    let hop_count = initial_ttl.saturating_sub(observed_ttl);

    let (os_family, os_guess, confidence) = match initial_ttl {
        64 => ("Linux/Unix", "Linux 2.6+/macOS/BSD/Android", 0.6),
        128 => ("Windows", "Windows 7/8/10/11/Server", 0.7),
        255 => ("Network Device", "Cisco IOS/Solaris/Router", 0.5),
        60 => ("AIX", "IBM AIX", 0.5),
        _ => ("Unknown", "Could not determine", 0.1),
    };

    let mut notes = vec![
        format!("Observed TTL: {}", observed_ttl),
        format!("Estimated initial TTL: {}", initial_ttl),
        format!("Estimated hop count: {}", hop_count),
    ];

    if hop_count > 30 {
        notes.push("Warning: High hop count, TTL-based detection may be unreliable".to_string());
    }

    OsFingerprint {
        os_family: os_family.to_string(),
        os_guess: os_guess.to_string(),
        confidence,
        ttl: Some(observed_ttl),
        window_size: None,
        tcp_options: None,
        notes,
    }
}

/// Estimate the initial TTL based on observed value
fn estimate_initial_ttl(observed_ttl: u8) -> u8 {
    // Common initial TTL values: 32, 60, 64, 128, 255
    match observed_ttl {
        1..=32 => 32,
        33..=60 => 60,
        61..=64 => 64,
        65..=128 => 128,
        129..=255 => 255,
        0 => 0, // Invalid/expired
    }
}

/// Fingerprint an OS based on TCP window size
/// Note: window_size is u32 to support scaled window values (window * 2^scale)
pub fn fingerprint_by_window_size(window_size: u32) -> OsFingerprint {
    let (os_family, os_guess, confidence) = match window_size {
        // Linux patterns
        5840 | 5720 => ("Linux", "Linux 2.6.x", 0.5),
        14600 => ("Linux", "Linux 3.x", 0.5),
        29200 | 28960 => ("Linux", "Linux 4.x/5.x", 0.5),
        64240 => ("Linux/Windows", "Linux 5.x+ or Windows 10+", 0.4),

        // Windows patterns
        8192 | 16384 => ("Windows", "Windows 7/Server 2008", 0.5),
        65535 => ("Multiple", "Windows/Linux/macOS (common)", 0.3),

        // macOS patterns (uses large scaled windows)
        131072 | 262144 => ("macOS", "macOS (large window, scaled)", 0.6),

        // BSD patterns
        32768 => ("BSD", "FreeBSD/OpenBSD", 0.4),

        // Cisco
        4128 => ("Cisco", "Cisco IOS", 0.6),

        // Solaris
        49232 | 32850 => ("Solaris", "Solaris 10/11", 0.5),

        _ => ("Unknown", "Window size not in database", 0.1),
    };

    OsFingerprint {
        os_family: os_family.to_string(),
        os_guess: os_guess.to_string(),
        confidence,
        ttl: None,
        window_size: Some(window_size),
        tcp_options: None,
        notes: vec![format!("Window size: {}", window_size)],
    }
}

/// Main fingerprinting function combining multiple indicators
pub fn fingerprint_os(ttl: u8, window_size: u16, tcp_options: &[u8]) -> Option<OsFingerprint> {
    let mut candidates: Vec<(String, String, f32)> = Vec::new();
    let mut notes = Vec::new();

    // Convert window_size to u32 for comparison with signature database
    let window_size_u32 = window_size as u32;

    // Parse TCP options
    let options_info = parse_tcp_options(tcp_options);
    let options_signature = options_info.as_ref().map(|o| o.options_order.clone());

    // Estimate initial TTL
    let initial_ttl = estimate_initial_ttl(ttl);
    notes.push(format!(
        "Observed TTL: {}, Initial TTL: {}",
        ttl, initial_ttl
    ));

    // Score each signature
    for sig in OS_SIGNATURES.iter() {
        let mut score: f32 = 0.0;
        let mut match_count = 0;

        // TTL match (weighted 0.3)
        if initial_ttl >= sig.ttl_range.0 && initial_ttl <= sig.ttl_range.1 {
            score += 0.3;
            match_count += 1;
        }

        // Window size match (weighted 0.3)
        if sig.window_sizes.contains(&window_size_u32) {
            score += 0.3;
            match_count += 1;
        } else {
            // Check for similar window sizes (within 5%)
            for &expected_ws in sig.window_sizes {
                let diff = (window_size_u32 as i64 - expected_ws as i64).abs();
                if diff < (expected_ws as i64 / 20) {
                    score += 0.15; // Partial match
                    break;
                }
            }
        }

        // TCP options pattern match (weighted 0.25)
        if let (Some(ref sig_pattern), Some(observed_pattern)) =
            (sig.options_pattern, &options_signature)
        {
            if sig_pattern == observed_pattern {
                score += 0.25;
                match_count += 1;
            } else if observed_pattern.contains(sig_pattern)
                || sig_pattern.contains(observed_pattern.as_str())
            {
                score += 0.1; // Partial match
            }
        }

        // MSS match (weighted 0.15)
        if let Some(ref opts) = options_info
            && let Some(mss) = opts.mss
            && sig.mss_values.contains(&mss)
        {
            score += 0.15;
            match_count += 1;
        }

        // Only consider if we have at least 2 matching indicators
        if match_count >= 2 && score > 0.3 {
            candidates.push((sig.family.to_string(), sig.version.to_string(), score));
        }
    }

    // Sort by score descending
    candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // Take the best match
    if let Some((family, version, confidence)) = candidates.first() {
        notes.push(format!("Window size: {}", window_size));
        if let Some(ref opts) = options_info {
            notes.push(format!("TCP options: {}", opts.options_order));
            if let Some(mss) = opts.mss {
                notes.push(format!("MSS: {}", mss));
            }
        }

        Some(OsFingerprint {
            os_family: family.clone(),
            os_guess: version.clone(),
            confidence: *confidence,
            ttl: Some(ttl),
            window_size: Some(window_size_u32),
            tcp_options: options_info,
            notes,
        })
    } else {
        // Fallback to TTL-only detection
        let ttl_fp = fingerprint_by_ttl(ttl);
        Some(OsFingerprint {
            window_size: Some(window_size_u32),
            tcp_options: options_info,
            ..ttl_fp
        })
    }
}

/// Parse TCP options from raw bytes
pub fn parse_tcp_options(options: &[u8]) -> Option<TcpOptionsInfo> {
    if options.is_empty() {
        return None;
    }

    let mut mss: Option<u16> = None;
    let mut window_scale: Option<u8> = None;
    let mut sack_permitted = false;
    let mut timestamps = false;
    let mut order = Vec::new();

    let mut i = 0;
    while i < options.len() {
        match options[i] {
            0 => {
                // End of Option List (EOL)
                order.push("EOL");
                break;
            }
            1 => {
                // No-Operation (NOP)
                order.push("NOP");
                i += 1;
            }
            2 => {
                // Maximum Segment Size (MSS)
                if i + 3 < options.len() && options[i + 1] == 4 {
                    mss = Some(u16::from_be_bytes([options[i + 2], options[i + 3]]));
                    order.push("MSS");
                    i += 4;
                } else {
                    break;
                }
            }
            3 => {
                // Window Scale
                if i + 2 < options.len() && options[i + 1] == 3 {
                    window_scale = Some(options[i + 2]);
                    order.push("WS");
                    i += 3;
                } else {
                    break;
                }
            }
            4 => {
                // SACK Permitted
                if i + 1 < options.len() && options[i + 1] == 2 {
                    sack_permitted = true;
                    order.push("SACK");
                    i += 2;
                } else {
                    break;
                }
            }
            5 => {
                // SACK (actual SACK data, variable length)
                if i + 1 < options.len() {
                    let len = options[i + 1] as usize;
                    order.push("SACK-DATA");
                    i += len;
                } else {
                    break;
                }
            }
            8 => {
                // Timestamps
                if i + 1 < options.len() && options[i + 1] == 10 {
                    timestamps = true;
                    order.push("TS");
                    i += 10;
                } else {
                    break;
                }
            }
            _ => {
                // Unknown option - try to skip it
                if i + 1 < options.len() && options[i + 1] > 0 {
                    i += options[i + 1] as usize;
                } else {
                    break;
                }
            }
        }
    }

    Some(TcpOptionsInfo {
        mss,
        window_scale,
        sack_permitted,
        timestamps,
        options_order: order.join(","),
    })
}

/// Quick OS guess based on just TTL (for simple scans)
pub fn quick_os_guess(ttl: u8) -> (&'static str, &'static str, f32) {
    match estimate_initial_ttl(ttl) {
        64 => ("Linux/Unix", "Linux 2.6+/macOS/BSD", 0.6),
        128 => ("Windows", "Windows 7/8/10/11/Server", 0.7),
        255 => ("Network Device", "Cisco IOS/Solaris/Router", 0.5),
        60 => ("AIX", "IBM AIX", 0.5),
        32 => ("Embedded", "Embedded/Legacy", 0.4),
        _ => ("Unknown", "Could not determine", 0.1),
    }
}

/// Get all possible OS families for a given TTL
pub fn get_os_families_for_ttl(ttl: u8) -> Vec<&'static str> {
    TTL_OS_MAP
        .get(&ttl)
        .cloned()
        .unwrap_or_else(|| vec!["Unknown"])
}

/// Calculate confidence boost when multiple indicators agree
pub fn calculate_combined_confidence(indicators: &[f32]) -> f32 {
    if indicators.is_empty() {
        return 0.0;
    }

    // Combine using product of confidences with boost for agreement
    let base: f32 = indicators.iter().product();
    let agreement_boost = if indicators.iter().all(|&c| c > 0.5) {
        0.1 * indicators.len() as f32
    } else {
        0.0
    };

    (base + agreement_boost).min(0.95)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_initial_ttl() {
        assert_eq!(estimate_initial_ttl(64), 64);
        assert_eq!(estimate_initial_ttl(63), 64);
        assert_eq!(estimate_initial_ttl(128), 128);
        assert_eq!(estimate_initial_ttl(127), 128);
        assert_eq!(estimate_initial_ttl(255), 255);
        assert_eq!(estimate_initial_ttl(250), 255);
    }

    #[test]
    fn test_fingerprint_by_ttl() {
        let fp = fingerprint_by_ttl(64);
        assert!(fp.os_family.contains("Linux"));
        assert!(fp.confidence > 0.5);

        let fp = fingerprint_by_ttl(128);
        assert!(fp.os_family.contains("Windows"));
        assert!(fp.confidence > 0.5);

        let fp = fingerprint_by_ttl(255);
        assert!(fp.os_family.contains("Network"));
    }

    #[test]
    fn test_fingerprint_by_window_size() {
        let fp = fingerprint_by_window_size(29200);
        assert!(fp.os_guess.contains("Linux"));

        let fp = fingerprint_by_window_size(8192);
        assert!(fp.os_family.contains("Windows"));
    }

    #[test]
    fn test_parse_tcp_options() {
        // MSS=1460 (kind=2, len=4, value=0x05B4)
        let options = [2, 4, 0x05, 0xB4];
        let info = parse_tcp_options(&options).unwrap();
        assert_eq!(info.mss, Some(1460));
        assert!(info.options_order.contains("MSS"));
    }

    #[test]
    fn test_quick_os_guess() {
        let (family, _, confidence) = quick_os_guess(64);
        assert!(family.contains("Linux"));
        assert!(confidence > 0.5);

        let (family, _, confidence) = quick_os_guess(128);
        assert!(family.contains("Windows"));
        assert!(confidence > 0.5);
    }

    #[test]
    fn test_combined_fingerprint() {
        // Test with Linux-like characteristics
        let options = [2, 4, 0x05, 0xB4, 4, 2]; // MSS=1460, SACK permitted
        let fp = fingerprint_os(64, 29200, &options);
        assert!(fp.is_some());
        let fp = fp.unwrap();
        assert!(fp.os_family.contains("Linux"));
    }

    #[test]
    fn test_os_signatures_database() {
        // Ensure we have a reasonable number of signatures
        assert!(OS_SIGNATURES.len() >= 10);

        // Check that signatures have valid data
        for sig in OS_SIGNATURES.iter() {
            assert!(!sig.family.is_empty());
            assert!(!sig.version.is_empty());
            assert!(sig.ttl_range.0 <= sig.ttl_range.1);
            assert!(!sig.window_sizes.is_empty());
        }
    }

    #[test]
    fn test_ttl_os_map() {
        // Check common TTL values are mapped
        assert!(TTL_OS_MAP.contains_key(&64));
        assert!(TTL_OS_MAP.contains_key(&128));
        assert!(TTL_OS_MAP.contains_key(&255));

        // Linux should be mapped to TTL 64
        let linux_ttls = TTL_OS_MAP.get(&64).unwrap();
        assert!(linux_ttls.contains(&"Linux"));

        // Windows should be mapped to TTL 128
        let windows_ttls = TTL_OS_MAP.get(&128).unwrap();
        assert!(windows_ttls.contains(&"Windows"));
    }
}
