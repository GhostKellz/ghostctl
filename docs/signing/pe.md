# PE Authenticode Signing

Windows executables (`.exe`, `.dll`, `.sys`) are signed with embedded Authenticode signatures.

## How It Works

1. The PE file is parsed to locate the checksum field and certificate table directory entry
2. An Authenticode digest is computed (SHA-256) excluding the checksum, cert table pointer, and any existing certificate data
3. A CMS/PKCS#7 SignedData envelope is built with the SPC_INDIRECT_DATA_CONTENT structure
4. The digest is sent to Azure Key Vault for PKCS#1 v1.5 RSA signing
5. The signed CMS envelope is embedded in the PE as a `WIN_CERTIFICATE` structure

## Usage

```bash
# Sign an EXE (auto-detected)
ghostctl sign file app.exe

# With timestamp (default when TSA URL configured)
ghostctl sign file driver.sys --timestamp

# Without timestamp
ghostctl sign file tool.dll --no-timestamp

# Dry run
ghostctl sign file app.exe --dry-run
```

## Timestamping

RFC 3161 timestamps are embedded as unauthenticated attributes in the CMS SignerInfo. By default, PE signing requests a timestamp when a TSA URL is configured. Use `--no-timestamp` to disable.

When `--timestamp` is passed explicitly, timestamp failure aborts signing. Default timestamp attempts still warn and continue so an unavailable TSA does not block local test signing.

Default TSA: `http://timestamp.digicert.com`

## Verification

```bash
# Using osslsigncode
osslsigncode verify app.exe

# Using signtool (Windows)
signtool verify /pa app.exe
```

## Limitations

- RSA only (RS256/RS384/RS512)
- Single certificate, no chain building
- No dual-signing or appending to existing signatures
- No page hashing (SPC_PE_IMAGE_PAGE_HASHES)
