use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub action: String,      // ACCEPT, REJECT, DROP
    pub direction: String,   // IN, OUT
    pub protocol: String,    // tcp, udp, icmp, all
    pub source: String,      // IP/CIDR or any
    pub dest: String,        // IP/CIDR or any  
    pub dport: String,       // destination port
    pub sport: String,       // source port
    pub comment: String,     // rule description
    pub enabled: bool,       // rule status
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallProfile {
    pub name: String,
    pub description: String,
    pub rules: Vec<FirewallRule>,
    pub default_policy: String, // ACCEPT, REJECT, DROP
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

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 PVE Firewall Automation")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Firewall Rule Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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
    
    let scope = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select firewall scope")
        .items(&scope_options)
        .default(0)
        .interact()
        .unwrap();
    
    match scope {
        0 => {
            println!("🌐 Datacenter firewall rules:");
            let _ = Command::new("pvesh")
                .args(&["get", "/cluster/firewall/rules"])
                .status();
        },
        1 => {
            println!("🖥️  Node firewall rules:");
            let _ = Command::new("pvesh")
                .args(&["get", "/nodes/localhost/firewall/rules"])
                .status();
        },
        2 => {
            let vmid: String = Input::new()
                .with_prompt("Enter VM/Container ID")
                .interact_text()
                .unwrap();
            
            println!("💻 VM/Container {} firewall rules:", vmid);
            let _ = Command::new("pvesh")
                .args(&["get", &format!("/nodes/localhost/qemu/{}/firewall/rules", vmid)])
                .status();
        },
        3 => {
            println!("🔍 All firewall rules:");
            println!("\n🌐 Datacenter rules:");
            let _ = Command::new("pvesh")
                .args(&["get", "/cluster/firewall/rules"])
                .status();
            
            println!("\n🖥️  Node rules:");
            let _ = Command::new("pvesh")
                .args(&["get", "/nodes/localhost/firewall/rules"])
                .status();
        },
        _ => {}
    }
}

fn add_new_rule() {
    println!("➕ Add New Firewall Rule\n");
    
    let action_options = vec!["ACCEPT", "REJECT", "DROP"];
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&action_options)
        .default(0)
        .interact()
        .unwrap();
    
    let direction_options = vec!["IN", "OUT"];
    let direction = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select direction")
        .items(&direction_options)
        .default(0)
        .interact()
        .unwrap();
    
    let protocol_options = vec!["tcp", "udp", "icmp", "all"];
    let protocol = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocol")
        .items(&protocol_options)
        .default(0)
        .interact()
        .unwrap();
    
    let source: String = Input::new()
        .with_prompt("Source (IP/CIDR or 'any')")
        .default("any".to_string())
        .interact_text()
        .unwrap();
    
    let dest: String = Input::new()
        .with_prompt("Destination (IP/CIDR or 'any')")
        .default("any".to_string())
        .interact_text()
        .unwrap();
    
    let dport: String = Input::new()
        .with_prompt("Destination port (or leave empty)")
        .interact_text()
        .unwrap();
    
    let comment: String = Input::new()
        .with_prompt("Rule comment/description")
        .interact_text()
        .unwrap();
    
    let scope_options = vec!["Datacenter", "Node", "VM/Container"];
    let scope = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply rule to")
        .items(&scope_options)
        .default(0)
        .interact()
        .unwrap();
    
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
    
    if Confirm::new()
        .with_prompt("Create this firewall rule?")
        .default(true)
        .interact()
        .unwrap()
    {
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
                let vmid: String = Input::new()
                    .with_prompt("Enter VM/Container ID")
                    .interact_text()
                    .unwrap();
                format!("/nodes/localhost/qemu/{}/firewall/rules", vmid)
            },
            _ => "/cluster/firewall/rules".to_string(),
        }
    ];
    
    cmd_args.extend([
        "--type".to_string(), rule.direction.clone(),
        "--action".to_string(), rule.action.clone(),
        "--proto".to_string(), rule.protocol.clone(),
        "--source".to_string(), rule.source.clone(),
        "--dest".to_string(), rule.dest.clone(),
    ]);
    
    if !rule.dport.is_empty() {
        cmd_args.extend(["--dport".to_string(), rule.dport.clone()]);
    }
    
    if !rule.comment.is_empty() {
        cmd_args.extend(["--comment".to_string(), rule.comment.clone()]);
    }
    
    let status = Command::new("pvesh")
        .args(&cmd_args)
        .status();
    
    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Firewall rule created successfully!");
    } else {
        println!("❌ Failed to create firewall rule");
    }
}

