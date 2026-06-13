use anyhow::{Context, Result};
use sha2::{Digest, Sha256, Sha384, Sha512};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const BUF_SIZE: usize = 8192;

/// Detected file format based on magic bytes and extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// Windows PE executable (EXE, DLL, SYS)
    Pe,
    /// RPM package
    Rpm,
    /// Debian package
    Deb,
    /// Arch Linux package (.pkg.tar.zst, .db.tar.gz, etc.)
    Pacman,
    /// Generic file (detached signature)
    Generic,
}

impl FileFormat {
    /// Auto-detect file format from magic bytes and extension
    pub fn detect(path: &Path) -> Result<Self> {
        let mut file =
            File::open(path).with_context(|| format!("Cannot open file: {}", path.display()))?;

        let mut magic = [0u8; 4];
        let bytes_read = file.read(&mut magic).unwrap_or(0);

        if bytes_read >= 2 && magic[0] == b'M' && magic[1] == b'Z' {
            return Ok(FileFormat::Pe);
        }

        if bytes_read >= 4 && magic == [0xED, 0xAB, 0xEE, 0xDB] {
            return Ok(FileFormat::Rpm);
        }

        // DEB packages start with "!<arch>\n" (ar archive format)
        if bytes_read >= 4 && &magic[..4] == b"!<ar" {
            return Ok(FileFormat::Deb);
        }

        // Fall back to extension-based detection
        let path_str = path.to_string_lossy();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "exe" | "dll" | "sys" | "msi" | "msix" => return Ok(FileFormat::Pe),
                "rpm" => return Ok(FileFormat::Rpm),
                "deb" => return Ok(FileFormat::Deb),
                _ => {}
            }
        }

        // Arch Linux packages use compound extensions
        if path_str.ends_with(".pkg.tar.zst")
            || path_str.ends_with(".pkg.tar.xz")
            || path_str.ends_with(".pkg.tar.gz")
            || path_str.ends_with(".db.tar.gz")
            || path_str.ends_with(".db.tar.zst")
            || path_str.ends_with(".files.tar.gz")
            || path_str.ends_with(".files.tar.zst")
        {
            return Ok(FileFormat::Pacman);
        }

        Ok(FileFormat::Generic)
    }

    /// Human-readable format name
    pub fn name(&self) -> &'static str {
        match self {
            FileFormat::Pe => "PE (Windows Executable)",
            FileFormat::Rpm => "RPM Package",
            FileFormat::Deb => "DEB Package",
            FileFormat::Pacman => "Arch Linux Package",
            FileFormat::Generic => "Generic (Detached Signature)",
        }
    }
}

/// Supported digest algorithms
#[derive(Debug, Clone, Copy)]
pub enum DigestAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

impl DigestAlgorithm {
    /// Determine digest algorithm from signing algorithm string
    pub fn from_sign_algorithm(alg: &str) -> Self {
        match alg {
            "RS384" | "ES384" | "PS384" => DigestAlgorithm::Sha384,
            "RS512" | "ES512" | "PS512" => DigestAlgorithm::Sha512,
            _ => DigestAlgorithm::Sha256, // RS256, ES256, PS256 and default
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DigestAlgorithm::Sha256 => "SHA-256",
            DigestAlgorithm::Sha384 => "SHA-384",
            DigestAlgorithm::Sha512 => "SHA-512",
        }
    }
}

/// Compute a streaming digest of a file
pub fn file_digest(path: &Path, algorithm: DigestAlgorithm) -> Result<Vec<u8>> {
    let mut file = File::open(path)
        .with_context(|| format!("Cannot open file for hashing: {}", path.display()))?;

    let mut buf = [0u8; BUF_SIZE];

    match algorithm {
        DigestAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let n = file.read(&mut buf).context("Read error during hashing")?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            Ok(hasher.finalize().to_vec())
        }
        DigestAlgorithm::Sha384 => {
            let mut hasher = Sha384::new();
            loop {
                let n = file.read(&mut buf).context("Read error during hashing")?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            Ok(hasher.finalize().to_vec())
        }
        DigestAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            loop {
                let n = file.read(&mut buf).context("Read error during hashing")?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            Ok(hasher.finalize().to_vec())
        }
    }
}

