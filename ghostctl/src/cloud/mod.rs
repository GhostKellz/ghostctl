pub mod aws;
pub mod azure;
pub mod gcp;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn infrastructure_menu() {
    loop {
        let options = [
            "🎭 Ansible Management",
            "🏗️  Terraform Management",
            "☁️  Cloud Provider Tools",
            "📊 Infrastructure Dashboard",
            "🔄 CI/CD Pipeline Integration",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🏗️  Infrastructure as Code")
            .items(&options)
            .default(0)
            .interact_opt()
            .ok()
            .flatten();

        let Some(choice) = choice else {
            break;
        };

        match choice {
            0 => ansible_management(),
            1 => terraform_management(),
            2 => cloud_provider_tools(),
            3 => infrastructure_dashboard(),
            4 => cicd_integration(),
            _ => break,
        }
    }
}

pub fn ansible_management() {
    let options = [
        "🚀 Quick Start (Install & Setup)",
        "📋 List Playbooks",
        "▶️  Run Playbook",
        "📝 Create New Playbook",
        "🏠 Inventory Management",
        "🔧 Ansible Configuration",
        "📊 Playbook History",
        "🧪 Test Connection",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎭 Ansible Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => ansible_quick_start(),
        1 => list_ansible_playbooks(),
        2 => run_ansible_playbook(),
        3 => create_ansible_playbook(),
        4 => manage_ansible_inventory(),
        5 => configure_ansible(),
        6 => ansible_history(),
        7 => test_ansible_connection(),
        _ => return,
    }
}

pub fn terraform_management() {
    let options = [
        "🚀 Quick Start (Install & Setup)",
        "📋 List Terraform Projects",
        "🔧 Initialize Project",
        "📝 Plan Changes",
        "✅ Apply Changes",
        "🗑️  Destroy Infrastructure",
        "📊 Show State",
        "🔒 Manage State",
        "📦 Module Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🏗️  Terraform Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => terraform_quick_start(),
        1 => list_terraform_projects(),
        2 => init_terraform_project(),
        3 => terraform_plan(),
        4 => terraform_apply(),
        5 => terraform_destroy(),
        6 => terraform_show(),
        7 => manage_terraform_state(),
        8 => terraform_modules(),
        _ => return,
    }
}

pub fn cloud_provider_tools() {
    let options = [
        "☁️  AWS CLI Tools",
        "🌐 Google Cloud (gcloud)",
        "🔷 Azure CLI",
        "🌊 DigitalOcean",
        "🔥 Hetzner Cloud",
        "🐙 Linode/Akamai",
        "⚙️  Multi-Cloud Setup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("☁️  Cloud Provider Tools")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => aws::aws_cli_tools(),
        1 => gcp::gcloud_tools(),
        2 => azure::azure_cli_tools(),
        3 => digitalocean_tools(),
        4 => hetzner_tools(),
        5 => linode_tools(),
        6 => multicloud_setup(),
        _ => return,
    }
}

pub fn infrastructure_dashboard() {
    println!("📊 Infrastructure Dashboard");

    let options = [
        "🌐 Multi-Cloud Status Overview",
        "📊 Resource Inventory",
        "💰 Cost Summary",
        "🔍 Health Checks",
        "📈 Usage Metrics",
        "🗄️  Object Storage (MinIO/S3)",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Infrastructure Dashboard")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => multicloud_status_overview(),
        1 => resource_inventory(),
        2 => cost_summary(),
        3 => health_checks(),
        4 => usage_metrics(),
        5 => object_storage_management(),
        _ => return,
    }
}

use dialoguer::{Confirm, Input, MultiSelect};
use std::fs;
use std::path::Path;
use std::process::Command;

// ============================================================================
// ANSIBLE IMPLEMENTATIONS
// ============================================================================

fn ansible_quick_start() {
    println!("🚀 Ansible Quick Start");
    println!("======================\n");

    // Check if Ansible is installed
    let ansible_check = Command::new("ansible").arg("--version").output();

    match ansible_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            let first_line = version.lines().next().unwrap_or("Unknown");
            println!("✅ Ansible installed: {}\n", first_line);
        }
        _ => {
            println!("📦 Ansible not found. Installing...\n");

            let install_method = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Installation method")
                .items(&[
                    "pip install ansible (Recommended)",
                    "pipx install ansible",
                    "Package manager (pacman)",
                ])
                .default(0)
                .interact_opt()
                .ok()
                .flatten();

            let Some(method) = install_method else {
                return;
            };

            let status = match method {
                0 => Command::new("pip")
                    .args(["install", "--user", "ansible"])
                    .status(),
                1 => Command::new("pipx").args(["install", "ansible"]).status(),
                _ => Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "ansible"])
                    .status(),
            };

            match status {
                Ok(s) if s.success() => println!("✅ Ansible installed successfully!"),
                _ => {
                    println!("❌ Failed to install Ansible");
                    return;
                }
            }
        }
    }

    // Quick setup options
    let options = [
        "📁 Create project structure",
        "📝 Generate ansible.cfg",
        "🏠 Create initial inventory",
        "📋 Create sample playbook",
        "🔧 Install common collections",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quick Start Options")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => create_ansible_project_structure(),
        1 => generate_ansible_cfg(),
        2 => create_initial_inventory(),
        3 => create_ansible_playbook(),
        4 => install_ansible_collections(),
        _ => {}
    }
}

fn create_ansible_project_structure() {
    println!("📁 Creating Ansible Project Structure...\n");

    let project_name: String = match Input::new()
        .with_prompt("Project name")
        .default("ansible-project".into())
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let dirs = [
        format!("{}/inventory/group_vars", project_name),
        format!("{}/inventory/host_vars", project_name),
        format!("{}/playbooks", project_name),
        format!("{}/roles", project_name),
        format!("{}/files", project_name),
        format!("{}/templates", project_name),
        format!("{}/vars", project_name),
    ];

    for dir in &dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            println!("❌ Failed to create {}: {}", dir, e);
            return;
        }
    }

    // Create ansible.cfg
    let ansible_cfg = format!(
        r#"[defaults]
inventory = inventory/hosts.yml
roles_path = roles
host_key_checking = False
retry_files_enabled = False
stdout_callback = yaml
forks = 10

[privilege_escalation]
become = True
become_method = sudo
become_user = root
become_ask_pass = False
"#
    );

    let _ = fs::write(format!("{}/ansible.cfg", project_name), ansible_cfg);

    // Create empty inventory
    let inventory = r#"---
all:
  hosts:
    localhost:
      ansible_connection: local
  children:
    webservers:
      hosts:
    databases:
      hosts:
"#;

    let _ = fs::write(format!("{}/inventory/hosts.yml", project_name), inventory);

    // Create sample playbook
    let playbook = r#"---
- name: Sample Playbook
  hosts: all
  become: yes

  tasks:
    - name: Ensure system is updated
      package:
        name: "*"
        state: latest
      when: ansible_os_family == "Debian" or ansible_os_family == "RedHat"

    - name: Print hello message
      debug:
        msg: "Hello from {{ inventory_hostname }}!"
"#;

    let _ = fs::write(format!("{}/playbooks/site.yml", project_name), playbook);

    // Create .gitignore
    let gitignore = r#"*.retry
*.pyc
__pycache__/
.vault_pass
*.log
"#;

    let _ = fs::write(format!("{}/.gitignore", project_name), gitignore);

    println!("✅ Project structure created: {}/", project_name);
    println!("   ├── ansible.cfg");
    println!("   ├── inventory/");
    println!("   │   ├── hosts.yml");
    println!("   │   ├── group_vars/");
    println!("   │   └── host_vars/");
    println!("   ├── playbooks/");
    println!("   │   └── site.yml");
    println!("   ├── roles/");
    println!("   ├── files/");
    println!("   ├── templates/");
    println!("   └── vars/");
}

