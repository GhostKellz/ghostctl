// use crate::utils::run_command;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    pub provider: S3Provider,
    pub endpoint: Option<String>,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum S3Provider {
    AWS,
    MinIO,
    Azure,
    Backblaze,
    Wasabi,
    DigitalOcean,
    Custom,
}

impl std::fmt::Display for S3Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3Provider::AWS => write!(f, "Amazon S3"),
            S3Provider::MinIO => write!(f, "MinIO"),
            S3Provider::Azure => write!(f, "Azure Blob Storage"),
            S3Provider::Backblaze => write!(f, "Backblaze B2"),
            S3Provider::Wasabi => write!(f, "Wasabi"),
            S3Provider::DigitalOcean => write!(f, "DigitalOcean Spaces"),
            S3Provider::Custom => write!(f, "Custom S3-compatible"),
        }
    }
}

const CONFIG_DIR: &str = ".config/ghostctl/s3";

pub fn s3_menu() {
    loop {
        let options = vec![
            "Configure S3 Provider",
            "Bucket Operations",
            "File Operations",
            "Sync Operations",
            "Restic Integration",
            "Test Connection",
            "Manage Profiles",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("☁️  S3 Storage Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => configure_provider(),
            1 => bucket_operations(),
            2 => file_operations(),
            3 => sync_operations(),
            4 => restic_integration(),
            5 => test_connection(),
            6 => manage_profiles(),
            _ => break,
        }
    }
}

fn configure_provider() {
    println!("🔧 Configure S3 Provider\n");
    
    let providers = vec![
        "Amazon S3 (AWS)",
        "MinIO",
        "Azure Blob Storage",
        "Backblaze B2",
        "Wasabi",
        "DigitalOcean Spaces",
        "Custom S3-compatible",
    ];
    
    let provider_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select S3 provider")
        .items(&providers)
        .default(0)
        .interact()
        .unwrap();
    
    let provider = match provider_idx {
        0 => S3Provider::AWS,
        1 => S3Provider::MinIO,
        2 => S3Provider::Azure,
        3 => S3Provider::Backblaze,
        4 => S3Provider::Wasabi,
        5 => S3Provider::DigitalOcean,
        _ => S3Provider::Custom,
    };
    
    let (endpoint, region) = match &provider {
        S3Provider::AWS => {
            let regions = vec![
                "us-east-1", "us-east-2", "us-west-1", "us-west-2",
                "eu-west-1", "eu-west-2", "eu-west-3", "eu-central-1",
                "ap-southeast-1", "ap-southeast-2", "ap-northeast-1",
                "Custom region",
            ];
            
            let region_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select AWS region")
                .items(&regions)
                .default(0)
                .interact()
                .unwrap();
            
            let region = if region_idx == regions.len() - 1 {
                Input::new()
                    .with_prompt("Enter custom region")
                    .interact_text()
                    .unwrap()
            } else {
                regions[region_idx].to_string()
            };
            
            (None, region)
        }
        S3Provider::MinIO => {
            let endpoint = Input::new()
                .with_prompt("MinIO endpoint URL (e.g., http://localhost:9000)")
                .interact_text()
                .unwrap();
            
            (Some(endpoint), "us-east-1".to_string())
        }
        S3Provider::Azure => {
            let account = Input::<String>::new()
                .with_prompt("Azure storage account name")
                .interact_text()
                .unwrap();
            
            let endpoint = format!("https://{}.blob.core.windows.net", account);
            (Some(endpoint), "".to_string())
        }
        S3Provider::Backblaze => {
            let region = Input::new()
                .with_prompt("Backblaze region (e.g., us-west-002)")
                .default("us-west-002".to_string())
                .interact_text()
                .unwrap();
            
            let endpoint = format!("https://s3.{}.backblazeb2.com", region);
            (Some(endpoint), region)
        }
        S3Provider::Wasabi => {
            let regions = vec![
                "us-east-1 (N. Virginia)",
                "us-east-2 (N. Virginia)",
                "us-west-1 (Oregon)",
                "eu-central-1 (Amsterdam)",
                "ap-northeast-1 (Tokyo)",
            ];
            
            let region_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select Wasabi region")
                .items(&regions)
                .interact()
                .unwrap();
            
            let (region, endpoint_base) = match region_idx {
                0 | 1 => ("us-east-1", "s3.wasabisys.com"),
                2 => ("us-west-1", "s3.us-west-1.wasabisys.com"),
                3 => ("eu-central-1", "s3.eu-central-1.wasabisys.com"),
                4 => ("ap-northeast-1", "s3.ap-northeast-1.wasabisys.com"),
                _ => ("us-east-1", "s3.wasabisys.com"),
            };
            
            (Some(format!("https://{}", endpoint_base)), region.to_string())
        }
        S3Provider::DigitalOcean => {
            let regions = vec![
                "nyc3 (New York)",
                "sfo3 (San Francisco)",
                "ams3 (Amsterdam)",
                "sgp1 (Singapore)",
                "fra1 (Frankfurt)",
            ];
            
            let region_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select DigitalOcean region")
                .items(&regions)
                .interact()
                .unwrap();
            
            let region = match region_idx {
                0 => "nyc3",
                1 => "sfo3",
                2 => "ams3",
                3 => "sgp1",
                4 => "fra1",
                _ => "nyc3",
            };
            
            let endpoint = format!("https://{}.digitaloceanspaces.com", region);
            (Some(endpoint), region.to_string())
        }
        S3Provider::Custom => {
            let endpoint = Input::new()
                .with_prompt("S3-compatible endpoint URL")
                .interact_text()
                .unwrap();
            
            let region = Input::new()
                .with_prompt("Region (or press Enter for default)")
                .default("us-east-1".to_string())
                .interact_text()
                .unwrap();
            
            (Some(endpoint), region)
        }
    };
    
    let access_key = Input::new()
        .with_prompt("Access Key ID")
        .interact_text()
        .unwrap();
    
    let secret_key = Password::new()
        .with_prompt("Secret Access Key")
        .interact()
        .unwrap();
    
    let bucket = Input::<String>::new()
        .with_prompt("Default bucket (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let config = S3Config {
        provider,
        endpoint,
        region,
        access_key,
        secret_key,
        bucket: if bucket.is_empty() { None } else { Some(bucket) },
    };
    
    // Save configuration
    let profile_name = Input::new()
        .with_prompt("Profile name")
        .default("default".to_string())
        .interact_text()
        .unwrap();
    
    save_s3_config(&profile_name, &config);
    
    // Configure AWS CLI if needed
    if matches!(config.provider, S3Provider::AWS) {
        if Confirm::new()
            .with_prompt("Configure AWS CLI with these credentials?")
            .default(true)
            .interact()
            .unwrap()
        {
            configure_aws_cli(&config);
        }
    }
    
    println!("✅ S3 configuration saved as profile: {}", profile_name);
}

fn bucket_operations() {
    let config = load_current_config();
    if config.is_none() {
        println!("❌ No S3 configuration found. Please configure a provider first.");
        return;
    }
    
    let config = config.unwrap();
    
    loop {
        let options = vec![
            "List Buckets",
            "Create Bucket",
            "Delete Bucket",
            "Bucket Info",
            "Set Bucket Policy",
            "Enable Versioning",
            "Configure Lifecycle",
            "Back",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🪣 Bucket Operations")
            .items(&options)
            .interact()
            .unwrap();
        
        match selection {
            0 => list_buckets(&config),
            1 => create_bucket(&config),
            2 => delete_bucket(&config),
            3 => bucket_info(&config),
            4 => set_bucket_policy(&config),
            5 => enable_versioning(&config),
            6 => configure_lifecycle(&config),
            _ => break,
        }
    }
}

fn list_buckets(config: &S3Config) {
    println!("\n📋 Listing buckets...");
    
    let mut cmd = build_s3_command(config, "ls");
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if result.trim().is_empty() {
            println!("No buckets found.");
        } else {
            println!("\nBuckets:");
            for line in result.lines() {
                println!("  {}", line);
            }
        }
    } else {
        println!("❌ Failed to list buckets: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn create_bucket(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap();
    
    println!("📦 Creating bucket: {}", bucket_name);
    
    let mut cmd = build_s3_command(config, "mb");
    cmd.arg(format!("s3://{}", bucket_name));
    
    if !config.region.is_empty() && config.region != "us-east-1" {
        cmd.arg("--region").arg(&config.region);
    }
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Bucket created successfully: {}", bucket_name);
    } else {
        println!("❌ Failed to create bucket: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn delete_bucket(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name to delete")
        .interact_text()
        .unwrap();
    
    let force = Confirm::new()
        .with_prompt("Force delete? (removes all objects first)")
        .default(false)
        .interact()
        .unwrap();
    
    if !Confirm::new()
        .with_prompt(&format!("Are you sure you want to delete bucket '{}'?", bucket_name))
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    println!("🗑️  Deleting bucket: {}", bucket_name);
    
    let mut cmd = build_s3_command(config, "rb");
    cmd.arg(format!("s3://{}", bucket_name));
    
    if force {
        cmd.arg("--force");
    }
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Bucket deleted: {}", bucket_name);
    } else {
        println!("❌ Failed to delete bucket: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn bucket_info(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap();
    
    println!("\n📊 Bucket Information: {}", bucket_name);
    
    // Get bucket location
    let mut cmd = build_s3api_command(config, "get-bucket-location");
    cmd.arg("--bucket").arg(&bucket_name);
    
    let output = cmd.output().unwrap_or_default();
    if output.status.success() {
        println!("\n📍 Location:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    // Get bucket versioning
    let mut cmd = build_s3api_command(config, "get-bucket-versioning");
    cmd.arg("--bucket").arg(&bucket_name);
    
    let output = cmd.output().unwrap_or_default();
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if !result.trim().is_empty() {
            println!("\n📚 Versioning:");
            println!("{}", result);
        }
    }
    
    // Get bucket size
    println!("\n📏 Calculating bucket size...");
    let mut cmd = build_s3_command(config, "ls");
    cmd.arg(format!("s3://{}", bucket_name))
       .arg("--recursive")
       .arg("--summarize")
       .arg("--human-readable");
    
    let output = cmd.output().unwrap_or_default();
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        for line in result.lines().rev().take(2) {
            println!("  {}", line);
        }
    }
}

fn set_bucket_policy(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap();
    
    let policies = vec![
        "Public Read",
        "Private (default)",
        "Public Read-Write (dangerous!)",
        "Custom JSON policy",
    ];
    
    let policy_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bucket policy")
        .items(&policies)
        .interact()
        .unwrap();
    
    match policy_idx {
        0 => {
            // Public read policy
            let policy = format!(r#"{{
    "Version": "2012-10-17",
    "Statement": [
        {{
            "Sid": "PublicRead",
            "Effect": "Allow",
            "Principal": "*",
            "Action": ["s3:GetObject"],
            "Resource": ["arn:aws:s3:::{}/*"]
        }}
    ]
}}"#, bucket_name);
            
            apply_bucket_policy(config, &bucket_name, &policy);
        }
        1 => {
            // Remove public access
            let mut cmd = build_s3api_command(config, "delete-bucket-policy");
            cmd.arg("--bucket").arg(&bucket_name);
            
            let output = cmd.output().unwrap_or_default();
            if output.status.success() {
                println!("✅ Bucket set to private");
            } else {
                println!("❌ Failed to update policy");
            }
        }
        2 => {
            if Confirm::new()
                .with_prompt("⚠️  WARNING: This will make your bucket publicly writable. Continue?")
                .default(false)
                .interact()
                .unwrap()
            {
                let policy = format!(r#"{{
    "Version": "2012-10-17",
    "Statement": [
        {{
            "Sid": "PublicReadWrite",
            "Effect": "Allow",
            "Principal": "*",
            "Action": ["s3:*"],
            "Resource": ["arn:aws:s3:::{}/*", "arn:aws:s3:::{}"]
        }}
    ]
}}"#, bucket_name, bucket_name);
                
                apply_bucket_policy(config, &bucket_name, &policy);
            }
        }
        3 => {
            println!("Enter custom policy JSON (end with Ctrl+D):");
            let mut policy = String::new();
            std::io::stdin().read_line(&mut policy).ok();
            
            apply_bucket_policy(config, &bucket_name, &policy);
        }
        _ => {}
    }
}

fn apply_bucket_policy(config: &S3Config, bucket: &str, policy: &str) {
    // Write policy to temp file
    let temp_file = "/tmp/bucket-policy.json";
    fs::write(temp_file, policy).expect("Failed to write policy file");
    
    let mut cmd = build_s3api_command(config, "put-bucket-policy");
    cmd.arg("--bucket").arg(bucket)
       .arg("--policy").arg(format!("file://{}", temp_file));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Bucket policy updated");
    } else {
        println!("❌ Failed to update policy: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    fs::remove_file(temp_file).ok();
}

fn enable_versioning(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap();
    
    let enable = Confirm::new()
        .with_prompt("Enable versioning?")
        .default(true)
        .interact()
        .unwrap();
    
    let mut cmd = build_s3api_command(config, "put-bucket-versioning");
    cmd.arg("--bucket").arg(&bucket_name);
    
    if enable {
        cmd.arg("--versioning-configuration").arg("Status=Enabled");
    } else {
        cmd.arg("--versioning-configuration").arg("Status=Suspended");
    }
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Versioning {} for bucket: {}", 
            if enable { "enabled" } else { "suspended" }, 
            bucket_name
        );
    } else {
        println!("❌ Failed to update versioning: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn configure_lifecycle(config: &S3Config) {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap();
    
    let rules = vec![
        "Delete old versions after 30 days",
        "Move to Glacier after 90 days",
        "Delete incomplete multipart uploads after 7 days",
        "Custom lifecycle rule",
        "Remove all lifecycle rules",
    ];
    
    let rule_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select lifecycle rule")
        .items(&rules)
        .interact()
        .unwrap();
    
    match rule_idx {
        0 => {
            let lifecycle = r#"{
    "Rules": [{
        "ID": "DeleteOldVersions",
        "Status": "Enabled",
        "NoncurrentVersionExpiration": {
            "NoncurrentDays": 30
        }
    }]
}"#;
            apply_lifecycle(config, &bucket_name, lifecycle);
        }
        1 => {
            let lifecycle = r#"{
    "Rules": [{
        "ID": "MoveToGlacier",
        "Status": "Enabled",
        "Transitions": [{
            "Days": 90,
            "StorageClass": "GLACIER"
        }]
    }]
}"#;
            apply_lifecycle(config, &bucket_name, lifecycle);
        }
        2 => {
            let lifecycle = r#"{
    "Rules": [{
        "ID": "AbortIncompleteMultipartUpload",
        "Status": "Enabled",
        "AbortIncompleteMultipartUpload": {
            "DaysAfterInitiation": 7
        }
    }]
}"#;
            apply_lifecycle(config, &bucket_name, lifecycle);
        }
        3 => {
            println!("Enter custom lifecycle JSON:");
            let mut lifecycle = String::new();
            std::io::stdin().read_line(&mut lifecycle).ok();
            apply_lifecycle(config, &bucket_name, &lifecycle);
        }
        4 => {
            let mut cmd = build_s3api_command(config, "delete-bucket-lifecycle");
            cmd.arg("--bucket").arg(&bucket_name);
            
            let output = cmd.output().unwrap_or_default();
            if output.status.success() {
                println!("✅ Lifecycle rules removed");
            }
        }
        _ => {}
    }
}

