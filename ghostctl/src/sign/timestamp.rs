// RFC 3161 Timestamping client
//
// Builds a TimeStampReq, POSTs to a TSA endpoint, and parses the response.
// The returned TimeStampToken (DER) can be embedded as an unauthenticated
// attribute in a CMS SignerInfo for Authenticode or generic signatures.

use anyhow::{Context, Result, bail};
use std::time::Duration;

use super::asn1;
use super::hash::{DigestAlgorithm, digest_bytes};

/// Request a timestamp from an RFC 3161 TSA for the given signature bytes.
/// Returns the raw DER-encoded TimeStampToken on success.
pub fn timestamp_signature(
    signature: &[u8],
    digest_algorithm: DigestAlgorithm,
    tsa_url: &str,
) -> Result<Vec<u8>> {
    // Hash the signature to create the message imprint
    let imprint_digest = digest_bytes(signature, digest_algorithm);

    // Build the TimeStampReq DER
    let ts_req = build_timestamp_request(&imprint_digest, digest_algorithm);

    // POST to TSA
    let http = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("ghostctl")
        .build()
        .context("Failed to create HTTP client for timestamping")?;

    let response = http
        .post(tsa_url)
        .header("Content-Type", "application/timestamp-query")
        .body(ts_req)
        .send()
        .with_context(|| format!("Timestamp request failed to {}", tsa_url))?;

    if !response.status().is_success() {
        bail!(
            "Timestamp server returned HTTP {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let response_bytes = response
        .bytes()
        .context("Failed to read timestamp response body")?
        .to_vec();

    // Parse TimeStampResp to extract the TimeStampToken
    extract_timestamp_token(&response_bytes)
}

/// Build an RFC 3161 TimeStampReq DER structure
///
/// TimeStampReq ::= SEQUENCE {
///   version        INTEGER { v1(1) },
///   messageImprint MessageImprint,
///   certReq        BOOLEAN DEFAULT FALSE
/// }
///
/// MessageImprint ::= SEQUENCE {
///   hashAlgorithm  AlgorithmIdentifier,
///   hashedMessage  OCTET STRING
/// }
fn build_timestamp_request(digest: &[u8], algorithm: DigestAlgorithm) -> Vec<u8> {
    let version = asn1::integer_u64(1);

    let digest_oid = asn1::digest_oid(algorithm.name());
    let alg_id = asn1::algorithm_identifier(digest_oid);
    let hashed_message = asn1::octet_string(digest);
    let message_imprint = asn1::sequence(&[&alg_id, &hashed_message]);

    let cert_req = asn1::boolean(true);

    asn1::sequence(&[&version, &message_imprint, &cert_req])
}

/// Extract the TimeStampToken from a TimeStampResp
///
/// TimeStampResp ::= SEQUENCE {
///   status         PKIStatusInfo,
///   timeStampToken ContentInfo OPTIONAL
/// }
///
/// PKIStatusInfo ::= SEQUENCE {
///   status    INTEGER,
///   ...
/// }
fn extract_timestamp_token(response: &[u8]) -> Result<Vec<u8>> {
    // Parse outer SEQUENCE
    if response.is_empty() || response[0] != 0x30 {
        bail!("Invalid TimeStampResp: not a SEQUENCE");
    }

    let (header_len, content_len) = read_length(&response[1..])?;
    let content_start = 1 + header_len;

    if response.len() < content_start + content_len {
        bail!("TimeStampResp truncated");
    }

    let content = &response[content_start..content_start + content_len];

    // First element: PKIStatusInfo SEQUENCE
    if content.is_empty() || content[0] != 0x30 {
        bail!("Invalid PKIStatusInfo in TimeStampResp");
    }

    let (status_header_len, status_content_len) = read_length(&content[1..])?;
    let status_total = 1 + status_header_len + status_content_len;

    // Check status value (first INTEGER in PKIStatusInfo)
    let status_content_start = 1 + status_header_len;
    let status_inner = &content[status_content_start..status_total];
    if !status_inner.is_empty() && status_inner[0] == 0x02 {
        let (_, int_len) = read_length(&status_inner[1..])?;
        let int_start = 1 + read_length(&status_inner[1..])?.0;
        if int_len > 0 {
            let status_value = status_inner[int_start];
            if status_value != 0 {
                // 0 = granted, 1 = grantedWithMods, 2+ = rejection/waiting/etc
                if status_value >= 2 {
                    bail!(
                        "Timestamp server rejected request (status={})",
                        status_value
                    );
                }
            }
        }
    }

    // Second element: TimeStampToken (ContentInfo SEQUENCE)
    if status_total >= content.len() {
        bail!("TimeStampResp does not contain a TimeStampToken");
    }

    let token = &content[status_total..];
    if token.is_empty() || token[0] != 0x30 {
        bail!("TimeStampToken is not a SEQUENCE");
    }

    // Verify we can read the full token
    let (token_header_len, token_content_len) = read_length(&token[1..])?;
    let token_total = 1 + token_header_len + token_content_len;

    if token.len() < token_total {
        bail!("TimeStampToken truncated");
    }

    Ok(token[..token_total].to_vec())
}

/// Read a DER length field. Returns (bytes_consumed, length_value)
fn read_length(data: &[u8]) -> Result<(usize, usize)> {
    if data.is_empty() {
        bail!("Unexpected end of DER data reading length");
    }
    if data[0] < 0x80 {
        Ok((1, data[0] as usize))
    } else {
        let num_bytes = (data[0] & 0x7F) as usize;
        if num_bytes == 0 || num_bytes > 4 {
            bail!("Invalid DER length encoding");
        }
        if data.len() < 1 + num_bytes {
            bail!("DER length field truncated");
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | (data[1 + i] as usize);
        }
        Ok((1 + num_bytes, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_timestamp_request() {
        let digest = vec![0xAB; 32]; // fake SHA-256 digest
        let req = build_timestamp_request(&digest, DigestAlgorithm::Sha256);

        // Should be a valid SEQUENCE
        assert_eq!(req[0], 0x30);
        assert!(!req.is_empty());
    }

    #[test]
    fn test_read_length_short() {
        assert_eq!(read_length(&[0x05]).unwrap(), (1, 5));
        assert_eq!(read_length(&[0x7F]).unwrap(), (1, 127));
    }

    #[test]
    fn test_read_length_long() {
        assert_eq!(read_length(&[0x81, 0x80]).unwrap(), (2, 128));
        assert_eq!(read_length(&[0x82, 0x01, 0x00]).unwrap(), (3, 256));
    }

    #[test]
    fn test_extract_token_rejects_bad_input() {
        // Empty input
        assert!(extract_timestamp_token(&[]).is_err());
        // Not a SEQUENCE
        assert!(extract_timestamp_token(&[0x02, 0x01, 0x00]).is_err());
    }

    #[test]
    fn test_build_timestamp_request_sha384() {
        let digest = vec![0xCC; 48];
        let req = build_timestamp_request(&digest, DigestAlgorithm::Sha384);
        assert_eq!(req[0], 0x30);
    }
}