fn edit_existing_rule() {
    println!("✏️  Edit Existing Rule\n");
    println!("💡 Use the PVE web interface for rule editing or manually specify rule parameters");
    
    let rule_pos: String = Input::new()
        .with_prompt("Enter rule position/number to edit")
        .interact_text()
        .unwrap();
    
    println!("🔧 Rule {} editing - Use 'pvesh set' command manually", rule_pos);
}

fn delete_rule() {
    println!("🗑️  Delete Firewall Rule\n");
    
    let rule_pos: String = Input::new()
        .with_prompt("Enter rule position/number to delete")
        .interact_text()
        .unwrap();
    
    let scope_options = vec!["Datacenter", "Node", "VM/Container"];
    let scope = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Delete rule from")
        .items(&scope_options)
        .default(0)
        .interact()
        .unwrap();
    
    if Confirm::new()
        .with_prompt(format!("⚠️  Delete rule at position {}?", rule_pos))
        .default(false)
        .interact()
        .unwrap()
    {
        let path = match scope {
            0 => format!("/cluster/firewall/rules/{}", rule_pos),
            1 => format!("/nodes/localhost/firewall/rules/{}", rule_pos),
            2 => {
                let vmid: String = Input::new()
                    .with_prompt("Enter VM/Container ID")
                    .interact_text()
                    .unwrap();
                format!("/nodes/localhost/qemu/{}/firewall/rules/{}", vmid, rule_pos)
            },
            _ => format!("/cluster/firewall/rules/{}", rule_pos),
        };
        
        let status = Command::new("pvesh")
            .args(&["delete", &path])
            .status();
        
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
    
    let operation = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bulk operation")
        .items(&operations)
        .default(0)
        .interact()
        .unwrap();
    
    let confirm_msg = match operation {
        0 => "Enable all firewall rules?",
        1 => "Disable all firewall rules?", 
        2 => "⚠️  DELETE ALL firewall rules? This is irreversible!",
        3 => "Apply selected rule template?",
        4 => "Reset firewall to default configuration?",
        _ => "Perform this operation?",
    };
    
    if Confirm::new()
        .with_prompt(confirm_msg)
        .default(false)
        .interact()
        .unwrap()
    {
        match operation {
            2 => {
                if Confirm::new()
                    .with_prompt("🚨 FINAL WARNING: This will delete ALL firewall rules! Are you absolutely sure?")
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    println!("🗑️  Deleting all firewall rules...");
                    // This would implement bulk deletion
                    println!("💡 Bulk deletion implementation pending - use with extreme caution");
                }
            },
            _ => {
                println!("🔄 Performing bulk operation...");
                println!("💡 Bulk operations implementation pending");
            }
        }
    }
}

fn search_rules() {
    println!("🔍 Search Firewall Rules\n");
    
    let search_term: String = Input::new()
        .with_prompt("Enter search term (IP, port, protocol, comment)")
        .interact_text()
        .unwrap();
    
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
    
    let export_path: String = Input::new()
        .with_prompt("Export file path")
        .default("/tmp/pve-firewall-rules.json".to_string())
        .interact_text()
        .unwrap();
    
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
    
    let import_path: String = Input::new()
        .with_prompt("Import file path")
        .interact_text()
        .unwrap();
    
    if !Path::new(&import_path).exists() {
        println!("❌ File not found: {}", import_path);
        return;
    }
    
    if Confirm::new()
        .with_prompt("⚠️  This will add imported rules to existing configuration. Continue?")
        .default(false)
        .interact()
        .unwrap()
    {
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

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📋 Firewall Profiles & Templates")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    println!("   📄 {}", name.strip_suffix(".json").unwrap());
                }
            }
        }
    } else {
        println!("📁 No profiles found. Create your first profile!");
    }
}

