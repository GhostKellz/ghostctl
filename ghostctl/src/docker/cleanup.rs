use dialoguer::{Confirm, Select, theme::ColorfulTheme, MultiSelect};
use std::process::Command;

pub fn cleanup_menu() {
    loop {
        let options = vec![
            "Quick Clean (Safe)",
            "Deep Clean (Aggressive)",
            "Container Cleanup",
            "Image Cleanup",
            "Volume Cleanup",
            "Network Cleanup",
            "Build Cache Cleanup",
            "Log Cleanup",
            "Custom Cleanup",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🧹 Docker Cleanup Tools")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => quick_clean(),
            1 => deep_clean(),
            2 => container_cleanup(),
            3 => image_cleanup(),
            4 => volume_cleanup(),
            5 => network_cleanup(),
            6 => build_cache_cleanup(),
            7 => log_cleanup(),
            8 => custom_cleanup(),
            _ => break,
        }
    }
}

fn quick_clean() {
    println!("🧹 Quick Docker Cleanup (Safe)\n");
    
    // Show current usage
    show_docker_usage();
    
    println!("\nThis will remove:");
    println!("  • Stopped containers");
    println!("  • Dangling images");
    println!("  • Unused networks");
    println!("  • Dangling build cache");
    
    if !Confirm::new()
        .with_prompt("Proceed with quick cleanup?")
        .default(true)
        .interact()
        .unwrap()
    {
        return;
    }
    
    // Remove stopped containers
    println!("\n🗑️  Removing stopped containers...");
    let _ = Command::new("docker")
        .args(&["container", "prune", "-f"])
        .status();
    
    // Remove dangling images
    println!("🗑️  Removing dangling images...");
    let _ = Command::new("docker")
        .args(&["image", "prune", "-f"])
        .status();
    
    // Remove unused networks
    println!("🗑️  Removing unused networks...");
    let _ = Command::new("docker")
        .args(&["network", "prune", "-f"])
        .status();
    
    // Remove dangling build cache
    println!("🗑️  Cleaning build cache...");
    let _ = Command::new("docker")
        .args(&["builder", "prune", "-f"])
        .status();
    
    println!("\n✅ Quick cleanup complete!");
    show_docker_usage();
}