/// Compute a digest over in-memory bytes
pub fn digest_bytes(data: &[u8], algorithm: DigestAlgorithm) -> Vec<u8> {
    match algorithm {
        DigestAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        DigestAlgorithm::Sha384 => {
            let mut hasher = Sha384::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        DigestAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
    }
}

/// PE Authenticode byte ranges to hash
pub struct AuthenticodeOffsets {
    /// PE header checksum field offset (4 bytes to skip)
    pub checksum_offset: usize,
    /// Data directory Certificate Table entry offset (8 bytes to skip)
    pub cert_table_offset: usize,
    /// Existing cert table file offset (0 if none)
    pub cert_table_rva: u32,
    /// Existing cert table size (0 if none)
    pub cert_table_size: u32,
    /// End of optional header
    pub header_end: usize,
}

/// Compute Authenticode digest of a PE file.
///
/// Hashes all bytes in order, skipping:
/// - Checksum field (4 bytes at checksum_offset)
/// - Certificate Table directory entry (8 bytes at cert_table_offset)
/// - Any existing certificate data (from cert_table_rva to end of cert data)
pub fn authenticode_digest(
    pe_bytes: &[u8],
    offsets: &AuthenticodeOffsets,
    algorithm: DigestAlgorithm,
) -> Result<Vec<u8>> {
    // Build the list of (start, end) ranges to hash, in order
    let file_len = pe_bytes.len();

    // The Authenticode spec says we hash everything except:
    // 1. The checksum field (4 bytes)
    // 2. The cert table directory entry (8 bytes)
    // 3. The attribute certificate table (the cert data at the end)
    //
    // We do this by collecting byte ranges in order, skipping the excluded regions

    let cert_data_start = if offsets.cert_table_rva > 0 {
        offsets.cert_table_rva as usize
    } else {
        file_len // no cert data to skip
    };

    // Excluded regions sorted by offset
    let mut excludes: Vec<(usize, usize)> = vec![
        (offsets.checksum_offset, offsets.checksum_offset + 4),
        (offsets.cert_table_offset, offsets.cert_table_offset + 8),
    ];
    if cert_data_start < file_len {
        excludes.push((cert_data_start, file_len));
    }
    excludes.sort_by_key(|r| r.0);

    // Build hash ranges (gaps between excluded regions)
    let mut ranges = Vec::new();
    let mut pos = 0;
    for (excl_start, excl_end) in &excludes {
        if pos < *excl_start {
            ranges.push((pos, *excl_start));
        }
        pos = *excl_end;
    }
    if pos < file_len && cert_data_start >= file_len {
        ranges.push((pos, file_len));
    } else if pos < cert_data_start {
        ranges.push((pos, cert_data_start));
    }

    // Hash the ranges in order
    match algorithm {
        DigestAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            for (start, end) in &ranges {
                hasher.update(&pe_bytes[*start..*end]);
            }
            Ok(hasher.finalize().to_vec())
        }
        DigestAlgorithm::Sha384 => {
            let mut hasher = Sha384::new();
            for (start, end) in &ranges {
                hasher.update(&pe_bytes[*start..*end]);
            }
            Ok(hasher.finalize().to_vec())
        }
        DigestAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            for (start, end) in &ranges {
                hasher.update(&pe_bytes[*start..*end]);
            }
            Ok(hasher.finalize().to_vec())
        }
    }
}

/// Format a digest as a hex string
pub fn hex_digest(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_sha256_digest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"hello world").unwrap();

        let digest = file_digest(&path, DigestAlgorithm::Sha256).unwrap();
        let hex = hex_digest(&digest);
        // SHA-256 of "hello world"
        assert_eq!(
            hex,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha384_digest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"hello world").unwrap();

        let digest = file_digest(&path, DigestAlgorithm::Sha384).unwrap();
        assert_eq!(digest.len(), 48); // SHA-384 produces 48 bytes
    }

    #[test]
    fn test_sha512_digest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"hello world").unwrap();

        let digest = file_digest(&path, DigestAlgorithm::Sha512).unwrap();
        assert_eq!(digest.len(), 64); // SHA-512 produces 64 bytes
    }

    #[test]
    fn test_format_detection_generic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"just some text").unwrap();

        assert_eq!(FileFormat::detect(&path).unwrap(), FileFormat::Generic);
    }

    #[test]
    fn test_format_detection_pe_magic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.bin");
        let mut f = File::create(&path).unwrap();
        f.write_all(b"MZ\x90\x00").unwrap(); // MZ magic bytes
        drop(f);

        assert_eq!(FileFormat::detect(&path).unwrap(), FileFormat::Pe);
    }

    #[test]
    fn test_format_detection_rpm_magic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.bin");
        let mut f = File::create(&path).unwrap();
        f.write_all(&[0xED, 0xAB, 0xEE, 0xDB]).unwrap();
        drop(f);

        assert_eq!(FileFormat::detect(&path).unwrap(), FileFormat::Rpm);
    }

    #[test]
    fn test_format_detection_by_extension() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("app.exe");
        std::fs::write(&path, b"\x00\x00\x00\x00").unwrap(); // no MZ magic but .exe extension

        assert_eq!(FileFormat::detect(&path).unwrap(), FileFormat::Pe);
    }

    #[test]
    fn test_digest_algorithm_mapping() {
        assert!(matches!(
            DigestAlgorithm::from_sign_algorithm("RS256"),
            DigestAlgorithm::Sha256
        ));
        assert!(matches!(
            DigestAlgorithm::from_sign_algorithm("RS384"),
            DigestAlgorithm::Sha384
        ));
        assert!(matches!(
            DigestAlgorithm::from_sign_algorithm("ES512"),
            DigestAlgorithm::Sha512
        ));
    }

    #[test]
    fn test_digest_bytes() {
        let digest = digest_bytes(b"hello world", DigestAlgorithm::Sha256);
        let hex = hex_digest(&digest);
        assert_eq!(
            hex,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_authenticode_digest_skips_regions() {
        // Create a simple byte array and verify the excluded regions are skipped
        let data = vec![0xAA; 100];
        let offsets = AuthenticodeOffsets {
            checksum_offset: 10,
            cert_table_offset: 30,
            cert_table_rva: 0,
            cert_table_size: 0,
            header_end: 50,
        };
        let digest1 = authenticode_digest(&data, &offsets, DigestAlgorithm::Sha256).unwrap();

        // Modify bytes in excluded regions -- digest should be the same
        let mut data2 = data.clone();
        data2[10] = 0xBB; // checksum field
        data2[30] = 0xCC; // cert table entry
        let digest2 = authenticode_digest(&data2, &offsets, DigestAlgorithm::Sha256).unwrap();
        assert_eq!(digest1, digest2);

        // Modify bytes outside excluded regions -- digest should differ
        let mut data3 = data;
        data3[0] = 0xDD;
        let digest3 = authenticode_digest(&data3, &offsets, DigestAlgorithm::Sha256).unwrap();
        assert_ne!(digest1, digest3);
    }
}
