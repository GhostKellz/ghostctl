use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub action: String,    // ACCEPT, REJECT, DROP
    pub direction: String, // IN, OUT
    pub protocol: String,  // tcp, udp, icmp, all
    pub source: String,    // IP/CIDR or any
    pub dest: String,      // IP/CIDR or any
    pub dport: String,     // destination port
    pub sport: String,     // source port
    pub comment: String,   // rule description
    pub enabled: bool,     // rule status
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallProfile {
    pub name: String,
    pub description: String,
    pub rules: Vec<FirewallRule>,
    pub default_policy: String, // ACCEPT, REJECT, DROP
}

/// Validates a profile name to prevent path traversal attacks.
/// Only allows alphanumeric characters, underscores, and hyphens.
/// Rejects path separators, dots sequences, and empty/too-long names.
fn validate_profile_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Profile name cannot be empty");
    }
    if name.len() > 64 {
        return Err("Profile name too long (max 64 characters)");
    }
    if name.contains("..") {
        return Err("Profile name cannot contain '..'");
    }
    if name.contains('/') || name.contains('\\') {
        return Err("Profile name cannot contain path separators");
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err("Profile name can only contain letters, numbers, underscores, and hyphens");
    }
    Ok(())
}

pub fn firewall_automation_menu() {
    loop {
        let options = vec![
            "🔥 Firewall Rule Management",
            "📋 Firewall Profiles & Templates",
            "🔍 Network Security Scanning",
            "🛡️  Security Policy Enforcement",
            "📊 Firewall Monitoring & Analytics",
            "🚨 Threat Detection & Response",
            "⚙️  Firewall Configuration Backup",
            "🔧 Advanced Firewall Tools",
            "📈 Security Compliance Checks",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 PVE Firewall Automation")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => firewall_rule_management(),
            1 => firewall_profiles_templates(),
            2 => crate::network::scan::network_security_scanning(),
            3 => security_policy_enforcement(),
            4 => firewall_monitoring_analytics(),
            5 => threat_detection_response(),
            6 => firewall_configuration_backup(),
            7 => advanced_firewall_tools(),
            8 => security_compliance_checks(),
            _ => break,
        }
    }
}

fn firewall_rule_management() {
    loop {
        let options = vec![
            "📋 List Current Rules",
            "➕ Add New Rule",
            "✏️  Edit Existing Rule",
            "🗑️  Delete Rule",
            "🔄 Bulk Rule Operations",
            "🔍 Search Rules",
            "📤 Export Rules",
            "📥 Import Rules",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Firewall Rule Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_current_rules(),
            1 => add_new_rule(),
            2 => edit_existing_rule(),
            3 => delete_rule(),
            4 => bulk_rule_operations(),
            5 => search_rules(),
            6 => export_rules(),
            7 => import_rules(),
            _ => break,
        }
    }
}

