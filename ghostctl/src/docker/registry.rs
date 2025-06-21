use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn registry_management() {
    println!("🗄️  Docker Registry Management");
    println!("==============================");

    let options = [
        "🏗️  Registry Selection & Auth",
        "🔍 Search images",
        "📥 Pull image",
        "📤 Push image",
        "📋 List local images",
        "🗑️  Remove image",
        "🏷️  Tag image",
        "📊 Image history",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => registry_selection(),
        1 => search_images(),
        2 => pull_image(),
        3 => push_image(),
        4 => list_images(),
        5 => remove_image(),
        6 => tag_image(),
        7 => image_history(),
        _ => return,
    }
}

fn registry_selection() {
    println!("🏗️  Registry Selection & Authentication");
    println!("=======================================");

    let registries = [
        "🏗️  docker.cktechx.io (Default - GhostKellz)",
        "🐳 Docker Hub (docker.io)",
        "📦 GitHub Container Registry (ghcr.io)",
        "🔴 Red Hat Quay (quay.io)",
        "📊 Google Container Registry (gcr.io)",
        "🌐 Amazon ECR",
        "💙 Azure Container Registry",
        "🔧 Custom Registry",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Registry")
        .items(&registries)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_cktech_registry(),
        1 => setup_docker_hub(),
        2 => setup_github_registry(),
        3 => setup_quay_registry(),
        4 => setup_gcr_registry(),
        5 => setup_ecr_registry(),
        6 => setup_azure_registry(),
        7 => setup_custom_registry(),
        _ => return,
    }
}

fn setup_cktech_registry() {
    println!("🏗️  CKTech Registry (docker.cktechx.io)");
    println!("======================================");

    println!("This is the default GhostKellz self-hosted registry.");

    let login = Confirm::new()
        .with_prompt("Login to docker.cktechx.io?")
        .default(true)
        .interact()
        .unwrap();

    if login {
        let username: String = Input::new()
            .with_prompt("Username")
            .interact_text()
            .unwrap();

        println!("🔑 Logging into docker.cktechx.io...");
        let status = Command::new("docker")
            .args(["login", "docker.cktechx.io", "-u", &username])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Successfully logged into docker.cktechx.io");
                println!("💡 You can now push/pull from docker.cktechx.io/namespace/image");
            }
            _ => println!("❌ Failed to login to docker.cktechx.io"),
        }
    }
}

fn setup_docker_hub() {
    println!("🐳 Docker Hub Registry");
    println!("======================");

    let login = Confirm::new()
        .with_prompt("Login to Docker Hub?")
        .default(true)
        .interact()
        .unwrap();

    if login {
        let username: String = Input::new()
            .with_prompt("Docker Hub username")
            .interact_text()
            .unwrap();

        println!("🔑 Logging into Docker Hub...");
        let status = Command::new("docker")
            .args(["login", "-u", &username])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Successfully logged into Docker Hub");
                println!("💡 You can now push/pull from docker.io/username/image");
            }
            _ => println!("❌ Failed to login to Docker Hub"),
        }
    }
}

fn setup_github_registry() {
    println!("📦 GitHub Container Registry");
    println!("============================");

    println!("💡 Use a Personal Access Token with 'read:packages' and 'write:packages' scopes");

    let login = Confirm::new()
        .with_prompt("Login to GitHub Container Registry?")
        .default(true)
        .interact()
        .unwrap();

    if login {
        let username: String = Input::new()
            .with_prompt("GitHub username")
            .interact_text()
            .unwrap();

        println!("🔑 Logging into ghcr.io...");
        let status = Command::new("docker")
            .args(["login", "ghcr.io", "-u", &username])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Successfully logged into GitHub Container Registry");
                println!("💡 You can now push/pull from ghcr.io/username/image");
            }
            _ => println!("❌ Failed to login to GitHub Container Registry"),
        }
    }
}

fn setup_quay_registry() {
    println!("🔴 Red Hat Quay Registry");
    println!("========================");

    let login = Confirm::new()
        .with_prompt("Login to Quay.io?")
        .default(true)
        .interact()
        .unwrap();

    if login {
        let username: String = Input::new()
            .with_prompt("Quay username")
            .interact_text()
            .unwrap();

        println!("🔑 Logging into quay.io...");
        let status = Command::new("docker")
            .args(["login", "quay.io", "-u", &username])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Successfully logged into Quay.io");
                println!("💡 You can now push/pull from quay.io/username/image");
            }
            _ => println!("❌ Failed to login to Quay.io"),
        }
    }
}

fn setup_gcr_registry() {
    println!("📊 Google Container Registry");
    println!("============================");
    println!("💡 Use 'gcloud auth configure-docker' for authentication");

    let status = Command::new("gcloud")
        .args(["auth", "configure-docker"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ GCR authentication configured");
            println!("💡 You can now push/pull from gcr.io/project/image");
        }
        _ => println!("❌ Failed to configure GCR authentication (gcloud CLI required)"),
    }
}

