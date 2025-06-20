#!/bin/bash

# 🌐 CKTech GhostCert — Phantom ACME Issuer 🛡️👻✅💯

PDNS_SERVER="198.96.95.68"
CERT_DIR="/etc/nginx/certs"

issue_cert() {
    local domain="$1"
    local domain_dir="$CERT_DIR/$domain"

    echo "--------------------------------------------"
    echo "🔵 Starting certificate issuance for: $domain and *.$domain"
    echo "--------------------------------------------"

    acme.sh --issue --dns dns_pdns -d "$domain" -d "*.$domain" --dnssleep 20 --log --debug
    if [ $? -ne 0 ]; then
        echo "❌ ACME issuance failed for $domain"
        exit 1
    fi

    # Create target directory if missing
    if [ ! -d "$domain_dir" ]; then
        echo "📁 Creating certificate directory: $domain_dir"
        mkdir -p "$domain_dir"
    fi

    echo "🛠️ Installing certificates to $domain_dir/"
    acme.sh --install-cert -d "$domain" \
      --cert-file "$domain_dir/cert.pem" \
      --key-file "$domain_dir/privkey.pem" \
      --fullchain-file "$domain_dir/fullchain.pem" \
      --reloadcmd "systemctl reload nginx"

    if [ $? -eq 0 ]; then
        echo "✅ Certificate installed successfully to $domain_dir and nginx reloaded!"
    else
        echo "❌ Certificate installation failed."
        exit 1
    fi
}

setup_cron() {
    if ! crontab -l 2>/dev/null | grep -q '/root/.acme.sh/acme.sh --cron'; then
        echo "⏰ Setting up monthly renewal cronjob..."
        (crontab -l 2>/dev/null; echo "0 3 1 * * /root/.acme.sh/acme.sh --cron --home /root/.acme.sh > /dev/null 2>&1") | crontab -
        echo "✅ Cronjob added: monthly renewal on the 1st at 3:00 AM"
    else
        echo "✅ Renewal cronjob already exists."
    fi
}

main_menu() {
    clear
    echo "👻 CKTech GhostCert — Phantom ACME Issuer ✅"
    echo "--------------------------------------------"
    read -rp "👉 Enter the domain you want to issue (example: cktech.dev): " user_domain

    if [ -n "$user_domain" ]; then
        issue_cert "$user_domain"
        setup_cron
        echo "🏁 All done! GhostCert issuance complete. 🏁"
    else
        echo "❌ No domain entered, exiting."
        exit 1
    fi
}

# 🛫 Start
main_menu