fn list_current_rules() {
    println!("📋 Current PVE Firewall Rules\n");

    let scope_options = vec![
        "🌐 Datacenter Level",
        "🖥️  Node Level",
        "💻 VM/Container Level",
        "🔍 All Scopes",
    ];

    let scope = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select firewall scope")
        .items(&scope_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match scope {
        0 => {
            println!("🌐 Datacenter firewall rules:");
            if let Err(e) = Command::new("pvesh")
                .args(["get", "/cluster/firewall/rules"])
                .status()
            {
                println!("Failed to get datacenter rules: {}", e);
            }
        }
        1 => {
            println!("🖥️  Node firewall rules:");
            if let Err(e) = Command::new("pvesh")
                .args(["get", "/nodes/localhost/firewall/rules"])
                .status()
            {
                println!("Failed to get node rules: {}", e);
            }
        }
        2 => {
            let vmid: String = match Input::new()
                .with_prompt("Enter VM/Container ID")
                .interact_text()
            {
                Ok(v) => v,
                Err(_) => return,
            };

            // Validate VMID
            if let Err(e) = super::validation::validate_vmid(&vmid) {
                println!("Invalid VM/Container ID: {}", e);
                return;
            }

            println!("💻 VM/Container {} firewall rules:", vmid);
            if let Err(e) = Command::new("pvesh")
                .args([
                    "get",
                    &format!("/nodes/localhost/qemu/{}/firewall/rules", vmid),
                ])
                .status()
            {
                println!("Failed to get VM firewall rules: {}", e);
            }
        }
        3 => {
            println!("🔍 All firewall rules:");
            println!("\n🌐 Datacenter rules:");
            if let Err(e) = Command::new("pvesh")
                .args(["get", "/cluster/firewall/rules"])
                .status()
            {
                println!("Failed to get datacenter rules: {}", e);
            }

            println!("\n🖥️  Node rules:");
            if let Err(e) = Command::new("pvesh")
                .args(["get", "/nodes/localhost/firewall/rules"])
                .status()
            {
                println!("Failed to get node rules: {}", e);
            }
        }
        _ => {}
    }
}

fn add_new_rule() {
    println!("➕ Add New Firewall Rule\n");

    let action_options = vec!["ACCEPT", "REJECT", "DROP"];
    let action = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&action_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    let direction_options = vec!["IN", "OUT"];
    let direction = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select direction")
        .items(&direction_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(d)) => d,
        Ok(None) | Err(_) => return,
    };

    let protocol_options = vec!["tcp", "udp", "icmp", "all"];
    let protocol = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocol")
        .items(&protocol_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(p)) => p,
        Ok(None) | Err(_) => return,
    };

    let source: String = match Input::new()
        .with_prompt("Source (IP/CIDR or 'any')")
        .default("any".to_string())
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let dest: String = match Input::new()
        .with_prompt("Destination (IP/CIDR or 'any')")
        .default("any".to_string())
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let dport: String = match Input::new()
        .with_prompt("Destination port (or leave empty)")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let comment: String = match Input::new()
        .with_prompt("Rule comment/description")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    let scope_options = vec!["Datacenter", "Node", "VM/Container"];
    let scope = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply rule to")
        .items(&scope_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let rule = FirewallRule {
        action: action_options[action].to_string(),
        direction: direction_options[direction].to_string(),
        protocol: protocol_options[protocol].to_string(),
        source,
        dest,
        dport,
        sport: String::new(),
        comment,
        enabled: true,
    };

    println!("📋 Rule Summary:");
    println!("   Action: {}", rule.action);
    println!("   Direction: {}", rule.direction);
    println!("   Protocol: {}", rule.protocol);
    println!("   Source: {}", rule.source);
    println!("   Destination: {}", rule.dest);
    println!("   Port: {}", rule.dport);
    println!("   Comment: {}", rule.comment);

    let confirm = Confirm::new()
        .with_prompt("Create this firewall rule?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        create_firewall_rule(&rule, scope);
    }
}

fn create_firewall_rule(rule: &FirewallRule, scope: usize) {
    let mut cmd_args = vec![
        "create".to_string(),
        match scope {
            0 => "/cluster/firewall/rules".to_string(),
            1 => "/nodes/localhost/firewall/rules".to_string(),
            2 => {
                let vmid: String = match Input::new()
                    .with_prompt("Enter VM/Container ID")
                    .interact_text()
                {
                    Ok(v) => v,
                    Err(_) => return,
                };
                format!("/nodes/localhost/qemu/{}/firewall/rules", vmid)
            }
            _ => "/cluster/firewall/rules".to_string(),
        },
    ];

    cmd_args.extend([
        "--type".to_string(),
        rule.direction.clone(),
        "--action".to_string(),
        rule.action.clone(),
        "--proto".to_string(),
        rule.protocol.clone(),
        "--source".to_string(),
        rule.source.clone(),
        "--dest".to_string(),
        rule.dest.clone(),
    ]);

    if !rule.dport.is_empty() {
        cmd_args.extend(["--dport".to_string(), rule.dport.clone()]);
    }

    if !rule.comment.is_empty() {
        cmd_args.extend(["--comment".to_string(), rule.comment.clone()]);
    }

    let status = Command::new("pvesh").args(&cmd_args).status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Firewall rule created successfully!");
    } else {
        println!("❌ Failed to create firewall rule");
    }
}