fn deep_clean() {
    println!("🚨 Deep Docker Cleanup (Aggressive)\n");
    
    println!("⚠️  WARNING: This will remove:");
    println!("  • ALL stopped containers");
    println!("  • ALL unused images (not just dangling)");
    println!("  • ALL unused volumes");
    println!("  • ALL unused networks");
    println!("  • ALL build cache");
    
    if !Confirm::new()
        .with_prompt("This is DESTRUCTIVE. Continue?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    if !Confirm::new()
        .with_prompt("Are you REALLY sure?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    // Full system prune
    println!("\n🗑️  Running full system prune...");
    let _ = Command::new("docker")
        .args(&["system", "prune", "-af", "--volumes"])
        .status();
    
    println!("\n✅ Deep cleanup complete!");
    show_docker_usage();
}

fn container_cleanup() {
    println!("🐳 Container Cleanup\n");
    
    let options = vec![
        "Remove all stopped containers",
        "Remove exited containers",
        "Remove containers by age",
        "Remove containers by pattern",
        "Remove orphaned containers",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            let _ = Command::new("docker")
                .args(&["container", "prune", "-f"])
                .status();
            println!("✅ Removed all stopped containers");
        }
        1 => {
            // Remove exited containers
            let output = Command::new("docker")
                .args(&["ps", "-aq", "--filter", "status=exited"])
                .output();
            
            if let Ok(output) = output {
                let containers = String::from_utf8_lossy(&output.stdout);
                if !containers.trim().is_empty() {
                    for container in containers.lines() {
                        let _ = Command::new("docker")
                            .args(&["rm", container])
                            .status();
                    }
                    println!("✅ Removed exited containers");
                } else {
                    println!("No exited containers found");
                }
            }
        }
        2 => {
            // Remove by age
            println!("Remove containers older than (e.g., 24h, 7d):");
            let mut age = String::new();
            std::io::stdin().read_line(&mut age).ok();
            
            let _ = Command::new("docker")
                .args(&["container", "prune", "-f", "--filter", &format!("until={}", age.trim())])
                .status();
        }
        3 => {
            // Remove by pattern
            println!("Enter container name pattern:");
            let mut pattern = String::new();
            std::io::stdin().read_line(&mut pattern).ok();
            
            let output = Command::new("docker")
                .args(&["ps", "-aq", "--filter", &format!("name={}", pattern.trim())])
                .output();
            
            if let Ok(output) = output {
                let containers = String::from_utf8_lossy(&output.stdout);
                for container in containers.lines() {
                    let _ = Command::new("docker")
                        .args(&["rm", "-f", container])
                        .status();
                }
            }
        }
        4 => {
            // Remove orphaned containers
            println!("🔍 Finding orphaned containers...");
            
            // Get all container IDs
            let all_containers = Command::new("docker")
                .args(&["ps", "-aq"])
                .output();
            
            // Get containers in docker-compose projects
            let compose_containers = Command::new("docker")
                .args(&["ps", "-aq", "--filter", "label=com.docker.compose.project"])
                .output();
            
            // Find orphans (containers not in compose projects)
            if let (Ok(all), Ok(compose)) = (all_containers, compose_containers) {
                let all_ids: Vec<&str> = std::str::from_utf8(&all.stdout).unwrap_or("").lines().collect();
                let compose_ids: Vec<&str> = std::str::from_utf8(&compose.stdout).unwrap_or("").lines().collect();
                
                let orphans: Vec<&str> = all_ids.into_iter()
                    .filter(|id| !compose_ids.contains(id) && !id.is_empty())
                    .collect();
                
                if !orphans.is_empty() {
                    println!("Found {} orphaned containers", orphans.len());
                    if Confirm::new()
                        .with_prompt("Remove orphaned containers?")
                        .default(false)
                        .interact()
                        .unwrap()
                    {
                        for id in orphans {
                            let _ = Command::new("docker")
                                .args(&["rm", "-f", id])
                                .status();
                        }
                        println!("✅ Removed orphaned containers");
                    }
                } else {
                    println!("No orphaned containers found");
                }
            }
        }
        _ => {}
    }
}

fn image_cleanup() {
    println!("🖼️  Image Cleanup\n");
    
    let options = vec![
        "Remove dangling images",
        "Remove unused images",
        "Remove images by pattern",
        "Remove old versions (keep latest)",
        "Remove large images",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            let _ = Command::new("docker")
                .args(&["image", "prune", "-f"])
                .status();
            println!("✅ Removed dangling images");
        }
        1 => {
            let _ = Command::new("docker")
                .args(&["image", "prune", "-af"])
                .status();
            println!("✅ Removed unused images");
        }
        2 => {
            println!("Enter image pattern (e.g., 'redis:*'):");
            let mut pattern = String::new();
            std::io::stdin().read_line(&mut pattern).ok();
            
            let output = Command::new("docker")
                .args(&["images", "--format", "{{.Repository}}:{{.Tag}}", pattern.trim()])
                .output();
            
            if let Ok(output) = output {
                let images = String::from_utf8_lossy(&output.stdout);
                for image in images.lines() {
                    if image != "<none>:<none>" {
                        let _ = Command::new("docker")
                            .args(&["rmi", "-f", image])
                            .status();
                    }
                }
                println!("✅ Removed matching images");
            }
        }
        3 => {
            // Remove old versions
            println!("🔍 Finding duplicate image versions...");
            
            let output = Command::new("docker")
                .args(&["images", "--format", "{{.Repository}}:{{.Tag}}"])
                .output();
            
            if let Ok(output) = output {
                let images = String::from_utf8_lossy(&output.stdout);
                let mut image_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                
                for image in images.lines() {
                    if let Some(repo) = image.split(':').next() {
                        image_map.entry(repo.to_string())
                            .or_insert_with(Vec::new)
                            .push(image.to_string());
                    }
                }
                
                // Remove all but latest
                for (repo, versions) in image_map {
                    if versions.len() > 1 {
                        println!("Found {} versions of {}", versions.len(), repo);
                        // Keep latest (usually first in list)
                        for version in versions.iter().skip(1) {
                            let _ = Command::new("docker")
                                .args(&["rmi", "-f", version])
                                .status();
                        }
                    }
                }
                println!("✅ Removed old image versions");
            }
        }
        4 => {
            // Remove large images
            println!("🔍 Finding large images (>500MB)...");
            
            let output = Command::new("docker")
                .args(&["images", "--format", "table {{.Repository}}:{{.Tag}}\t{{.Size}}"])
                .output();
            
            if let Ok(output) = output {
                let images = String::from_utf8_lossy(&output.stdout);
                println!("{}", images);
                
                if Confirm::new()
                    .with_prompt("Remove images larger than 500MB?")
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    // Parse and remove large images
                    for line in images.lines().skip(1) {
                        if line.contains("GB") || (line.contains("MB") && !line.contains("MB")) {
                            if let Some(image) = line.split_whitespace().next() {
                                let _ = Command::new("docker")
                                    .args(&["rmi", "-f", image])
                                    .status();
                            }
                        }
                    }
                    println!("✅ Removed large images");
                }
            }
        }
        _ => {}
    }
}

