use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};
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

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🧹 Docker Cleanup Tools")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

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

    let proceed = match Confirm::new()
        .with_prompt("Proceed with quick cleanup?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(p)) => p,
        Ok(None) | Err(_) => return,
    };
    if !proceed {
        return;
    }

    // Remove stopped containers
    println!("\n🗑️  Removing stopped containers...");
    match Command::new("docker")
        .args(["container", "prune", "-f"])
        .status()
    {
        Ok(s) if s.success() => println!("  Done"),
        Ok(_) => println!("  Warning: container prune returned non-zero exit"),
        Err(e) => println!("  Error: {}", e),
    }

    // Remove dangling images
    println!("🗑️  Removing dangling images...");
    match Command::new("docker")
        .args(["image", "prune", "-f"])
        .status()
    {
        Ok(s) if s.success() => println!("  Done"),
        Ok(_) => println!("  Warning: image prune returned non-zero exit"),
        Err(e) => println!("  Error: {}", e),
    }

    // Remove unused networks
    println!("🗑️  Removing unused networks...");
    match Command::new("docker")
        .args(["network", "prune", "-f"])
        .status()
    {
        Ok(s) if s.success() => println!("  Done"),
        Ok(_) => println!("  Warning: network prune returned non-zero exit"),
        Err(e) => println!("  Error: {}", e),
    }

    // Remove dangling build cache
    println!("🗑️  Cleaning build cache...");
    match Command::new("docker")
        .args(["builder", "prune", "-f"])
        .status()
    {
        Ok(s) if s.success() => println!("  Done"),
        Ok(_) => println!("  Warning: builder prune returned non-zero exit"),
        Err(e) => println!("  Error: {}", e),
    }

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

    let confirm1 = match Confirm::new()
        .with_prompt("This is DESTRUCTIVE. Continue?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };
    if !confirm1 {
        return;
    }

    let confirm2 = match Confirm::new()
        .with_prompt("Are you REALLY sure?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };
    if !confirm2 {
        return;
    }

    // Full system prune
    println!("\n🗑️  Running full system prune...");
    match Command::new("docker")
        .args(["system", "prune", "-af", "--volumes"])
        .status()
    {
        Ok(s) if s.success() => println!("\n✅ Deep cleanup complete!"),
        Ok(_) => println!("\n⚠️  Deep cleanup completed with warnings"),
        Err(e) => println!("\n❌ Deep cleanup failed: {}", e),
    }
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            match Command::new("docker")
                .args(["container", "prune", "-f"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed all stopped containers"),
                Ok(_) => println!("⚠️  Container prune completed with warnings"),
                Err(e) => println!("❌ Failed to remove containers: {}", e),
            }
        }
        1 => {
            // Remove exited containers
            let output = Command::new("docker")
                .args(["ps", "-aq", "--filter", "status=exited"])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let containers = String::from_utf8_lossy(&output.stdout);
                    if !containers.trim().is_empty() {
                        let mut removed = 0;
                        let mut failed = 0;
                        for container in containers.lines() {
                            if container.trim().is_empty() {
                                continue;
                            }
                            // Validate container ID before using
                            if let Err(_) = crate::docker::validate_container_name(container) {
                                continue;
                            }
                            match Command::new("docker").args(["rm", container]).status() {
                                Ok(s) if s.success() => removed += 1,
                                _ => failed += 1,
                            }
                        }
                        println!(
                            "✅ Removed {} exited containers ({} failed)",
                            removed, failed
                        );
                    } else {
                        println!("No exited containers found");
                    }
                }
                Ok(_) => println!("⚠️  Failed to list exited containers"),
                Err(e) => println!("❌ Error listing containers: {}", e),
            }
        }
        2 => {
            // Remove by age
            println!("Remove containers older than (e.g., 24h, 7d):");
            let mut age = String::new();
            if std::io::stdin().read_line(&mut age).is_err() {
                println!("❌ Failed to read input");
                return;
            }

            // Validate the duration format
            if let Err(e) = crate::docker::validate_duration_filter(age.trim()) {
                println!("❌ Invalid duration format: {}", e);
                return;
            }

            match Command::new("docker")
                .args([
                    "container",
                    "prune",
                    "-f",
                    "--filter",
                    &format!("until={}", age.trim()),
                ])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed old containers"),
                Ok(_) => println!("⚠️  Container prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        3 => {
            // Remove by pattern
            println!("Enter container name pattern:");
            let mut pattern = String::new();
            if std::io::stdin().read_line(&mut pattern).is_err() {
                println!("❌ Failed to read input");
                return;
            }

            let pattern = pattern.trim();
            // Validate pattern doesn't contain shell injection characters
            if pattern.contains(|c: char| {
                matches!(
                    c,
                    '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r'
                )
            }) {
                println!("❌ Pattern contains invalid characters");
                return;
            }

            let output = Command::new("docker")
                .args(["ps", "-aq", "--filter", &format!("name={}", pattern)])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let containers = String::from_utf8_lossy(&output.stdout);
                    let mut removed = 0;
                    for container in containers.lines() {
                        if container.trim().is_empty() {
                            continue;
                        }
                        if let Err(_) = crate::docker::validate_container_name(container) {
                            continue;
                        }
                        if Command::new("docker")
                            .args(["rm", "-f", container])
                            .status()
                            .map(|s| s.success())
                            .unwrap_or(false)
                        {
                            removed += 1;
                        }
                    }
                    println!("✅ Removed {} matching containers", removed);
                }
                Ok(_) => println!("⚠️  Failed to find matching containers"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        4 => {
            // Remove orphaned containers
            println!("🔍 Finding orphaned containers...");

            // Get all container IDs
            let all_containers = Command::new("docker").args(["ps", "-aq"]).output();

            // Get containers in docker-compose projects
            let compose_containers = Command::new("docker")
                .args(["ps", "-aq", "--filter", "label=com.docker.compose.project"])
                .output();

            // Find orphans (containers not in compose projects)
            if let (Ok(all), Ok(compose)) = (all_containers, compose_containers) {
                let all_ids: Vec<&str> = std::str::from_utf8(&all.stdout)
                    .unwrap_or("")
                    .lines()
                    .collect();
                let compose_ids: Vec<&str> = std::str::from_utf8(&compose.stdout)
                    .unwrap_or("")
                    .lines()
                    .collect();

                let orphans: Vec<&str> = all_ids
                    .into_iter()
                    .filter(|id| !compose_ids.contains(id) && !id.is_empty())
                    .collect();

                if !orphans.is_empty() {
                    println!("Found {} orphaned containers", orphans.len());
                    let remove = Confirm::new()
                        .with_prompt("Remove orphaned containers?")
                        .default(false)
                        .interact_opt()
                        .ok()
                        .flatten()
                        .unwrap_or(false);
                    if remove {
                        let mut removed = 0;
                        let mut failed = 0;
                        for id in orphans {
                            // Validate container ID
                            if crate::docker::validate_container_name(id).is_err() {
                                continue;
                            }
                            match Command::new("docker").args(["rm", "-f", id]).status() {
                                Ok(s) if s.success() => removed += 1,
                                _ => failed += 1,
                            }
                        }
                        println!(
                            "✅ Removed {} orphaned containers ({} failed)",
                            removed, failed
                        );
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            match Command::new("docker")
                .args(["image", "prune", "-f"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed dangling images"),
                Ok(_) => println!("⚠️  Image prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        1 => {
            match Command::new("docker")
                .args(["image", "prune", "-af"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed unused images"),
                Ok(_) => println!("⚠️  Image prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        2 => {
            println!("Enter image pattern (e.g., 'redis:*'):");
            let mut pattern = String::new();
            if std::io::stdin().read_line(&mut pattern).is_err() {
                println!("❌ Failed to read input");
                return;
            }

            let pattern = pattern.trim();
            // Validate pattern
            if crate::docker::validate_image_name(pattern).is_err() && !pattern.contains('*') {
                println!("❌ Invalid image pattern");
                return;
            }

            let output = Command::new("docker")
                .args(["images", "--format", "{{.Repository}}:{{.Tag}}", pattern])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let images = String::from_utf8_lossy(&output.stdout);
                    let mut removed = 0;
                    for image in images.lines() {
                        if image != "<none>:<none>" && !image.trim().is_empty() {
                            if crate::docker::validate_image_name(image).is_err() {
                                continue;
                            }
                            if Command::new("docker")
                                .args(["rmi", "-f", image])
                                .status()
                                .map(|s| s.success())
                                .unwrap_or(false)
                            {
                                removed += 1;
                            }
                        }
                    }
                    println!("✅ Removed {} matching images", removed);
                }
                Ok(_) => println!("⚠️  Failed to list images"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        3 => {
            // Remove old versions
            println!("🔍 Finding duplicate image versions...");

            let output = Command::new("docker")
                .args(["images", "--format", "{{.Repository}}:{{.Tag}}"])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let images = String::from_utf8_lossy(&output.stdout);
                    let mut image_map: std::collections::HashMap<String, Vec<String>> =
                        std::collections::HashMap::new();

                    for image in images.lines() {
                        if crate::docker::validate_image_name(image).is_err() {
                            continue;
                        }
                        if let Some(repo) = image.split(':').next() {
                            image_map
                                .entry(repo.to_string())
                                .or_default()
                                .push(image.to_string());
                        }
                    }

                    // Remove all but latest
                    let mut total_removed = 0;
                    for (repo, versions) in image_map {
                        if versions.len() > 1 {
                            println!("Found {} versions of {}", versions.len(), repo);
                            // Keep latest (usually first in list)
                            for version in versions.iter().skip(1) {
                                if Command::new("docker")
                                    .args(["rmi", "-f", version])
                                    .status()
                                    .map(|s| s.success())
                                    .unwrap_or(false)
                                {
                                    total_removed += 1;
                                }
                            }
                        }
                    }
                    println!("✅ Removed {} old image versions", total_removed);
                }
                Ok(_) => println!("⚠️  Failed to list images"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        4 => {
            // Remove large images
            println!("🔍 Finding large images (>500MB)...");

            let output = Command::new("docker")
                .args([
                    "images",
                    "--format",
                    "table {{.Repository}}:{{.Tag}}\t{{.Size}}",
                ])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let images = String::from_utf8_lossy(&output.stdout);
                    println!("{}", images);

                    let remove = Confirm::new()
                        .with_prompt("Remove images larger than 500MB?")
                        .default(false)
                        .interact_opt()
                        .ok()
                        .flatten()
                        .unwrap_or(false);
                    if remove {
                        let mut removed = 0;
                        // Parse and remove large images
                        for line in images.lines().skip(1) {
                            if (line.contains("GB")
                                || (line.contains("MB") && !line.contains("MB")))
                                && let Some(image) = line.split_whitespace().next()
                            {
                                if crate::docker::validate_image_name(image).is_err() {
                                    continue;
                                }
                                if Command::new("docker")
                                    .args(["rmi", "-f", image])
                                    .status()
                                    .map(|s| s.success())
                                    .unwrap_or(false)
                                {
                                    removed += 1;
                                }
                            }
                        }
                        println!("✅ Removed {} large images", removed);
                    }
                }
                Ok(_) => println!("⚠️  Failed to list images"),
                Err(e) => println!("❌ Error: {}", e),
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            match Command::new("docker")
                .args(["volume", "prune", "-f"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed unused volumes"),
                Ok(_) => println!("⚠️  Volume prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        1 => {
            // Remove anonymous volumes
            let output = Command::new("docker")
                .args(["volume", "ls", "-q", "--filter", "dangling=true"])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let volumes = String::from_utf8_lossy(&output.stdout);
                    let mut removed = 0;
                    for volume in volumes.lines() {
                        // Check if volume name is a hash (anonymous)
                        if volume.len() == 64 && volume.chars().all(|c| c.is_ascii_hexdigit()) {
                            if Command::new("docker")
                                .args(["volume", "rm", "-f", volume])
                                .status()
                                .map(|s| s.success())
                                .unwrap_or(false)
                            {
                                removed += 1;
                            }
                        }
                    }
                    println!("✅ Removed {} anonymous volumes", removed);
                }
                Ok(_) => println!("⚠️  Failed to list volumes"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        2 => {
            println!("Enter volume pattern:");
            let mut pattern = String::new();
            if std::io::stdin().read_line(&mut pattern).is_err() {
                println!("❌ Failed to read input");
                return;
            }

            let pattern = pattern.trim();
            // Validate pattern doesn't contain shell injection characters
            if pattern.contains(|c: char| {
                matches!(
                    c,
                    '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r'
                )
            }) {
                println!("❌ Pattern contains invalid characters");
                return;
            }

            let output = Command::new("docker").args(["volume", "ls", "-q"]).output();

            match output {
                Ok(output) if output.status.success() => {
                    let volumes = String::from_utf8_lossy(&output.stdout);
                    let mut removed = 0;
                    for volume in volumes.lines() {
                        if volume.contains(pattern) {
                            if Command::new("docker")
                                .args(["volume", "rm", "-f", volume])
                                .status()
                                .map(|s| s.success())
                                .unwrap_or(false)
                            {
                                removed += 1;
                            }
                        }
                    }
                    println!("✅ Removed {} matching volumes", removed);
                }
                Ok(_) => println!("⚠️  Failed to list volumes"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        3 => {
            // Find orphaned volumes
            println!("🔍 Finding orphaned volumes...");

            let output = Command::new("docker")
                .args(["volume", "ls", "-q", "--filter", "dangling=true"])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let volumes = String::from_utf8_lossy(&output.stdout);
                    let volume_list: Vec<&str> =
                        volumes.lines().filter(|l| !l.is_empty()).collect();
                    let orphan_count = volume_list.len();

                    if orphan_count > 0 {
                        println!("Found {} orphaned volumes", orphan_count);

                        let remove = Confirm::new()
                            .with_prompt("Remove orphaned volumes?")
                            .default(false)
                            .interact_opt()
                            .ok()
                            .flatten()
                            .unwrap_or(false);
                        if remove {
                            let mut removed = 0;
                            for volume in volume_list {
                                if Command::new("docker")
                                    .args(["volume", "rm", "-f", volume])
                                    .status()
                                    .map(|s| s.success())
                                    .unwrap_or(false)
                                {
                                    removed += 1;
                                }
                            }
                            println!("✅ Removed {} orphaned volumes", removed);
                        }
                    } else {
                        println!("No orphaned volumes found");
                    }
                }
                Ok(_) => println!("⚠️  Failed to list volumes"),
                Err(e) => println!("❌ Error: {}", e),
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            match Command::new("docker")
                .args(["network", "prune", "-f"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed unused networks"),
                Ok(_) => println!("⚠️  Network prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        1 => {
            // Remove custom networks (not default ones)
            let output = Command::new("docker")
                .args(["network", "ls", "--format", "{{.Name}}"])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let networks = String::from_utf8_lossy(&output.stdout);
                    for network in networks.lines() {
                        if !["bridge", "host", "none"].contains(&network) && !network.is_empty() {
                            println!("Remove network '{}'?", network);
                            if Confirm::new()
                                .default(false)
                                .interact_opt()
                                .ok()
                                .flatten()
                                .unwrap_or(false)
                            {
                                match Command::new("docker")
                                    .args(["network", "rm", network])
                                    .status()
                                {
                                    Ok(s) if s.success() => println!("  Removed"),
                                    Ok(_) => {
                                        println!("  Warning: could not remove (may be in use)")
                                    }
                                    Err(e) => println!("  Error: {}", e),
                                }
                            }
                        }
                    }
                }
                Ok(_) => println!("⚠️  Failed to list networks"),
                Err(e) => println!("❌ Error: {}", e),
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
        .args(["network", "ls", "-q"])
        .output();

    if let Ok(output) = output {
        let networks = String::from_utf8_lossy(&output.stdout);
        let mut subnets = Vec::new();

        for network in networks.lines() {
            let inspect = Command::new("docker")
                .args([
                    "network",
                    "inspect",
                    network,
                    "--format",
                    "{{range .IPAM.Config}}{{.Subnet}}{{end}}",
                ])
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
            for j in i + 1..subnets.len() {
                if subnets[i].1 == subnets[j].1 {
                    println!(
                        "⚠️  Conflict found: {} and {} use subnet {}",
                        subnets[i].0, subnets[j].0, subnets[i].1
                    );
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

    let cont = Confirm::new()
        .with_prompt("Continue?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);
    if !cont {
        return;
    }

    // Stop all containers - get list first, then stop each
    println!("🛑 Stopping all containers...");
    let containers_output = Command::new("docker").args(["ps", "-aq"]).output();

    if let Ok(output) = containers_output {
        if output.status.success() {
            let containers = String::from_utf8_lossy(&output.stdout);
            for container in containers.lines() {
                if !container.trim().is_empty() {
                    if let Err(e) = Command::new("docker")
                        .args(["stop", container.trim()])
                        .status()
                    {
                        println!("  Warning: could not stop {}: {}", container, e);
                    }
                }
            }
        }
    }

    // Remove all custom networks
    println!("🔗 Pruning networks...");
    match Command::new("docker")
        .args(["network", "prune", "-f"])
        .status()
    {
        Ok(s) if s.success() => println!("  Networks pruned"),
        Ok(_) => println!("  Warning: network prune had issues"),
        Err(e) => println!("  Error pruning networks: {}", e),
    }

    // Restart Docker daemon
    println!("🔄 Restarting Docker daemon...");
    match Command::new("systemctl")
        .args(["restart", "docker"])
        .status()
    {
        Ok(s) if s.success() => println!("✅ Docker networks reset to defaults"),
        Ok(_) => println!("⚠️  Docker restart returned non-zero exit"),
        Err(e) => println!("❌ Failed to restart Docker: {}", e),
    }
}

fn build_cache_cleanup() {
    println!("🏗️  Build Cache Cleanup\n");

    // Show current cache usage
    if let Err(e) = Command::new("docker").args(["builder", "du"]).status() {
        println!("Warning: Could not get cache usage: {}", e);
    }

    println!("\nCleanup options:");
    let options = vec![
        "Remove all build cache",
        "Remove cache older than 7 days",
        "Remove cache older than 30 days",
        "Keep only recent builds",
        "Back",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup type")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            match Command::new("docker")
                .args(["builder", "prune", "-af"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed all build cache"),
                Ok(_) => println!("⚠️  Build cache prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        1 => {
            match Command::new("docker")
                .args(["builder", "prune", "-af", "--filter", "until=168h"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed cache older than 7 days"),
                Ok(_) => println!("⚠️  Build cache prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        2 => {
            match Command::new("docker")
                .args(["builder", "prune", "-af", "--filter", "until=720h"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Removed cache older than 30 days"),
                Ok(_) => println!("⚠️  Build cache prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        3 => {
            match Command::new("docker")
                .args(["builder", "prune", "-f", "--keep-storage", "10GB"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Kept only recent builds (max 10GB)"),
                Ok(_) => println!("⚠️  Build cache prune completed with warnings"),
                Err(e) => println!("❌ Failed: {}", e),
            }
        }
        _ => {}
    }
}

fn log_cleanup() {
    println!("📝 Log Cleanup\n");

    // Find containers with large logs
    println!("🔍 Finding containers with large logs...\n");

    let output = Command::new("docker").args(["ps", "-aq"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let containers = String::from_utf8_lossy(&output.stdout);
            let mut large_logs: Vec<(String, String, u64)> = Vec::new(); // (container_id, log_path, size_mb)

            for container in containers.lines() {
                if container.is_empty() {
                    continue;
                }
                // Validate container ID
                if crate::docker::validate_container_name(container).is_err() {
                    continue;
                }

                // Get log file path
                let inspect = Command::new("docker")
                    .args(["inspect", "--format", "{{.LogPath}}", container])
                    .output();

                if let Ok(inspect) = inspect {
                    if !inspect.status.success() {
                        continue;
                    }
                    let log_path = String::from_utf8_lossy(&inspect.stdout);
                    let log_path = log_path.trim();

                    // Validate log path - must be under /var/lib/docker
                    if !log_path.starts_with("/var/lib/docker/") {
                        continue;
                    }

                    // Check file size
                    if let Ok(metadata) = std::fs::metadata(log_path) {
                        let size_mb = metadata.len() / (1024 * 1024);
                        if size_mb > 100 {
                            large_logs.push((container.to_string(), log_path.to_string(), size_mb));
                            println!("  Container {} has {}MB of logs", container, size_mb);
                        }
                    }
                }
            }

            if !large_logs.is_empty() {
                let truncate = Confirm::new()
                    .with_prompt("Truncate large log files?")
                    .default(true)
                    .interact_opt()
                    .ok()
                    .flatten()
                    .unwrap_or(false);
                if truncate {
                    let mut truncated = 0;
                    for (_container, log_path, _) in large_logs {
                        // Truncate specific log file directly (not using glob pattern)
                        match Command::new("truncate")
                            .args(["-s", "0", &log_path])
                            .status()
                        {
                            Ok(s) if s.success() => truncated += 1,
                            Ok(_) => println!("  Warning: could not truncate {}", log_path),
                            Err(e) => println!("  Error truncating {}: {}", log_path, e),
                        }
                    }
                    println!("✅ Truncated {} large log files", truncated);
                }
            } else {
                println!("No large log files found");
            }
        }
        Ok(_) => println!("⚠️  Failed to list containers"),
        Err(e) => println!("❌ Error: {}", e),
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

    let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select items to clean")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if selections.is_empty() {
        println!("No items selected");
        return;
    }

    println!("\n🗑️  Starting custom cleanup...");

    for idx in selections {
        match idx {
            0 => {
                print!("Removing stopped containers... ");
                match Command::new("docker")
                    .args(["container", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            1 => {
                print!("Removing dangling images... ");
                match Command::new("docker")
                    .args(["image", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            2 => {
                print!("Removing unused images... ");
                match Command::new("docker")
                    .args(["image", "prune", "-af"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            3 => {
                print!("Removing unused volumes... ");
                match Command::new("docker")
                    .args(["volume", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            4 => {
                print!("Removing unused networks... ");
                match Command::new("docker")
                    .args(["network", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            5 => {
                print!("Cleaning build cache... ");
                match Command::new("docker")
                    .args(["builder", "prune", "-af"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    _ => println!("Warning"),
                }
            }
            _ => {}
        }
    }

    println!("\n✅ Custom cleanup complete!");
}

fn show_docker_usage() {
    println!("\n📊 Docker Disk Usage:");
    if let Err(e) = Command::new("docker").args(["system", "df"]).status() {
        println!("  Could not get disk usage: {}", e);
    }
}

/// Parse docker system df output to extract usage information
pub fn parse_docker_df_output(output: &str) -> DockerDiskUsage {
    let mut usage = DockerDiskUsage::default();

    for line in output.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let type_name = parts[0];
            let total = parts[1].parse().unwrap_or(0);
            let active = parts[2].parse().unwrap_or(0);
            let size = parts[3].to_string();

            match type_name {
                "Images" => {
                    usage.images_total = total;
                    usage.images_active = active;
                    usage.images_size = size;
                }
                "Containers" => {
                    usage.containers_total = total;
                    usage.containers_active = active;
                    usage.containers_size = size;
                }
                "Local" => {
                    // "Local Volumes" - next part is "Volumes"
                    usage.volumes_total = active; // Shifted due to "Local Volumes"
                    if parts.len() >= 5 {
                        usage.volumes_size = parts[4].to_string();
                    }
                }
                "Build" => {
                    // Build cache
                    if parts.len() >= 4 {
                        usage.build_cache_size = parts[3].to_string();
                    }
                }
                _ => {}
            }
        }
    }

    usage
}

/// Docker disk usage statistics
#[derive(Debug, Default, Clone)]
pub struct DockerDiskUsage {
    pub images_total: u32,
    pub images_active: u32,
    pub images_size: String,
    pub containers_total: u32,
    pub containers_active: u32,
    pub containers_size: String,
    pub volumes_total: u32,
    pub volumes_size: String,
    pub build_cache_size: String,
}

/// Check if a container ID is a valid Docker container ID format
pub fn is_valid_container_id(id: &str) -> bool {
    // Docker container IDs are either 12 or 64 hex characters
    let id = id.trim();
    (id.len() == 12 || id.len() == 64) && id.chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if a volume name appears to be anonymous (64 char hex)
pub fn is_anonymous_volume(name: &str) -> bool {
    let name = name.trim();
    name.len() == 64 && name.chars().all(|c| c.is_ascii_hexdigit())
}

/// Parse container age from Docker output format
pub fn parse_container_age(age_str: &str) -> Option<std::time::Duration> {
    // Parse formats like "2 hours ago", "3 days ago", "4 weeks ago"
    let parts: Vec<&str> = age_str.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let value: u64 = parts[0].parse().ok()?;
    let unit = parts[1].to_lowercase();

    let seconds = match unit.as_str() {
        "second" | "seconds" => value,
        "minute" | "minutes" => value * 60,
        "hour" | "hours" => value * 3600,
        "day" | "days" => value * 86400,
        "week" | "weeks" => value * 604800,
        "month" | "months" => value * 2592000,
        "year" | "years" => value * 31536000,
        _ => return None,
    };

    Some(std::time::Duration::from_secs(seconds))
}

/// Format bytes to human readable size
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2}TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_container_id_short() {
        assert!(is_valid_container_id("a1b2c3d4e5f6"));
        assert!(is_valid_container_id("0123456789ab"));
    }

    #[test]
    fn test_is_valid_container_id_long() {
        let long_id = "a".repeat(64);
        assert!(is_valid_container_id(&long_id));
    }

    #[test]
    fn test_is_valid_container_id_invalid() {
        assert!(!is_valid_container_id(""));
        assert!(!is_valid_container_id("short"));
        assert!(!is_valid_container_id("ghijklmnopqr")); // not hex
        assert!(!is_valid_container_id("a1b2c3d4e5f")); // 11 chars
        assert!(!is_valid_container_id("a1b2c3d4e5f67")); // 13 chars
    }

    #[test]
    fn test_is_anonymous_volume() {
        let anon_vol = "a".repeat(64);
        assert!(is_anonymous_volume(&anon_vol));

        let mixed_hex = "0123456789abcdef".repeat(4);
        assert!(is_anonymous_volume(&mixed_hex));
    }

    #[test]
    fn test_is_anonymous_volume_named() {
        assert!(!is_anonymous_volume("my-volume"));
        assert!(!is_anonymous_volume("postgres_data"));
        assert!(!is_anonymous_volume(""));
    }

    #[test]
    fn test_parse_container_age_seconds() {
        let age = parse_container_age("30 seconds ago");
        assert_eq!(age, Some(std::time::Duration::from_secs(30)));
    }

    #[test]
    fn test_parse_container_age_minutes() {
        let age = parse_container_age("5 minutes ago");
        assert_eq!(age, Some(std::time::Duration::from_secs(300)));
    }

    #[test]
    fn test_parse_container_age_hours() {
        let age = parse_container_age("2 hours ago");
        assert_eq!(age, Some(std::time::Duration::from_secs(7200)));
    }

    #[test]
    fn test_parse_container_age_days() {
        let age = parse_container_age("7 days ago");
        assert_eq!(age, Some(std::time::Duration::from_secs(604800)));
    }

    #[test]
    fn test_parse_container_age_weeks() {
        let age = parse_container_age("2 weeks ago");
        assert_eq!(age, Some(std::time::Duration::from_secs(1209600)));
    }

    #[test]
    fn test_parse_container_age_invalid() {
        assert_eq!(parse_container_age(""), None);
        assert_eq!(parse_container_age("invalid"), None);
        assert_eq!(parse_container_age("five minutes"), None);
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(100), "100B");
        assert_eq!(format_size(0), "0B");
    }

    #[test]
    fn test_format_size_kilobytes() {
        assert_eq!(format_size(1024), "1.00KB");
        assert_eq!(format_size(2048), "2.00KB");
    }

    #[test]
    fn test_format_size_megabytes() {
        assert_eq!(format_size(1024 * 1024), "1.00MB");
        assert_eq!(format_size(5 * 1024 * 1024), "5.00MB");
    }

    #[test]
    fn test_format_size_gigabytes() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00GB");
        assert_eq!(format_size(2 * 1024 * 1024 * 1024), "2.00GB");
    }

    #[test]
    fn test_format_size_terabytes() {
        assert_eq!(format_size(1024u64 * 1024 * 1024 * 1024), "1.00TB");
    }

    #[test]
    fn test_docker_disk_usage_default() {
        let usage = DockerDiskUsage::default();
        assert_eq!(usage.images_total, 0);
        assert_eq!(usage.containers_total, 0);
        assert_eq!(usage.volumes_total, 0);
        assert!(usage.images_size.is_empty());
    }
}