fn generate_ansible_cfg() {
    println!("📝 Generating ansible.cfg...\n");

    let inventory_path: String = match Input::new()
        .with_prompt("Inventory path")
        .default("inventory/hosts.yml".into())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let forks: String = match Input::new()
        .with_prompt("Parallel processes (forks)")
        .default("10".into())
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let host_key_checking = Confirm::new()
        .with_prompt("Enable SSH host key checking?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let cfg = format!(
        r#"[defaults]
inventory = {}
roles_path = roles
host_key_checking = {}
retry_files_enabled = False
stdout_callback = yaml
forks = {}
gathering = smart
fact_caching = jsonfile
fact_caching_connection = /tmp/ansible_facts_cache
fact_caching_timeout = 86400

[privilege_escalation]
become = True
become_method = sudo
become_user = root
become_ask_pass = False

[ssh_connection]
pipelining = True
ssh_args = -o ControlMaster=auto -o ControlPersist=60s
"#,
        inventory_path, host_key_checking, forks
    );

    let save = Confirm::new()
        .with_prompt("Save to ./ansible.cfg?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        match fs::write("ansible.cfg", &cfg) {
            Ok(_) => println!("✅ ansible.cfg created"),
            Err(e) => println!("❌ Failed to write: {}", e),
        }
    } else {
        println!("\n{}", cfg);
    }
}

fn create_initial_inventory() {
    println!("🏠 Creating Inventory File...\n");

    let format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Inventory format")
        .items(&["YAML (Recommended)", "INI"])
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(0);

    let filename = if format == 0 {
        "inventory/hosts.yml"
    } else {
        "inventory/hosts.ini"
    };

    if Path::new(filename).exists() {
        let overwrite = Confirm::new()
            .with_prompt(format!("{} exists. Overwrite?", filename))
            .default(false)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if !overwrite {
            return;
        }
    }

    let content = if format == 0 {
        r#"---
all:
  hosts:
    # Add hosts here
    # example_host:
    #   ansible_host: 192.168.1.100
    #   ansible_user: admin

  children:
    webservers:
      hosts:
        # web1:
        #   ansible_host: 192.168.1.10
      vars:
        http_port: 80

    databases:
      hosts:
        # db1:
        #   ansible_host: 192.168.1.20
      vars:
        db_port: 5432

    # Group of groups
    production:
      children:
        webservers:
        databases:

  vars:
    ansible_python_interpreter: /usr/bin/python3
"#
    } else {
        r#"[all:vars]
ansible_python_interpreter=/usr/bin/python3

[webservers]
# web1 ansible_host=192.168.1.10

[databases]
# db1 ansible_host=192.168.1.20

[webservers:vars]
http_port=80

[databases:vars]
db_port=5432

[production:children]
webservers
databases
"#
    };

    let _ = fs::create_dir_all("inventory");
    match fs::write(filename, content) {
        Ok(_) => println!("✅ Created: {}", filename),
        Err(e) => println!("❌ Failed: {}", e),
    }
}

fn create_ansible_playbook() {
    println!("📋 Create Sample Playbook...\n");

    let templates = [
        "🔧 Basic System Setup",
        "🐳 Docker Installation",
        "🌐 Nginx Web Server",
        "🗄️  PostgreSQL Database",
        "🔒 Security Hardening",
        "📊 Monitoring Stack",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select playbook template")
        .items(&templates)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    let (filename, content) = match choice {
        0 => (
            "playbooks/system-setup.yml",
            r#"---
- name: Basic System Setup
  hosts: all
  become: yes

  vars:
    packages:
      - vim
      - htop
      - curl
      - wget
      - git

  tasks:
    - name: Update package cache
      package:
        update_cache: yes
      when: ansible_os_family in ["Debian", "RedHat", "Archlinux"]

    - name: Install essential packages
      package:
        name: "{{ packages }}"
        state: present

    - name: Set timezone
      timezone:
        name: UTC

    - name: Configure SSH
      lineinfile:
        path: /etc/ssh/sshd_config
        regexp: "^PermitRootLogin"
        line: "PermitRootLogin no"
      notify: restart sshd

  handlers:
    - name: restart sshd
      service:
        name: sshd
        state: restarted
"#,
        ),
        1 => (
            "playbooks/docker-install.yml",
            r#"---
- name: Install Docker
  hosts: all
  become: yes

  tasks:
    - name: Install Docker dependencies
      package:
        name:
          - ca-certificates
          - curl
          - gnupg
        state: present

    - name: Add Docker GPG key (Debian/Ubuntu)
      apt_key:
        url: https://download.docker.com/linux/{{ ansible_distribution | lower }}/gpg
        state: present
      when: ansible_os_family == "Debian"

    - name: Add Docker repository (Debian/Ubuntu)
      apt_repository:
        repo: "deb https://download.docker.com/linux/{{ ansible_distribution | lower }} {{ ansible_distribution_release }} stable"
        state: present
      when: ansible_os_family == "Debian"

    - name: Install Docker (Arch)
      pacman:
        name: docker
        state: present
      when: ansible_os_family == "Archlinux"

    - name: Install Docker (Debian/Ubuntu)
      apt:
        name:
          - docker-ce
          - docker-ce-cli
          - containerd.io
          - docker-compose-plugin
        state: present
        update_cache: yes
      when: ansible_os_family == "Debian"

    - name: Start Docker service
      service:
        name: docker
        state: started
        enabled: yes

    - name: Add user to docker group
      user:
        name: "{{ ansible_user }}"
        groups: docker
        append: yes
"#,
        ),
        2 => (
            "playbooks/nginx-setup.yml",
            r#"---
- name: Setup Nginx Web Server
  hosts: webservers
  become: yes

  vars:
    nginx_port: 80
    server_name: "_"

  tasks:
    - name: Install Nginx
      package:
        name: nginx
        state: present

    - name: Create web root
      file:
        path: /var/www/html
        state: directory
        mode: '0755'

    - name: Deploy default page
      copy:
        content: |
          <!DOCTYPE html>
          <html>
          <head><title>Welcome</title></head>
          <body><h1>Server is running!</h1></body>
          </html>
        dest: /var/www/html/index.html

    - name: Configure Nginx
      template:
        src: nginx.conf.j2
        dest: /etc/nginx/sites-available/default
      notify: reload nginx

    - name: Enable and start Nginx
      service:
        name: nginx
        state: started
        enabled: yes

  handlers:
    - name: reload nginx
      service:
        name: nginx
        state: reloaded
"#,
        ),
        3 => (
            "playbooks/postgresql-setup.yml",
            r#"---
- name: Setup PostgreSQL Database
  hosts: databases
  become: yes

  vars:
    postgresql_version: 15
    postgresql_db: myapp
    postgresql_user: myapp_user
    # REQUIRED: Set via ansible-vault or --extra-vars
    # Generate with: openssl rand -base64 32
    postgresql_password: "{{ vault_db_password }}"

  tasks:
    - name: Install PostgreSQL
      package:
        name:
          - postgresql
          - postgresql-contrib
        state: present

    - name: Install Python psycopg2 for Ansible
      package:
        name: python3-psycopg2
        state: present

    - name: Start PostgreSQL service
      service:
        name: postgresql
        state: started
        enabled: yes

    - name: Create database
      become_user: postgres
      postgresql_db:
        name: "{{ postgresql_db }}"
        state: present

    - name: Create database user
      become_user: postgres
      postgresql_user:
        name: "{{ postgresql_user }}"
        password: "{{ postgresql_password }}"
        db: "{{ postgresql_db }}"
        priv: ALL
        state: present

    - name: Allow remote connections
      lineinfile:
        path: /etc/postgresql/{{ postgresql_version }}/main/pg_hba.conf
        line: "host all all 0.0.0.0/0 md5"
      notify: restart postgresql

  handlers:
    - name: restart postgresql
      service:
        name: postgresql
        state: restarted
"#,
        ),
        4 => (
            "playbooks/security-hardening.yml",
            r#"---
- name: Security Hardening
  hosts: all
  become: yes

  tasks:
    - name: Disable root login via SSH
      lineinfile:
        path: /etc/ssh/sshd_config
        regexp: "^PermitRootLogin"
        line: "PermitRootLogin no"
      notify: restart sshd

    - name: Disable password authentication
      lineinfile:
        path: /etc/ssh/sshd_config
        regexp: "^PasswordAuthentication"
        line: "PasswordAuthentication no"
      notify: restart sshd

    - name: Set SSH idle timeout
      lineinfile:
        path: /etc/ssh/sshd_config
        regexp: "^ClientAliveInterval"
        line: "ClientAliveInterval 300"
      notify: restart sshd

    - name: Install fail2ban
      package:
        name: fail2ban
        state: present

    - name: Configure fail2ban for SSH
      copy:
        content: |
          [sshd]
          enabled = true
          port = ssh
          filter = sshd
          logpath = /var/log/auth.log
          maxretry = 3
          bantime = 3600
        dest: /etc/fail2ban/jail.local
      notify: restart fail2ban

    - name: Enable fail2ban
      service:
        name: fail2ban
        state: started
        enabled: yes

    - name: Configure UFW defaults
      ufw:
        direction: "{{ item.direction }}"
        policy: "{{ item.policy }}"
      loop:
        - { direction: incoming, policy: deny }
        - { direction: outgoing, policy: allow }
      when: ansible_os_family == "Debian"

    - name: Allow SSH through firewall
      ufw:
        rule: allow
        port: "22"
        proto: tcp
      when: ansible_os_family == "Debian"

    - name: Enable UFW
      ufw:
        state: enabled
      when: ansible_os_family == "Debian"

  handlers:
    - name: restart sshd
      service:
        name: sshd
        state: restarted

    - name: restart fail2ban
      service:
        name: fail2ban
        state: restarted
"#,
        ),
        _ => (
            "playbooks/monitoring-stack.yml",
            r#"---
- name: Deploy Monitoring Stack
  hosts: all
  become: yes

  vars:
    prometheus_version: "2.45.0"
    grafana_version: "10.0.0"

  tasks:
    - name: Create prometheus user
      user:
        name: prometheus
        shell: /bin/false
        system: yes
        create_home: no

    - name: Create prometheus directories
      file:
        path: "{{ item }}"
        state: directory
        owner: prometheus
        group: prometheus
      loop:
        - /etc/prometheus
        - /var/lib/prometheus

    - name: Download Prometheus
      get_url:
        url: "https://github.com/prometheus/prometheus/releases/download/v{{ prometheus_version }}/prometheus-{{ prometheus_version }}.linux-amd64.tar.gz"
        dest: /tmp/prometheus.tar.gz

    - name: Extract Prometheus
      unarchive:
        src: /tmp/prometheus.tar.gz
        dest: /tmp
        remote_src: yes

    - name: Install Prometheus binaries
      copy:
        src: "/tmp/prometheus-{{ prometheus_version }}.linux-amd64/{{ item }}"
        dest: "/usr/local/bin/{{ item }}"
        mode: '0755'
        remote_src: yes
      loop:
        - prometheus
        - promtool

    - name: Configure Prometheus
      copy:
        content: |
          global:
            scrape_interval: 15s

          scrape_configs:
            - job_name: 'prometheus'
              static_configs:
                - targets: ['localhost:9090']

            - job_name: 'node'
              static_configs:
                - targets: ['localhost:9100']
        dest: /etc/prometheus/prometheus.yml
        owner: prometheus
        group: prometheus

    - name: Create Prometheus systemd service
      copy:
        content: |
          [Unit]
          Description=Prometheus
          Wants=network-online.target
          After=network-online.target

          [Service]
          User=prometheus
          Group=prometheus
          Type=simple
          ExecStart=/usr/local/bin/prometheus \
            --config.file=/etc/prometheus/prometheus.yml \
            --storage.tsdb.path=/var/lib/prometheus

          [Install]
          WantedBy=multi-user.target
        dest: /etc/systemd/system/prometheus.service
      notify: reload systemd

    - name: Start Prometheus
      service:
        name: prometheus
        state: started
        enabled: yes

  handlers:
    - name: reload systemd
      systemd:
        daemon_reload: yes
"#,
        ),
    };

    let _ = fs::create_dir_all("playbooks");
    match fs::write(filename, content) {
        Ok(_) => println!("✅ Created: {}", filename),
        Err(e) => println!("❌ Failed: {}", e),
    }
}

fn install_ansible_collections() {
    println!("🔧 Installing Ansible Collections...\n");

    let collections = [
        "community.general",
        "community.docker",
        "community.postgresql",
        "community.mysql",
        "ansible.posix",
        "ansible.netcommon",
        "cloud.common",
    ];

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select collections to install")
        .items(&collections)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

    for idx in selected {
        println!("📦 Installing {}...", collections[idx]);
        let status = Command::new("ansible-galaxy")
            .args(["collection", "install", collections[idx]])
            .status();

        match status {
            Ok(s) if s.success() => println!("   ✅ Installed"),
            _ => println!("   ❌ Failed"),
        }
    }
}

fn list_ansible_playbooks() {
    println!("📋 Ansible Playbooks\n");

    // Search for playbooks in common locations
    let search_paths = [".", "playbooks", "ansible", "../ansible"];

    let mut found_playbooks = Vec::new();

    for path in &search_paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename.ends_with(".yml") || filename.ends_with(".yaml") {
                    // Check if it looks like a playbook (contains hosts:)
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if content.contains("hosts:") {
                            found_playbooks.push((entry.path().display().to_string(), filename));
                        }
                    }
                }
            }
        }
    }

    if found_playbooks.is_empty() {
        println!("⚠️  No playbooks found in current directory or ./playbooks/");
        println!("💡 Create one with 'Create New Playbook' option");
        return;
    }

    println!("Found {} playbook(s):\n", found_playbooks.len());
    for (path, name) in &found_playbooks {
        println!("  📄 {} ({})", name, path);
    }
}

