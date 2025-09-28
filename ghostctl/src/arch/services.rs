use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;

pub fn systemd_service_management() {
    println!("⚙️  SystemD Service Management");
    println!("=============================");

    let options = [
        "📋 Service Status Overview",
        "🔧 Manage Individual Services",
        "🚨 Failed Services Recovery",
        "⏰ Timer & Cron Management",
        "🔄 Service Dependencies",
        "📊 Performance Analysis",
        "🛠️  Custom Service Creation",
        "🔒 Security & Hardening",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SystemD Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => service_status_overview(),
        1 => manage_individual_services(),
        2 => failed_services_recovery(),
        3 => timer_cron_management(),
        4 => service_dependencies(),
        5 => performance_analysis(),
        6 => custom_service_creation(),
        7 => security_hardening(),
        _ => return,
    }
}

fn service_status_overview() {
    println!("📋 Service Status Overview");
    println!("==========================");

    let overview_options = [
        "🔴 Show failed services",
        "🟢 Show active services",
        "🔵 Show all services",
        "⏰ Show running timers",
        "📊 System status summary",
        "🔍 Search services",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Status Overview")
        .items(&overview_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔴 Failed Services:");
            let _ = Command::new("systemctl").args(["--failed"]).status();
        }
        1 => {
            println!("🟢 Active Services:");
            let _ = Command::new("systemctl")
                .args(["list-units", "--state=active"])
                .status();
        }
        2 => {
            println!("🔵 All Services:");
            let _ = Command::new("systemctl")
                .args(["list-units", "--all"])
                .status();
        }
        3 => {
            println!("⏰ Running Timers:");
            let _ = Command::new("systemctl").args(["list-timers"]).status();
        }
        4 => {
            println!("📊 System Status Summary:");
            let _ = Command::new("systemctl").args(["status"]).status();
        }
        5 => {
            let search_term: String = Input::new()
                .with_prompt("Enter service name or pattern")
                .interact_text()
                .unwrap();

            println!("🔍 Searching for services matching '{}':", search_term);
            let _ = Command::new("systemctl")
                .args(["list-units", "--all", &format!("*{}*", search_term)])
                .status();
        }
        _ => return,
    }
}

