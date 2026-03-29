use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn azure_cli_tools() {
    println!("🔷 Azure CLI Tools");

    // Check if Azure CLI is installed
    if Command::new("az").arg("version").output().is_err() {
        println!("📦 Installing Azure CLI...");

        let install_method = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Installation method")
            .items(&[
                "Package manager (recommended)",
                "pip install",
                "Manual install",
            ])
            .default(0)
            .interact_opt()
            .ok()
            .flatten();

        let Some(install_method) = install_method else {
            return;
        };

        match install_method {
            0 => {
                println!("📦 Installing via package manager...");
                let _ = Command::new("curl")
                    .args(["-sL", "https://aka.ms/InstallAzureCLIDeb"])
                    .arg("|")
                    .arg("sudo")
                    .arg("bash")
                    .status();
            }
            1 => {
                let _ = Command::new("pip")
                    .args(["install", "--user", "azure-cli"])
                    .status();
            }
            _ => {
                println!("📖 Manual installation:");
                println!("Visit: https://docs.microsoft.com/en-us/cli/azure/install-azure-cli");
                return;
            }
        }
    } else {
        println!("✅ Azure CLI is installed");

        // Show current version
        let _ = Command::new("az").arg("version").status();
    }

    let azure_actions = [
        "🔧 Login to Azure",
        "📋 List subscriptions",
        "🏢 List resource groups",
        "🖥️  List virtual machines",
        "🗂️  List storage accounts",
        "🌐 List virtual networks",
        "🔒 List key vaults",
        "🐳 List container registries",
        "☁️  List App Services",
        "🗄️  List SQL databases",
        "⚡ List Azure Functions",
        "🚀 List AKS clusters",
        "💰 Cost analysis",
        "🔍 Resource search",
        "📊 Show resource usage",
        "⚙️  Azure configuration",
        "⬅️  Back",
    ];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Azure CLI Actions")
        .items(&azure_actions)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(action) = action else {
        return;
    };

    match action {
        0 => azure_login(),
        1 => list_subscriptions(),
        2 => list_resource_groups(),
        3 => list_virtual_machines(),
        4 => list_storage_accounts(),
        5 => list_virtual_networks(),
        6 => list_key_vaults(),
        7 => list_container_registries(),
        8 => list_app_services(),
        9 => list_sql_databases(),
        10 => list_azure_functions(),
        11 => list_aks_clusters(),
        12 => azure_cost_analysis(),
        13 => azure_resource_search(),
        14 => show_resource_usage(),
        15 => azure_configuration(),
        _ => return,
    }
}

fn azure_login() {
    println!("🔧 Logging into Azure...");
    let _ = Command::new("az").args(["login"]).status();

    // Show current account info
    println!("📋 Current account information:");
    let _ = Command::new("az")
        .args(["account", "show", "--output", "table"])
        .status();
}

fn list_subscriptions() {
    println!("📋 Azure Subscriptions:");
    let _ = Command::new("az")
        .args(["account", "list", "--output", "table"])
        .status();
}

fn list_resource_groups() {
    println!("🏢 Resource Groups:");
    let _ = Command::new("az")
        .args(["group", "list", "--output", "table"])
        .status();

    let options = [
        "📊 Resource group details",
        "💰 Cost by resource group",
        "🗑️  Delete resource group",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Resource Group Actions")
        .items(&options)
        .default(3)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => resource_group_details(),
        1 => cost_by_resource_group(),
        2 => delete_resource_group(),
        _ => return,
    }
}

fn list_virtual_machines() {
    println!("🖥️  Virtual Machines:");
    let _ = Command::new("az")
        .args(["vm", "list", "--output", "table"])
        .status();

    let show_details = Confirm::new()
        .with_prompt("Show VM details with sizes and status?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if show_details {
        let _ = Command::new("az")
            .args(["vm", "list", "--show-details", "--output", "table"])
            .status();
    }

    let options = [
        "🔄 Start/Stop VMs",
        "📊 VM performance metrics",
        "💰 VM cost analysis",
        "🔧 VM configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VM Actions")
        .items(&options)
        .default(4)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => vm_start_stop(),
        1 => vm_performance_metrics(),
        2 => vm_cost_analysis(),
        3 => vm_configuration(),
        _ => return,
    }
}