fn run_ansible_playbook() {
    println!("▶️  Run Ansible Playbook\n");

    let playbook: String = match Input::new()
        .with_prompt("Playbook path")
        .default("playbooks/site.yml".into())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    if !Path::new(&playbook).exists() {
        println!("❌ Playbook not found: {}", playbook);
        return;
    }

    let options = [
        "🚀 Run (normal)",
        "🔍 Check mode (dry-run)",
        "📋 Syntax check only",
        "🔧 Run with extra vars",
        "🎯 Run on specific hosts",
        "📊 Run with verbose output",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Run mode")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(0);

    let mut args = vec!["ansible-playbook"];
    args.push(&playbook);

    let extra_vars_str;
    let limit_str;

    match choice {
        1 => args.push("--check"),
        2 => args.push("--syntax-check"),
        3 => {
            let vars: String = match Input::new()
                .with_prompt("Extra vars (key=value)")
                .interact_text()
            {
                Ok(v) => v,
                Err(_) => return,
            };
            extra_vars_str = vars;
            args.push("-e");
            args.push(&extra_vars_str);
        }
        4 => {
            let limit: String = match Input::new().with_prompt("Host/group limit").interact_text() {
                Ok(l) => l,
                Err(_) => return,
            };
            limit_str = limit;
            args.push("--limit");
            args.push(&limit_str);
        }
        5 => args.push("-vvv"),
        _ => {}
    }

    println!("\n🚀 Running: {}\n", args.join(" "));

    let status = Command::new(args[0]).args(&args[1..]).status();

    match status {
        Ok(s) if s.success() => println!("\n✅ Playbook completed successfully!"),
        Ok(s) => println!("\n❌ Playbook failed with exit code: {:?}", s.code()),
        Err(e) => println!("\n❌ Failed to run playbook: {}", e),
    }
}

fn manage_ansible_inventory() {
    println!("🏠 Ansible Inventory Management\n");

    let options = [
        "📋 List inventory",
        "📝 Edit inventory",
        "➕ Add host",
        "➕ Add group",
        "🔍 Test host connectivity",
        "📊 Show host facts",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Inventory Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            println!("📋 Inventory:\n");
            let _ = Command::new("ansible-inventory")
                .args(["--list", "--yaml"])
                .status();
        }
        1 => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
            let _ = Command::new(&editor).arg("inventory/hosts.yml").status();
        }
        2 => {
            let hostname: String = match Input::new().with_prompt("Hostname").interact_text() {
                Ok(h) => h,
                Err(_) => return,
            };
            let ip: String = match Input::new().with_prompt("IP address").interact_text() {
                Ok(i) => i,
                Err(_) => return,
            };
            println!("💡 Add to inventory/hosts.yml:");
            println!("    {}:", hostname);
            println!("      ansible_host: {}", ip);
        }
        3 => {
            let group: String = match Input::new().with_prompt("Group name").interact_text() {
                Ok(g) => g,
                Err(_) => return,
            };
            println!("💡 Add to inventory/hosts.yml under 'children:':");
            println!("    {}:", group);
            println!("      hosts:");
        }
        4 => {
            let host: String = match Input::new()
                .with_prompt("Host/group to test")
                .default("all".into())
                .interact_text()
            {
                Ok(h) => h,
                Err(_) => return,
            };
            println!("🔍 Testing connectivity to {}...\n", host);
            let _ = Command::new("ansible").args([&host, "-m", "ping"]).status();
        }
        5 => {
            let host: String = match Input::new()
                .with_prompt("Host to gather facts from")
                .interact_text()
            {
                Ok(h) => h,
                Err(_) => return,
            };
            println!("📊 Gathering facts from {}...\n", host);
            let _ = Command::new("ansible")
                .args([&host, "-m", "setup"])
                .status();
        }
        _ => {}
    }
}