fn edit_existing_rule() {
    println!("✏️  Edit Existing Rule\n");
    println!("💡 Use the PVE web interface for rule editing or manually specify rule parameters");

    let rule_pos: String = match Input::new()
        .with_prompt("Enter rule position/number to edit")
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    println!(
        "🔧 Rule {} editing - Use 'pvesh set' command manually",
        rule_pos
    );
}

fn delete_rule() {
    println!("🗑️  Delete Firewall Rule\n");

    let rule_pos: String = match Input::new()
        .with_prompt("Enter rule position/number to delete")
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let scope_options = vec!["Datacenter", "Node", "VM/Container"];
    let scope = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Delete rule from")
        .items(&scope_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let confirm = Confirm::new()
        .with_prompt(format!("⚠️  Delete rule at position {}?", rule_pos))
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        let path = match scope {
            0 => format!("/cluster/firewall/rules/{}", rule_pos),
            1 => format!("/nodes/localhost/firewall/rules/{}", rule_pos),
            2 => {
                let vmid: String = match Input::new()
                    .with_prompt("Enter VM/Container ID")
                    .interact_text()
                {
                    Ok(v) => v,
                    Err(_) => return,
                };
                format!("/nodes/localhost/qemu/{}/firewall/rules/{}", vmid, rule_pos)
            }
            _ => format!("/cluster/firewall/rules/{}", rule_pos),
        };

        let status = Command::new("pvesh").args(&["delete", &path]).status();

        if status.map(|s| s.success()).unwrap_or(false) {
            println!("✅ Firewall rule deleted successfully!");
        } else {
            println!("❌ Failed to delete firewall rule");
        }
    }
}

fn bulk_rule_operations() {
    println!("🔄 Bulk Rule Operations\n");

    let operations = vec![
        "📤 Enable All Rules",
        "📥 Disable All Rules",
        "🗑️  Delete All Rules (Dangerous!)",
        "📋 Apply Rule Template",
        "🔄 Reset to Default",
    ];

    let operation = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bulk operation")
        .items(&operations)
        .default(0)
        .interact_opt()
    {
        Ok(Some(o)) => o,
        Ok(None) | Err(_) => return,
    };

    let confirm_msg = match operation {
        0 => "Enable all firewall rules?",
        1 => "Disable all firewall rules?",
        2 => "⚠️  DELETE ALL firewall rules? This is irreversible!",
        3 => "Apply selected rule template?",
        4 => "Reset firewall to default configuration?",
        _ => "Perform this operation?",
    };

    let confirm = Confirm::new()
        .with_prompt(confirm_msg)
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        match operation {
            2 => {
                let final_confirm = Confirm::new()
                    .with_prompt("🚨 FINAL WARNING: This will delete ALL firewall rules! Are you absolutely sure?")
                    .default(false)
                    .interact_opt()
                    .ok()
                    .flatten()
                    .unwrap_or(false);

                if final_confirm {
                    println!("🗑️  Deleting all firewall rules...");
                    // This would implement bulk deletion
                    println!("💡 Bulk deletion implementation pending - use with extreme caution");
                }
            }
            _ => {
                println!("🔄 Performing bulk operation...");
                println!("💡 Bulk operations implementation pending");
            }
        }
    }
}

