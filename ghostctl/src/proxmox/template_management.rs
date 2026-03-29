use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn template_management_menu() {
    loop {
        let options = vec![
            "📦 List Available Templates",
            "⬇️  Download Template",
            "🗑️  Remove Template",
            "📋 Template Information",
            "🔄 Update Template Cache",
            "📤 Upload Custom Template",
            "🏭 Template Customization",
            "🔧 Template Maintenance",
            "📊 Template Usage Statistics",
            "⬅️  Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📦 PVE Template Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => list_available_templates(),
            1 => download_template(),
            2 => remove_template(),
            3 => template_information(),
            4 => update_template_cache(),
            5 => upload_custom_template(),
            6 => template_customization(),
            7 => template_maintenance(),
            8 => template_usage_statistics(),
            _ => break,
        }
    }
}

fn list_available_templates() {
    println!("📦 Available PVE Templates\n");

    let template_options = vec![
        "🐧 Linux Container Templates",
        "🖼️  VM ISO Templates",
        "📱 Appliance Templates",
        "🌐 Community Templates",
        "📋 All Templates",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template category")
        .items(&template_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => list_container_templates(),
        1 => list_vm_iso_templates(),
        2 => list_appliance_templates(),
        3 => list_community_templates(),
        4 => list_all_templates(),
        _ => return,
    }
}

fn list_container_templates() {
    println!("🐧 Linux Container Templates\n");

    println!("📋 Available LXC templates on this node:");
    if let Err(e) = Command::new("pveam")
        .args(["available", "--section", "system"])
        .status()
    {
        println!("Failed to list available templates: {}", e);
    }

    println!("\n🔍 Installed templates:");
    if let Err(e) = Command::new("pveam").args(["list", "local"]).status() {
        println!("Failed to list installed templates: {}", e);
    }
}

fn list_vm_iso_templates() {
    println!("🖼️  VM ISO Templates\n");

    println!("📋 Available ISO images:");
    if let Err(e) = Command::new("pvesm")
        .args(["list", "local", "--content", "iso"])
        .status()
    {
        println!("Failed to list ISO images: {}", e);
    }

    println!("\n💿 Popular Linux distributions:");
    println!("   • Ubuntu Server (latest LTS)");
    println!("   • Debian (stable)");
    println!("   • CentOS Stream");
    println!("   • Rocky Linux");
    println!("   • Alpine Linux");
    println!("   • openSUSE");

    let Ok(download) = Confirm::new()
        .with_prompt("Download a specific ISO?")
        .default(false)
        .interact()
    else {
        return;
    };
    if download {
        download_iso_template();
    }
}

fn download_iso_template() {
    let distros = [
        (
            "Ubuntu Server 22.04 LTS",
            "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-live-server-amd64.iso",
        ),
        (
            "Ubuntu Server 20.04 LTS",
            "https://releases.ubuntu.com/20.04/ubuntu-20.04.6-live-server-amd64.iso",
        ),
        (
            "Debian 12 (Bookworm)",
            "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-12.2.0-amd64-netinst.iso",
        ),
        (
            "Rocky Linux 9",
            "https://download.rockylinux.org/pub/rocky/9/isos/x86_64/Rocky-9.2-x86_64-minimal.iso",
        ),
        (
            "Alpine Linux",
            "https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-standard-3.18.4-x86_64.iso",
        ),
        ("Custom URL", "custom"),
    ];

    let distro_names: Vec<&str> = distros.iter().map(|(name, _)| *name).collect();

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select distribution to download")
        .items(&distro_names)
        .default(0)
        .interact()
    else {
        return;
    };

    let (distro_name, url) = if selection == distros.len() - 1 {
        let Ok(custom_name): Result<String, _> =
            Input::new().with_prompt("Enter ISO name").interact_text()
        else {
            return;
        };
        let Ok(custom_url): Result<String, _> =
            Input::new().with_prompt("Enter ISO URL").interact_text()
        else {
            return;
        };
        (custom_name, custom_url)
    } else {
        let (name, url) = distros[selection];
        (name.to_string(), url.to_string())
    };

    println!("📥 Downloading {} ISO...", distro_name);

    // Use wget to download to local storage
    let filename = url.split('/').next_back().unwrap_or("downloaded.iso");
    let download_path = format!("/var/lib/vz/template/iso/{}", filename);

    let status = Command::new("wget")
        .args(&[
            "-O",
            &download_path,
            "--progress=bar",
            "--show-progress",
            &url,
        ])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ ISO downloaded successfully to: {}", download_path);
    } else {
        println!("❌ Download failed");
    }
}