fn configure_ansible() {
    println!("🔧 Ansible Configuration\n");

    let options = [
        "📝 Edit ansible.cfg",
        "📋 Show current config",
        "🔐 Configure Vault",
        "📁 Set roles path",
        "⚡ Performance tuning",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
            let _ = Command::new(&editor).arg("ansible.cfg").status();
        }
        1 => {
            println!("📋 Current Ansible Configuration:\n");
            let _ = Command::new("ansible-config").arg("dump").status();
        }
        2 => {
            println!("🔐 Ansible Vault Configuration\n");
            let vault_options = [
                "Create vault password file",
                "Encrypt file",
                "Decrypt file",
                "Edit encrypted file",
                "View encrypted file",
            ];

            let vault_choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Vault action")
                .items(&vault_options)
                .default(0)
                .interact_opt()
                .ok()
                .flatten()
                .unwrap_or(0);

            match vault_choice {
                0 => {
                    println!("💡 Create .vault_pass with your password");
                    println!("   Add to ansible.cfg: vault_password_file = .vault_pass");
                }
                1 => {
                    let file: String =
                        match Input::new().with_prompt("File to encrypt").interact_text() {
                            Ok(f) => f,
                            Err(_) => return,
                        };
                    let _ = Command::new("ansible-vault")
                        .args(["encrypt", &file])
                        .status();
                }
                2 => {
                    let file: String =
                        match Input::new().with_prompt("File to decrypt").interact_text() {
                            Ok(f) => f,
                            Err(_) => return,
                        };
                    let _ = Command::new("ansible-vault")
                        .args(["decrypt", &file])
                        .status();
                }
                3 => {
                    let file: String =
                        match Input::new().with_prompt("File to edit").interact_text() {
                            Ok(f) => f,
                            Err(_) => return,
                        };
                    let _ = Command::new("ansible-vault").args(["edit", &file]).status();
                }
                4 => {
                    let file: String =
                        match Input::new().with_prompt("File to view").interact_text() {
                            Ok(f) => f,
                            Err(_) => return,
                        };
                    let _ = Command::new("ansible-vault").args(["view", &file]).status();
                }
                _ => {}
            }
        }
        3 => {
            let roles_path: String = match Input::new()
                .with_prompt("Roles path")
                .default("roles".into())
                .interact_text()
            {
                Ok(p) => p,
                Err(_) => return,
            };
            println!("💡 Add to ansible.cfg [defaults]:");
            println!("   roles_path = {}", roles_path);
        }
        4 => {
            println!("⚡ Performance Tuning Tips:\n");
            println!("Add to ansible.cfg [defaults]:");
            println!("   forks = 20");
            println!("   gathering = smart");
            println!("   fact_caching = jsonfile");
            println!("   fact_caching_connection = /tmp/ansible_facts");
            println!("\nAdd to [ssh_connection]:");
            println!("   pipelining = True");
            println!("   ssh_args = -o ControlMaster=auto -o ControlPersist=60s");
        }
        _ => {}
    }
}

fn ansible_history() {
    println!("📊 Playbook Run History\n");

    // Check for common log locations
    let log_paths = [
        "/var/log/ansible.log",
        "ansible.log",
        ".ansible/ansible.log",
    ];

    for path in &log_paths {
        if Path::new(path).exists() {
            println!("📄 Found log: {}\n", path);
            println!("Last 20 lines:");
            let _ = Command::new("tail").args(["-20", path]).status();
            return;
        }
    }

    println!("⚠️  No Ansible log files found.");
    println!("\n💡 Enable logging in ansible.cfg:");
    println!("   [defaults]");
    println!("   log_path = ./ansible.log");
}

fn test_ansible_connection() {
    println!("🧪 Test Ansible Connection\n");

    let target: String = match Input::new()
        .with_prompt("Target hosts/group")
        .default("all".into())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    println!("🔍 Testing connection to {}...\n", target);

    // Ping test
    let _ = Command::new("ansible")
        .args([&target, "-m", "ping", "-o"])
        .status();

    println!("\n📊 Gathering basic facts...\n");

    // Get basic facts
    let _ = Command::new("ansible")
        .args([
            &target,
            "-m",
            "setup",
            "-a",
            "filter=ansible_distribution*,ansible_hostname",
        ])
        .status();
}

// ============================================================================
// TERRAFORM IMPLEMENTATIONS
// ============================================================================

fn terraform_quick_start() {
    println!("🚀 Terraform Quick Start");
    println!("========================\n");

    // Check if Terraform is installed
    let tf_check = Command::new("terraform").arg("version").output();

    match tf_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            let first_line = version.lines().next().unwrap_or("Unknown");
            println!("✅ Terraform installed: {}\n", first_line);
        }
        _ => {
            println!("📦 Terraform not found. Installing...\n");

            let install_method = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Installation method")
                .items(&[
                    "HashiCorp APT repository",
                    "Package manager (pacman)",
                    "Download binary",
                ])
                .default(1)
                .interact_opt()
                .ok()
                .flatten();

            let Some(method) = install_method else {
                return;
            };

            let status = match method {
                0 => {
                    println!("📖 Follow HashiCorp's official instructions:");
                    println!("https://developer.hashicorp.com/terraform/install");
                    return;
                }
                1 => Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "terraform"])
                    .status(),
                _ => {
                    println!("📖 Download from: https://releases.hashicorp.com/terraform/");
                    return;
                }
            };

            match status {
                Ok(s) if s.success() => println!("✅ Terraform installed!"),
                _ => {
                    println!("❌ Installation failed");
                    return;
                }
            }
        }
    }

    let options = [
        "📁 Create new project",
        "🔧 Initialize existing project",
        "📋 Show provider examples",
        "📝 Generate .gitignore",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quick Start Options")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => create_terraform_project(),
        1 => {
            let _ = Command::new("terraform").arg("init").status();
        }
        2 => show_terraform_provider_examples(),
        3 => create_terraform_gitignore(),
        _ => {}
    }
}

fn create_terraform_project() {
    println!("📁 Creating Terraform Project...\n");

    let project_name: String = match Input::new()
        .with_prompt("Project name")
        .default("terraform-project".into())
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let provider = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cloud provider")
        .items(&[
            "AWS",
            "Google Cloud",
            "Azure",
            "DigitalOcean",
            "Hetzner",
            "Local/Null",
        ])
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(0);

    let _ = fs::create_dir_all(&project_name);
    let _ = fs::create_dir_all(format!("{}/modules", project_name));

    // Create main.tf
    let provider_config = match provider {
        0 => {
            r#"terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.region
}
"#
        }
        1 => {
            r#"terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}
"#
        }
        2 => {
            r#"terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
}
"#
        }
        3 => {
            r#"terraform {
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

provider "digitalocean" {
  token = var.do_token
}
"#
        }
        4 => {
            r#"terraform {
  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.0"
    }
  }
}

provider "hcloud" {
  token = var.hcloud_token
}
"#
        }
        _ => {
            r#"terraform {
  required_providers {
    null = {
      source  = "hashicorp/null"
      version = "~> 3.0"
    }
  }
}
"#
        }
    };

    let _ = fs::write(format!("{}/main.tf", project_name), provider_config);

    // Create variables.tf
    let variables = match provider {
        0 => {
            r#"variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}
"#
        }
        1 => {
            r#"variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}
"#
        }
        _ => {
            r#"variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}
"#
        }
    };

    let _ = fs::write(format!("{}/variables.tf", project_name), variables);

    // Create outputs.tf
    let outputs = r#"# Add outputs here