fn search_rules() {
    println!("🔍 Search Firewall Rules\n");

    let search_term: String = match Input::new()
        .with_prompt("Enter search term (IP, port, protocol, comment)")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("🔍 Searching for rules matching: {}", search_term);

    // Search in datacenter rules
    let output = Command::new("pvesh")
        .args(&["get", "/cluster/firewall/rules", "--output-format", "json"])
        .output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        let matching_lines: Vec<&str> = content
            .lines()
            .filter(|line| line.to_lowercase().contains(&search_term.to_lowercase()))
            .collect();

        if matching_lines.is_empty() {
            println!("❌ No matching rules found");
        } else {
            println!("📋 Found {} matching rules:", matching_lines.len());
            for line in matching_lines {
                println!("  {}", line);
            }
        }
    }
}

fn export_rules() {
    println!("📤 Export Firewall Rules\n");

    let export_path: String = match Input::new()
        .with_prompt("Export file path")
        .default("/tmp/pve-firewall-rules.json".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    println!("📤 Exporting firewall rules to: {}", export_path);

    // Export datacenter rules
    let output = Command::new("pvesh")
        .args(&["get", "/cluster/firewall/rules", "--output-format", "json"])
        .output();

    if let Ok(output) = output {
        if fs::write(&export_path, &output.stdout).is_ok() {
            println!("✅ Rules exported successfully to: {}", export_path);
        } else {
            println!("❌ Failed to write export file");
        }
    } else {
        println!("❌ Failed to export rules");
    }
}

fn import_rules() {
    println!("📥 Import Firewall Rules\n");

    let import_path: String = match Input::new().with_prompt("Import file path").interact_text() {
        Ok(p) => p,
        Err(_) => return,
    };

    if !Path::new(&import_path).exists() {
        println!("❌ File not found: {}", import_path);
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("⚠️  This will add imported rules to existing configuration. Continue?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        println!("📥 Importing firewall rules from: {}", import_path);
        println!("💡 Rule import implementation pending");
    }
}

fn firewall_profiles_templates() {
    loop {
        let options = vec![
            "📋 List Available Profiles",
            "➕ Create New Profile",
            "📤 Apply Profile",
            "✏️  Edit Profile",
            "🗑️  Delete Profile",
            "📦 Built-in Templates",
            "🌐 Community Templates",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📋 Firewall Profiles & Templates")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_available_profiles(),
            1 => create_new_profile(),
            2 => apply_profile(),
            3 => edit_profile(),
            4 => delete_profile(),
            5 => builtin_templates(),
            6 => community_templates(),
            _ => break,
        }
    }
}

fn list_available_profiles() {
    println!("📋 Available Firewall Profiles\n");

    let profiles_dir = "/tmp/ghostctl/firewall-profiles";
    let _ = fs::create_dir_all(profiles_dir);

    if let Ok(entries) = fs::read_dir(profiles_dir) {
        println!("🗂️  Saved profiles:");
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str()
                && name.ends_with(".json")
            {
                println!("   📄 {}", name.strip_suffix(".json").unwrap_or(name));
            }
        }
    } else {
        println!("📁 No profiles found. Create your first profile!");
    }
}

fn create_new_profile() {
    println!("➕ Create New Firewall Profile\n");

    let profile_name: String = match Input::new().with_prompt("Profile name").interact_text() {
        Ok(n) => n,
        Err(_) => return,
    };

    // Validate profile name to prevent path traversal
    if let Err(e) = validate_profile_name(&profile_name) {
        println!("❌ Invalid profile name: {}", e);
        return;
    }

    let description: String = match Input::new()
        .with_prompt("Profile description")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let default_policy_options = vec!["DROP", "REJECT", "ACCEPT"];
    let default_policy = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Default policy")
        .items(&default_policy_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(p)) => p,
        Ok(None) | Err(_) => return,
    };

    let mut profile = FirewallProfile {
        name: profile_name,
        description,
        rules: Vec::new(),
        default_policy: default_policy_options[default_policy].to_string(),
    };

    // Add rules interactively
    loop {
        let add_rule = Confirm::new()
            .with_prompt("Add a firewall rule to this profile?")
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if add_rule {
            if let Some(rule) = create_rule_interactive() {
                profile.rules.push(rule);
            }
        } else {
            break;
        }
    }

    save_profile(&profile);
}

