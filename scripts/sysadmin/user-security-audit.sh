#!/bin/bash
# User Account Security Audit
# Comprehensive user account and security audit for production systems

set -e

echo "👔 User Account Security Audit"
echo "=============================="

# Check for users with UID 0 (root privileges)
echo "🔴 Users with UID 0 (root privileges):"
echo "-------------------------------------"
awk -F: '$3 == 0 {print $1}' /etc/passwd

# Check for users with empty passwords
echo -e "\n⚠️  Users with empty passwords:"
echo "-----------------------------"
awk -F: '$2 == "" {print $1}' /etc/shadow 2>/dev/null || echo "Need root access to check shadow file"

# Check for users with shell access
echo -e "\n🐚 Users with shell access:"
echo "--------------------------"
awk -F: '$7 ~ /\/(bash|sh|zsh|fish)$/ {print $1 " -> " $7}' /etc/passwd

# Check for users not in /etc/passwd but in /etc/shadow
echo -e "\n👻 Orphaned shadow entries:"
echo "--------------------------"
if [[ $EUID -eq 0 ]]; then
    comm -23 <(awk -F: '{print $1}' /etc/shadow | sort) <(awk -F: '{print $1}' /etc/passwd | sort)
else
    echo "Need root access to check shadow file"
fi

# Check sudo access
echo -e "\n🔑 Sudo access:"
echo "--------------"
if [[ -f /etc/sudoers.d/* ]] || [[ -f /etc/sudoers ]]; then
    echo "Users/groups with sudo access:"
    sudo grep -h "^[^#]" /etc/sudoers /etc/sudoers.d/* 2>/dev/null | grep -E "(ALL|sudo|wheel)" | head -10
fi

# Check recent logins
echo -e "\n🕐 Recent logins (last 10):"
echo "--------------------------"
last -n 10

# Check failed login attempts
echo -e "\n❌ Recent failed logins:"
echo "-----------------------"
if [[ -f /var/log/auth.log ]]; then
    grep "Failed password" /var/log/auth.log 2>/dev/null | tail -5 || echo "No recent failed attempts"
elif [[ -f /var/log/secure ]]; then
    grep "Failed password" /var/log/secure 2>/dev/null | tail -5 || echo "No recent failed attempts"
else
    journalctl -u ssh -g "Failed password" --since "1 week ago" | tail -5 || echo "No recent failed attempts"
fi

# Check SSH configuration security
echo -e "\n🔒 SSH Security Configuration:"
echo "-----------------------------"
if [[ -f /etc/ssh/sshd_config ]]; then
    echo "Root login: $(grep -E "^PermitRootLogin" /etc/ssh/sshd_config || echo "Not explicitly set")"
    echo "Password auth: $(grep -E "^PasswordAuthentication" /etc/ssh/sshd_config || echo "Not explicitly set")"
    echo "Empty passwords: $(grep -E "^PermitEmptyPasswords" /etc/ssh/sshd_config || echo "Not explicitly set")"
fi

# Check for world-writable files in sensitive directories
echo -e "\n🌍 World-writable files in sensitive areas:"
echo "------------------------------------------"
find /etc /usr/bin /usr/sbin -type f -perm -002 2>/dev/null | head -5 || echo "None found (or no access)"

# Check password policy
echo -e "\n🔐 Password Policy:"
echo "-----------------"
if [[ -f /etc/login.defs ]]; then
    grep -E "^(PASS_MAX_DAYS|PASS_MIN_DAYS|PASS_MIN_LEN)" /etc/login.defs 2>/dev/null || echo "Default policy in effect"
fi

# Check for unusual cron jobs
echo -e "\n⏰ System cron jobs:"
echo "------------------"
if [[ -d /etc/cron.d ]]; then
    ls -la /etc/cron.d/ 2>/dev/null | head -5
fi

echo -e "\n✅ Security audit complete!"
echo "💡 Review any unusual findings and take appropriate action"
echo "🔍 For detailed analysis, check /var/log/auth.log or /var/log/secure"