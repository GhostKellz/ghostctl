// Minimal DER/ASN.1 encoder for CMS SignedData (Authenticode + RFC 3161)
//
// This is intentionally minimal -- only the subset needed for building
// PKCS#7/CMS envelopes for PE Authenticode signing and RFC 3161 timestamps.
// No full ASN.1 parser or schema validation.

/// DER tag constants
const TAG_BOOLEAN: u8 = 0x01;
const TAG_INTEGER: u8 = 0x02;
const TAG_BIT_STRING: u8 = 0x03;
const TAG_OCTET_STRING: u8 = 0x04;
const TAG_NULL: u8 = 0x05;
const TAG_OID: u8 = 0x06;
const TAG_UTF8_STRING: u8 = 0x0C;
const TAG_SEQUENCE: u8 = 0x30;
const TAG_SET: u8 = 0x31;
const TAG_UTC_TIME: u8 = 0x17;

// PKCS#7 / CMS
pub const OID_SIGNED_DATA: &[u32] = &[1, 2, 840, 113549, 1, 7, 2];
pub const OID_DATA: &[u32] = &[1, 2, 840, 113549, 1, 7, 1];

// Authenticode (SPC)
pub const OID_SPC_INDIRECT_DATA: &[u32] = &[1, 3, 6, 1, 4, 1, 311, 2, 1, 4];
pub const OID_SPC_PE_IMAGE_DATA: &[u32] = &[1, 3, 6, 1, 4, 1, 311, 2, 1, 15];

// Hash algorithms
pub const OID_SHA256: &[u32] = &[2, 16, 840, 1, 101, 3, 4, 2, 1];
pub const OID_SHA384: &[u32] = &[2, 16, 840, 1, 101, 3, 4, 2, 2];
pub const OID_SHA512: &[u32] = &[2, 16, 840, 1, 101, 3, 4, 2, 3];

// RSA
pub const OID_RSA_ENCRYPTION: &[u32] = &[1, 2, 840, 113549, 1, 1, 1];

// PKCS#9 attributes
pub const OID_CONTENT_TYPE: &[u32] = &[1, 2, 840, 113549, 1, 9, 3];
pub const OID_MESSAGE_DIGEST: &[u32] = &[1, 2, 840, 113549, 1, 9, 4];
pub const OID_SIGNING_TIME: &[u32] = &[1, 2, 840, 113549, 1, 9, 5];

// Timestamping
pub const OID_TST_INFO: &[u32] = &[1, 2, 840, 113549, 1, 9, 16, 1, 4];
pub const OID_TIMESTAMP_TOKEN: &[u32] = &[1, 2, 840, 113549, 1, 9, 16, 2, 14];

/// Encode DER length bytes
fn encode_length(len: usize) -> Vec<u8> {
    if len < 0x80 {
        vec![len as u8]
    } else if len <= 0xFF {
        vec![0x81, len as u8]
    } else if len <= 0xFFFF {
        vec![0x82, (len >> 8) as u8, len as u8]
    } else if len <= 0xFF_FFFF {
        vec![0x83, (len >> 16) as u8, (len >> 8) as u8, len as u8]
    } else {
        vec![
            0x84,
            (len >> 24) as u8,
            (len >> 16) as u8,
            (len >> 8) as u8,
            len as u8,
        ]
    }
}

/// Wrap content in a TLV (tag-length-value) triplet
pub fn tlv(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    out.extend_from_slice(&encode_length(content.len()));
    out.extend_from_slice(content);
    out
}

/// SEQUENCE (0x30) wrapping concatenated items
pub fn sequence(items: &[&[u8]]) -> Vec<u8> {
    let mut content = Vec::new();
    for item in items {
        content.extend_from_slice(item);
    }
    tlv(TAG_SEQUENCE, &content)
}

/// SET (0x31) wrapping concatenated items
pub fn set(items: &[&[u8]]) -> Vec<u8> {
    let mut content = Vec::new();
    for item in items {
        content.extend_from_slice(item);
    }
    tlv(TAG_SET, &content)
}

/// Context-specific EXPLICIT tag [tag_num] (constructed)
pub fn context_tag(tag_num: u8, content: &[u8]) -> Vec<u8> {
    let tag = 0xA0 | (tag_num & 0x1F);
    tlv(tag, content)
}

/// Context-specific IMPLICIT tag [tag_num] (primitive)
pub fn context_tag_implicit(tag_num: u8, content: &[u8]) -> Vec<u8> {
    let tag = 0x80 | (tag_num & 0x1F);
    tlv(tag, content)
}

/// Encode an OID from an array of arc values
pub fn oid(arcs: &[u32]) -> Vec<u8> {
    assert!(arcs.len() >= 2, "OID must have at least 2 components");

    let mut content = Vec::new();
    // First two arcs are encoded as (arc0 * 40 + arc1)
    content.push((arcs[0] * 40 + arcs[1]) as u8);

    for &arc in &arcs[2..] {
        encode_oid_arc(&mut content, arc);
    }

    tlv(TAG_OID, &content)
}