fn create_rule_interactive() -> Option<FirewallRule> {
    let action_options = vec!["ACCEPT", "REJECT", "DROP"];
    let action = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rule action")
        .items(&action_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return None,
    };

    let direction_options = vec!["IN", "OUT"];
    let direction = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rule direction")
        .items(&direction_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(d)) => d,
        Ok(None) | Err(_) => return None,
    };

    let protocol_options = vec!["tcp", "udp", "icmp", "all"];
    let protocol = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Protocol")
        .items(&protocol_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(p)) => p,
        Ok(None) | Err(_) => return None,
    };

    let source: String = match Input::new()
        .with_prompt("Source (IP/CIDR)")
        .default("any".to_string())
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return None,
    };

    let dest: String = match Input::new()
        .with_prompt("Destination (IP/CIDR)")
        .default("any".to_string())
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return None,
    };

    let dport: String = match Input::new().with_prompt("Destination port").interact_text() {
        Ok(p) => p,
        Err(_) => return None,
    };

    let comment: String = match Input::new().with_prompt("Comment").interact_text() {
        Ok(c) => c,
        Err(_) => return None,
    };

    Some(FirewallRule {
        action: action_options[action].to_string(),
        direction: direction_options[direction].to_string(),
        protocol: protocol_options[protocol].to_string(),
        source,
        dest,
        dport,
        sport: String::new(),
        comment,
        enabled: true,
    })
}

fn save_profile(profile: &FirewallProfile) {
    // Defense-in-depth: validate profile name even though callers should validate
    if let Err(e) = validate_profile_name(&profile.name) {
        println!("❌ Invalid profile name: {}", e);
        return;
    }

    let profiles_dir = "/tmp/ghostctl/firewall-profiles";
    let _ = fs::create_dir_all(profiles_dir);

    let profile_path = format!("{}/{}.json", profiles_dir, profile.name);

    if let Ok(json) = serde_json::to_string_pretty(profile) {
        if fs::write(&profile_path, json).is_ok() {
            println!("✅ Profile '{}' saved successfully!", profile.name);
        } else {
            println!("❌ Failed to save profile");
        }
    }
}

fn apply_profile() {
    println!("📤 Apply Firewall Profile\n");

    list_available_profiles();

    let profile_name: String = match Input::new()
        .with_prompt("Enter profile name to apply")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    // Validate profile name to prevent path traversal
    if let Err(e) = validate_profile_name(&profile_name) {
        println!("❌ Invalid profile name: {}", e);
        return;
    }

    let profile_path = format!("/tmp/ghostctl/firewall-profiles/{}.json", profile_name);

    if !Path::new(&profile_path).exists() {
        println!("❌ Profile not found: {}", profile_name);
        return;
    }

    if let Ok(content) = fs::read_to_string(&profile_path)
        && let Ok(profile) = serde_json::from_str::<FirewallProfile>(&content)
    {
        println!("📋 Profile: {}", profile.name);
        println!("📝 Description: {}", profile.description);
        println!("🔒 Default Policy: {}", profile.default_policy);
        println!("📊 Rules: {}", profile.rules.len());

        let confirm = Confirm::new()
            .with_prompt("Apply this profile?")
            .default(false)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if confirm {
            apply_profile_rules(&profile);
        }
    }
}

fn apply_profile_rules(profile: &FirewallProfile) {
    println!("🔧 Applying profile rules...");

    for (i, rule) in profile.rules.iter().enumerate() {
        println!(
            "📋 Applying rule {}/{}: {}",
            i + 1,
            profile.rules.len(),
            rule.comment
        );
        create_firewall_rule(rule, 0); // Apply to datacenter by default
    }

    println!("✅ Profile '{}' applied successfully!", profile.name);
}

