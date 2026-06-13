use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("Azure authentication failed: {0}")]
    Auth(String),

    #[error("Key Vault API error ({status}): {message}")]
    KeyVault { status: u16, message: String },

    #[error("Azure token expired or invalid")]
    TokenExpired,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Signing configuration missing: {0}")]
    ConfigMissing(String),

    #[error("HTTP request failed: {0}")]
    Http(String),

    #[error("Invalid key/certificate name: {0}")]
    InvalidName(String),

    #[error("PE parsing failed: {0}")]
    PeParse(String),

    #[error("Invalid PE file: {0}")]
    InvalidPe(String),

    #[error("Certificate parsing failed: {0}")]
    CertParse(String),

    #[error("DER encoding failed: {0}")]
    DerEncode(String),

    #[error("Timestamp request failed: {0}")]
    Timestamp(String),

    #[error("RPM parsing failed: {0}")]
    RpmParse(String),

    #[error("OpenPGP packet construction failed: {0}")]
    PgpBuild(String),

    #[error("AR archive parsing failed: {0}")]
    ArParse(String),

    #[error("Signature verification failed: {0}")]
    VerifyFailed(String),

    #[error("Key export failed: {0}")]
    KeyExport(String),

    #[error("Unsupported signature format: {0}")]
    UnsupportedSignature(String),
}