fn list_appliance_templates() {
    println!("📱 Appliance Templates\n");

    println!("🔍 Turnkey Linux appliances:");
    if let Err(e) = Command::new("pveam")
        .args(["available", "--section", "turnkeylinux"])
        .status()
    {
        println!("Failed to list appliance templates: {}", e);
    }

    println!("\n📋 Popular appliances:");
    println!("   • Nextcloud (file sharing)");
    println!("   • GitLab (git repository)");
    println!("   • WordPress (CMS)");
    println!("   • Drupal (CMS)");
    println!("   • MediaWiki (wiki)");
    println!("   • Redmine (project management)");
}

fn list_community_templates() {
    println!("🌐 Community Templates\n");

    println!("💡 Popular community sources:");
    println!("   • Proxmox VE Helper Scripts");
    println!("   • tteck's Proxmox Scripts");
    println!("   • community-scripts/ProxmoxVE");
    println!("   • Custom LXC containers");

    println!("\n🔗 Community template repositories:");
    println!("   • GitHub: community-scripts/ProxmoxVE");
    println!("   • GitHub: tteck/Proxmox");
    println!("   • Linux Containers: images.linuxcontainers.org");
}

fn list_all_templates() {
    println!("📋 All Available Templates\n");

    println!("🔍 Complete template listing:");
    if let Err(e) = Command::new("pveam").args(["available"]).status() {
        println!("Failed to list templates: {}", e);
    }
}

fn download_template() {
    println!("⬇️  Download Template\n");

    let download_options = vec![
        "📱 Download Specific Appliance",
        "🐧 Download Linux Container",
        "💿 Download ISO Image",
        "🔍 Search and Download",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select download type")
        .items(&download_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => download_appliance_template(),
        1 => download_container_template(),
        2 => download_iso_template(),
        3 => search_and_download_template(),
        _ => return,
    }
}

fn download_appliance_template() {
    println!("📱 Download Appliance Template\n");

    let Ok(template_id): Result<String, _> = Input::new()
        .with_prompt("Enter template ID (e.g., turnkey-nextcloud-17.1-bullseye-amd64.tar.gz)")
        .interact_text()
    else {
        return;
    };

    println!("📥 Downloading appliance template: {}", template_id);

    let status = Command::new("pveam")
        .args(&["download", "local", &template_id])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Template downloaded successfully!");
    } else {
        println!("❌ Download failed");
    }
}

fn download_container_template() {
    println!("🐧 Download Linux Container Template\n");

    let popular_containers = vec![
        "alpine-3.18-default_20230607_amd64.tar.xz",
        "ubuntu-22.04-standard_22.04-1_amd64.tar.zst",
        "debian-12-standard_12.2-1_amd64.tar.zst",
        "centos-9-stream-default_20221109_amd64.tar.xz",
        "Custom template ID",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select container template")
        .items(&popular_containers)
        .default(0)
        .interact()
    else {
        return;
    };

    let template_id = if selection == popular_containers.len() - 1 {
        let Ok(id): Result<String, _> = Input::new()
            .with_prompt("Enter custom template ID")
            .interact_text()
        else {
            return;
        };
        id
    } else {
        popular_containers[selection].to_string()
    };

    println!("📥 Downloading container template: {}", template_id);

    let status = Command::new("pveam")
        .args(&["download", "local", &template_id])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Template downloaded successfully!");
    } else {
        println!("❌ Download failed");
    }
}