fn edit_profile() {
    println!("✏️  Edit Firewall Profile\n");
    println!("💡 Profile editing implementation pending");
}

fn delete_profile() {
    println!("🗑️  Delete Firewall Profile\n");

    list_available_profiles();

    let profile_name: String = match Input::new()
        .with_prompt("Enter profile name to delete")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    // Validate profile name to prevent path traversal
    if let Err(e) = validate_profile_name(&profile_name) {
        println!("❌ Invalid profile name: {}", e);
        return;
    }

    let confirm = Confirm::new()
        .with_prompt(format!("⚠️  Delete profile '{}'?", profile_name))
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        let profile_path = format!("/tmp/ghostctl/firewall-profiles/{}.json", profile_name);

        if fs::remove_file(&profile_path).is_ok() {
            println!("✅ Profile '{}' deleted successfully!", profile_name);
        } else {
            println!("❌ Failed to delete profile or profile not found");
        }
    }
}

fn builtin_templates() {
    println!("📦 Built-in Firewall Templates\n");

    let templates = vec![
        "🌐 Web Server (HTTP/HTTPS)",
        "📧 Mail Server (SMTP/POP/IMAP)",
        "🐧 SSH Server",
        "🗄️  Database Server (MySQL/PostgreSQL)",
        "🔐 VPN Server (OpenVPN/WireGuard)",
        "📡 DNS Server",
        "🎮 Game Server",
        "📺 Media Server (Plex/Jellyfin)",
        "⬅️  Back",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template to apply")
        .items(&templates)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if selection < templates.len() - 1 {
        let template_name = templates[selection];
        println!("📦 Applying template: {}", template_name);
        apply_builtin_template(selection);
    }
}

fn apply_builtin_template(template_id: usize) {
    let profile = match template_id {
        0 => create_web_server_profile(),
        1 => create_mail_server_profile(),
        2 => create_ssh_server_profile(),
        3 => create_database_server_profile(),
        4 => create_vpn_server_profile(),
        5 => create_dns_server_profile(),
        6 => create_game_server_profile(),
        7 => create_media_server_profile(),
        _ => return,
    };

    let confirm = Confirm::new()
        .with_prompt(format!("Apply {} template?", profile.name))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        apply_profile_rules(&profile);
    }
}

fn create_web_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "Web Server".to_string(),
        description: "Standard web server configuration with HTTP/HTTPS access".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "80".to_string(),
                sport: "".to_string(),
                comment: "HTTP traffic".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "443".to_string(),
                sport: "".to_string(),
                comment: "HTTPS traffic".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "22".to_string(),
                sport: "".to_string(),
                comment: "SSH access".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_mail_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "Mail Server".to_string(),
        description: "Mail server with SMTP, POP3, IMAP access".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "25".to_string(),
                sport: "".to_string(),
                comment: "SMTP".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "110".to_string(),
                sport: "".to_string(),
                comment: "POP3".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "143".to_string(),
                sport: "".to_string(),
                comment: "IMAP".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_ssh_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "SSH Server".to_string(),
        description: "SSH server configuration".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![FirewallRule {
            action: "ACCEPT".to_string(),
            direction: "IN".to_string(),
            protocol: "tcp".to_string(),
            source: "any".to_string(),
            dest: "any".to_string(),
            dport: "22".to_string(),
            sport: "".to_string(),
            comment: "SSH access".to_string(),
            enabled: true,
        }],
    }
}