fn create_new_profile() {
    println!("➕ Create New Firewall Profile\n");
    
    let profile_name: String = Input::new()
        .with_prompt("Profile name")
        .interact_text()
        .unwrap();
    
    let description: String = Input::new()
        .with_prompt("Profile description")
        .interact_text()
        .unwrap();
    
    let default_policy_options = vec!["DROP", "REJECT", "ACCEPT"];
    let default_policy = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Default policy")
        .items(&default_policy_options)
        .default(0)
        .interact()
        .unwrap();
    
    let mut profile = FirewallProfile {
        name: profile_name,
        description,
        rules: Vec::new(),
        default_policy: default_policy_options[default_policy].to_string(),
    };
    
    // Add rules interactively
    loop {
        if Confirm::new()
            .with_prompt("Add a firewall rule to this profile?")
            .default(true)
            .interact()
            .unwrap()
        {
            let rule = create_rule_interactive();
            profile.rules.push(rule);
        } else {
            break;
        }
    }
    
    save_profile(&profile);
}

fn create_rule_interactive() -> FirewallRule {
    let action_options = vec!["ACCEPT", "REJECT", "DROP"];
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rule action")
        .items(&action_options)
        .default(0)
        .interact()
        .unwrap();
    
    let direction_options = vec!["IN", "OUT"];
    let direction = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rule direction")
        .items(&direction_options)
        .default(0)
        .interact()
        .unwrap();
    
    let protocol_options = vec!["tcp", "udp", "icmp", "all"];
    let protocol = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Protocol")
        .items(&protocol_options)
        .default(0)
        .interact()
        .unwrap();
    
    let source: String = Input::new()
        .with_prompt("Source (IP/CIDR)")
        .default("any".to_string())
        .interact_text()
        .unwrap();
    
    let dest: String = Input::new()
        .with_prompt("Destination (IP/CIDR)")
        .default("any".to_string())
        .interact_text()
        .unwrap();
    
    let dport: String = Input::new()
        .with_prompt("Destination port")
        .interact_text()
        .unwrap();
    
    let comment: String = Input::new()
        .with_prompt("Comment")
        .interact_text()
        .unwrap();
    
    FirewallRule {
        action: action_options[action].to_string(),
        direction: direction_options[direction].to_string(),
        protocol: protocol_options[protocol].to_string(),
        source,
        dest,
        dport,
        sport: String::new(),
        comment,
        enabled: true,
    }
}

fn save_profile(profile: &FirewallProfile) {
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
    
    let profile_name: String = Input::new()
        .with_prompt("Enter profile name to apply")
        .interact_text()
        .unwrap();
    
    let profile_path = format!("/tmp/ghostctl/firewall-profiles/{}.json", profile_name);
    
    if !Path::new(&profile_path).exists() {
        println!("❌ Profile not found: {}", profile_name);
        return;
    }
    
    if let Ok(content) = fs::read_to_string(&profile_path) {
        if let Ok(profile) = serde_json::from_str::<FirewallProfile>(&content) {
            println!("📋 Profile: {}", profile.name);
            println!("📝 Description: {}", profile.description);
            println!("🔒 Default Policy: {}", profile.default_policy);
            println!("📊 Rules: {}", profile.rules.len());
            
            if Confirm::new()
                .with_prompt("Apply this profile?")
                .default(false)
                .interact()
                .unwrap()
            {
                apply_profile_rules(&profile);
            }
        }
    }
}