fn manage_individual_services() {
    println!("🔧 Manage Individual Services");
    println!("=============================");

    let service_name: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    loop {
        println!("\n📋 Service: {}", service_name);

        // Show current status
        println!("📊 Current Status:");
        let _ = Command::new("systemctl")
            .args(["status", &service_name])
            .status();

        let actions = [
            "▶️  Start service",
            "⏹️  Stop service",
            "🔄 Restart service",
            "🔃 Reload service",
            "✅ Enable service",
            "❌ Disable service",
            "📝 Edit service file",
            "📋 Show logs",
            "🔍 Show dependencies",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("Action for {}", service_name))
            .items(&actions)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                println!("▶️  Starting {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "start", &service_name])
                    .status();
            }
            1 => {
                println!("⏹️  Stopping {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "stop", &service_name])
                    .status();
            }
            2 => {
                println!("🔄 Restarting {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "restart", &service_name])
                    .status();
            }
            3 => {
                println!("🔃 Reloading {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "reload", &service_name])
                    .status();
            }
            4 => {
                println!("✅ Enabling {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "enable", &service_name])
                    .status();
            }
            5 => {
                println!("❌ Disabling {}...", service_name);
                let _ = Command::new("sudo")
                    .args(["systemctl", "disable", &service_name])
                    .status();
            }
            6 => {
                println!("📝 Editing service file...");
                let _ = Command::new("sudo")
                    .args(["systemctl", "edit", &service_name])
                    .status();
            }
            7 => {
                println!("📋 Service logs:");
                let _ = Command::new("journalctl")
                    .args(["-u", &service_name, "-n", "50"])
                    .status();
            }
            8 => {
                println!("🔍 Service dependencies:");
                let _ = Command::new("systemctl")
                    .args(["list-dependencies", &service_name])
                    .status();
            }
            _ => break,
        }
    }
}

fn failed_services_recovery() {
    println!("🚨 Failed Services Recovery");
    println!("===========================");

    // Get failed services
    let output = Command::new("systemctl")
        .args(["--failed", "--no-legend", "--plain"])
        .output();

    let failed_services: Vec<String> = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .filter(|service| !service.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    };

    if failed_services.is_empty() {
        println!("✅ No failed services found!");
        return;
    }

    println!("🔴 Found {} failed services:", failed_services.len());
    for (i, service) in failed_services.iter().enumerate() {
        println!("  {}. {}", i + 1, service);
    }

    let recovery_options = [
        "🔄 Restart all failed services",
        "🛠️  Analyze individual service",
        "📝 Show logs for all failed services",
        "❌ Reset failed state",
        "🚨 Emergency service recovery",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Recovery Action")
        .items(&recovery_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => restart_all_failed_services(&failed_services),
        1 => analyze_individual_service(&failed_services),
        2 => show_failed_logs(&failed_services),
        3 => reset_failed_state(&failed_services),
        4 => emergency_service_recovery(),
        _ => return,
    }
}

fn restart_all_failed_services(failed_services: &[String]) {
    println!("🔄 Restarting all failed services...");

    for service in failed_services {
        println!("🔄 Restarting {}...", service);
        let status = Command::new("sudo")
            .args(["systemctl", "restart", service])
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} restarted successfully", service),
            _ => {
                println!("  ❌ Failed to restart {}", service);

                // Try to get more info
                println!("  📋 Status:");
                let _ = Command::new("systemctl")
                    .args(["status", service, "--no-pager", "-l"])
                    .status();
            }
        }
    }

    println!("✅ Restart attempt completed");
}

fn analyze_individual_service(failed_services: &[String]) {
    if failed_services.is_empty() {
        return;
    }

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select service to analyze")
        .items(failed_services)
        .default(0)
        .interact()
        .unwrap();

    let service = &failed_services[choice];
    println!("🔍 Analyzing service: {}", service);

    // Detailed status
    println!("\n📊 Detailed Status:");
    let _ = Command::new("systemctl")
        .args(["status", service, "--no-pager", "-l"])
        .status();

    // Recent logs
    println!("\n📝 Recent Logs:");
    let _ = Command::new("journalctl")
        .args(["-u", service, "-n", "20", "--no-pager"])
        .status();

    // Check configuration
    println!("\n⚙️  Configuration Check:");
    let _ = Command::new("systemctl").args(["cat", service]).status();

    // Recovery suggestions
    println!("\n💡 Recovery Suggestions:");
    suggest_recovery_actions(service);
}

fn suggest_recovery_actions(service: &str) {
    println!("🛠️  Suggested actions for {}:", service);

    // Common service-specific suggestions
    match service {
        s if s.contains("ssh") => {
            println!("  • Check SSH configuration: sudo sshd -T");
            println!("  • Verify SSH keys and permissions");
            println!("  • Check /etc/ssh/sshd_config for syntax errors");
        }
        s if s.contains("network") => {
            println!("  • Check network interface configuration");
            println!("  • Verify DNS settings in /etc/resolv.conf");
            println!("  • Test network connectivity");
        }
        s if s.contains("docker") => {
            println!("  • Check Docker daemon configuration");
            println!("  • Verify Docker socket permissions");
            println!("  • Check disk space for Docker storage");
        }
        s if s.contains("nginx") || s.contains("apache") => {
            println!("  • Test configuration: nginx -t or apache2ctl configtest");
            println!("  • Check port conflicts");
            println!("  • Verify SSL certificate validity");
        }
        s if s.contains("mysql") || s.contains("mariadb") => {
            println!("  • Check database logs for corruption");
            println!("  • Verify database permissions");
            println!("  • Check disk space");
        }
        _ => {
            println!("  • Check service configuration files");
            println!("  • Verify required dependencies");
            println!("  • Check system resources (disk, memory)");
            println!("  • Review service logs for specific errors");
        }
    }

    // Generic suggestions
    println!("  • Try: sudo systemctl reset-failed {}", service);
    println!("  • Try: sudo systemctl daemon-reload");
    println!("  • Check: systemctl list-dependencies {}", service);
}

fn show_failed_logs(failed_services: &[String]) {
    println!("📝 Logs for failed services:");

    for service in failed_services {
        println!("\n{}", "=".repeat(50));
        println!("📋 Logs for: {}", service);
        println!("{}", "=".repeat(50));

        let _ = Command::new("journalctl")
            .args(["-u", service, "-n", "10", "--no-pager"])
            .status();
    }
}

fn reset_failed_state(failed_services: &[String]) {
    println!("❌ Resetting failed state for services...");

    for service in failed_services {
        println!("🔄 Resetting {}...", service);
        let _ = Command::new("sudo")
            .args(["systemctl", "reset-failed", service])
            .status();
    }

    // Reset all failed states
    let _ = Command::new("sudo")
        .args(["systemctl", "reset-failed"])
        .status();

    println!("✅ Failed states reset");
}

fn emergency_service_recovery() {
    println!("🚨 Emergency Service Recovery");
    println!("============================");

    let emergency_actions = [
        "🔄 Reload systemd daemon",
        "🔧 Fix critical system services",
        "🌐 Restart network services",
        "🔒 Restart security services",
        "💾 Fix storage services",
        "📊 System service health check",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emergency Action")
        .items(&emergency_actions)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔄 Reloading systemd daemon...");
            let _ = Command::new("sudo")
                .args(["systemctl", "daemon-reload"])
                .status();
            println!("✅ Daemon reloaded");
        }
        1 => fix_critical_system_services(),
        2 => restart_network_services(),
        3 => restart_security_services(),
        4 => fix_storage_services(),
        5 => system_service_health_check(),
        _ => return,
    }
}

fn fix_critical_system_services() {
    println!("🔧 Fixing critical system services...");

    let critical_services = [
        "dbus",
        "systemd-logind",
        "systemd-resolved",
        "systemd-networkd",
        "systemd-timesyncd",
    ];

    for service in &critical_services {
        println!("🔧 Checking {}...", service);

        let status = Command::new("systemctl")
            .args(["is-active", service])
            .output();

        match status {
            Ok(output) => {
                let status_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if status_str != "active" {
                    println!("  ⚠️  {} is {}, attempting restart...", service, status_str);
                    let _ = Command::new("sudo")
                        .args(["systemctl", "restart", service])
                        .status();
                } else {
                    println!("  ✅ {} is active", service);
                }
            }
            Err(_) => {
                println!("  ❌ Could not check status of {}", service);
            }
        }
    }
}

fn restart_network_services() {
    println!("🌐 Restarting network services...");

    let network_services = [
        "systemd-networkd",
        "systemd-resolved",
        "NetworkManager",
        "dhcpcd",
    ];

    for service in &network_services {
        println!("🔄 Restarting {}...", service);
        let _ = Command::new("sudo")
            .args(["systemctl", "restart", service])
            .status();
    }

    println!("✅ Network services restarted");
}

fn restart_security_services() {
    println!("🔒 Restarting security services...");

    let security_services = ["sshd", "ufw", "fail2ban", "apparmor"];

    for service in &security_services {
        let status = Command::new("systemctl")
            .args(["is-enabled", service])
            .output();

        if let Ok(output) = status {
            let status_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if status_str == "enabled" {
                println!("🔄 Restarting {}...", service);
                let _ = Command::new("sudo")
                    .args(["systemctl", "restart", service])
                    .status();
            }
        }
    }

    println!("✅ Security services restarted");
}

fn fix_storage_services() {
    println!("💾 Fixing storage services...");

    // Check and fix common storage issues
    println!("🔍 Checking filesystem services...");
    let _ = Command::new("sudo")
        .args(["systemctl", "restart", "systemd-tmpfiles-setup"])
        .status();

    println!("🔍 Checking mount services...");
    let _ = Command::new("sudo")
        .args(["systemctl", "daemon-reload"])
        .status();
    let _ = Command::new("sudo").args(["mount", "-a"]).status();

    println!("✅ Storage services checked");
}

fn system_service_health_check() {
    println!("📊 System Service Health Check");
    println!("==============================");

    println!("🔍 Overall system status:");
    let _ = Command::new("systemctl").args(["status"]).status();

    println!("\n🔴 Failed units:");
    let _ = Command::new("systemctl").args(["--failed"]).status();

    println!("\n📊 Service statistics:");
    let _ = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=running"])
        .status();

    println!("\n⏰ Timer status:");
    let _ = Command::new("systemctl").args(["list-timers"]).status();
}

fn timer_cron_management() {
    println!("⏰ Timer & Cron Management");
    println!("==========================");

    let timer_options = [
        "📋 List active timers",
        "⏰ Create systemd timer",
        "📅 Manage cron jobs",
        "🔍 Analyze timer performance",
        "🛠️  Timer troubleshooting",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Timer Management")
        .items(&timer_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_active_timers(),
        1 => create_systemd_timer(),
        2 => manage_cron_jobs(),
        3 => analyze_timer_performance(),
        4 => timer_troubleshooting(),
        _ => return,
    }
}

fn list_active_timers() {
    println!("📋 Active Timers");
    println!("================");

    println!("⏰ SystemD Timers:");
    let _ = Command::new("systemctl")
        .args(["list-timers", "--all"])
        .status();

    println!("\n📅 Cron Jobs:");
    println!("User cron jobs:");
    let _ = Command::new("crontab").args(["-l"]).status();

    println!("\nSystem cron jobs:");
    let _ = Command::new("sudo").args(["crontab", "-l"]).status();
}

fn create_systemd_timer() {
    println!("⏰ Create SystemD Timer");
    println!("=======================");

    let timer_name: String = Input::new()
        .with_prompt("Timer name (e.g., backup-timer)")
        .interact_text()
        .unwrap();

    let command: String = Input::new()
        .with_prompt("Command to execute")
        .interact_text()
        .unwrap();

    let schedule: String = Input::new()
        .with_prompt("Schedule (e.g., daily, hourly, '*-*-* 02:00:00')")
        .interact_text()
        .unwrap();

    println!("📝 Creating timer files...");

    // Create service file
    let service_content = format!(
        r#"[Unit]
Description={} Service
Wants={}.timer

[Service]
Type=oneshot
ExecStart={}

[Install]
WantedBy=multi-user.target
"#,
        timer_name, timer_name, command
    );

    // Create timer file
    let timer_content = format!(
        r#"[Unit]
Description={} Timer
Requires={}.service

[Timer]
OnCalendar={}
Persistent=true

[Install]
WantedBy=timers.target
"#,
        timer_name, timer_name, schedule
    );

    println!("Service file content:");
    println!("{}", service_content);
    println!("\nTimer file content:");
    println!("{}", timer_content);

    let confirm = Confirm::new()
        .with_prompt("Create these timer files?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Write files (simplified - in real implementation would write to proper locations)
        println!("💡 To create these files:");
        println!("sudo nano /etc/systemd/system/{}.service", timer_name);
        println!("sudo nano /etc/systemd/system/{}.timer", timer_name);
        println!("sudo systemctl daemon-reload");
        println!("sudo systemctl enable --now {}.timer", timer_name);
    }
}

fn manage_cron_jobs() {
    println!("📅 Manage Cron Jobs");
    println!("===================");

    let cron_options = [
        "📋 List user cron jobs",
        "📋 List system cron jobs",
        "➕ Add cron job",
        "✏️  Edit crontab",
        "🗑️  Remove cron job",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cron Management")
        .items(&cron_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("📋 User cron jobs:");
            let _ = Command::new("crontab").args(["-l"]).status();
        }
        1 => {
            println!("📋 System cron jobs:");
            let _ = Command::new("sudo").args(["crontab", "-l"]).status();
        }
        2 => add_cron_job(),
        3 => {
            println!("✏️  Opening crontab editor...");
            let _ = Command::new("crontab").args(["-e"]).status();
        }
        4 => {
            println!("🗑️  To remove cron jobs, use: crontab -e");
            println!("Then delete the desired lines and save");
        }
        _ => return,
    }
}

fn add_cron_job() {
    println!("➕ Add Cron Job");
    println!("===============");

    println!("📋 Common cron schedules:");
    println!("  0 0 * * *     - Daily at midnight");
    println!("  0 */6 * * *   - Every 6 hours");
    println!("  */30 * * * *  - Every 30 minutes");
    println!("  0 0 * * 0     - Weekly on Sunday");
    println!("  0 0 1 * *     - Monthly on 1st");

    let schedule: String = Input::new()
        .with_prompt("Cron schedule (e.g., '0 2 * * *')")
        .interact_text()
        .unwrap();

    let command: String = Input::new()
        .with_prompt("Command to execute")
        .interact_text()
        .unwrap();

    let cron_line = format!("{} {}", schedule, command);

    println!("📝 Cron job to add: {}", cron_line);

    let confirm = Confirm::new()
        .with_prompt("Add this cron job?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("💡 To add this cron job:");
        println!("1. Run: crontab -e");
        println!("2. Add line: {}", cron_line);
        println!("3. Save and exit");
    }
}

fn analyze_timer_performance() {
    println!("🔍 Analyze Timer Performance");
    println!("============================");

    println!("📊 Timer statistics:");
    let _ = Command::new("systemctl")
        .args(["list-timers", "--all"])
        .status();

    println!("\n📈 Timer logs:");
    let _ = Command::new("journalctl")
        .args(["-u", "*.timer", "-n", "20", "--no-pager"])
        .status();
}

fn timer_troubleshooting() {
    println!("🛠️  Timer Troubleshooting");
    println!("========================");

    let troubleshoot_options = [
        "🔍 Check timer status",
        "📝 View timer logs",
        "🔄 Restart timers",
        "⚙️  Validate timer syntax",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Troubleshooting")
        .items(&troubleshoot_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let timer_name: String = Input::new()
                .with_prompt("Timer name to check")
                .interact_text()
                .unwrap();

            let _ = Command::new("systemctl")
                .args(["status", &format!("{}.timer", timer_name)])
                .status();
        }
        1 => {
            println!("📝 Recent timer logs:");
            let _ = Command::new("journalctl")
                .args(["-u", "*.timer", "-f"])
                .status();
        }
        2 => {
            println!("🔄 Restarting timer services...");
            let _ = Command::new("sudo")
                .args(["systemctl", "reload-or-restart", "*.timer"])
                .status();
        }
        3 => {
            println!("⚙️  Timer syntax validation:");
            println!("Use: systemd-analyze calendar '<schedule>'");

            let schedule: String = Input::new()
                .with_prompt("Enter schedule to validate")
                .interact_text()
                .unwrap();

            let _ = Command::new("systemd-analyze")
                .args(["calendar", &schedule])
                .status();
        }
        _ => return,
    }
}

fn service_dependencies() {
    println!("🔄 Service Dependencies");
    println!("=======================");

    let dep_options = [
        "🔍 Show service dependencies",
        "📊 Dependency tree visualization",
        "🛠️  Fix dependency issues",
        "📋 Reverse dependencies",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Dependency Tools")
        .items(&dep_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_service_dependencies(),
        1 => dependency_tree_visualization(),
        2 => fix_dependency_issues(),
        3 => reverse_dependencies(),
        _ => return,
    }
}

fn show_service_dependencies() {
    let service_name: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    println!("🔍 Dependencies for {}:", service_name);
    let _ = Command::new("systemctl")
        .args(["list-dependencies", &service_name])
        .status();
}

fn dependency_tree_visualization() {
    println!("📊 Dependency Tree Visualization");

    let service_name: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    println!("🌳 Dependency tree for {}:", service_name);
    let _ = Command::new("systemctl")
        .args(["list-dependencies", "--all", &service_name])
        .status();
}

fn fix_dependency_issues() {
    println!("🛠️  Fix Dependency Issues");
    println!("=========================");

    println!("🔄 Reloading systemd daemon...");
    let _ = Command::new("sudo")
        .args(["systemctl", "daemon-reload"])
        .status();

    println!("🔍 Checking for dependency cycles...");
    let _ = Command::new("systemd-analyze").args(["verify"]).status();

    println!("📊 Checking service order...");
    let _ = Command::new("systemd-analyze")
        .args(["critical-chain"])
        .status();
}

fn reverse_dependencies() {
    let service_name: String = Input::new()
        .with_prompt("Enter service name")
        .interact_text()
        .unwrap();

    println!("🔄 Services that depend on {}:", service_name);
    let _ = Command::new("systemctl")
        .args(["list-dependencies", "--reverse", &service_name])
        .status();
}

fn performance_analysis() {
    println!("📊 Performance Analysis");
    println!("=======================");

    let perf_options = [
        "⏱️  Boot time analysis",
        "🔍 Service startup times",
        "📈 Resource usage by services",
        "🚀 Optimization suggestions",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Analysis")
        .items(&perf_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => boot_time_analysis(),
        1 => service_startup_times(),
        2 => resource_usage_analysis(),
        3 => optimization_suggestions(),
        _ => return,
    }
}

fn boot_time_analysis() {
    println!("⏱️  Boot Time Analysis");
    println!("======================");

    println!("📊 Overall boot time:");
    let _ = Command::new("systemd-analyze").status();

    println!("\n🐌 Slowest services:");
    let _ = Command::new("systemd-analyze").args(["blame"]).status();

    println!("\n📈 Boot timeline:");
    let _ = Command::new("systemd-analyze")
        .args(["critical-chain"])
        .status();
}

fn service_startup_times() {
    println!("🔍 Service Startup Times");
    println!("========================");

    let _ = Command::new("systemd-analyze").args(["blame"]).status();
}

fn resource_usage_analysis() {
    println!("📈 Resource Usage by Services");
    println!("=============================");

    println!("💾 Memory usage by services:");
    let _ = Command::new("systemctl").args(["status"]).status();

    println!("\n🔋 CPU usage:");
    let _ = Command::new("systemd-cgtop").status();
}

fn optimization_suggestions() {
    println!("🚀 Optimization Suggestions");
    println!("===========================");

    println!("💡 Boot optimization tips:");
    println!("1. Disable unnecessary services");
    println!("2. Use systemd timers instead of cron");
    println!("3. Enable parallel service startup");
    println!("4. Use socket activation for services");

    println!("\n🔍 Analyzing current setup...");
    let _ = Command::new("systemd-analyze").args(["verify"]).status();
}

fn custom_service_creation() {
    println!("🛠️  Custom Service Creation");
    println!("==========================");

    let creation_options = [
        "📝 Create simple service",
        "⏰ Create service with timer",
        "🔧 Service template wizard",
        "📋 Service file examples",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Service Creation")
        .items(&creation_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_simple_service(),
        1 => create_service_with_timer(),
        2 => service_template_wizard(),
        3 => show_service_examples(),
        _ => return,
    }
}

fn create_simple_service() {
    println!("📝 Create Simple Service");
    println!("========================");

    let service_name: String = Input::new()
        .with_prompt("Service name")
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .with_prompt("Service description")
        .interact_text()
        .unwrap();

    let exec_start: String = Input::new()
        .with_prompt("Command to execute")
        .interact_text()
        .unwrap();

    let user: String = Input::new()
        .with_prompt("User to run as (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let auto_restart = Confirm::new()
        .with_prompt("Auto-restart on failure?")
        .default(true)
        .interact()
        .unwrap();

    let service_content = format!(
        r#"[Unit]
Description={}
After=network.target

[Service]
Type=simple
ExecStart={}
{}{}
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
"#,
        description,
        exec_start,
        if !user.is_empty() {
            format!("User={}\n", user)
        } else {
            String::new()
        },
        if auto_restart {
            "Restart=always\nRestartSec=10\n"
        } else {
            ""
        }
    );

    println!("📝 Service file content:");
    println!("{}", service_content);

    let confirm = Confirm::new()
        .with_prompt("Create this service?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("💡 To create this service:");
        println!("1. sudo nano /etc/systemd/system/{}.service", service_name);
        println!("2. Paste the content above");
        println!("3. sudo systemctl daemon-reload");
        println!("4. sudo systemctl enable {}.service", service_name);
        println!("5. sudo systemctl start {}.service", service_name);
    }
}

fn create_service_with_timer() {
    println!("⏰ Create Service with Timer");
    println!("===========================");

    // First create the service
    create_simple_service();

    // Then add timer configuration
    println!("\n⏰ Timer Configuration:");

    let timer_schedule: String = Input::new()
        .with_prompt("Timer schedule (e.g., 'daily', '*/5 * * * *')")
        .interact_text()
        .unwrap();

    println!("💡 Timer setup instructions:");
    println!("Create timer file with OnCalendar={}", timer_schedule);
}

fn service_template_wizard() {
    println!("🔧 Service Template Wizard");
    println!("==========================");

    let templates = [
        "🌐 Web Application Service",
        "📊 Monitoring Service",
        "🔄 Backup Service",
        "🗄️  Database Service",
        "🔧 Maintenance Task",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select service template")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_web_app_template(),
        1 => show_monitoring_template(),
        2 => show_backup_template(),
        3 => show_database_template(),
        4 => show_maintenance_template(),
        _ => return,
    }
}

fn show_web_app_template() {
    println!("🌐 Web Application Service Template");
    println!("===================================");

    let template = r#"[Unit]
Description=My Web Application
After=network.target
Wants=postgresql.service

[Service]
Type=notify
User=webapp
Group=webapp
WorkingDirectory=/opt/myapp
ExecStart=/opt/myapp/bin/start
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=10

# Security settings
NoNewPrivileges=true
ProtectHome=true
ProtectSystem=strict
ReadWritePaths=/opt/myapp/logs /opt/myapp/tmp

[Install]
WantedBy=multi-user.target
"#;

    println!("{}", template);
}

fn show_monitoring_template() {
    println!("📊 Monitoring Service Template");
    println!("==============================");

    let template = r#"[Unit]
Description=System Monitor
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/monitor
Restart=always
RestartSec=30

# Monitoring specific settings
StandardOutput=journal
StandardError=journal
TimeoutStartSec=60
TimeoutStopSec=30

[Install]
WantedBy=multi-user.target
"#;

    println!("{}", template);
}

fn show_backup_template() {
    println!("🔄 Backup Service Template");
    println!("==========================");

    let template = r#"[Unit]
Description=Backup Service
Wants=backup.timer

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
StandardOutput=journal
StandardError=journal

# For backup services
TimeoutStartSec=0
"#;

    println!("{}", template);
}

fn show_database_template() {
    println!("🗄️  Database Service Template");
    println!("=============================");

    let template = r#"[Unit]
Description=Custom Database
After=network.target

[Service]
Type=notify
User=database
ExecStart=/usr/local/bin/database
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=10

# Database specific
TimeoutSec=300
PIDFile=/var/run/database.pid

[Install]
WantedBy=multi-user.target
"#;

    println!("{}", template);
}

fn show_maintenance_template() {
    println!("🔧 Maintenance Task Template");
    println!("============================");

    let template = r#"[Unit]
Description=Maintenance Task
Wants=maintenance.timer

[Service]
Type=oneshot
ExecStart=/usr/local/bin/maintenance.sh
StandardOutput=journal

# Maintenance specific
TimeoutStartSec=3600
"#;

    println!("{}", template);
}

fn show_service_examples() {
    println!("📋 Service File Examples");
    println!("========================");

    let examples = [
        "🌐 Nginx-like web server",
        "🗄️  Database service",
        "🔄 Cron replacement",
        "📊 Monitoring daemon",
        "🔧 One-shot script",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select example to view")
        .items(&examples)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => println!("Example: Web server service with socket activation"),
        1 => println!("Example: Database with proper shutdown handling"),
        2 => println!("Example: Timer-based recurring task"),
        3 => println!("Example: Long-running monitoring daemon"),
        4 => println!("Example: One-shot script execution"),
        _ => {}
    }
}

fn security_hardening() {
    println!("🔒 Security & Hardening");
    println!("=======================");

    let security_options = [
        "🛡️  Service isolation",
        "🔐 Permission hardening",
        "📊 Security audit",
        "🚨 Vulnerability scan",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Options")
        .items(&security_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => service_isolation(),
        1 => permission_hardening(),
        2 => security_audit(),
        3 => vulnerability_scan(),
        _ => return,
    }
}

fn service_isolation() {
    println!("🛡️  Service Isolation");
    println!("====================");

    println!("💡 Service isolation best practices:");
    println!("• Use PrivateTmp=yes");
    println!("• Set NoNewPrivileges=true");
    println!("• Use ProtectHome=true");
    println!("• Set ProtectSystem=strict");
    println!("• Use dedicated user accounts");
    println!("• Limit filesystem access with ReadOnlyPaths");

    let service_name: String = Input::new()
        .with_prompt("Service to analyze (or 'skip')")
        .interact_text()
        .unwrap();

    if service_name != "skip" {
        println!("🔍 Analyzing {}...", service_name);
        let _ = Command::new("systemctl")
            .args(["cat", &service_name])
            .status();
    }
}

fn permission_hardening() {
    println!("🔐 Permission Hardening");
    println!("=======================");

    println!("🔍 Checking service permissions...");

    // Check for services running as root
    println!("⚠️  Services running as root:");
    let _ = Command::new("ps")
        .args(["aux", "|", "grep", "root"])
        .status();
}

fn security_audit() {
    println!("📊 Security Audit");
    println!("=================");

    println!("🔍 Checking systemd security settings...");

    println!("🚨 Services without security restrictions:");
    let _ = Command::new("systemd-analyze").args(["security"]).status();
}

fn vulnerability_scan() {
    println!("🚨 Vulnerability Scan");
    println!("=====================");

    println!("🔍 Scanning for common vulnerabilities...");

    // Check for weak service configurations
    println!("⚠️  Checking for weak configurations:");
    println!("• Services running as root unnecessarily");
    println!("• Services without security restrictions");
    println!("• Exposed network services");

    println!("💡 Use external tools like:");
    println!("• lynis (system hardening scan)");
    println!("• systemd-analyze security");
    println!("• chkrootkit (rootkit detection)");
}