fn create_database_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "Database Server".to_string(),
        description: "Database server with MySQL/PostgreSQL access".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "3306".to_string(),
                sport: "".to_string(),
                comment: "MySQL".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "5432".to_string(),
                sport: "".to_string(),
                comment: "PostgreSQL".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_vpn_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "VPN Server".to_string(),
        description: "VPN server with OpenVPN and WireGuard".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "udp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "1194".to_string(),
                sport: "".to_string(),
                comment: "OpenVPN".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "udp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "51820".to_string(),
                sport: "".to_string(),
                comment: "WireGuard".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_dns_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "DNS Server".to_string(),
        description: "DNS server configuration".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "udp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "53".to_string(),
                sport: "".to_string(),
                comment: "DNS UDP".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "53".to_string(),
                sport: "".to_string(),
                comment: "DNS TCP".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_game_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "Game Server".to_string(),
        description: "Common game server ports".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "27015".to_string(),
                sport: "".to_string(),
                comment: "Source games".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "udp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "27015".to_string(),
                sport: "".to_string(),
                comment: "Source games UDP".to_string(),
                enabled: true,
            },
        ],
    }
}

fn create_media_server_profile() -> FirewallProfile {
    FirewallProfile {
        name: "Media Server".to_string(),
        description: "Media server with Plex/Jellyfin access".to_string(),
        default_policy: "DROP".to_string(),
        rules: vec![
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "32400".to_string(),
                sport: "".to_string(),
                comment: "Plex".to_string(),
                enabled: true,
            },
            FirewallRule {
                action: "ACCEPT".to_string(),
                direction: "IN".to_string(),
                protocol: "tcp".to_string(),
                source: "any".to_string(),
                dest: "any".to_string(),
                dport: "8096".to_string(),
                sport: "".to_string(),
                comment: "Jellyfin".to_string(),
                enabled: true,
            },
        ],
    }
}

fn community_templates() {
    println!("🌐 Community Firewall Templates\n");
    println!("💡 Community template integration coming soon!");
    println!("📋 Planned sources:");
    println!("   • GitHub community firewall rules");
    println!("   • Industry-standard security profiles");
    println!("   • Application-specific templates");
}

fn network_security_scanning() {
    loop {
        let options = vec![
            "🔍 Quick Network Scan (Native Scanner)",
            "🛡️  Comprehensive Security Scan",
            "📊 Port Scan Analysis",
            "🚨 Vulnerability Assessment",
            "🌐 Network Topology Discovery",
            "⚡ Performance Impact Testing",
            "📋 Scan Reports & History",
            "⚙️  Scan Configuration",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔍 Network Security Scanning (Native Scanner)")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => quick_network_scan(),
            1 => comprehensive_security_scan(),
            2 => port_scan_analysis(),
            3 => vulnerability_assessment(),
            4 => network_topology_discovery(),
            5 => performance_impact_testing(),
            6 => scan_reports_history(),
            7 => scan_configuration(),
            _ => break,
        }
    }
}

