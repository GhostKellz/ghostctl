use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn aws_cli_tools() {
    println!("☁️  AWS CLI Tools");

    // Check if AWS CLI is installed
    if Command::new("aws").arg("--version").output().is_err() {
        println!("📦 Installing AWS CLI...");

        let install_method = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Installation method")
            .items(&["pip install", "Download installer", "Package manager"])
            .default(0)
            .interact_opt()
            .ok()
            .flatten();

        let Some(install_method) = install_method else {
            return;
        };

        match install_method {
            0 => {
                let _ = Command::new("pip")
                    .args(["install", "--user", "awscli"])
                    .status();
            }
            1 => {
                println!("📖 Download AWS CLI v2:");
                println!(
                    "curl 'https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip' -o 'awscliv2.zip'"
                );
                println!("unzip awscliv2.zip && sudo ./aws/install");
                return;
            }
            _ => {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "aws-cli"])
                    .status();
            }
        }
    }

    let aws_actions = [
        "🔧 Configure AWS credentials",
        "📋 List EC2 instances",
        "🗂️  List S3 buckets",
        "🌐 List VPCs",
        "💰 Cost estimation",
        "📊 CloudWatch metrics",
        "🔒 Security groups",
        "🚀 Lambda functions",
        "📡 RDS databases",
        "⚡ ECS clusters",
        "📈 Auto Scaling groups",
        "🔐 IAM management",
        "⬅️  Back",
    ];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AWS CLI Actions")
        .items(&aws_actions)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(action) = action else {
        return;
    };

    match action {
        0 => configure_aws_credentials(),
        1 => list_ec2_instances(),
        2 => list_s3_buckets(),
        3 => list_vpcs(),
        4 => aws_cost_estimation(),
        5 => cloudwatch_metrics(),
        6 => manage_security_groups(),
        7 => manage_lambda_functions(),
        8 => manage_rds_databases(),
        9 => manage_ecs_clusters(),
        10 => manage_autoscaling_groups(),
        11 => manage_iam(),
        _ => return,
    }
}

fn configure_aws_credentials() {
    println!("🔧 Configuring AWS CLI...");
    let _ = Command::new("aws").arg("configure").status();

    // Test connection
    println!("🔍 Testing AWS connection...");
    let status = Command::new("aws")
        .args(["sts", "get-caller-identity"])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ AWS credentials configured successfully!"),
        _ => println!("❌ Failed to verify AWS credentials"),
    }
}

fn list_ec2_instances() {
    println!("📋 EC2 Instances:");
    let _ = Command::new("aws")
        .args(["ec2", "describe-instances", "--query", 
               "Reservations[*].Instances[*].[InstanceId,State.Name,InstanceType,PublicIpAddress,Tags[?Key=='Name'].Value|[0]]",
               "--output", "table"])
        .status();

    // Show additional options
    let options = [
        "🔍 Filter by state",
        "📊 Instance types summary",
        "💰 Cost per instance",
        "🔄 Start/Stop instances",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("EC2 Actions")
        .items(&options)
        .default(4)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => filter_instances_by_state(),
        1 => instance_types_summary(),
        2 => instance_cost_analysis(),
        3 => start_stop_instances(),
        _ => return,
    }
}

fn list_s3_buckets() {
    println!("🗂️  S3 Buckets:");
    let _ = Command::new("aws").args(["s3", "ls"]).status();

    let options = [
        "📊 Bucket sizes",
        "🔒 Bucket policies",
        "📁 List bucket contents",
        "💰 Storage costs",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("S3 Actions")
        .items(&options)
        .default(4)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => bucket_sizes(),
        1 => bucket_policies(),
        2 => list_bucket_contents(),
        3 => storage_costs(),
        _ => return,
    }
}

fn list_vpcs() {
    println!("🌐 VPCs:");
    let _ = Command::new("aws")
        .args(["ec2", "describe-vpcs", "--output", "table"])
        .status();
}

fn aws_cost_estimation() {
    println!("💰 AWS Cost Analysis");

    let options = [
        "📊 Current month costs",
        "📈 Last 30 days trend",
        "🏷️  Costs by service",
        "🔍 Cost explorer",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cost Analysis")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => current_month_costs(),
        1 => cost_trend(),
        2 => costs_by_service(),
        3 => cost_explorer(),
        _ => return,
    }
}

fn cloudwatch_metrics() {
    println!("📊 CloudWatch Metrics");

    let options = [
        "🖥️  EC2 metrics",
        "🗂️  S3 metrics",
        "🚀 Lambda metrics",
        "📡 RDS metrics",
        "📈 Custom metrics",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("CloudWatch Metrics")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => ec2_metrics(),
        1 => s3_metrics(),
        2 => lambda_metrics(),
        3 => rds_metrics(),
        4 => custom_metrics(),
        _ => return,
    }
}

fn manage_security_groups() {
    println!("🔒 Security Groups Management");

    let _ = Command::new("aws")
        .args(["ec2", "describe-security-groups", "--output", "table"])
        .status();
}

fn manage_lambda_functions() {
    println!("🚀 Lambda Functions Management");

    let _ = Command::new("aws")
        .args(["lambda", "list-functions", "--output", "table"])
        .status();
}