fn setup_ecr_registry() {
    println!("🌐 Amazon Elastic Container Registry");
    println!("====================================");
    println!("💡 Use 'aws ecr get-login-password' for authentication");

    let region: String = Input::new()
        .with_prompt("AWS region")
        .default("us-east-1".to_string())
        .interact_text()
        .unwrap();

    println!("🔑 Getting ECR login token...");
    let status = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "aws ecr get-login-password --region {} | docker login --username AWS --password-stdin {}.dkr.ecr.{}.amazonaws.com",
            region, "123456789012", region
        ))
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ ECR authentication successful");
            println!("💡 Replace account ID in the command above");
        }
        _ => println!("❌ Failed to authenticate with ECR (AWS CLI required)"),
    }
}

fn setup_azure_registry() {
    println!("💙 Azure Container Registry");
    println!("===========================");

    let registry_name: String = Input::new()
        .with_prompt("ACR registry name (without .azurecr.io)")
        .interact_text()
        .unwrap();

    println!("🔑 Logging into Azure Container Registry...");
    let status = Command::new("az")
        .args(["acr", "login", "--name", &registry_name])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Successfully logged into {}.azurecr.io", registry_name);
            println!(
                "💡 You can now push/pull from {}.azurecr.io/image",
                registry_name
            );
        }
        _ => println!("❌ Failed to login to ACR (Azure CLI required)"),
    }
}

fn setup_custom_registry() {
    println!("🔧 Custom Registry Setup");
    println!("========================");

    let registry_url: String = Input::new()
        .with_prompt("Registry URL (e.g., registry.example.com)")
        .interact_text()
        .unwrap();

    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    println!("🔑 Logging into {}...", registry_url);
    let status = Command::new("docker")
        .args(["login", &registry_url, "-u", &username])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Successfully logged into {}", registry_url);
            println!(
                "💡 You can now push/pull from {}/namespace/image",
                registry_url
            );
        }
        _ => println!("❌ Failed to login to {}", registry_url),
    }
}

fn search_images() {
    let search_term: String = Input::new()
        .with_prompt("Search term")
        .interact_text()
        .unwrap();

    println!("🔍 Searching for: {}", search_term);
    let _ = Command::new("docker")
        .args(["search", &search_term])
        .status();
}

fn pull_image() {
    let image: String = Input::new()
        .with_prompt("Image name (e.g., nginx:latest)")
        .interact_text()
        .unwrap();

    println!("📥 Pulling image: {}", image);
    let status = Command::new("docker").args(["pull", &image]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully pulled {}", image),
        _ => println!("❌ Failed to pull {}", image),
    }
}

fn push_image() {
    println!("📤 Push Image to Registry");
    println!("========================");

    println!("💡 Examples:");
    println!("  docker.cktechx.io/namespace/image:tag (GhostKellz default)");
    println!("  docker.io/username/image:tag (Docker Hub)");
    println!("  ghcr.io/username/image:tag (GitHub)");

    let image: String = Input::new()
        .with_prompt("Image name to push")
        .interact_text()
        .unwrap();

    // If no registry specified, suggest the default
    let final_image =
        if !image.contains('/') || (!image.contains('.') && !image.starts_with("docker.io")) {
            let suggest_default = Confirm::new()
                .with_prompt(&format!(
                    "Use GhostKellz registry? (docker.cktechx.io/{})",
                    image
                ))
                .default(true)
                .interact()
                .unwrap();

            if suggest_default {
                format!("docker.cktechx.io/{}", image)
            } else {
                image
            }
        } else {
            image
        };

    println!("📤 Pushing image: {}", final_image);
    let status = Command::new("docker").args(["push", &final_image]).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Successfully pushed {}", final_image);
            if final_image.starts_with("docker.cktechx.io") {
                println!("🏗️  Available at: https://docker.cktechx.io");
            }
        }
        _ => {
            println!("❌ Failed to push {}", final_image);
            println!("💡 Make sure you're logged in to the registry!");
        }
    }
}

fn list_images() {
    println!("📋 Local Docker Images");
    let _ = Command::new("docker").args(["images"]).status();
}

fn remove_image() {
    let image: String = Input::new()
        .with_prompt("Image name or ID to remove")
        .interact_text()
        .unwrap();

    println!("🗑️  Removing image: {}", image);
    let status = Command::new("docker").args(["rmi", &image]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully removed {}", image),
        _ => println!("❌ Failed to remove {}", image),
    }
}

fn tag_image() {
    let source: String = Input::new()
        .with_prompt("Source image name")
        .interact_text()
        .unwrap();

    let target: String = Input::new()
        .with_prompt("Target tag name")
        .interact_text()
        .unwrap();

    println!("🏷️  Tagging {} as {}", source, target);
    let status = Command::new("docker")
        .args(["tag", &source, &target])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully tagged {} as {}", source, target),
        _ => println!("❌ Failed to tag image"),
    }
}

fn image_history() {
    let image: String = Input::new()
        .with_prompt("Image name")
        .interact_text()
        .unwrap();

    println!("📊 Image History for: {}", image);
    let _ = Command::new("docker").args(["history", &image]).status();
}