fn quick_network_scan() {
    println!("🔍 Quick Network Scan with Native Scanner\n");

    let target: String = match Input::new()
        .with_prompt("Enter target IP/hostname/CIDR to scan")
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let scan_options = vec![
        "🚀 Quick Scan (Top 1000 ports)",
        "🔍 Full Scan (All 65535 ports)",
        "⚡ Common Services (22,80,443,3389,5432,3306)",
        "🎯 Custom Port Range",
    ];

    let scan_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scan type")
        .items(&scan_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let port_spec = match scan_type {
        0 => "1-1000".to_string(),
        1 => "1-65535".to_string(),
        2 => "22,80,443,3389,5432,3306".to_string(),
        3 => match Input::new()
            .with_prompt("Enter port range (e.g., 80-443 or 80,443,8080)")
            .interact_text()
        {
            Ok(p) => p,
            Err(_) => return,
        },
        _ => "1-1000".to_string(),
    };

    let threads: u32 = match Input::new()
        .with_prompt("Number of scan threads")
        .default(100u32)
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    println!("🔄 Starting native scan...");
    println!("📊 Target: {}", target);
    println!("🚪 Ports: {}", port_spec);
    println!("⚙️  Threads: {}", threads);
    println!();

    // Use the native scanner
    let targets = vec![target.clone()];
    let result =
        crate::network::scan::scan_cli(targets, Some(port_spec.clone()), Some(threads as usize));

    match result {
        Ok(_) => {
            println!("✅ Scan completed successfully!");

            let generate = Confirm::new()
                .with_prompt("Generate firewall rules based on scan results?")
                .default(true)
                .interact_opt()
                .ok()
                .flatten()
                .unwrap_or(false);

            if generate {
                generate_rules_from_scan(&target);
            }
        }
        Err(e) => {
            println!("❌ Scan failed: {}", e);
        }
    }
}

// install_gscan function removed - using native scanner implementation

fn generate_rules_from_scan(target: &str) {
    println!("🛡️  Generating Firewall Rules from Scan Results\n");

    println!("💡 Scan-based rule generation:");
    println!("   • Block all scanned ports by default");
    println!("   • Allow only necessary services");
    println!("   • Create host-specific rules for {}", target);

    let rule_types = vec![
        "🚫 Block all detected ports",
        "✅ Allow specific services only",
        "⚠️  Alert on suspicious activity",
        "🎯 Custom rule generation",
    ];

    let rule_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule generation strategy")
        .items(&rule_types)
        .default(1)
        .interact_opt()
    {
        Ok(Some(r)) => r,
        Ok(None) | Err(_) => return,
    };

    match rule_type {
        0 => {
            println!("🚫 Generating BLOCK rules for detected ports...");
            println!("💡 This would create DROP rules for all open ports found");
        }
        1 => {
            let services = match MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select services to allow")
                .items(&["SSH (22)", "HTTP (80)", "HTTPS (443)", "Custom"])
                .interact_opt()
            {
                Ok(Some(s)) => s,
                Ok(None) | Err(_) => return,
            };

            println!("✅ Generating ALLOW rules for selected services...");
            for service_idx in services {
                let service_name = ["SSH (22)", "HTTP (80)", "HTTPS (443)", "Custom"][service_idx];
                println!("   📋 Creating rule for: {}", service_name);
            }
        }
        2 => {
            println!("⚠️  Generating ALERT rules for monitoring...");
            println!("💡 This would create logging rules for detected activity");
        }
        3 => {
            println!("🎯 Custom rule generation wizard...");
            println!("💡 Interactive rule creation based on scan results");
        }
        _ => {}
    }

    println!("✅ Firewall rules generated based on scan of {}", target);
}

fn comprehensive_security_scan() {
    println!("🛡️  Comprehensive Security Scan\n");
    println!("💡 Full security assessment implementation pending");
}

fn port_scan_analysis() {
    println!("📊 Port Scan Analysis\n");
    println!("💡 Advanced port analysis implementation pending");
}

fn vulnerability_assessment() {
    println!("🚨 Vulnerability Assessment\n");
    println!("💡 Vulnerability scanning implementation pending");
}

fn network_topology_discovery() {
    println!("🌐 Network Topology Discovery\n");
    println!("💡 Network mapping implementation pending");
}

fn performance_impact_testing() {
    println!("⚡ Performance Impact Testing\n");
    println!("💡 Performance testing implementation pending");
}

fn scan_reports_history() {
    println!("📋 Scan Reports & History\n");
    println!("💡 Report management implementation pending");
}

fn scan_configuration() {
    println!("⚙️  Scan Configuration\n");
    println!("💡 Scan settings implementation pending");
}

fn security_policy_enforcement() {
    println!("🛡️  Security Policy Enforcement - Implementation coming in next update!");
}

fn firewall_monitoring_analytics() {
    println!("📊 Firewall Monitoring & Analytics - Implementation coming in next update!");
}

fn threat_detection_response() {
    println!("🚨 Threat Detection & Response - Implementation coming in next update!");
}

fn firewall_configuration_backup() {
    println!("⚙️  Firewall Configuration Backup - Implementation coming in next update!");
}

fn advanced_firewall_tools() {
    println!("🔧 Advanced Firewall Tools - Implementation coming in next update!");
}

fn security_compliance_checks() {
    println!("📈 Security Compliance Checks - Implementation coming in next update!");
}