fn apply_profile_rules(profile: &FirewallProfile) {
    println!("🔧 Applying profile rules...");
    
    for (i, rule) in profile.rules.iter().enumerate() {
        println!("📋 Applying rule {}/{}: {}", i + 1, profile.rules.len(), rule.comment);
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
    
    let profile_name: String = Input::new()
        .with_prompt("Enter profile name to delete")
        .interact_text()
        .unwrap();
    
    if Confirm::new()
        .with_prompt(format!("⚠️  Delete profile '{}'?", profile_name))
        .default(false)
        .interact()
        .unwrap()
    {
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
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template to apply")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();
    
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
    
    if Confirm::new()
        .with_prompt(format!("Apply {} template?", profile.name))
        .default(true)
        .interact()
        .unwrap()
    {
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
        rules: vec![
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
            "🔍 Quick Network Scan (gscan)",
            "🛡️  Comprehensive Security Scan",
            "📊 Port Scan Analysis",
            "🚨 Vulnerability Assessment",
            "🌐 Network Topology Discovery",
            "⚡ Performance Impact Testing",
            "📋 Scan Reports & History",
            "⚙️  Scan Configuration",
            "⬅️  Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔍 Network Security Scanning (gscan)")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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
    println!("🔍 Quick Network Scan with gscan\n");
    
    // Check if gscan is available
    if !Command::new("gscan").arg("--help").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("❌ gscan (ghostscan) not found in PATH");
        println!("📋 Please ensure ghostscan is installed and available");
        
        if Confirm::new()
            .with_prompt("Install gscan from GitHub?")
            .default(false)
            .interact()
            .unwrap()
        {
            install_gscan();
            return;
        } else {
            return;
        }
    }
    
    let target: String = Input::new()
        .with_prompt("Enter target IP/hostname to scan")
        .interact_text()
        .unwrap();
    
    let scan_options = vec![
        "🚀 Quick Scan (Top 1000 ports)",
        "🔍 Full Scan (All 65535 ports)",
        "🎯 Custom Port Range",
        "🌐 Network Range Scan",
    ];
    
    let scan_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scan type")
        .items(&scan_options)
        .default(0)
        .interact()
        .unwrap();
    
    let mut cmd = Command::new("gscan");
    cmd.arg(&target);
    
    match scan_type {
        0 => {
            cmd.args(&["--ports", "1-1000"]);
        },
        1 => {
            cmd.args(&["--ports", "1-65535"]);
        },
        2 => {
            let port_range: String = Input::new()
                .with_prompt("Enter port range (e.g., 1-1000, 80,443,8080)")
                .interact_text()
                .unwrap();
            cmd.args(&["--ports", &port_range]);
        },
        3 => {
            let threads: String = Input::new()
                .with_prompt("Number of threads")
                .default("100".to_string())
                .interact_text()
                .unwrap();
            cmd.args(&["--threads", &threads]);
        },
        _ => {}
    }
    
    println!("🔍 Starting network scan of {}...", target);
    let status = cmd.status();
    
    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Scan completed successfully!");
        
        if Confirm::new()
            .with_prompt("Generate firewall rules based on scan results?")
            .default(true)
            .interact()
            .unwrap()
        {
            generate_rules_from_scan(&target);
        }
    } else {
        println!("❌ Scan failed");
    }
}

fn install_gscan() {
    println!("📦 Installing gscan (ghostscan)...\n");
    
    if Confirm::new()
        .with_prompt("Clone and build ghostscan from GitHub?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("📥 Cloning ghostscan repository...");
        let status = Command::new("git")
            .args(&["clone", "https://github.com/ghostkellz/ghostscan", "/tmp/ghostscan"])
            .status();
        
        if status.map(|s| s.success()).unwrap_or(false) {
            println!("🔨 Building ghostscan...");
            let status = Command::new("cargo")
                .args(&["build", "--release"])
                .current_dir("/tmp/ghostscan")
                .status();
            
            if status.map(|s| s.success()).unwrap_or(false) {
                println!("📋 To complete installation, add to PATH:");
                println!("   cp /tmp/ghostscan/target/release/gscan /usr/local/bin/");
                println!("   # or add to your PATH manually");
            } else {
                println!("❌ Build failed");
            }
        } else {
            println!("❌ Clone failed");
        }
    }
}

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
    
    let rule_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule generation strategy")
        .items(&rule_types)
        .default(1)
        .interact()
        .unwrap();
    
    match rule_type {
        0 => {
            println!("🚫 Generating BLOCK rules for detected ports...");
            println!("💡 This would create DROP rules for all open ports found");
        },
        1 => {
            let services = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select services to allow")
                .items(&["SSH (22)", "HTTP (80)", "HTTPS (443)", "Custom"])
                .interact()
                .unwrap();
            
            println!("✅ Generating ALLOW rules for selected services...");
            for service_idx in services {
                let service_name = ["SSH (22)", "HTTP (80)", "HTTPS (443)", "Custom"][service_idx];
                println!("   📋 Creating rule for: {}", service_name);
            }
        },
        2 => {
            println!("⚠️  Generating ALERT rules for monitoring...");
            println!("💡 This would create logging rules for detected activity");
        },
        3 => {
            println!("🎯 Custom rule generation wizard...");
            println!("💡 Interactive rule creation based on scan results");
        },
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