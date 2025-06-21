use std::process::Command;

/// Send a file using netcat
pub fn send_file(file_path: &str, host: &str, port: u16) {
    println!("📡 Sending file {} to {}:{}...", file_path, host, port);
    
    // Check if file exists
    if !std::path::Path::new(file_path).exists() {
        println!("❌ File {} does not exist", file_path);
        return;
    }
    
    // Use shell command to properly handle redirection
    let command = format!("nc {} {} < {}", host, port, file_path);
    println!("💡 Executing: {}", command);
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ File sent successfully");
            } else {
                println!("❌ Failed to send file: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => println!("❌ Error executing netcat: {}", e),
    }
}

/// Receive a file using netcat
pub fn receive_file(file_path: &str, port: u16) {
    println!("📥 Listening on port {} for incoming file (will save as {})...", port, file_path);
    println!("💡 Use 'nc <this_host> {} < file.txt' from sender", port);
    
    // Use shell command to properly handle redirection
    let command = format!("nc -l {} > {}", port, file_path);
    println!("💡 Executing: {}", command);
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ File received and saved as {}", file_path);
            } else {
                println!("❌ Failed to receive file: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => println!("❌ Error executing netcat: {}", e),
    }
}

/// Simple chat using netcat
pub fn chat(host: Option<&str>, port: u16) {
    match host {
        Some(host) => {
            println!("💬 Connecting to chat at {}:{}...", host, port);
            println!("💡 Type your messages and press Enter. Ctrl+C to exit.");
            
            let _ = Command::new("nc")
                .arg(host)
                .arg(port.to_string())
                .status();
        }
        None => {
            println!("💬 Starting chat server on port {}...", port);
            println!("💡 Others can connect with 'nc <your_ip> {}'", port);
            println!("💡 Type your messages and press Enter. Ctrl+C to exit.");
            
            let _ = Command::new("nc")
                .arg("-l")
                .arg(port.to_string())
                .status();
        }
    }
}

/// Check port connectivity (kept for backward compatibility)
pub fn check_port(host: &str, port: u16) {
    println!("🔍 Checking connectivity to {}:{}...", host, port);
    
    let output = Command::new("nc")
        .arg("-z")
        .arg("-v")
        .arg(host)
        .arg(port.to_string())
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ Port is open and reachable");
            } else {
                println!("❌ Port is closed or unreachable");
            }
        }
        Err(e) => println!("❌ Error executing netcat: {}", e),
    }
}