fn volume_cleanup() {
    println!("💾 Volume Cleanup\n");
    
    let options = vec![
        "Remove all unused volumes",
        "Remove anonymous volumes",
        "Remove volumes by pattern",
        "Remove orphaned volumes",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            let _ = Command::new("docker")
                .args(&["volume", "prune", "-f"])
                .status();
            println!("✅ Removed unused volumes");
        }
        1 => {
            // Remove anonymous volumes
            let output = Command::new("docker")
                .args(&["volume", "ls", "-q", "--filter", "dangling=true"])
                .output();
            
            if let Ok(output) = output {
                let volumes = String::from_utf8_lossy(&output.stdout);
                for volume in volumes.lines() {
                    // Check if volume name is a hash (anonymous)
                    if volume.len() == 64 && volume.chars().all(|c| c.is_ascii_hexdigit()) {
                        let _ = Command::new("docker")
                            .args(&["volume", "rm", "-f", volume])
                            .status();
                    }
                }
                println!("✅ Removed anonymous volumes");
            }
        }
        2 => {
            println!("Enter volume pattern:");
            let mut pattern = String::new();
            std::io::stdin().read_line(&mut pattern).ok();
            
            let output = Command::new("docker")
                .args(&["volume", "ls", "-q"])
                .output();
            
            if let Ok(output) = output {
                let volumes = String::from_utf8_lossy(&output.stdout);
                for volume in volumes.lines() {
                    if volume.contains(pattern.trim()) {
                        let _ = Command::new("docker")
                            .args(&["volume", "rm", "-f", volume])
                            .status();
                    }
                }
                println!("✅ Removed matching volumes");
            }
        }
        3 => {
            // Find orphaned volumes
            println!("🔍 Finding orphaned volumes...");
            
            let output = Command::new("docker")
                .args(&["volume", "ls", "-q", "--filter", "dangling=true"])
                .output();
            
            if let Ok(output) = output {
                let volumes = String::from_utf8_lossy(&output.stdout);
                let orphan_count = volumes.lines().count();
                
                if orphan_count > 0 {
                    println!("Found {} orphaned volumes", orphan_count);
                    
                    if Confirm::new()
                        .with_prompt("Remove orphaned volumes?")
                        .default(false)
                        .interact()
                        .unwrap()
                    {
                        for volume in volumes.lines() {
                            let _ = Command::new("docker")
                                .args(&["volume", "rm", "-f", volume])
                                .status();
                        }
                        println!("✅ Removed orphaned volumes");
                    }
                } else {
                    println!("No orphaned volumes found");
                }
            }
        }
        _ => {}
    }
}

