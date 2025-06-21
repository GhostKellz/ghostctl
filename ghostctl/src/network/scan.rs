use std::process::Command;

pub fn gscan_port_scan(target: &str, start_port: Option<&String>, end_port: Option<&String>, banner: bool) {
    println!("🔍 Scanning {} with gscan...", target);
    
    let mut cmd = Command::new("gscan");
    cmd.arg(target);
    
    // Add start port if specified
    if let Some(start) = start_port {
        cmd.arg("-s").arg(start);
    }
    
    // Add end port if specified
    if let Some(end) = end_port {
        cmd.arg("-e").arg(end);
    }
    
    // Add banner grabbing if specified
    if banner {
        cmd.arg("--banner");
    }
    
    // Execute the command
    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("✅ Scan completed successfully");
            } else {
                println!("❌ Scan failed with exit code: {:?}", status.code());
            }
        }
        Err(e) => {
            println!("❌ Failed to execute gscan: {}", e);
            println!("💡 Make sure gscan is installed and in your PATH");
        }
    }
}

pub fn gscan_interactive() {
    println!("🖥️  Launching gscan in interactive TUI mode...");
    
    let status = Command::new("gscan")
        .arg("--tui")
        .status();
    
    match status {
        Ok(_) => println!("✅ gscan TUI completed"),
        Err(e) => {
            println!("❌ Failed to launch gscan TUI: {}", e);
            println!("💡 Make sure gscan is installed and in your PATH");
        }
    }
}