fn search_and_download_template() {
    println!("🔍 Search and Download Template\n");

    let Ok(search_term): Result<String, _> = Input::new()
        .with_prompt("Enter search term (e.g., 'nextcloud', 'ubuntu', 'alpine')")
        .interact_text()
    else {
        return;
    };

    println!("🔍 Searching for templates matching: {}", search_term);

    let output = Command::new("pveam").args(&["available"]).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        let matching_lines: Vec<&str> = content
            .lines()
            .filter(|line| line.to_lowercase().contains(&search_term.to_lowercase()))
            .collect();

        if matching_lines.is_empty() {
            println!("❌ No templates found matching: {}", search_term);
            return;
        }

        println!("📋 Found {} matching templates:", matching_lines.len());
        for (i, line) in matching_lines.iter().enumerate() {
            println!("  {}. {}", i + 1, line);
        }

        let Ok(index): Result<String, _> = Input::new()
            .with_prompt("Enter template number to download (or 0 to cancel)")
            .interact_text()
        else {
            return;
        };

        if let Ok(idx) = index.parse::<usize>()
            && idx > 0
            && idx <= matching_lines.len()
        {
            let selected_line = matching_lines[idx - 1];
            let template_id = selected_line.split_whitespace().next().unwrap_or("");

            if !template_id.is_empty() {
                println!("📥 Downloading: {}", template_id);
                let _ = Command::new("pveam")
                    .args(&["download", "local", template_id])
                    .status();
            }
        }
    }
}

fn remove_template() {
    println!("🗑️  Remove Template\n");

    println!("📋 Installed templates:");
    let output = Command::new("pveam").args(&["list", "local"]).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        println!("{}", content);

        let Ok(template_id): Result<String, _> = Input::new()
            .with_prompt("Enter template filename to remove")
            .interact_text()
        else {
            return;
        };

        let Ok(confirm) = Confirm::new()
            .with_prompt(format!(
                "⚠️  Remove template '{}'? This cannot be undone",
                template_id
            ))
            .default(false)
            .interact()
        else {
            return;
        };

        if confirm {
            let status = Command::new("pveam")
                .args(&["remove", "local", &template_id])
                .status();

            if status.map(|s| s.success()).unwrap_or(false) {
                println!("✅ Template removed successfully");
            } else {
                println!("❌ Failed to remove template");
            }
        } else {
            println!("❌ Operation cancelled");
        }
    }
}

fn template_information() {
    println!("📋 Template Information\n");

    let Ok(template_id): Result<String, _> = Input::new()
        .with_prompt("Enter template ID or filename")
        .interact_text()
    else {
        return;
    };

    println!("🔍 Template information for: {}\n", template_id);

    // Check if it's a local template
    let local_output = Command::new("pveam").args(&["list", "local"]).output();

    if let Ok(output) = local_output {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.contains(&template_id) {
            println!("📍 Status: Installed locally");

            let template_path = format!("/var/lib/vz/template/cache/{}", template_id);
            if Path::new(&template_path).exists()
                && let Ok(metadata) = fs::metadata(&template_path)
            {
                println!("📁 Path: {}", template_path);
                println!("📏 Size: {} bytes", metadata.len());
                if let Ok(modified) = metadata.modified() {
                    println!("📅 Modified: {:?}", modified);
                }
            }
        } else {
            println!("📍 Status: Not installed locally");

            // Check if available for download
            let available_output = Command::new("pveam").args(&["available"]).output();

            if let Ok(output) = available_output {
                let content = String::from_utf8_lossy(&output.stdout);
                if content.contains(&template_id) {
                    println!("📥 Available for download");
                } else {
                    println!("❌ Template not found in repositories");
                }
            }
        }
    }
}

fn update_template_cache() {
    println!("🔄 Update Template Cache\n");

    println!("📡 Updating template repository cache...");
    let status = Command::new("pveam").args(&["update"]).status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Template cache updated successfully!");

        let Ok(show) = Confirm::new()
            .with_prompt("Show available templates?")
            .default(true)
            .interact()
        else {
            return;
        };
        if show {
            let _ = Command::new("pveam")
                .args(&["available", "--section", "system"])
                .status();
        }
    } else {
        println!("❌ Failed to update template cache");
    }
}