fn apply_lifecycle(config: &S3Config, bucket: &str, lifecycle: &str) {
    let temp_file = "/tmp/lifecycle.json";
    fs::write(temp_file, lifecycle).expect("Failed to write lifecycle file");
    
    let mut cmd = build_s3api_command(config, "put-bucket-lifecycle-configuration");
    cmd.arg("--bucket").arg(bucket)
       .arg("--lifecycle-configuration").arg(format!("file://{}", temp_file));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Lifecycle configuration updated");
    } else {
        println!("❌ Failed to update lifecycle: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    fs::remove_file(temp_file).ok();
}

fn file_operations() {
    let config = load_current_config();
    if config.is_none() {
        println!("❌ No S3 configuration found.");
        return;
    }
    
    let config = config.unwrap();
    
    loop {
        let options = vec![
            "Upload File",
            "Download File",
            "List Files",
            "Delete File",
            "Copy File",
            "Move File",
            "Generate Presigned URL",
            "Back",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📁 File Operations")
            .items(&options)
            .interact()
            .unwrap();
        
        match selection {
            0 => upload_file(&config),
            1 => download_file(&config),
            2 => list_files(&config),
            3 => delete_file(&config),
            4 => copy_file(&config),
            5 => move_file(&config),
            6 => generate_presigned_url(&config),
            _ => break,
        }
    }
}

fn upload_file(config: &S3Config) {
    let local_file: String = Input::new()
        .with_prompt("Local file path")
        .interact_text()
        .unwrap();
    
    if !Path::new(&local_file).exists() {
        println!("❌ File not found: {}", local_file);
        return;
    }
    
    let bucket = get_bucket_name(config);
    
    let s3_key: String = Input::new()
        .with_prompt("S3 key (path in bucket)")
        .default(Path::new(&local_file).file_name().unwrap().to_string_lossy().to_string())
        .interact_text()
        .unwrap();
    
    let storage_class = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Storage class")
        .items(&["STANDARD", "REDUCED_REDUNDANCY", "GLACIER", "DEEP_ARCHIVE"])
        .default(0)
        .interact()
        .unwrap();
    
    println!("📤 Uploading {} to s3://{}/{}", local_file, bucket, s3_key);
    
    let mut cmd = build_s3_command(config, "cp");
    cmd.arg(&local_file)
       .arg(format!("s3://{}/{}", bucket, s3_key));
    
    match storage_class {
        1 => cmd.arg("--storage-class").arg("REDUCED_REDUNDANCY"),
        2 => cmd.arg("--storage-class").arg("GLACIER"),
        3 => cmd.arg("--storage-class").arg("DEEP_ARCHIVE"),
        _ => &mut cmd,
    };
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ File uploaded successfully");
    } else {
        println!("❌ Upload failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn download_file(config: &S3Config) {
    let bucket = get_bucket_name(config);
    
    let s3_key: String = Input::new()
        .with_prompt("S3 key (path in bucket)")
        .interact_text()
        .unwrap();
    
    let local_file: String = Input::new()
        .with_prompt("Local file path")
        .default(Path::new(&s3_key).file_name().unwrap_or_default().to_string_lossy().to_string())
        .interact_text()
        .unwrap();
    
    println!("📥 Downloading s3://{}/{} to {}", bucket, s3_key, local_file);
    
    let mut cmd = build_s3_command(config, "cp");
    cmd.arg(format!("s3://{}/{}", bucket, s3_key))
       .arg(&local_file);
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ File downloaded successfully");
    } else {
        println!("❌ Download failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn list_files(config: &S3Config) {
    let bucket = get_bucket_name(config);
    
    let prefix: String = Input::new()
        .with_prompt("Prefix (leave empty for all)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let recursive = Confirm::new()
        .with_prompt("List recursively?")
        .default(true)
        .interact()
        .unwrap();
    
    println!("\n📋 Listing files in s3://{}/{}", bucket, prefix);
    
    let mut cmd = build_s3_command(config, "ls");
    cmd.arg(format!("s3://{}/{}", bucket, prefix));
    
    if recursive {
        cmd.arg("--recursive");
    }
    
    cmd.arg("--human-readable");
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if result.trim().is_empty() {
            println!("No files found");
        } else {
            for line in result.lines() {
                println!("  {}", line);
            }
        }
    } else {
        println!("❌ Failed to list files: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn delete_file(config: &S3Config) {
    let bucket = get_bucket_name(config);
    
    let s3_key: String = Input::new()
        .with_prompt("S3 key to delete")
        .interact_text()
        .unwrap();
    
    if !Confirm::new()
        .with_prompt(&format!("Delete s3://{}/{}?", bucket, s3_key))
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    let mut cmd = build_s3_command(config, "rm");
    cmd.arg(format!("s3://{}/{}", bucket, s3_key));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ File deleted");
    } else {
        println!("❌ Delete failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn copy_file(config: &S3Config) {
    let source_bucket = get_bucket_name(config);
    
    let source_key: String = Input::new()
        .with_prompt("Source S3 key")
        .interact_text()
        .unwrap();
    
    let dest_bucket: String = Input::new()
        .with_prompt("Destination bucket")
        .default(source_bucket.clone())
        .interact_text()
        .unwrap();
    
    let dest_key: String = Input::new()
        .with_prompt("Destination S3 key")
        .default(source_key.clone())
        .interact_text()
        .unwrap();
    
    println!("📋 Copying s3://{}/{} to s3://{}/{}", source_bucket, source_key, dest_bucket, dest_key);
    
    let mut cmd = build_s3_command(config, "cp");
    cmd.arg(format!("s3://{}/{}", source_bucket, source_key))
       .arg(format!("s3://{}/{}", dest_bucket, dest_key));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ File copied successfully");
    } else {
        println!("❌ Copy failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn move_file(config: &S3Config) {
    let source_bucket = get_bucket_name(config);
    
    let source_key: String = Input::new()
        .with_prompt("Source S3 key")
        .interact_text()
        .unwrap();
    
    let dest_bucket: String = Input::new()
        .with_prompt("Destination bucket")
        .default(source_bucket.clone())
        .interact_text()
        .unwrap();
    
    let dest_key: String = Input::new()
        .with_prompt("Destination S3 key")
        .interact_text()
        .unwrap();
    
    println!("📦 Moving s3://{}/{} to s3://{}/{}", source_bucket, source_key, dest_bucket, dest_key);
    
    let mut cmd = build_s3_command(config, "mv");
    cmd.arg(format!("s3://{}/{}", source_bucket, source_key))
       .arg(format!("s3://{}/{}", dest_bucket, dest_key));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ File moved successfully");
    } else {
        println!("❌ Move failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn generate_presigned_url(config: &S3Config) {
    let bucket = get_bucket_name(config);
    
    let s3_key: String = Input::new()
        .with_prompt("S3 key")
        .interact_text()
        .unwrap();
    
    let expires_in: u32 = Input::new()
        .with_prompt("Expiration time (seconds)")
        .default(3600)
        .interact()
        .unwrap();
    
    let mut cmd = build_s3_command(config, "presign");
    cmd.arg(format!("s3://{}/{}", bucket, s3_key))
       .arg("--expires-in")
       .arg(expires_in.to_string());
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout);
        println!("\n✅ Presigned URL (expires in {} seconds):", expires_in);
        println!("{}", url);
    } else {
        println!("❌ Failed to generate URL: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn sync_operations() {
    let config = load_current_config();
    if config.is_none() {
        println!("❌ No S3 configuration found.");
        return;
    }
    
    let config = config.unwrap();
    
    loop {
        let options = vec![
            "Sync Local to S3",
            "Sync S3 to Local",
            "Sync S3 to S3",
            "Mirror (with delete)",
            "Dry Run Sync",
            "Back",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔄 Sync Operations")
            .items(&options)
            .interact()
            .unwrap();
        
        match selection {
            0 => sync_local_to_s3(&config),
            1 => sync_s3_to_local(&config),
            2 => sync_s3_to_s3(&config),
            3 => mirror_sync(&config),
            4 => dry_run_sync(&config),
            _ => break,
        }
    }
}

fn sync_local_to_s3(config: &S3Config) {
    let local_dir: String = Input::new()
        .with_prompt("Local directory")
        .interact_text()
        .unwrap();
    
    if !Path::new(&local_dir).is_dir() {
        println!("❌ Directory not found: {}", local_dir);
        return;
    }
    
    let bucket = get_bucket_name(config);
    
    let s3_prefix: String = Input::new()
        .with_prompt("S3 prefix (leave empty for root)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let exclude: String = Input::new()
        .with_prompt("Exclude pattern (e.g., *.tmp, leave empty for none)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    println!("🔄 Syncing {} to s3://{}/{}", local_dir, bucket, s3_prefix);
    
    let mut cmd = build_s3_command(config, "sync");
    cmd.arg(&local_dir)
       .arg(format!("s3://{}/{}", bucket, s3_prefix));
    
    if !exclude.is_empty() {
        cmd.arg("--exclude").arg(&exclude);
    }
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Sync completed");
    } else {
        println!("❌ Sync failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn sync_s3_to_local(config: &S3Config) {
    let bucket = get_bucket_name(config);
    
    let s3_prefix: String = Input::new()
        .with_prompt("S3 prefix")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let local_dir: String = Input::new()
        .with_prompt("Local directory")
        .interact_text()
        .unwrap();
    
    println!("🔄 Syncing s3://{}/{} to {}", bucket, s3_prefix, local_dir);
    
    let mut cmd = build_s3_command(config, "sync");
    cmd.arg(format!("s3://{}/{}", bucket, s3_prefix))
       .arg(&local_dir);
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Sync completed");
    } else {
        println!("❌ Sync failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn sync_s3_to_s3(config: &S3Config) {
    let source_bucket: String = Input::new()
        .with_prompt("Source bucket")
        .interact_text()
        .unwrap();
    
    let source_prefix: String = Input::new()
        .with_prompt("Source prefix")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let dest_bucket: String = Input::new()
        .with_prompt("Destination bucket")
        .interact_text()
        .unwrap();
    
    let dest_prefix: String = Input::new()
        .with_prompt("Destination prefix")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    println!("🔄 Syncing s3://{}/{} to s3://{}/{}", 
        source_bucket, source_prefix, dest_bucket, dest_prefix);
    
    let mut cmd = build_s3_command(config, "sync");
    cmd.arg(format!("s3://{}/{}", source_bucket, source_prefix))
       .arg(format!("s3://{}/{}", dest_bucket, dest_prefix));
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Sync completed");
    } else {
        println!("❌ Sync failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn mirror_sync(config: &S3Config) {
    println!("⚠️  Mirror sync will DELETE files in destination that don't exist in source!");
    
    if !Confirm::new()
        .with_prompt("Continue?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    let direction = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mirror direction")
        .items(&["Local → S3", "S3 → Local"])
        .interact()
        .unwrap();
    
    if direction == 0 {
        let local_dir: String = Input::new()
            .with_prompt("Local directory")
            .interact_text()
            .unwrap();
        
        let bucket = get_bucket_name(config);
        let s3_prefix: String = Input::new()
            .with_prompt("S3 prefix")
            .allow_empty(true)
            .interact_text()
            .unwrap();
        
        println!("🔄 Mirroring {} to s3://{}/{}", local_dir, bucket, s3_prefix);
        
        let mut cmd = build_s3_command(config, "sync");
        cmd.arg(&local_dir)
           .arg(format!("s3://{}/{}", bucket, s3_prefix))
           .arg("--delete");
        
        let output = cmd.output().unwrap_or_default();
        
        if output.status.success() {
            println!("✅ Mirror sync completed");
        } else {
            println!("❌ Mirror sync failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        let bucket = get_bucket_name(config);
        let s3_prefix: String = Input::new()
            .with_prompt("S3 prefix")
            .allow_empty(true)
            .interact_text()
            .unwrap();
        
        let local_dir: String = Input::new()
            .with_prompt("Local directory")
            .interact_text()
            .unwrap();
        
        println!("🔄 Mirroring s3://{}/{} to {}", bucket, s3_prefix, local_dir);
        
        let mut cmd = build_s3_command(config, "sync");
        cmd.arg(format!("s3://{}/{}", bucket, s3_prefix))
           .arg(&local_dir)
           .arg("--delete");
        
        let output = cmd.output().unwrap_or_default();
        
        if output.status.success() {
            println!("✅ Mirror sync completed");
        } else {
            println!("❌ Mirror sync failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

fn dry_run_sync(config: &S3Config) {
    let source: String = Input::new()
        .with_prompt("Source (local path or s3://bucket/prefix)")
        .interact_text()
        .unwrap();
    
    let dest: String = Input::new()
        .with_prompt("Destination (local path or s3://bucket/prefix)")
        .interact_text()
        .unwrap();
    
    let delete = Confirm::new()
        .with_prompt("Include --delete flag?")
        .default(false)
        .interact()
        .unwrap();
    
    println!("🔍 Dry run: {} → {}", source, dest);
    
    let mut cmd = build_s3_command(config, "sync");
    cmd.arg(&source)
       .arg(&dest)
       .arg("--dryrun");
    
    if delete {
        cmd.arg("--delete");
    }
    
    let output = cmd.output().unwrap_or_default();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !stdout.is_empty() {
        println!("\n📋 Operations that would be performed:");
        for line in stdout.lines() {
            println!("  {}", line);
        }
    }
    
    if !stderr.is_empty() {
        println!("\n⚠️  Warnings:");
        for line in stderr.lines() {
            println!("  {}", line);
        }
    }
}

fn restic_integration() {
    let config = load_current_config();
    if config.is_none() {
        println!("❌ No S3 configuration found.");
        return;
    }
    
    let config = config.unwrap();
    
    loop {
        let options = vec![
            "Configure Restic S3 Backend",
            "Initialize Restic Repository",
            "Create Backup to S3",
            "Restore from S3",
            "List Snapshots",
            "Prune Old Snapshots",
            "Back",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔐 Restic S3 Integration")
            .items(&options)
            .interact()
            .unwrap();
        
        match selection {
            0 => configure_restic_s3(&config),
            1 => init_restic_repo(&config),
            2 => create_restic_backup(&config),
            3 => restore_restic_backup(&config),
            4 => list_restic_snapshots(&config),
            5 => prune_restic_snapshots(&config),
            _ => break,
        }
    }
}

fn configure_restic_s3(config: &S3Config) {
    println!("🔧 Configuring Restic for S3 backend\n");
    
    let bucket = get_bucket_name(config);
    
    let repo_path: String = Input::new()
        .with_prompt("Repository path in bucket (e.g., backups/restic)")
        .default("restic".to_string())
        .interact_text()
        .unwrap();
    
    let password = Password::new()
        .with_prompt("Restic repository password")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()
        .unwrap();
    
    // Build restic S3 URL
    let repo_url = if let Some(endpoint) = &config.endpoint {
        format!("s3:{}/{}/{}", endpoint, bucket, repo_path)
    } else {
        format!("s3:s3.amazonaws.com/{}/{}", bucket, repo_path)
    };
    
    // Save restic environment
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    fs::create_dir_all(env_file.parent().unwrap()).ok();
    
    let env_content = format!(
        "export RESTIC_REPOSITORY=\"{}\"\n\
         export RESTIC_PASSWORD=\"{}\"\n\
         export AWS_ACCESS_KEY_ID=\"{}\"\n\
         export AWS_SECRET_ACCESS_KEY=\"{}\"\n",
        repo_url, password, config.access_key, config.secret_key
    );
    
    fs::write(&env_file, env_content).expect("Failed to write restic env");
    
    println!("✅ Restic S3 configuration saved to: {:?}", env_file);
    println!("\nTo use: source {:?}", env_file);
    println!("Repository URL: {}", repo_url);
}

fn init_restic_repo(config: &S3Config) {
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    
    if !env_file.exists() {
        println!("❌ Please configure Restic S3 backend first");
        return;
    }
    
    println!("🔄 Initializing Restic repository...");
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("source {:?} && restic init", env_file))
        .output()
        .unwrap_or_default();
    
    if output.status.success() {
        println!("✅ Repository initialized successfully");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        if error.contains("already initialized") {
            println!("ℹ️  Repository already initialized");
        } else {
            println!("❌ Failed to initialize: {}", error);
        }
    }
}

fn create_restic_backup(config: &S3Config) {
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    
    if !env_file.exists() {
        println!("❌ Please configure Restic S3 backend first");
        return;
    }
    
    let backup_path: String = Input::new()
        .with_prompt("Path to backup")
        .interact_text()
        .unwrap();
    
    if !Path::new(&backup_path).exists() {
        println!("❌ Path not found: {}", backup_path);
        return;
    }
    
    let tags: String = Input::new()
        .with_prompt("Tags (comma-separated, optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    println!("📦 Creating backup of {} to S3...", backup_path);
    
    let mut cmd_str = format!("source {:?} && restic backup \"{}\"", env_file, backup_path);
    
    if !tags.is_empty() {
        for tag in tags.split(',') {
            cmd_str.push_str(&format!(" --tag {}", tag.trim()));
        }
    }
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(&cmd_str)
        .status();
    
    if output.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Backup completed successfully");
    } else {
        println!("❌ Backup failed");
    }
}

fn restore_restic_backup(config: &S3Config) {
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    
    if !env_file.exists() {
        println!("❌ Please configure Restic S3 backend first");
        return;
    }
    
    // List snapshots first
    println!("📋 Available snapshots:");
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("source {:?} && restic snapshots", env_file))
        .output()
        .unwrap_or_default();
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    let snapshot_id: String = Input::new()
        .with_prompt("Snapshot ID (or 'latest')")
        .default("latest".to_string())
        .interact_text()
        .unwrap();
    
    let restore_path: String = Input::new()
        .with_prompt("Restore to path")
        .interact_text()
        .unwrap();
    
    println!("📥 Restoring snapshot {} to {}...", snapshot_id, restore_path);
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {:?} && restic restore {} --target \"{}\"",
            env_file, snapshot_id, restore_path
        ))
        .status();
    
    if output.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Restore completed successfully");
    } else {
        println!("❌ Restore failed");
    }
}

fn list_restic_snapshots(config: &S3Config) {
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    
    if !env_file.exists() {
        println!("❌ Please configure Restic S3 backend first");
        return;
    }
    
    println!("📋 Listing snapshots...\n");
    
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!("source {:?} && restic snapshots", env_file))
        .status();
}

fn prune_restic_snapshots(config: &S3Config) {
    let env_file = home_dir().join(".config/ghostctl/restic-s3.env");
    
    if !env_file.exists() {
        println!("❌ Please configure Restic S3 backend first");
        return;
    }
    
    println!("🗑️  Prune policy configuration\n");
    
    let keep_last: u32 = Input::new()
        .with_prompt("Keep last N snapshots")
        .default(5)
        .interact()
        .unwrap();
    
    let keep_daily: u32 = Input::new()
        .with_prompt("Keep daily snapshots for N days")
        .default(7)
        .interact()
        .unwrap();
    
    let keep_weekly: u32 = Input::new()
        .with_prompt("Keep weekly snapshots for N weeks")
        .default(4)
        .interact()
        .unwrap();
    
    let keep_monthly: u32 = Input::new()
        .with_prompt("Keep monthly snapshots for N months")
        .default(6)
        .interact()
        .unwrap();
    
    if !Confirm::new()
        .with_prompt("Proceed with pruning?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    println!("🔄 Pruning snapshots...");
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {:?} && restic forget --prune \
             --keep-last {} --keep-daily {} --keep-weekly {} --keep-monthly {}",
            env_file, keep_last, keep_daily, keep_weekly, keep_monthly
        ))
        .status();
    
    if output.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Pruning completed successfully");
    } else {
        println!("❌ Pruning failed");
    }
}

fn test_connection() {
    let config = load_current_config();
    if config.is_none() {
        println!("❌ No S3 configuration found.");
        return;
    }
    
    let config = config.unwrap();
    
    println!("🔍 Testing S3 connection...\n");
    println!("Provider: {}", config.provider);
    if let Some(endpoint) = &config.endpoint {
        println!("Endpoint: {}", endpoint);
    }
    println!("Region: {}", config.region);
    
    // Try to list buckets
    let mut cmd = build_s3_command(&config, "ls");
    
    let output = cmd.output().unwrap_or_default();
    
    if output.status.success() {
        println!("\n✅ Connection successful!");
        let buckets = String::from_utf8_lossy(&output.stdout);
        if !buckets.trim().is_empty() {
            println!("\nAccessible buckets:");
            for line in buckets.lines() {
                println!("  {}", line);
            }
        }
    } else {
        println!("\n❌ Connection failed!");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn manage_profiles() {
    loop {
        let options = vec![
            "List Profiles",
            "Switch Profile",
            "Delete Profile",
            "Export Profile",
            "Back",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("👤 Profile Management")
            .items(&options)
            .interact()
            .unwrap();
        
        match selection {
            0 => list_profiles(),
            1 => switch_profile(),
            2 => delete_profile(),
            3 => export_profile(),
            _ => break,
        }
    }
}

fn list_profiles() {
    let config_dir = home_dir().join(CONFIG_DIR);
    
    if !config_dir.exists() {
        println!("No profiles found");
        return;
    }
    
    println!("\n📋 S3 Profiles:");
    
    let current = get_current_profile();
    
    for entry in fs::read_dir(&config_dir).unwrap() {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".json") {
                let profile = name_str.trim_end_matches(".json");
                if Some(profile.to_string()) == current {
                    println!("  * {} (current)", profile);
                } else {
                    println!("    {}", profile);
                }
            }
        }
    }
}

fn switch_profile() {
    let profiles = get_all_profiles();
    
    if profiles.is_empty() {
        println!("No profiles found");
        return;
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile")
        .items(&profiles)
        .interact()
        .unwrap();
    
    let profile = &profiles[selection];
    set_current_profile(profile);
    
    println!("✅ Switched to profile: {}", profile);
}

fn delete_profile() {
    let profiles = get_all_profiles();
    
    if profiles.is_empty() {
        println!("No profiles found");
        return;
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to delete")
        .items(&profiles)
        .interact()
        .unwrap();
    
    let profile = &profiles[selection];
    
    if !Confirm::new()
        .with_prompt(&format!("Delete profile '{}'?", profile))
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    let config_file = home_dir().join(CONFIG_DIR).join(format!("{}.json", profile));
    fs::remove_file(config_file).ok();
    
    println!("✅ Profile deleted: {}", profile);
}

fn export_profile() {
    let profiles = get_all_profiles();
    
    if profiles.is_empty() {
        println!("No profiles found");
        return;
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to export")
        .items(&profiles)
        .interact()
        .unwrap();
    
    let profile = &profiles[selection];
    let config = load_profile(profile);
    
    if let Some(config) = config {
        println!("\n📋 Profile '{}' configuration:", profile);
        println!("\n# AWS CLI configuration:");
        println!("aws configure set aws_access_key_id {}", config.access_key);
        println!("aws configure set aws_secret_access_key {}", config.secret_key);
        println!("aws configure set region {}", config.region);
        
        if let Some(endpoint) = &config.endpoint {
            println!("\n# Custom endpoint:");
            println!("export AWS_ENDPOINT_URL=\"{}\"", endpoint);
        }
        
        println!("\n# Environment variables:");
        println!("export AWS_ACCESS_KEY_ID=\"{}\"", config.access_key);
        println!("export AWS_SECRET_ACCESS_KEY=\"{}\"", config.secret_key);
        println!("export AWS_DEFAULT_REGION=\"{}\"", config.region);
    }
}

// Helper functions

fn build_s3_command(config: &S3Config, operation: &str) -> Command {
    let mut cmd = Command::new("aws");
    cmd.arg("s3").arg(operation);
    
    // Add credentials
    cmd.env("AWS_ACCESS_KEY_ID", &config.access_key)
       .env("AWS_SECRET_ACCESS_KEY", &config.secret_key);
    
    // Add region if specified
    if !config.region.is_empty() {
        cmd.env("AWS_DEFAULT_REGION", &config.region);
    }
    
    // Add custom endpoint if specified
    if let Some(endpoint) = &config.endpoint {
        cmd.arg("--endpoint-url").arg(endpoint);
    }
    
    cmd
}

fn build_s3api_command(config: &S3Config, operation: &str) -> Command {
    let mut cmd = Command::new("aws");
    cmd.arg("s3api").arg(operation);
    
    cmd.env("AWS_ACCESS_KEY_ID", &config.access_key)
       .env("AWS_SECRET_ACCESS_KEY", &config.secret_key);
    
    if !config.region.is_empty() {
        cmd.env("AWS_DEFAULT_REGION", &config.region);
    }
    
    if let Some(endpoint) = &config.endpoint {
        cmd.arg("--endpoint-url").arg(endpoint);
    }
    
    cmd
}

fn get_bucket_name(config: &S3Config) -> String {
    if let Some(bucket) = &config.bucket {
        return bucket.clone();
    }
    
    Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap()
}

fn save_s3_config(profile: &str, config: &S3Config) {
    let config_dir = home_dir().join(CONFIG_DIR);
    fs::create_dir_all(&config_dir).ok();
    
    let config_file = config_dir.join(format!("{}.json", profile));
    let json = serde_json::to_string_pretty(config).unwrap();
    fs::write(config_file, json).expect("Failed to save S3 config");
    
    // Set as current profile
    set_current_profile(profile);
}

fn load_current_config() -> Option<S3Config> {
    let current = get_current_profile()?;
    load_profile(&current)
}

fn load_profile(profile: &str) -> Option<S3Config> {
    let config_file = home_dir().join(CONFIG_DIR).join(format!("{}.json", profile));
    
    if !config_file.exists() {
        return None;
    }
    
    let content = fs::read_to_string(config_file).ok()?;
    serde_json::from_str(&content).ok()
}

fn get_current_profile() -> Option<String> {
    let current_file = home_dir().join(CONFIG_DIR).join("current");
    fs::read_to_string(current_file).ok()
}

fn set_current_profile(profile: &str) {
    let config_dir = home_dir().join(CONFIG_DIR);
    fs::create_dir_all(&config_dir).ok();
    
    let current_file = config_dir.join("current");
    fs::write(current_file, profile).ok();
}

fn get_all_profiles() -> Vec<String> {
    let config_dir = home_dir().join(CONFIG_DIR);
    
    if !config_dir.exists() {
        return Vec::new();
    }
    
    let mut profiles = Vec::new();
    
    for entry in fs::read_dir(&config_dir).unwrap() {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".json") {
                profiles.push(name_str.trim_end_matches(".json").to_string());
            }
        }
    }
    
    profiles
}

fn configure_aws_cli(config: &S3Config) {
    println!("🔧 Configuring AWS CLI...");
    
    Command::new("aws")
        .args(&["configure", "set", "aws_access_key_id", &config.access_key])
        .status()
        .ok();
    
    Command::new("aws")
        .args(&["configure", "set", "aws_secret_access_key", &config.secret_key])
        .status()
        .ok();
    
    Command::new("aws")
        .args(&["configure", "set", "region", &config.region])
        .status()
        .ok();
    
    println!("✅ AWS CLI configured");
}

fn home_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("/tmp")
}