/// Encode a single OID arc value in base-128
fn encode_oid_arc(buf: &mut Vec<u8>, mut value: u32) {
    if value < 128 {
        buf.push(value as u8);
        return;
    }

    let mut bytes = Vec::new();
    while value > 0 {
        bytes.push((value & 0x7F) as u8);
        value >>= 7;
    }
    bytes.reverse();

    for (i, b) in bytes.iter().enumerate() {
        if i < bytes.len() - 1 {
            buf.push(b | 0x80); // continuation bit
        } else {
            buf.push(*b); // last byte, no continuation
        }
    }
}

/// INTEGER -- handles proper sign-extension
pub fn integer(val: &[u8]) -> Vec<u8> {
    if val.is_empty() {
        return tlv(TAG_INTEGER, &[0]);
    }
    // If high bit is set, prepend 0x00 to keep positive
    if val[0] & 0x80 != 0 {
        let mut padded = vec![0x00];
        padded.extend_from_slice(val);
        tlv(TAG_INTEGER, &padded)
    } else {
        // Strip leading zeros but keep at least one byte
        let start = val.iter().position(|&b| b != 0).unwrap_or(val.len() - 1);
        let trimmed = &val[start..];
        // Re-check if high bit set after trimming
        if trimmed[0] & 0x80 != 0 {
            let mut padded = vec![0x00];
            padded.extend_from_slice(trimmed);
            tlv(TAG_INTEGER, &padded)
        } else {
            tlv(TAG_INTEGER, trimmed)
        }
    }
}

/// Small integer from u64
pub fn integer_u64(val: u64) -> Vec<u8> {
    if val == 0 {
        return tlv(TAG_INTEGER, &[0]);
    }
    let bytes = val.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(7);
    integer(&bytes[start..])
}

/// OCTET STRING
pub fn octet_string(data: &[u8]) -> Vec<u8> {
    tlv(TAG_OCTET_STRING, data)
}

/// BIT STRING (with 0 unused bits)
pub fn bit_string(data: &[u8]) -> Vec<u8> {
    let mut content = vec![0x00]; // 0 unused bits
    content.extend_from_slice(data);
    tlv(TAG_BIT_STRING, &content)
}

/// NULL
pub fn null() -> Vec<u8> {
    vec![TAG_NULL, 0x00]
}

/// BOOLEAN
pub fn boolean(val: bool) -> Vec<u8> {
    tlv(TAG_BOOLEAN, &[if val { 0xFF } else { 0x00 }])
}

/// UTCTime -- expects "YYMMDDHHMMSSZ" format
pub fn utc_time(timestamp: &str) -> Vec<u8> {
    tlv(TAG_UTC_TIME, timestamp.as_bytes())
}

/// UTF8String
pub fn utf8_string(s: &str) -> Vec<u8> {
    tlv(TAG_UTF8_STRING, s.as_bytes())
}