fn list_storage_accounts() {
    println!("🗂️  Storage Accounts:");
    let _ = Command::new("az")
        .args(["storage", "account", "list", "--output", "table"])
        .status();

    let options = [
        "📊 Storage usage",
        "🔒 Access keys",
        "📁 List containers",
        "💰 Storage costs",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Storage Actions")
        .items(&options)
        .default(4)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => storage_usage(),
        1 => storage_access_keys(),
        2 => list_storage_containers(),
        3 => storage_costs(),
        _ => return,
    }
}

fn list_virtual_networks() {
    println!("🌐 Virtual Networks:");
    let _ = Command::new("az")
        .args(["network", "vnet", "list", "--output", "table"])
        .status();
}

fn list_key_vaults() {
    println!("🔒 Key Vaults:");
    let _ = Command::new("az")
        .args(["keyvault", "list", "--output", "table"])
        .status();
}

fn list_container_registries() {
    println!("🐳 Container Registries:");
    let _ = Command::new("az")
        .args(["acr", "list", "--output", "table"])
        .status();
}

fn list_app_services() {
    println!("☁️  App Services:");
    let _ = Command::new("az")
        .args(["webapp", "list", "--output", "table"])
        .status();
}

fn list_sql_databases() {
    println!("🗄️  SQL Databases:");
    let _ = Command::new("az")
        .args(["sql", "server", "list", "--output", "table"])
        .status();

    let show_databases = Confirm::new()
        .with_prompt("Show databases for each server?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if show_databases {
        println!("📋 Listing all databases...");
        let _ = Command::new("az")
            .args([
                "sql",
                "db",
                "list",
                "--server",
                "your-server",
                "--resource-group",
                "your-rg",
                "--output",
                "table",
            ])
            .status();
    }
}

fn list_azure_functions() {
    println!("⚡ Azure Functions:");
    let _ = Command::new("az")
        .args(["functionapp", "list", "--output", "table"])
        .status();
}

fn list_aks_clusters() {
    println!("🚀 AKS Clusters:");
    let _ = Command::new("az")
        .args(["aks", "list", "--output", "table"])
        .status();
}

fn azure_cost_analysis() {
    println!("💰 Cost Analysis:");
    println!("📊 Getting cost analysis (last 30 days)...");
    let _ = Command::new("az")
        .args([
            "consumption",
            "usage",
            "list",
            "--start-date",
            "2024-01-01",
            "--end-date",
            "2024-01-31",
            "--output",
            "table",
        ])
        .status();
}

fn azure_resource_search() {
    let search_term: String = Input::new()
        .with_prompt("Search for resources (name/type)")
        .interact_text()
        .unwrap_or_default();

    if search_term.is_empty() {
        return;
    }

    println!("🔍 Searching for: {}", search_term);
    let _ = Command::new("az")
        .args([
            "resource",
            "list",
            "--query",
            &format!("[?contains(name, '{}')]", search_term),
            "--output",
            "table",
        ])
        .status();
}

fn show_resource_usage() {
    println!("📊 Resource Usage and Quotas:");
    let _ = Command::new("az")
        .args([
            "vm",
            "list-usage",
            "--location",
            "eastus",
            "--output",
            "table",
        ])
        .status();
}

fn azure_configuration() {
    println!("⚙️  Azure CLI Configuration:");
    let _ = Command::new("az")
        .args(["configure", "--list-defaults"])
        .status();

    let config_action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration action")
        .items(&[
            "Set default location",
            "Set default resource group",
            "Show all config",
            "Back",
        ])
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(config_action) = config_action else {
        return;
    };

    match config_action {
        0 => set_default_location(),
        1 => set_default_resource_group(),
        2 => show_all_config(),
        _ => {}
    }
}

// Helper function implementations
fn resource_group_details() {
    let rg_name: String = Input::new()
        .with_prompt("Resource group name")
        .interact_text()
        .unwrap_or_default();

    if rg_name.is_empty() {
        return;
    }

    println!("📊 Resource group details for: {}", rg_name);
    let _ = Command::new("az")
        .args(["group", "show", "--name", &rg_name, "--output", "table"])
        .status();

    println!("📋 Resources in group:");
    let _ = Command::new("az")
        .args([
            "resource",
            "list",
            "--resource-group",
            &rg_name,
            "--output",
            "table",
        ])
        .status();
}

