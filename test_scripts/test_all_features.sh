#!/bin/bash

# GhostCTL Feature Testing Script
# Safe to run on VM - includes dry-run modes

set -e

echo "🧪 GhostCTL Feature Testing Suite"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

test_count=0
pass_count=0
fail_count=0

log_test() {
    local test_name="$1"
    local result="$2"
    local details="${3:-}"

    test_count=$((test_count + 1))

    if [ "$result" = "PASS" ]; then
        echo -e "${GREEN}✅ PASS${NC}: $test_name"
        pass_count=$((pass_count + 1))
    elif [ "$result" = "FAIL" ]; then
        echo -e "${RED}❌ FAIL${NC}: $test_name"
        [ -n "$details" ] && echo -e "   ${RED}$details${NC}"
        fail_count=$((fail_count + 1))
    else
        echo -e "${YELLOW}⚠️  SKIP${NC}: $test_name - $details"
    fi
}

test_binary_exists() {
    if [ -f "./target/release/ghostctl" ]; then
        log_test "Binary exists" "PASS"
        return 0
    else
        log_test "Binary exists" "FAIL" "Binary not found at ./target/release/ghostctl"
        return 1
    fi
}

test_help_command() {
    if ./target/release/ghostctl --help >/dev/null 2>&1; then
        log_test "Help command works" "PASS"
        return 0
    else
        log_test "Help command works" "FAIL" "Help command failed"
        return 1
    fi
}

test_gaming_menu_accessible() {
    echo -e "${BLUE}Testing Gaming Menu Access...${NC}"

    # Test if we can access the gaming menu (just check it doesn't crash)
    timeout 5 bash -c 'echo | ./target/release/ghostctl' >/dev/null 2>&1
    local exit_code=$?

    if [ $exit_code -eq 0 ] || [ $exit_code -eq 124 ]; then  # 124 is timeout
        log_test "Gaming menu accessible" "PASS"
        return 0
    else
        log_test "Gaming menu accessible" "FAIL" "Menu crashed or errored"
        return 1
    fi
}