/// Raw bytes (already DER-encoded, pass through unchanged)
pub fn raw(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

/// AlgorithmIdentifier SEQUENCE { oid, NULL }
pub fn algorithm_identifier(oid_arcs: &[u32]) -> Vec<u8> {
    let oid_bytes = oid(oid_arcs);
    let null_bytes = null();
    sequence(&[&oid_bytes, &null_bytes])
}

/// Attribute SEQUENCE { oid, SET { value } }
pub fn attribute(oid_arcs: &[u32], value: &[u8]) -> Vec<u8> {
    let oid_bytes = oid(oid_arcs);
    let value_set = set(&[value]);
    sequence(&[&oid_bytes, &value_set])
}

/// Get the OID arcs for a digest algorithm name
pub fn digest_oid(alg: &str) -> &'static [u32] {
    match alg {
        "SHA-256" | "RS256" | "ES256" | "PS256" => OID_SHA256,
        "SHA-384" | "RS384" | "ES384" | "PS384" => OID_SHA384,
        "SHA-512" | "RS512" | "ES512" | "PS512" => OID_SHA512,
        _ => OID_SHA256,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_encoding_short() {
        assert_eq!(encode_length(0), vec![0x00]);
        assert_eq!(encode_length(127), vec![0x7F]);
    }

    #[test]
    fn test_length_encoding_long() {
        assert_eq!(encode_length(128), vec![0x81, 0x80]);
        assert_eq!(encode_length(255), vec![0x81, 0xFF]);
        assert_eq!(encode_length(256), vec![0x82, 0x01, 0x00]);
        assert_eq!(encode_length(65535), vec![0x82, 0xFF, 0xFF]);
    }

    #[test]
    fn test_null() {
        assert_eq!(null(), vec![0x05, 0x00]);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(boolean(true), vec![0x01, 0x01, 0xFF]);
        assert_eq!(boolean(false), vec![0x01, 0x01, 0x00]);
    }

    #[test]
    fn test_integer_zero() {
        assert_eq!(integer(&[0]), vec![0x02, 0x01, 0x00]);
    }

    #[test]
    fn test_integer_positive() {
        assert_eq!(integer(&[0x01]), vec![0x02, 0x01, 0x01]);
        assert_eq!(integer(&[0x7F]), vec![0x02, 0x01, 0x7F]);
    }

    #[test]
    fn test_integer_needs_padding() {
        // High bit set: needs 0x00 prefix to stay positive
        assert_eq!(integer(&[0x80]), vec![0x02, 0x02, 0x00, 0x80]);
        assert_eq!(integer(&[0xFF]), vec![0x02, 0x02, 0x00, 0xFF]);
    }

    #[test]
    fn test_integer_u64() {
        assert_eq!(integer_u64(0), vec![0x02, 0x01, 0x00]);
        assert_eq!(integer_u64(1), vec![0x02, 0x01, 0x01]);
        assert_eq!(integer_u64(256), vec![0x02, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn test_oid_simple() {
        // OID 1.2.840.113549 (PKCS prefix)
        let encoded = oid(&[1, 2, 840, 113549]);
        assert_eq!(encoded[0], TAG_OID);
        // First byte of content: 1*40+2 = 42
        assert_eq!(encoded[2], 42);
    }

    #[test]
    fn test_oid_sha256() {
        // 2.16.840.1.101.3.4.2.1
        let encoded = oid(OID_SHA256);
        assert_eq!(encoded[0], TAG_OID);
        // First byte: 2*40+16 = 96 = 0x60
        assert_eq!(encoded[2], 0x60);
    }

    #[test]
    fn test_sequence() {
        let int1 = integer_u64(1);
        let int2 = integer_u64(2);
        let seq = sequence(&[&int1, &int2]);
        assert_eq!(seq[0], TAG_SEQUENCE);
        // Total content: 3+3 = 6 bytes
        assert_eq!(seq[1], 6);
    }

    #[test]
    fn test_set() {
        let int1 = integer_u64(42);
        let s = set(&[&int1]);
        assert_eq!(s[0], TAG_SET);
    }

    #[test]
    fn test_octet_string() {
        let data = octet_string(&[0xDE, 0xAD]);
        assert_eq!(data, vec![TAG_OCTET_STRING, 0x02, 0xDE, 0xAD]);
    }

    #[test]
    fn test_context_tag() {
        let inner = integer_u64(1);
        let tagged = context_tag(0, &inner);
        assert_eq!(tagged[0], 0xA0); // context-specific, constructed, tag 0
    }

    #[test]
    fn test_context_tag_implicit() {
        let tagged = context_tag_implicit(0, &[0x01]);
        assert_eq!(tagged[0], 0x80);
    }

    #[test]
    fn test_bit_string() {
        let bs = bit_string(&[0xFF]);
        assert_eq!(bs, vec![TAG_BIT_STRING, 0x02, 0x00, 0xFF]);
    }

    #[test]
    fn test_utc_time() {
        let t = utc_time("260524120000Z");
        assert_eq!(t[0], TAG_UTC_TIME);
        assert_eq!(t[1], 13); // "260524120000Z" is 13 bytes
    }

    #[test]
    fn test_algorithm_identifier() {
        let ai = algorithm_identifier(OID_SHA256);
        assert_eq!(ai[0], TAG_SEQUENCE);
        // Should contain OID + NULL
    }

    #[test]
    fn test_attribute() {
        let digest = octet_string(&[0x01, 0x02]);
        let attr = attribute(OID_MESSAGE_DIGEST, &digest);
        assert_eq!(attr[0], TAG_SEQUENCE); // SEQUENCE { OID, SET { value } }
    }

    #[test]
    fn test_large_payload_length() {
        // Create a payload >127 bytes to test long-form length
        let data = vec![0xAA; 200];
        let encoded = octet_string(&data);
        assert_eq!(encoded[0], TAG_OCTET_STRING);
        assert_eq!(encoded[1], 0x81); // long form, 1 length byte
        assert_eq!(encoded[2], 200);
        assert_eq!(encoded.len(), 203); // tag + 2 length bytes + 200 data
    }

    #[test]
    fn test_digest_oid_mapping() {
        assert_eq!(digest_oid("SHA-256"), OID_SHA256);
        assert_eq!(digest_oid("RS384"), OID_SHA384);
        assert_eq!(digest_oid("ES512"), OID_SHA512);
        assert_eq!(digest_oid("unknown"), OID_SHA256); // default
    }
}