fn manage_rds_databases() {
    println!("📡 RDS Databases Management");

    let _ = Command::new("aws")
        .args(["rds", "describe-db-instances", "--output", "table"])
        .status();
}

fn manage_ecs_clusters() {
    println!("⚡ ECS Clusters Management");

    let _ = Command::new("aws")
        .args(["ecs", "list-clusters", "--output", "table"])
        .status();
}

fn manage_autoscaling_groups() {
    println!("📈 Auto Scaling Groups Management");

    let _ = Command::new("aws")
        .args([
            "autoscaling",
            "describe-auto-scaling-groups",
            "--output",
            "table",
        ])
        .status();
}

fn manage_iam() {
    println!("🔐 IAM Management");

    let options = [
        "👥 List users",
        "🏷️  List roles",
        "📋 List policies",
        "🔑 Access keys",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("IAM Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => list_iam_users(),
        1 => list_iam_roles(),
        2 => list_iam_policies(),
        3 => manage_access_keys(),
        _ => return,
    }
}

// Helper function implementations
fn filter_instances_by_state() {
    let state: String = Input::new()
        .with_prompt("Instance state (running, stopped, pending, etc.)")
        .default("running".into())
        .interact_text()
        .unwrap_or_default();

    if state.is_empty() {
        return;
    }

    let _ = Command::new("aws")
        .args([
            "ec2",
            "describe-instances",
            "--filters",
            &format!("Name=instance-state-name,Values={}", state),
            "--output",
            "table",
        ])
        .status();
}

fn instance_types_summary() {
    println!("📊 Instance Types Summary:");
    let _ = Command::new("aws")
        .args([
            "ec2",
            "describe-instances",
            "--query",
            "Reservations[*].Instances[*].InstanceType",
            "--output",
            "text",
        ])
        .status();
}

fn instance_cost_analysis() {
    println!("💰 Instance Cost Analysis:");
    println!("Note: Use AWS Cost Explorer for detailed cost analysis");
}

fn start_stop_instances() {
    println!("🔄 Start/Stop Instances");
    // Implementation for starting/stopping instances
}

fn bucket_sizes() {
    println!("📊 S3 Bucket Sizes:");
    let _ = Command::new("aws")
        .args(["s3", "ls", "--summarize", "--human-readable", "--recursive"])
        .status();
}

fn bucket_policies() {
    println!("🔒 Bucket Policies");
    // Implementation for bucket policies
}

fn list_bucket_contents() {
    let bucket_name: String = Input::new()
        .with_prompt("Bucket name")
        .interact_text()
        .unwrap_or_default();

    if bucket_name.is_empty() {
        return;
    }

    let _ = Command::new("aws")
        .args([
            "s3",
            "ls",
            &format!("s3://{}", bucket_name),
            "--human-readable",
        ])
        .status();
}

fn storage_costs() {
    println!("💰 S3 Storage Costs");
    // Implementation for storage cost analysis
}

fn current_month_costs() {
    println!("📊 Current Month Costs");
    let _ = Command::new("aws")
        .args([
            "ce",
            "get-cost-and-usage",
            "--time-period",
            "Start=2024-01-01,End=2024-01-31",
            "--granularity",
            "MONTHLY",
            "--metrics",
            "BlendedCost",
        ])
        .status();
}

fn cost_trend() {
    println!("📈 Cost Trend (Last 30 days)");
    // Implementation for cost trend analysis
}

fn costs_by_service() {
    println!("🏷️  Costs by Service");
    let _ = Command::new("aws")
        .args([
            "ce",
            "get-cost-and-usage",
            "--time-period",
            "Start=2024-01-01,End=2024-01-31",
            "--granularity",
            "MONTHLY",
            "--metrics",
            "BlendedCost",
            "--group-by",
            "Type=DIMENSION,Key=SERVICE",
        ])
        .status();
}

fn cost_explorer() {
    println!("🔍 Cost Explorer");
    println!("Visit: https://console.aws.amazon.com/cost-management/home");
}

fn ec2_metrics() {
    println!("🖥️  EC2 CloudWatch Metrics");
    // Implementation for EC2 metrics
}

fn s3_metrics() {
    println!("🗂️  S3 CloudWatch Metrics");
    // Implementation for S3 metrics
}

fn lambda_metrics() {
    println!("🚀 Lambda CloudWatch Metrics");
    // Implementation for Lambda metrics
}

fn rds_metrics() {
    println!("📡 RDS CloudWatch Metrics");
    // Implementation for RDS metrics
}

fn custom_metrics() {
    println!("📈 Custom CloudWatch Metrics");
    // Implementation for custom metrics
}

fn list_iam_users() {
    println!("👥 IAM Users:");
    let _ = Command::new("aws")
        .args(["iam", "list-users", "--output", "table"])
        .status();
}

fn list_iam_roles() {
    println!("🏷️  IAM Roles:");
    let _ = Command::new("aws")
        .args(["iam", "list-roles", "--output", "table"])
        .status();
}

fn list_iam_policies() {
    println!("📋 IAM Policies:");
    let _ = Command::new("aws")
        .args([
            "iam",
            "list-policies",
            "--scope",
            "Local",
            "--output",
            "table",
        ])
        .status();
}

fn manage_access_keys() {
    println!("🔑 Access Keys Management");
    // Implementation for access key management
}