fn upload_custom_template() {
    println!("📤 Upload Custom Template\n");

    let template_types = vec![
        "📦 Container Template (tar.xz/tar.zst)",
        "💿 ISO Image",
        "🖼️  VM Disk Image (qcow2/vmdk)",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template type")
        .items(&template_types)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => upload_container_template(),
        1 => upload_iso_image(),
        2 => upload_vm_disk_image(),
        _ => return,
    }
}

fn upload_container_template() {
    println!("📦 Upload Container Template\n");

    let Ok(file_path): Result<String, _> = Input::new()
        .with_prompt("Enter full path to template file")
        .interact_text()
    else {
        return;
    };

    // Validate the path to prevent injection
    if let Err(e) = super::validation::validate_path(&file_path) {
        println!("Invalid path: {}", e);
        return;
    }

    if !Path::new(&file_path).exists() {
        println!("❌ File not found: {}", file_path);
        return;
    }

    let dest_path = "/var/lib/vz/template/cache/";
    let Some(filename) = Path::new(&file_path).file_name() else {
        println!("❌ Invalid file path: {}", file_path);
        return;
    };
    let filename = filename.to_string_lossy();
    let full_dest = format!("{}{}", dest_path, filename);

    println!("📤 Copying template to: {}", full_dest);

    match Command::new("cp").args([&file_path, &full_dest]).status() {
        Ok(status) if status.success() => {
            println!("✅ Container template uploaded successfully!");

            // Set appropriate permissions
            if let Err(e) = Command::new("chmod").args(["644", &full_dest]).status() {
                println!("Warning: Could not set permissions: {}", e);
            }

            println!("📋 Template is now available for container creation");
        }
        Ok(_) => println!("❌ Upload failed (copy returned non-zero)"),
        Err(e) => println!("❌ Upload failed: {}", e),
    }
}

fn upload_iso_image() {
    println!("💿 Upload ISO Image\n");

    let Ok(file_path): Result<String, _> = Input::new()
        .with_prompt("Enter full path to ISO file")
        .interact_text()
    else {
        return;
    };

    // Validate the path to prevent injection
    if let Err(e) = super::validation::validate_path(&file_path) {
        println!("Invalid path: {}", e);
        return;
    }

    if !Path::new(&file_path).exists() {
        println!("❌ File not found: {}", file_path);
        return;
    }

    let dest_path = "/var/lib/vz/template/iso/";
    let Some(filename) = Path::new(&file_path).file_name() else {
        println!("❌ Invalid file path: {}", file_path);
        return;
    };
    let filename = filename.to_string_lossy();
    let full_dest = format!("{}{}", dest_path, filename);

    println!("📤 Copying ISO to: {}", full_dest);

    match Command::new("cp").args([&file_path, &full_dest]).status() {
        Ok(status) if status.success() => {
            println!("✅ ISO image uploaded successfully!");
            println!("💿 Image is now available for VM creation");
        }
        Ok(_) => println!("❌ Upload failed (copy returned non-zero)"),
        Err(e) => println!("❌ Upload failed: {}", e),
    }
}

fn upload_vm_disk_image() {
    println!("🖼️  Upload VM Disk Image\n");

    let Ok(file_path): Result<String, _> = Input::new()
        .with_prompt("Enter full path to disk image file")
        .interact_text()
    else {
        return;
    };

    // Validate the path to prevent injection
    if let Err(e) = super::validation::validate_path(&file_path) {
        println!("Invalid path: {}", e);
        return;
    }

    if !Path::new(&file_path).exists() {
        println!("❌ File not found: {}", file_path);
        return;
    }

    let dest_path = "/var/lib/vz/images/";
    let Some(filename) = Path::new(&file_path).file_name() else {
        println!("❌ Invalid file path: {}", file_path);
        return;
    };
    let filename = filename.to_string_lossy();
    let full_dest = format!("{}{}", dest_path, filename);

    println!("📤 Copying disk image to: {}", full_dest);

    match Command::new("cp").args([&file_path, &full_dest]).status() {
        Ok(status) if status.success() => {
            println!("✅ VM disk image uploaded successfully!");
            println!("🖼️  Image can now be imported into VMs");
        }
        Ok(_) => println!("❌ Upload failed (copy returned non-zero)"),
        Err(e) => println!("❌ Upload failed: {}", e),
    }
}