fn network_cleanup() {
    println!("🌐 Network Cleanup\n");
    
    let options = vec![
        "Remove unused networks",
        "Remove custom networks",
        "Fix network conflicts",
        "Reset to default networks",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            let _ = Command::new("docker")
                .args(&["network", "prune", "-f"])
                .status();
            println!("✅ Removed unused networks");
        }
        1 => {
            // Remove custom networks (not default ones)
            let output = Command::new("docker")
                .args(&["network", "ls", "--format", "{{.Name}}"])
                .output();
            
            if let Ok(output) = output {
                let networks = String::from_utf8_lossy(&output.stdout);
                for network in networks.lines() {
                    if !["bridge", "host", "none"].contains(&network) {
                        println!("Remove network '{}'?", network);
                        if Confirm::new()
                            .default(false)
                            .interact()
                            .unwrap()
                        {
                            let _ = Command::new("docker")
                                .args(&["network", "rm", network])
                                .status();
                        }
                    }
                }
            }
        }
        2 => {
            fix_network_conflicts();
        }
        3 => {
            reset_docker_networks();
        }
        _ => {}
    }
}

fn fix_network_conflicts() {
    println!("🔧 Fixing network conflicts...");
    
    // Check for subnet conflicts
    let output = Command::new("docker")
        .args(&["network", "ls", "-q"])
        .output();
    
    if let Ok(output) = output {
        let networks = String::from_utf8_lossy(&output.stdout);
        let mut subnets = Vec::new();
        
        for network in networks.lines() {
            let inspect = Command::new("docker")
                .args(&["network", "inspect", network, "--format", "{{range .IPAM.Config}}{{.Subnet}}{{end}}"])
                .output();
            
            if let Ok(inspect) = inspect {
                let subnet = String::from_utf8_lossy(&inspect.stdout);
                if !subnet.trim().is_empty() {
                    subnets.push((network.to_string(), subnet.trim().to_string()));
                }
            }
        }
        
        // Check for duplicates
        for i in 0..subnets.len() {
            for j in i+1..subnets.len() {
                if subnets[i].1 == subnets[j].1 {
                    println!("⚠️  Conflict found: {} and {} use subnet {}", 
                        subnets[i].0, subnets[j].0, subnets[i].1);
                }
            }
        }
        
        if subnets.is_empty() {
            println!("✅ No network conflicts found");
        }
    }
}

fn reset_docker_networks() {
    println!("⚠️  This will reset Docker networks to defaults!");
    
    if !Confirm::new()
        .with_prompt("Continue?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    // Stop all containers
    let _ = Command::new("docker")
        .args(&["stop", "$(docker ps -aq)"])
        .status();
    
    // Remove all custom networks
    let _ = Command::new("docker")
        .args(&["network", "prune", "-f"])
        .status();
    
    // Restart Docker daemon
    let _ = Command::new("systemctl")
        .args(&["restart", "docker"])
        .status();
    
    println!("✅ Docker networks reset to defaults");
}

fn build_cache_cleanup() {
    println!("🏗️  Build Cache Cleanup\n");
    
    // Show current cache usage
    let _ = Command::new("docker")
        .args(&["builder", "du"])
        .status();
    
    println!("\nCleanup options:");
    let options = vec![
        "Remove all build cache",
        "Remove cache older than 7 days",
        "Remove cache older than 30 days",
        "Keep only recent builds",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            let _ = Command::new("docker")
                .args(&["builder", "prune", "-af"])
                .status();
            println!("✅ Removed all build cache");
        }
        1 => {
            let _ = Command::new("docker")
                .args(&["builder", "prune", "-af", "--filter", "until=168h"])
                .status();
            println!("✅ Removed cache older than 7 days");
        }
        2 => {
            let _ = Command::new("docker")
                .args(&["builder", "prune", "-af", "--filter", "until=720h"])
                .status();
            println!("✅ Removed cache older than 30 days");
        }
        3 => {
            let _ = Command::new("docker")
                .args(&["builder", "prune", "-f", "--keep-storage", "10GB"])
                .status();
            println!("✅ Kept only recent builds (max 10GB)");
        }
        _ => {}
    }
}