# output "example" {
#   value = "example_value"
# }
"#;
    let _ = fs::write(format!("{}/outputs.tf", project_name), outputs);

    // Create terraform.tfvars.example
    let tfvars_example =
        "# Copy this file to terraform.tfvars and fill in values\n# region = \"us-east-1\"\n";
    let _ = fs::write(
        format!("{}/terraform.tfvars.example", project_name),
        tfvars_example,
    );

    // Create .gitignore
    let gitignore = r#"# Terraform
*.tfstate
*.tfstate.*
.terraform/
.terraform.lock.hcl
*.tfvars
!*.tfvars.example
crash.log
override.tf
override.tf.json
*_override.tf
*_override.tf.json
"#;
    let _ = fs::write(format!("{}/.gitignore", project_name), gitignore);

    println!("✅ Project created: {}/", project_name);
    println!("   ├── main.tf");
    println!("   ├── variables.tf");
    println!("   ├── outputs.tf");
    println!("   ├── terraform.tfvars.example");
    println!("   ├── modules/");
    println!("   └── .gitignore");
    println!("\n💡 Next: cd {} && terraform init", project_name);
}

fn show_terraform_provider_examples() {
    println!("📋 Terraform Provider Examples\n");

    let providers = [
        "AWS - EC2 Instance",
        "GCP - Compute Instance",
        "Azure - Virtual Machine",
        "DigitalOcean - Droplet",
        "Hetzner - Server",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select example")
        .items(&providers)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(0);

    let example = match choice {
        0 => {
            r#"
# AWS EC2 Instance Example
resource "aws_instance" "web" {
  ami           = "ami-0c55b159cbfafe1f0"
  instance_type = "t3.micro"

  tags = {
    Name = "web-server"
  }
}
"#
        }
        1 => {
            r#"
# GCP Compute Instance Example
resource "google_compute_instance" "vm" {
  name         = "web-server"
  machine_type = "e2-micro"
  zone         = "us-central1-a"

  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
    }
  }

  network_interface {
    network = "default"
    access_config {}
  }
}
"#
        }
        2 => {
            r#"
# Azure Virtual Machine Example
resource "azurerm_linux_virtual_machine" "vm" {
  name                = "web-server"
  resource_group_name = azurerm_resource_group.rg.name
  location            = azurerm_resource_group.rg.location
  size                = "Standard_B1s"
  admin_username      = "adminuser"

  network_interface_ids = [
    azurerm_network_interface.nic.id,
  ]

  os_disk {
    caching              = "ReadWrite"
    storage_account_type = "Standard_LRS"
  }

  source_image_reference {
    publisher = "Canonical"
    offer     = "UbuntuServer"
    sku       = "18.04-LTS"
    version   = "latest"
  }
}
"#
        }
        3 => {
            r#"
# DigitalOcean Droplet Example
resource "digitalocean_droplet" "web" {
  image  = "ubuntu-22-04-x64"
  name   = "web-server"
  region = "nyc1"
  size   = "s-1vcpu-1gb"
}
"#
        }
        _ => {
            r#"
# Hetzner Cloud Server Example
resource "hcloud_server" "web" {
  name        = "web-server"
  server_type = "cx11"
  image       = "ubuntu-22.04"
  location    = "nbg1"
}
"#
        }
    };

    println!("{}", example);
}

fn create_terraform_gitignore() {
    let terraform_entries = [
        "# Terraform state files",
        "*.tfstate",
        "*.tfstate.*",
        "",
        "# Terraform directory",
        ".terraform/",
        ".terraform.lock.hcl",
        "",
        "# Variable files (may contain secrets)",
        "*.tfvars",
        "*.tfvars.json",
        "!*.tfvars.example",
        "",
        "# Crash log files",
        "crash.log",
        "crash.*.log",
        "",
        "# Override files",
        "override.tf",
        "override.tf.json",
        "*_override.tf",
        "*_override.tf.json",
        "",
        "# CLI configuration",
        ".terraformrc",
        "terraform.rc",
        "",
        "# Plan files",
        "*.tfplan",
    ];

    let gitignore_path = std::path::Path::new(".gitignore");

    // Read existing content if file exists
    let existing_content = if gitignore_path.exists() {
        match fs::read_to_string(gitignore_path) {
            Ok(content) => content,
            Err(e) => {
                println!("❌ Failed to read existing .gitignore: {}", e);
                return;
            }
        }
    } else {
        String::new()
    };

    // Collect existing entries (trimmed, non-empty)
    let existing_entries: std::collections::HashSet<&str> = existing_content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    // Find entries that need to be added
    let mut new_entries: Vec<&str> = Vec::new();
    for entry in &terraform_entries {
        let trimmed = entry.trim();
        // Skip empty lines for comparison, but include them in output
        if trimmed.is_empty() || !existing_entries.contains(trimmed) {
            new_entries.push(entry);
        }
    }

    // Remove leading empty lines from new entries
    while !new_entries.is_empty() && new_entries[0].trim().is_empty() {
        new_entries.remove(0);
    }

    if new_entries.iter().all(|e| e.trim().is_empty()) {
        println!("✅ .gitignore already contains Terraform entries");
        return;
    }

    // Append new entries to existing file
    let mut final_content = existing_content.clone();
    if !final_content.is_empty() && !final_content.ends_with('\n') {
        final_content.push('\n');
    }
    if !final_content.is_empty() {
        final_content.push_str("\n# Terraform (added by ghostctl)\n");
    }
    for entry in &new_entries {
        final_content.push_str(entry);
        final_content.push('\n');
    }

    match fs::write(gitignore_path, final_content) {
        Ok(_) => {
            if existing_content.is_empty() {
                println!("✅ Created .gitignore with Terraform entries");
            } else {
                println!("✅ Added Terraform entries to existing .gitignore");
            }
        }
        Err(e) => println!("❌ Failed to write .gitignore: {}", e),
    }
}