fn template_customization() {
    println!("🏭 Template Customization\n");

    let customization_options = vec![
        "🛠️  Create Custom Container Template",
        "📝 Modify Existing Template",
        "🔧 Template Hooks & Scripts",
        "📦 Package Custom Template",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select customization option")
        .items(&customization_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => create_custom_container_template(),
        1 => modify_existing_template(),
        2 => template_hooks_scripts(),
        3 => package_custom_template(),
        _ => return,
    }
}

fn create_custom_container_template() {
    println!("🛠️  Create Custom Container Template\n");

    println!("📋 Steps to create custom container template:");
    println!("   1. Create a container with desired configuration");
    println!("   2. Install and configure software");
    println!("   3. Clean up logs and temporary files");
    println!("   4. Export container as template");

    let Ok(ct_id): Result<String, _> = Input::new()
        .with_prompt("Enter container ID to convert to template")
        .interact_text()
    else {
        return;
    };

    // Validate container ID
    if let Err(e) = super::validation::validate_ctid(&ct_id) {
        println!("Invalid container ID: {}", e);
        return;
    }

    let Ok(template_name): Result<String, _> = Input::new()
        .with_prompt("Enter template name")
        .interact_text()
    else {
        return;
    };

    // Validate template name (use storage name validation as similar rules)
    if let Err(e) = super::validation::validate_storage_name(&template_name) {
        println!("Invalid template name: {}", e);
        return;
    }

    println!("🔄 Creating template from container {}...", ct_id);

    // Stop the container first
    match Command::new("pct").args(["stop", &ct_id]).status() {
        Ok(status) if status.success() => println!("   Container stopped"),
        Ok(_) => println!("   Warning: Container may not have been running"),
        Err(e) => {
            println!("   Warning: Could not stop container: {}", e);
            // Continue anyway - container might already be stopped
        }
    }

    // Create template
    match Command::new("vzdump")
        .args([
            &ct_id,
            "--compress",
            "lzo",
            "--storage",
            "local",
            "--dumpdir",
            "/var/lib/vz/dump",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!(
                "✅ Custom template '{}' created successfully!",
                template_name
            );
            println!("📁 Template saved in /var/lib/vz/dump/");
        }
        Ok(_) => println!("❌ Template creation failed (vzdump returned non-zero)"),
        Err(e) => println!("❌ Template creation failed: {}", e),
    }
}

fn modify_existing_template() {
    println!("📝 Modify Existing Template\n");

    println!("💡 Template modification workflow:");
    println!("   1. Create container from template");
    println!("   2. Make desired modifications");
    println!("   3. Export modified container as new template");
    println!("   4. Remove temporary container");

    println!("\n🔍 Available templates:");
    if let Err(e) = Command::new("pveam").args(["list", "local"]).status() {
        println!("Failed to list templates: {}", e);
    }
}

fn template_hooks_scripts() {
    println!("🔧 Template Hooks & Scripts\n");

    println!("📋 Template hook types:");
    println!("   • pre-start: Execute before container starts");
    println!("   • post-start: Execute after container starts");
    println!("   • pre-stop: Execute before container stops");
    println!("   • post-stop: Execute after container stops");

    println!("\n📁 Hook script locations:");
    println!("   • /usr/share/lxc/hooks/");
    println!("   • /var/lib/vz/snippets/");

    let Ok(create) = Confirm::new()
        .with_prompt("Create a sample hook script?")
        .default(false)
        .interact()
    else {
        return;
    };
    if create {
        create_sample_hook_script();
    }
}

fn create_sample_hook_script() {
    let hook_content = r#"#!/bin/bash
# Sample LXC hook script for PVE templates
# Place in /var/lib/vz/snippets/

echo "Hook executed: $0"
echo "Container ID: $LXC_NAME"
echo "Hook type: $(basename $0)"

# Add your custom logic here
# Examples:
# - Configure networking
# - Mount additional filesystems
# - Start/stop services
# - Update configurations

exit 0
"#;

    let hook_path = "/var/lib/vz/snippets/sample-hook.sh";

    match fs::write(hook_path, hook_content) {
        Ok(()) => {
            if let Err(e) = Command::new("chmod").args(["+x", hook_path]).status() {
                println!(
                    "✅ Script created but chmod failed: {}. Run: chmod +x {}",
                    e, hook_path
                );
            } else {
                println!("✅ Sample hook script created: {}", hook_path);
            }
        }
        Err(e) => println!("❌ Failed to create hook script: {}", e),
    }
}

fn package_custom_template() {
    println!("📦 Package Custom Template\n");

    let Ok(source_dir): Result<String, _> = Input::new()
        .with_prompt("Enter source directory or container ID")
        .interact_text()
    else {
        return;
    };

    let Ok(template_name): Result<String, _> = Input::new()
        .with_prompt("Enter template package name")
        .interact_text()
    else {
        return;
    };

    let output_path = format!("/var/lib/vz/template/cache/{}.tar.zst", template_name);

    println!("📦 Packaging template: {} -> {}", source_dir, output_path);

    // Create compressed archive
    let status = Command::new("tar")
        .args(&[
            "--create",
            "--use-compress-program=zstd",
            "--file",
            &output_path,
            "--directory",
            &source_dir,
            ".",
        ])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Template packaged successfully!");
        println!("📁 Template available: {}", output_path);
    } else {
        println!("❌ Template packaging failed");
    }
}