fn log_cleanup() {
    println!("📝 Log Cleanup\n");
    
    // Find containers with large logs
    println!("🔍 Finding containers with large logs...\n");
    
    let output = Command::new("docker")
        .args(&["ps", "-aq"])
        .output();
    
    if let Ok(output) = output {
        let containers = String::from_utf8_lossy(&output.stdout);
        let mut large_logs = Vec::new();
        
        for container in containers.lines() {
            if !container.is_empty() {
                // Get log file path
                let inspect = Command::new("docker")
                    .args(&["inspect", "--format", "{{.LogPath}}", container])
                    .output();
                
                if let Ok(inspect) = inspect {
                    let log_path = String::from_utf8_lossy(&inspect.stdout);
                    let log_path = log_path.trim();
                    
                    // Check file size
                    if let Ok(metadata) = std::fs::metadata(log_path) {
                        let size_mb = metadata.len() / (1024 * 1024);
                        if size_mb > 100 {
                            large_logs.push((container.to_string(), size_mb));
                            println!("  Container {} has {}MB of logs", container, size_mb);
                        }
                    }
                }
            }
        }
        
        if !large_logs.is_empty() {
            if Confirm::new()
                .with_prompt("Truncate large log files?")
                .default(true)
                .interact()
                .unwrap()
            {
                for (container, _) in large_logs {
                    // Truncate logs
                    let _ = Command::new("truncate")
                        .args(&["-s", "0", &format!("/var/lib/docker/containers/{}/*.log", container)])
                        .status();
                }
                println!("✅ Truncated large log files");
            }
        } else {
            println!("No large log files found");
        }
    }
}

fn custom_cleanup() {
    println!("🎯 Custom Cleanup\n");
    
    let options = vec![
        "Stopped containers",
        "Dangling images",
        "Unused images",
        "Unused volumes",
        "Unused networks",
        "Build cache",
    ];
    
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select items to clean")
        .items(&options)
        .interact()
        .unwrap();
    
    if selections.is_empty() {
        println!("No items selected");
        return;
    }
    
    println!("\n🗑️  Starting custom cleanup...");
    
    for idx in selections {
        match idx {
            0 => {
                println!("Removing stopped containers...");
                let _ = Command::new("docker")
                    .args(&["container", "prune", "-f"])
                    .status();
            }
            1 => {
                println!("Removing dangling images...");
                let _ = Command::new("docker")
                    .args(&["image", "prune", "-f"])
                    .status();
            }
            2 => {
                println!("Removing unused images...");
                let _ = Command::new("docker")
                    .args(&["image", "prune", "-af"])
                    .status();
            }
            3 => {
                println!("Removing unused volumes...");
                let _ = Command::new("docker")
                    .args(&["volume", "prune", "-f"])
                    .status();
            }
            4 => {
                println!("Removing unused networks...");
                let _ = Command::new("docker")
                    .args(&["network", "prune", "-f"])
                    .status();
            }
            5 => {
                println!("Cleaning build cache...");
                let _ = Command::new("docker")
                    .args(&["builder", "prune", "-af"])
                    .status();
            }
            _ => {}
        }
    }
    
    println!("\n✅ Custom cleanup complete!");
}

fn show_docker_usage() {
    println!("\n📊 Docker Disk Usage:");
    let _ = Command::new("docker")
        .args(&["system", "df"])
        .status();
}