fn cost_by_resource_group() {
    println!("💰 Cost by Resource Group");
    // Implementation for cost analysis by resource group
}

fn delete_resource_group() {
    let rg_name: String = Input::new()
        .with_prompt("Resource group name to delete")
        .interact_text()
        .unwrap_or_default();

    if rg_name.is_empty() {
        return;
    }

    let confirm = Confirm::new()
        .with_prompt(format!("⚠️  Are you sure you want to delete '{}'? This will delete ALL resources in the group!", rg_name))
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if confirm {
        println!("🗑️  Deleting resource group: {}", rg_name);
        let _ = Command::new("az")
            .args(["group", "delete", "--name", &rg_name, "--yes", "--no-wait"])
            .status();
    }
}

fn vm_start_stop() {
    let vm_name: String = Input::new()
        .with_prompt("VM name")
        .interact_text()
        .unwrap_or_default();

    if vm_name.is_empty() {
        return;
    }

    let rg_name: String = Input::new()
        .with_prompt("Resource group name")
        .interact_text()
        .unwrap_or_default();

    if rg_name.is_empty() {
        return;
    }

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VM Action")
        .items(&["Start", "Stop", "Restart", "Deallocate"])
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(action) = action else {
        return;
    };

    let command = match action {
        0 => "start",
        1 => "stop",
        2 => "restart",
        3 => "deallocate",
        _ => return,
    };

    println!("🔄 {}ing VM: {}", command, vm_name);
    let _ = Command::new("az")
        .args([
            "vm",
            command,
            "--name",
            &vm_name,
            "--resource-group",
            &rg_name,
        ])
        .status();
}

fn vm_performance_metrics() {
    println!("📊 VM Performance Metrics");
    // Implementation for VM performance metrics
}

fn vm_cost_analysis() {
    println!("💰 VM Cost Analysis");
    // Implementation for VM cost analysis
}

fn vm_configuration() {
    println!("🔧 VM Configuration");
    // Implementation for VM configuration
}

fn storage_usage() {
    println!("📊 Storage Usage");
    // Implementation for storage usage analysis
}

fn storage_access_keys() {
    let storage_name: String = Input::new()
        .with_prompt("Storage account name")
        .interact_text()
        .unwrap_or_default();

    if storage_name.is_empty() {
        return;
    }

    let rg_name: String = Input::new()
        .with_prompt("Resource group name")
        .interact_text()
        .unwrap_or_default();

    if rg_name.is_empty() {
        return;
    }

    println!("🔒 Storage account access keys:");
    let _ = Command::new("az")
        .args([
            "storage",
            "account",
            "keys",
            "list",
            "--account-name",
            &storage_name,
            "--resource-group",
            &rg_name,
            "--output",
            "table",
        ])
        .status();
}

fn list_storage_containers() {
    let storage_name: String = Input::new()
        .with_prompt("Storage account name")
        .interact_text()
        .unwrap_or_default();

    if storage_name.is_empty() {
        return;
    }

    println!("📁 Storage containers:");
    let _ = Command::new("az")
        .args([
            "storage",
            "container",
            "list",
            "--account-name",
            &storage_name,
            "--output",
            "table",
        ])
        .status();
}

fn storage_costs() {
    println!("💰 Storage Costs Analysis");
    // Implementation for storage cost analysis
}

fn set_default_location() {
    let location: String = Input::new()
        .with_prompt("Default location (e.g., eastus, westus2)")
        .default("eastus".into())
        .interact_text()
        .unwrap_or_default();

    if location.is_empty() {
        return;
    }

    let _ = Command::new("az")
        .args(["configure", "--defaults", &format!("location={}", location)])
        .status();

    println!("✅ Default location set to: {}", location);
}

fn set_default_resource_group() {
    let rg: String = Input::new()
        .with_prompt("Default resource group")
        .interact_text()
        .unwrap_or_default();

    if rg.is_empty() {
        return;
    }

    let _ = Command::new("az")
        .args(["configure", "--defaults", &format!("group={}", rg)])
        .status();

    println!("✅ Default resource group set to: {}", rg);
}

fn show_all_config() {
    let _ = Command::new("az")
        .args(["configure", "--list-defaults"])
        .status();
}