fn list_terraform_projects() {
    println!("📋 Terraform Projects\n");

    // Look for directories with .tf files
    let mut projects = Vec::new();

    for entry in fs::read_dir(".").into_iter().flatten().flatten() {
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            let path = entry.path();
            if fs::read_dir(&path)
                .into_iter()
                .flatten()
                .flatten()
                .any(|e| e.file_name().to_string_lossy().ends_with(".tf"))
            {
                projects.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }

    // Check current directory
    if fs::read_dir(".")
        .into_iter()
        .flatten()
        .flatten()
        .any(|e| e.file_name().to_string_lossy().ends_with(".tf"))
    {
        projects.insert(0, "./ (current directory)".to_string());
    }

    if projects.is_empty() {
        println!("⚠️  No Terraform projects found");
        return;
    }

    println!("Found {} project(s):\n", projects.len());
    for project in &projects {
        // Check if initialized
        let tf_dir = if project.starts_with("./") {
            ".terraform".to_string()
        } else {
            format!("{}/.terraform", project)
        };
        let status = if Path::new(&tf_dir).exists() {
            "✅ initialized"
        } else {
            "⚠️  not initialized"
        };
        println!("  📁 {} ({})", project, status);
    }
}

fn init_terraform_project() {
    println!("🔧 Initialize Terraform Project\n");

    let upgrade = Confirm::new()
        .with_prompt("Upgrade providers to latest?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let mut args = vec!["init"];
    if upgrade {
        args.push("-upgrade");
    }

    println!("🚀 Running terraform {}...\n", args.join(" "));
    let _ = Command::new("terraform").args(&args).status();
}

fn terraform_plan() {
    println!("📝 Terraform Plan\n");

    let options = [
        "🔍 Plan (show changes)",
        "💾 Plan and save to file",
        "🎯 Plan for specific target",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Plan options")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("terraform").arg("plan").status();
        }
        1 => {
            let filename: String = match Input::new()
                .with_prompt("Plan file name")
                .default("tfplan".into())
                .interact_text()
            {
                Ok(f) => f,
                Err(_) => return,
            };
            let _ = Command::new("terraform")
                .args(["plan", "-out", &filename])
                .status();
            println!("\n💡 Apply with: terraform apply {}", filename);
        }
        2 => {
            let target: String = match Input::new()
                .with_prompt("Resource target (e.g., aws_instance.web)")
                .interact_text()
            {
                Ok(t) => t,
                Err(_) => return,
            };
            let _ = Command::new("terraform")
                .args(["plan", "-target", &target])
                .status();
        }
        _ => {}
    }
}

fn terraform_apply() {
    println!("✅ Terraform Apply\n");

    let auto_approve = Confirm::new()
        .with_prompt("Auto-approve (skip confirmation)?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let mut args = vec!["apply"];
    if auto_approve {
        args.push("-auto-approve");
    }

    // Check for saved plan file
    if Path::new("tfplan").exists() {
        let use_plan = Confirm::new()
            .with_prompt("Found tfplan file. Use it?")
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if use_plan {
            args.push("tfplan");
        }
    }

    println!("🚀 Running terraform {}...\n", args.join(" "));
    let _ = Command::new("terraform").args(&args).status();
}

fn terraform_destroy() {
    println!("🗑️  Terraform Destroy\n");

    println!("⚠️  WARNING: This will destroy all managed infrastructure!\n");

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to destroy?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        println!("Cancelled.");
        return;
    }

    let auto_approve = Confirm::new()
        .with_prompt("Auto-approve (skip second confirmation)?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let mut args = vec!["destroy"];
    if auto_approve {
        args.push("-auto-approve");
    }

    let _ = Command::new("terraform").args(&args).status();
}

fn terraform_show() {
    println!("📊 Terraform State\n");

    let options = [
        "📋 Show state summary",
        "📄 Show full state",
        "🔍 Show specific resource",
        "📊 List resources",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("State options")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("terraform").arg("show").status();
        }
        1 => {
            let _ = Command::new("terraform").args(["show", "-json"]).status();
        }
        2 => {
            let resource: String =
                match Input::new().with_prompt("Resource address").interact_text() {
                    Ok(r) => r,
                    Err(_) => return,
                };
            let _ = Command::new("terraform")
                .args(["state", "show", &resource])
                .status();
        }
        3 => {
            let _ = Command::new("terraform").args(["state", "list"]).status();
        }
        _ => {}
    }
}

fn manage_terraform_state() {
    println!("🔒 Terraform State Management\n");

    let options = [
        "📋 List resources in state",
        "🔄 Move resource",
        "🗑️  Remove resource from state",
        "📥 Import existing resource",
        "🔄 Refresh state",
        "💾 Configure remote backend",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("State Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("terraform").args(["state", "list"]).status();
        }
        1 => {
            let source: String = match Input::new().with_prompt("Source address").interact_text() {
                Ok(s) => s,
                Err(_) => return,
            };
            let dest: String = match Input::new()
                .with_prompt("Destination address")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };
            let _ = Command::new("terraform")
                .args(["state", "mv", &source, &dest])
                .status();
        }
        2 => {
            let resource: String = match Input::new()
                .with_prompt("Resource to remove")
                .interact_text()
            {
                Ok(r) => r,
                Err(_) => return,
            };
            let _ = Command::new("terraform")
                .args(["state", "rm", &resource])
                .status();
        }
        3 => {
            let address: String = match Input::new()
                .with_prompt("Resource address (e.g., aws_instance.web)")
                .interact_text()
            {
                Ok(a) => a,
                Err(_) => return,
            };
            let id: String = match Input::new().with_prompt("Resource ID").interact_text() {
                Ok(i) => i,
                Err(_) => return,
            };
            let _ = Command::new("terraform")
                .args(["import", &address, &id])
                .status();
        }
        4 => {
            let _ = Command::new("terraform").arg("refresh").status();
        }
        5 => configure_remote_backend(),
        _ => {}
    }
}

fn configure_remote_backend() {
    println!("💾 Configure Remote Backend\n");

    let backends = [
        "S3 (AWS)",
        "GCS (Google Cloud)",
        "Azure Blob",
        "Terraform Cloud",
        "Local",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backend")
        .items(&backends)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(0);

    let config = match choice {
        0 => {
            r#"
# Add this to your main.tf
terraform {
  backend "s3" {
    bucket         = "your-terraform-state-bucket"
    key            = "terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform-locks"
  }
}
"#
        }
        1 => {
            r#"
# Add this to your main.tf
terraform {
  backend "gcs" {
    bucket  = "your-terraform-state-bucket"
    prefix  = "terraform/state"
  }
}
"#
        }
        2 => {
            r#"
# Add this to your main.tf
terraform {
  backend "azurerm" {
    resource_group_name  = "terraform-state-rg"
    storage_account_name = "tfstateaccount"
    container_name       = "tfstate"
    key                  = "terraform.tfstate"
  }
}
"#
        }
        3 => {
            r#"
# Add this to your main.tf
terraform {
  cloud {
    organization = "your-org"
    workspaces {
      name = "your-workspace"
    }
  }
}
"#
        }
        _ => {
            r#"
# Local backend (default)
terraform {
  backend "local" {
    path = "terraform.tfstate"
  }
}
"#
        }
    };

    println!("{}", config);
    println!("\n💡 After adding, run: terraform init -migrate-state");
}

fn terraform_modules() {
    println!("📦 Terraform Module Management\n");

    let options = [
        "📋 List installed modules",
        "📥 Download module",
        "📝 Create new module",
        "🔍 Search Terraform Registry",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Module Management")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            if Path::new(".terraform/modules").exists() {
                println!("📦 Installed modules:\n");
                let _ = Command::new("ls")
                    .args(["-la", ".terraform/modules"])
                    .status();
            } else {
                println!("⚠️  No modules installed (run terraform init first)");
            }
        }
        1 => {
            println!("📥 Module Usage Example:\n");
            println!(
                r#"
# In your main.tf:
module "vpc" {{
  source  = "terraform-aws-modules/vpc/aws"
  version = "5.0.0"

  name = "my-vpc"
  cidr = "10.0.0.0/16"
}}
"#
            );
        }
        2 => {
            let module_name: String = match Input::new().with_prompt("Module name").interact_text()
            {
                Ok(n) => n,
                Err(_) => return,
            };

            let module_dir = format!("modules/{}", module_name);
            let _ = fs::create_dir_all(&module_dir);

            // Create module files
            let main_tf = "# Module resources go here\n";
            let variables_tf = r#"# Module input variables
variable "name" {
  description = "Resource name"
  type        = string
}
"#;
            let outputs_tf = r#"# Module outputs
# output "id" {
#   value = resource.id
# }
"#;

            let _ = fs::write(format!("{}/main.tf", module_dir), main_tf);
            let _ = fs::write(format!("{}/variables.tf", module_dir), variables_tf);
            let _ = fs::write(format!("{}/outputs.tf", module_dir), outputs_tf);

            println!("✅ Module created: {}/", module_dir);
        }
        3 => {
            println!("🔍 Terraform Registry: https://registry.terraform.io/\n");
            println!("Popular modules:");
            println!("  • terraform-aws-modules/vpc/aws");
            println!("  • terraform-aws-modules/eks/aws");
            println!("  • terraform-google-modules/network/google");
            println!("  • Azure/network/azurerm");
        }
        _ => {}
    }
}

// ============================================================================
// CLOUD PROVIDER IMPLEMENTATIONS
// ============================================================================

fn digitalocean_tools() {
    println!("🌊 DigitalOcean Tools\n");

    // Check if doctl is installed
    let doctl_check = Command::new("doctl").arg("version").output();

    match doctl_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            println!("✅ doctl installed: {}", version.trim());
        }
        _ => {
            println!("📦 doctl not found. Installing...\n");

            let status = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "doctl"])
                .status();

            if status.is_err() || !status.map(|s| s.success()).unwrap_or(false) {
                println!("💡 Install manually:");
                println!("   snap install doctl");
                println!("   or download from: https://github.com/digitalocean/doctl/releases");
                return;
            }
        }
    }

    let options = [
        "🔧 Authenticate",
        "📋 List Droplets",
        "🚀 Create Droplet",
        "🗑️  Delete Droplet",
        "📊 Account Info",
        "🗄️  Spaces (Object Storage)",
        "🌐 Kubernetes Clusters",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DigitalOcean Actions")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("doctl").args(["auth", "init"]).status();
        }
        1 => {
            let _ = Command::new("doctl")
                .args(["compute", "droplet", "list"])
                .status();
        }
        2 => {
            let name: String = match Input::new().with_prompt("Droplet name").interact_text() {
                Ok(n) => n,
                Err(_) => return,
            };
            let _ = Command::new("doctl")
                .args([
                    "compute",
                    "droplet",
                    "create",
                    &name,
                    "--image",
                    "ubuntu-22-04-x64",
                    "--size",
                    "s-1vcpu-1gb",
                    "--region",
                    "nyc1",
                ])
                .status();
        }
        3 => {
            let id: String = match Input::new().with_prompt("Droplet ID").interact_text() {
                Ok(i) => i,
                Err(_) => return,
            };
            let _ = Command::new("doctl")
                .args(["compute", "droplet", "delete", &id, "--force"])
                .status();
        }
        4 => {
            let _ = Command::new("doctl").args(["account", "get"]).status();
        }
        5 => {
            let _ = Command::new("doctl")
                .args(["compute", "cdn", "list"])
                .status();
        }
        6 => {
            let _ = Command::new("doctl")
                .args(["kubernetes", "cluster", "list"])
                .status();
        }
        _ => {}
    }
}