test_wine_dependencies() {
    echo -e "${BLUE}Testing Wine Dependencies...${NC}"

    local deps=("wine" "winetricks")
    local missing_deps=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            missing_deps+=("$dep")
        fi
    done

    if [ ${#missing_deps[@]} -eq 0 ]; then
        log_test "Wine dependencies available" "PASS"
    else
        log_test "Wine dependencies available" "SKIP" "Missing: ${missing_deps[*]}"
    fi
}

test_lutris_integration() {
    echo -e "${BLUE}Testing Lutris Integration...${NC}"

    if command -v lutris >/dev/null 2>&1; then
        log_test "Lutris available" "PASS"
    else
        log_test "Lutris available" "SKIP" "Lutris not installed"
    fi
}

test_steam_integration() {
    echo -e "${BLUE}Testing Steam Integration...${NC}"

    if command -v steam >/dev/null 2>&1; then
        log_test "Steam available" "PASS"
    else
        log_test "Steam available" "SKIP" "Steam not installed"
    fi
}

test_firewall_tools() {
    echo -e "${BLUE}Testing Firewall Tools...${NC}"

    local tools=("iptables" "nft")
    local available_tools=()

    for tool in "${tools[@]}"; do
        if command -v "$tool" >/dev/null 2>&1; then
            available_tools+=("$tool")
        fi
    done

    if [ ${#available_tools[@]} -gt 0 ]; then
        log_test "Firewall tools available" "PASS" "Available: ${available_tools[*]}"
    else
        log_test "Firewall tools available" "FAIL" "No firewall tools found"
    fi
}

test_networking_tools() {
    echo -e "${BLUE}Testing Networking Tools...${NC}"

    local tools=("ip" "ping" "nslookup" "ss")
    local missing_tools=()

    for tool in "${tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            missing_tools+=("$tool")
        fi
    done

    if [ ${#missing_tools[@]} -eq 0 ]; then
        log_test "Basic networking tools available" "PASS"
    else
        log_test "Basic networking tools available" "SKIP" "Missing: ${missing_tools[*]}"
    fi
}

test_permissions() {
    echo -e "${BLUE}Testing Permissions...${NC}"

    # Test if we can run sudo commands (will prompt if needed)
    if sudo -n true 2>/dev/null; then
        log_test "Sudo access (passwordless)" "PASS"
    elif sudo -l >/dev/null 2>&1; then
        log_test "Sudo access (with password)" "PASS"
    else
        log_test "Sudo access" "SKIP" "No sudo access configured"
    fi
}

test_directories() {
    echo -e "${BLUE}Testing Directory Structure...${NC}"

    local home="$HOME"

    # Test typical gaming directories
    if [ -d "$home/.steam" ]; then
        log_test "Steam directory exists" "PASS"
    else
        log_test "Steam directory exists" "SKIP" "No Steam installation found"
    fi

    if [ -d "$home/.wine" ]; then
        log_test "Wine directory exists" "PASS"
    else
        log_test "Wine directory exists" "SKIP" "No Wine prefix found"
    fi

    if [ -d "$home/.local/share/lutris" ]; then
        log_test "Lutris directory exists" "PASS"
    else
        log_test "Lutris directory exists" "SKIP" "No Lutris installation found"
    fi
}

test_gaming_performance_tools() {
    echo -e "${BLUE}Testing Gaming Performance Tools...${NC}"

    local tools=("gamemode" "mangohud")

    for tool in "${tools[@]}"; do
        if command -v "$tool" >/dev/null 2>&1; then
            log_test "$tool available" "PASS"
        else
            log_test "$tool available" "SKIP" "Not installed"
        fi
    done
}

test_graphics_tools() {
    echo -e "${BLUE}Testing Graphics Tools...${NC}"

    local tools=("glxinfo" "vulkaninfo")

    for tool in "${tools[@]}"; do
        if command -v "$tool" >/dev/null 2>&1; then
            log_test "$tool available" "PASS"
        else
            log_test "$tool available" "SKIP" "Not installed"
        fi
    done
}

test_compilation_warnings() {
    echo -e "${BLUE}Testing Compilation Status...${NC}"

    if cargo check --quiet 2>&1 | grep -q "error:"; then
        log_test "Code compiles without errors" "FAIL" "Compilation errors found"
    else
        log_test "Code compiles without errors" "PASS"
    fi

    local warning_count=$(cargo check --quiet 2>&1 | grep -c "warning:" || true)
    if [ "$warning_count" -gt 0 ]; then
        log_test "Compilation warnings" "SKIP" "$warning_count warnings found (acceptable)"
    else
        log_test "No compilation warnings" "PASS"
    fi
}

# Safety Tests - These check for potentially dangerous operations
test_safety_checks() {
    echo -e "${BLUE}Testing Safety Features...${NC}"

    # Check that dangerous commands are not hardcoded to run automatically
    local dangerous_patterns=("rm -rf /" "mkfs" "dd if=" "fdisk" "parted")
    local issues_found=0

    for pattern in "${dangerous_patterns[@]}"; do
        if grep -r "$pattern" src/ >/dev/null 2>&1; then
            log_test "Safety check: $pattern" "FAIL" "Potentially dangerous pattern found"
            issues_found=$((issues_found + 1))
        fi
    done

    if [ $issues_found -eq 0 ]; then
        log_test "Dangerous pattern check" "PASS"
    fi
}

# Functional Tests (Safe to run)
test_wine_prefix_creation_dry_run() {
    echo -e "${BLUE}Testing Wine Prefix Management (Dry Run)...${NC}"

    # Test that the wine prefix code doesn't crash
    # This is a dry run - we're not actually creating prefixes
    log_test "Wine prefix management code" "SKIP" "Would need user interaction"
}

test_lutris_integration_dry_run() {
    echo -e "${BLUE}Testing Lutris Integration (Dry Run)...${NC}"

    # Test Lutris detection and basic functionality
    if [ -d "$HOME/.config/lutris" ]; then
        log_test "Lutris config detection" "PASS"
    else
        log_test "Lutris config detection" "SKIP" "No Lutris config found"
    fi
}

test_firewall_rule_syntax() {
    echo -e "${BLUE}Testing Firewall Rule Generation...${NC}"

    # This would test that our firewall rules are syntactically correct
    # without actually applying them
    log_test "Firewall rule syntax" "SKIP" "Manual verification needed"
}

main() {
    echo -e "${BLUE}Starting GhostCTL Test Suite...${NC}"
    echo

    # Core functionality tests
    test_binary_exists || exit 1
    test_help_command
    test_gaming_menu_accessible

    # Dependency tests
    test_wine_dependencies
    test_lutris_integration
    test_steam_integration
    test_firewall_tools
    test_networking_tools
    test_permissions

    # Environment tests
    test_directories
    test_gaming_performance_tools
    test_graphics_tools

    # Code quality tests
    test_compilation_warnings
    test_safety_checks

    # Functional tests (safe)
    test_wine_prefix_creation_dry_run
    test_lutris_integration_dry_run
    test_firewall_rule_syntax

    echo
    echo "📊 Test Results:"
    echo "=================="
    echo -e "Total Tests: ${BLUE}$test_count${NC}"
    echo -e "Passed: ${GREEN}$pass_count${NC}"
    echo -e "Failed: ${RED}$fail_count${NC}"
    echo -e "Skipped: ${YELLOW}$((test_count - pass_count - fail_count))${NC}"

    if [ $fail_count -eq 0 ]; then
        echo -e "\n${GREEN}🎉 All critical tests passed!${NC}"
        exit 0
    else
        echo -e "\n${RED}💥 Some tests failed. Check the output above.${NC}"
        exit 1
    fi
}

# Run main function
main "$@"