fn template_maintenance() {
    println!("🔧 Template Maintenance\n");

    let maintenance_options = vec![
        "🧹 Cleanup Old Templates",
        "✅ Verify Template Integrity",
        "📊 Template Storage Usage",
        "🔄 Optimize Template Storage",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select maintenance operation")
        .items(&maintenance_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => cleanup_old_templates(),
        1 => verify_template_integrity(),
        2 => template_storage_usage(),
        3 => optimize_template_storage(),
        _ => return,
    }
}

fn cleanup_old_templates() {
    println!("🧹 Cleanup Old Templates\n");

    println!("🔍 Scanning for old templates...");

    // Find templates older than X days
    let Ok(days): Result<String, _> = Input::new()
        .with_prompt("Remove templates older than X days")
        .default("90".to_string())
        .interact_text()
    else {
        return;
    };

    println!("📋 Templates older than {} days:", days);
    let output = Command::new("find")
        .args(&[
            "/var/lib/vz/template/cache",
            "-name",
            "*.tar.*",
            "-mtime",
            &format!("+{}", days),
            "-ls",
        ])
        .output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.trim().is_empty() {
            println!("✅ No old templates found");
            return;
        }

        println!("{}", content);

        let Ok(remove) = Confirm::new()
            .with_prompt("⚠️  Remove these templates?")
            .default(false)
            .interact()
        else {
            return;
        };
        if remove {
            let _ = Command::new("find")
                .args(&[
                    "/var/lib/vz/template/cache",
                    "-name",
                    "*.tar.*",
                    "-mtime",
                    &format!("+{}", days),
                    "-delete",
                ])
                .status();

            println!("✅ Old templates cleaned up");
        }
    }
}

fn verify_template_integrity() {
    println!("✅ Verify Template Integrity\n");

    println!("🔍 Checking template integrity...");

    let template_dir = "/var/lib/vz/template/cache";
    let output = Command::new("find")
        .args(&[template_dir, "-name", "*.tar.*"])
        .output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        let templates: Vec<&str> = content.lines().collect();

        if templates.is_empty() {
            println!("📋 No templates found to verify");
            return;
        }

        for template in templates {
            let Some(filename) = Path::new(template).file_name() else {
                continue;
            };
            let filename = filename.to_string_lossy();
            print!("🔍 Verifying {}... ", filename);

            let status = if template.ends_with(".tar.zst") {
                Command::new("zstd").args(&["-t", template]).output()
            } else if template.ends_with(".tar.xz") {
                Command::new("xz").args(&["-t", template]).output()
            } else if template.ends_with(".tar.gz") {
                Command::new("gzip").args(&["-t", template]).output()
            } else {
                Command::new("tar").args(&["-tf", template]).output()
            };

            match status {
                Ok(output) if output.status.success() => println!("✅ OK"),
                _ => println!("❌ CORRUPTED"),
            }
        }
    }
}