fn hetzner_tools() {
    println!("🔥 Hetzner Cloud Tools\n");

    // Check if hcloud is installed
    let hcloud_check = Command::new("hcloud").arg("version").output();

    match hcloud_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            println!("✅ hcloud installed: {}", version.trim());
        }
        _ => {
            println!("📦 hcloud not found. Installing...\n");

            let status = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "hcloud"])
                .status();

            if status.is_err() || !status.map(|s| s.success()).unwrap_or(false) {
                println!("💡 Install manually:");
                println!("   brew install hcloud");
                println!("   or download from: https://github.com/hetznercloud/cli/releases");
                return;
            }
        }
    }

    let options = [
        "🔧 Configure context (authenticate)",
        "📋 List Servers",
        "🚀 Create Server",
        "🗑️  Delete Server",
        "📊 Server Types",
        "🖼️  Available Images",
        "🌐 Networks",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hetzner Cloud Actions")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("hcloud")
                .args(["context", "create", "default"])
                .status();
        }
        1 => {
            let _ = Command::new("hcloud").args(["server", "list"]).status();
        }
        2 => {
            let name: String = match Input::new().with_prompt("Server name").interact_text() {
                Ok(n) => n,
                Err(_) => return,
            };
            let _ = Command::new("hcloud")
                .args([
                    "server",
                    "create",
                    "--name",
                    &name,
                    "--type",
                    "cx11",
                    "--image",
                    "ubuntu-22.04",
                ])
                .status();
        }
        3 => {
            let name: String = match Input::new().with_prompt("Server name").interact_text() {
                Ok(n) => n,
                Err(_) => return,
            };
            let _ = Command::new("hcloud")
                .args(["server", "delete", &name])
                .status();
        }
        4 => {
            let _ = Command::new("hcloud")
                .args(["server-type", "list"])
                .status();
        }
        5 => {
            let _ = Command::new("hcloud").args(["image", "list"]).status();
        }
        6 => {
            let _ = Command::new("hcloud").args(["network", "list"]).status();
        }
        _ => {}
    }
}

fn linode_tools() {
    println!("🐙 Linode/Akamai Tools\n");

    // Check if linode-cli is installed
    let linode_check = Command::new("linode-cli").arg("--version").output();

    match linode_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            println!("✅ linode-cli installed: {}", version.trim());
        }
        _ => {
            println!("📦 linode-cli not found. Installing...\n");

            let status = Command::new("pip")
                .args(["install", "--user", "linode-cli"])
                .status();

            if status.is_err() || !status.map(|s| s.success()).unwrap_or(false) {
                println!("💡 Install manually: pip install linode-cli");
                return;
            }
        }
    }

    let options = [
        "🔧 Configure (authenticate)",
        "📋 List Linodes",
        "🚀 Create Linode",
        "🗑️  Delete Linode",
        "📊 Available Plans",
        "🖼️  Available Images",
        "🌐 Regions",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Linode Actions")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("linode-cli").arg("configure").status();
        }
        1 => {
            let _ = Command::new("linode-cli")
                .args(["linodes", "list"])
                .status();
        }
        2 => {
            let label: String = match Input::new().with_prompt("Linode label").interact_text() {
                Ok(l) => l,
                Err(_) => return,
            };
            let _ = Command::new("linode-cli")
                .args([
                    "linodes",
                    "create",
                    "--label",
                    &label,
                    "--type",
                    "g6-nanode-1",
                    "--region",
                    "us-east",
                    "--image",
                    "linode/ubuntu22.04",
                ])
                .status();
        }
        3 => {
            let id: String = match Input::new().with_prompt("Linode ID").interact_text() {
                Ok(i) => i,
                Err(_) => return,
            };
            let _ = Command::new("linode-cli")
                .args(["linodes", "delete", &id])
                .status();
        }
        4 => {
            let _ = Command::new("linode-cli")
                .args(["linodes", "types"])
                .status();
        }
        5 => {
            let _ = Command::new("linode-cli").args(["images", "list"]).status();
        }
        6 => {
            let _ = Command::new("linode-cli")
                .args(["regions", "list"])
                .status();
        }
        _ => {}
    }
}

fn multicloud_setup() {
    println!("⚙️  Multi-Cloud Setup\n");

    let options = [
        "📋 Check installed CLIs",
        "📦 Install all cloud CLIs",
        "🔧 Configure all providers",
        "📝 Create unified config",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multi-Cloud Setup")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            println!("📋 Cloud CLI Status:\n");

            let clis = [
                ("aws", "AWS CLI"),
                ("gcloud", "Google Cloud CLI"),
                ("az", "Azure CLI"),
                ("doctl", "DigitalOcean CLI"),
                ("hcloud", "Hetzner CLI"),
                ("linode-cli", "Linode CLI"),
                ("terraform", "Terraform"),
            ];

            for (cmd, name) in &clis {
                let status = Command::new(cmd).arg("--version").output();
                let icon = if status.is_ok() { "✅" } else { "❌" };
                println!("  {} {}", icon, name);
            }
        }
        1 => {
            println!("📦 Installing cloud CLIs...\n");
            let _ = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "aws-cli",
                    "hcloud",
                    "terraform",
                ])
                .status();
            println!("\n💡 For gcloud, az, doctl, linode-cli - see provider setup menus");
        }
        2 => {
            println!("🔧 Run individual provider setup to configure each CLI");
        }
        3 => {
            println!("📝 Unified config would require custom implementation");
            println!("💡 Consider using Terraform with multiple providers instead");
        }
        _ => {}
    }
}

// ============================================================================
// DASHBOARD IMPLEMENTATIONS
// ============================================================================

fn multicloud_status_overview() {
    println!("🌐 Multi-Cloud Status Overview\n");

    println!("Checking cloud providers...\n");

    // AWS
    print!("☁️  AWS: ");
    let aws_status = Command::new("aws")
        .args(["sts", "get-caller-identity", "--output", "text"])
        .output();
    match aws_status {
        Ok(out) if out.status.success() => println!("✅ Authenticated"),
        _ => println!("❌ Not configured"),
    }

    // GCP
    print!("🌐 GCP: ");
    let gcp_status = Command::new("gcloud")
        .args([
            "auth",
            "list",
            "--filter=status:ACTIVE",
            "--format=value(account)",
        ])
        .output();
    match gcp_status {
        Ok(out) if out.status.success() && !out.stdout.is_empty() => {
            let account = String::from_utf8_lossy(&out.stdout);
            println!("✅ {}", account.trim());
        }
        _ => println!("❌ Not configured"),
    }

    // Azure
    print!("🔷 Azure: ");
    let azure_status = Command::new("az")
        .args(["account", "show", "--query", "name", "-o", "tsv"])
        .output();
    match azure_status {
        Ok(out) if out.status.success() => {
            let name = String::from_utf8_lossy(&out.stdout);
            println!("✅ {}", name.trim());
        }
        _ => println!("❌ Not configured"),
    }

    // DigitalOcean
    print!("🌊 DigitalOcean: ");
    let do_status = Command::new("doctl")
        .args(["account", "get", "--format", "Email", "--no-header"])
        .output();
    match do_status {
        Ok(out) if out.status.success() => {
            let email = String::from_utf8_lossy(&out.stdout);
            println!("✅ {}", email.trim());
        }
        _ => println!("❌ Not configured"),
    }

    // Hetzner
    print!("🔥 Hetzner: ");
    let hetzner_status = Command::new("hcloud").args(["context", "active"]).output();
    match hetzner_status {
        Ok(out) if out.status.success() => {
            let ctx = String::from_utf8_lossy(&out.stdout);
            println!("✅ {}", ctx.trim());
        }
        _ => println!("❌ Not configured"),
    }
}

fn resource_inventory() {
    println!("📊 Resource Inventory\n");

    let options = [
        "☁️  AWS Resources",
        "🌐 GCP Resources",
        "🔷 Azure Resources",
        "🌊 DigitalOcean Resources",
        "🔥 Hetzner Resources",
        "📋 All Providers (Summary)",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Provider")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            println!("☁️  AWS Resources:\n");
            println!("EC2 Instances:");
            let _ = Command::new("aws")
                .args([
                    "ec2",
                    "describe-instances",
                    "--query",
                    "Reservations[*].Instances[*].[InstanceId,State.Name]",
                    "--output",
                    "table",
                ])
                .status();
        }
        1 => {
            println!("🌐 GCP Resources:\n");
            println!("Compute Instances:");
            let _ = Command::new("gcloud")
                .args(["compute", "instances", "list"])
                .status();
        }
        2 => {
            println!("🔷 Azure Resources:\n");
            let _ = Command::new("az")
                .args(["resource", "list", "--output", "table"])
                .status();
        }
        3 => {
            println!("🌊 DigitalOcean Resources:\n");
            let _ = Command::new("doctl")
                .args(["compute", "droplet", "list"])
                .status();
        }
        4 => {
            println!("🔥 Hetzner Resources:\n");
            let _ = Command::new("hcloud").args(["server", "list"]).status();
        }
        5 => {
            println!("📋 Collecting resources from all providers...\n");
            multicloud_status_overview();
        }
        _ => {}
    }
}

fn cost_summary() {
    println!("💰 Cloud Cost Summary\n");

    println!("⚠️  Note: Cost APIs require additional setup/permissions\n");

    let options = [
        "☁️  AWS Cost Explorer",
        "🌐 GCP Billing",
        "🔷 Azure Cost Management",
        "💡 Cost Optimization Tips",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cost Information")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            println!("☁️  AWS Cost (last 30 days):\n");
            let _ = Command::new("aws")
                .args([
                    "ce",
                    "get-cost-and-usage",
                    "--time-period",
                    "Start=2024-01-01,End=2024-01-31",
                    "--granularity",
                    "MONTHLY",
                    "--metrics",
                    "UnblendedCost",
                ])
                .status();
        }
        1 => {
            println!("🌐 GCP Billing:\n");
            println!("💡 Visit: https://console.cloud.google.com/billing");
            let _ = Command::new("gcloud")
                .args(["beta", "billing", "accounts", "list"])
                .status();
        }
        2 => {
            println!("🔷 Azure Cost:\n");
            let _ = Command::new("az")
                .args(["consumption", "usage", "list", "--top", "10"])
                .status();
        }
        3 => {
            println!("💡 Cost Optimization Tips:\n");
            println!("  1. Use reserved instances for predictable workloads");
            println!("  2. Right-size underutilized resources");
            println!("  3. Use spot/preemptible instances for fault-tolerant workloads");
            println!("  4. Set up billing alerts");
            println!("  5. Delete unused resources (snapshots, old AMIs, etc.)");
            println!("  6. Use auto-scaling to match demand");
            println!("  7. Consider multi-region vs single-region trade-offs");
        }
        _ => {}
    }
}

fn health_checks() {
    println!("🔍 Infrastructure Health Checks\n");

    let options = [
        "🌐 Check all endpoints",
        "📊 Service status",
        "🔧 Run connectivity tests",
        "📋 Recent alerts",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Health Checks")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            let endpoints: String = match Input::new()
                .with_prompt("Endpoints to check (comma-separated)")
                .default("https://google.com,https://github.com".into())
                .interact_text()
            {
                Ok(e) => e,
                Err(_) => return,
            };

            for endpoint in endpoints.split(',') {
                let endpoint = endpoint.trim();
                print!("  {} ", endpoint);
                let status = Command::new("curl")
                    .args(["-sL", "-o", "/dev/null", "-w", "%{http_code}", endpoint])
                    .output();
                match status {
                    Ok(out) => {
                        let code = String::from_utf8_lossy(&out.stdout);
                        if code.starts_with('2') || code.starts_with('3') {
                            println!("✅ {}", code);
                        } else {
                            println!("❌ {}", code);
                        }
                    }
                    Err(_) => println!("❌ Failed"),
                }
            }
        }
        1 => {
            println!("📊 Checking service status...\n");
            multicloud_status_overview();
        }
        2 => {
            println!("🔧 Running connectivity tests...\n");

            let targets = ["8.8.8.8", "1.1.1.1", "api.github.com"];
            for target in &targets {
                print!("  Ping {}: ", target);
                let status = Command::new("ping")
                    .args(["-c", "1", "-W", "2", target])
                    .output();
                match status {
                    Ok(out) if out.status.success() => println!("✅"),
                    _ => println!("❌"),
                }
            }
        }
        3 => {
            println!("📋 Check your cloud provider consoles for alerts");
            println!("  • AWS: CloudWatch Alarms");
            println!("  • GCP: Cloud Monitoring");
            println!("  • Azure: Azure Monitor");
        }
        _ => {}
    }
}

fn usage_metrics() {
    println!("📈 Usage Metrics\n");

    println!("💡 For detailed metrics, use provider-specific tools:\n");
    println!("  ☁️  AWS: aws cloudwatch get-metric-statistics");
    println!("  🌐 GCP: gcloud monitoring metrics list");
    println!("  🔷 Azure: az monitor metrics list");
    println!("\n📊 For dashboards:");
    println!("  • Grafana + Prometheus");
    println!("  • Datadog");
    println!("  • CloudWatch/Stackdriver/Azure Monitor dashboards");
}

fn object_storage_management() {
    println!("🗄️  Object Storage Management\n");

    let options = [
        "☁️  AWS S3",
        "🌐 GCP Cloud Storage",
        "🔷 Azure Blob Storage",
        "🌊 DigitalOcean Spaces",
        "📦 MinIO (Self-hosted)",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Object Storage")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    match choice {
        0 => {
            println!("☁️  S3 Buckets:\n");
            let _ = Command::new("aws").args(["s3", "ls"]).status();
        }
        1 => {
            println!("🌐 GCS Buckets:\n");
            let _ = Command::new("gsutil").arg("ls").status();
        }
        2 => {
            println!("🔷 Azure Blob Containers:\n");
            let _ = Command::new("az")
                .args(["storage", "container", "list", "--output", "table"])
                .status();
        }
        3 => {
            println!("🌊 DigitalOcean Spaces:\n");
            let _ = Command::new("doctl")
                .args(["compute", "cdn", "list"])
                .status();
        }
        4 => {
            println!("📦 MinIO Setup:\n");
            println!("💡 Install: docker run -p 9000:9000 minio/minio server /data");
            println!("   Or: sudo pacman -S minio");
        }
        _ => {}
    }
}

fn cicd_integration() {
    println!("🔄 CI/CD Pipeline Integration\n");

    let options = [
        "🐙 GitHub Actions",
        "🦊 GitLab CI",
        "🔵 Azure DevOps",
        "☁️  AWS CodePipeline",
        "🌐 GCP Cloud Build",
        "🏗️  Jenkins",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("CI/CD Platform")
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten();

    let Some(choice) = choice else {
        return;
    };

    let example = match choice {
        0 => {
            r#"
# .github/workflows/terraform.yml
name: Terraform

on:
  push:
    branches: [main]
  pull_request:

jobs:
  terraform:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: hashicorp/setup-terraform@v3

      - name: Terraform Init
        run: terraform init

      - name: Terraform Plan
        run: terraform plan

      - name: Terraform Apply
        if: github.ref == 'refs/heads/main'
        run: terraform apply -auto-approve
"#
        }
        1 => {
            r#"
# .gitlab-ci.yml
stages:
  - validate
  - plan
  - apply

terraform:validate:
  stage: validate
  script:
    - terraform init
    - terraform validate

terraform:plan:
  stage: plan
  script:
    - terraform init
    - terraform plan -out=tfplan
  artifacts:
    paths:
      - tfplan

terraform:apply:
  stage: apply
  script:
    - terraform apply tfplan
  when: manual
  only:
    - main
"#
        }
        _ => "See provider-specific documentation for CI/CD setup.",
    };

    println!("{}", example);
}