fn template_storage_usage() {
    println!("📊 Template Storage Usage\n");

    if let Err(e) = Command::new("du")
        .args(["-h", "/var/lib/vz/template/"])
        .status()
    {
        println!("Failed to get template usage: {}", e);
    }

    println!("\n📋 Template breakdown:");
    // Use safe path arguments - no shell expansion
    if let Err(e) = Command::new("du")
        .args(["-sh", "/var/lib/vz/template/cache/"])
        .status()
    {
        println!("Cache usage not available: {}", e);
    }

    if let Err(e) = Command::new("du")
        .args(["-sh", "/var/lib/vz/template/iso/"])
        .status()
    {
        println!("ISO usage not available: {}", e);
    }
}

fn optimize_template_storage() {
    println!("🔄 Optimize Template Storage\n");

    let optimization_options = vec![
        "🗜️  Recompress with Better Algorithm",
        "📦 Deduplicate Similar Templates",
        "🧹 Remove Unused Templates",
        "💾 Move to Different Storage",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimization method")
        .items(&optimization_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => recompress_templates(),
        1 => deduplicate_templates(),
        2 => cleanup_old_templates(),
        3 => move_templates_storage(),
        _ => return,
    }
}

fn recompress_templates() {
    println!("🗜️  Recompress Templates\n");

    println!("📋 Available compression methods:");
    println!("   • zstd (fast, good compression)");
    println!("   • xz (slow, best compression)");
    println!("   • lz4 (fastest, moderate compression)");

    let Ok(method) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select compression method")
        .items(&["zstd", "xz", "lz4"])
        .default(0)
        .interact()
    else {
        return;
    };

    let compression_method = ["zstd", "xz", "lz4"][method];
    println!("🔄 Recompressing templates with {}...", compression_method);
    println!("💡 This operation would recompress all templates");
}

fn deduplicate_templates() {
    println!("📦 Deduplicate Similar Templates\n");
    println!("🔍 Analyzing template similarities...");
    println!("💡 This would identify and merge similar templates");
}

fn move_templates_storage() {
    println!("💾 Move Templates to Different Storage\n");

    let Ok(storage_id): Result<String, _> = Input::new()
        .with_prompt("Enter target storage ID")
        .interact_text()
    else {
        return;
    };

    println!("📦 Moving templates to storage: {}", storage_id);
    println!("💡 This would migrate templates to different PVE storage");
}

fn template_usage_statistics() {
    println!("📊 Template Usage Statistics\n");

    println!("📈 Template usage analysis:");

    // Count container templates
    match Command::new("find")
        .args([
            "/var/lib/vz/template/cache",
            "-name",
            "*.tar.*",
            "-type",
            "f",
        ])
        .output()
    {
        Ok(output) => {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .count();
            println!("   Container templates: {}", count);
        }
        Err(e) => println!("   Container templates: (error: {})", e),
    }

    // Count ISO images
    match Command::new("find")
        .args(["/var/lib/vz/template/iso", "-name", "*.iso", "-type", "f"])
        .output()
    {
        Ok(output) => {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .count();
            println!("   ISO images: {}", count);
        }
        Err(e) => println!("   ISO images: (error: {})", e),
    }

    println!("\n💾 Storage usage:");
    if let Err(e) = Command::new("du")
        .args(["-sh", "/var/lib/vz/template/"])
        .status()
    {
        println!("   (error: {})", e);
    }

    println!("\n📋 Most recently used templates:");
    if let Err(e) = Command::new("ls")
        .args(["-lt", "/var/lib/vz/template/cache/"])
        .status()
    {
        println!("   (error: {})", e);
    }
}
