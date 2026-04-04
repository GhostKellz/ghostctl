#!/bin/bash

# GhostCTL Gaming Features Test Script
# Specifically tests all the new gaming/wine/lutris features

set -e

echo "🎮 GhostCTL Gaming Features Test"
echo "================================"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

test_wine_prefix_management() {
    echo -e "${BLUE}Testing Wine Prefix Management Features...${NC}"

    # Test directory structure for prefixes
    local prefix_base="$HOME/Games/prefixes"

    echo "  📁 Testing prefix directory structure"
    if mkdir -p "$prefix_base/test_prefix" 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create prefix directories"
        rmdir "$prefix_base/test_prefix" 2>/dev/null
    else
        echo -e "    ${RED}❌${NC} Cannot create prefix directories"
    fi

    # Test metadata handling
    echo "  📋 Testing prefix metadata"
    local test_metadata="name=test\narch=win64\ncreated=$(date)"
    if echo -e "$test_metadata" > /tmp/test_prefix.info 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create prefix metadata"
        rm -f /tmp/test_prefix.info
    else
        echo -e "    ${RED}❌${NC} Cannot create prefix metadata"
    fi

    # Test backup directory
    echo "  💾 Testing backup functionality"
    local backup_base="$HOME/Games/prefix_backups"
    if mkdir -p "$backup_base" 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create backup directories"
    else
        echo -e "    ${RED}❌${NC} Cannot create backup directories"
    fi

    # Test Wine availability
    if command -v wine >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Wine is available"
        wine --version 2>/dev/null | head -1 | sed 's/^/      /'
    else
        echo -e "    ${YELLOW}⚠️${NC} Wine not installed - prefix management will be limited"
    fi

    # Test winetricks availability
    if command -v winetricks >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Winetricks is available"
    else
        echo -e "    ${YELLOW}⚠️${NC} Winetricks not installed - component management limited"
    fi
}

test_lutris_integration() {
    echo -e "${BLUE}Testing Lutris Integration...${NC}"

    # Test Lutris installation
    if command -v lutris >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Lutris is installed"

        # Test Lutris directories
        local lutris_config="$HOME/.config/lutris"
        local lutris_data="$HOME/.local/share/lutris"

        if [ -d "$lutris_config" ]; then
            echo -e "    ${GREEN}✅${NC} Lutris config directory exists"

            # Check for games config
            if [ -d "$lutris_config/games" ]; then
                local game_count=$(find "$lutris_config/games" -name "*.yml" 2>/dev/null | wc -l)
                echo -e "    ${GREEN}✅${NC} Found $game_count game configurations"
            else
                echo -e "    ${YELLOW}⚠️${NC} No game configurations found"
            fi
        else
            echo -e "    ${YELLOW}⚠️${NC} Lutris config directory not found"
        fi

        if [ -d "$lutris_data" ]; then
            echo -e "    ${GREEN}✅${NC} Lutris data directory exists"

            # Check for runners
            if [ -d "$lutris_data/runners/wine" ]; then
                local runner_count=$(find "$lutris_data/runners/wine" -maxdepth 1 -type d | wc -l)
                echo -e "    ${GREEN}✅${NC} Found $((runner_count - 1)) Wine runners"
            else
                echo -e "    ${YELLOW}⚠️${NC} No Wine runners found"
            fi
        else
            echo -e "    ${YELLOW}⚠️${NC} Lutris data directory not found"
        fi

        # Test database access (if available)
        local lutris_db="$lutris_data/pga.db"
        if [ -f "$lutris_db" ] && command -v sqlite3 >/dev/null 2>&1; then
            local db_games=$(sqlite3 "$lutris_db" "SELECT COUNT(*) FROM games;" 2>/dev/null || echo "0")
            echo -e "    ${GREEN}✅${NC} Database contains $db_games games"
        else
            echo -e "    ${YELLOW}⚠️${NC} Cannot access Lutris database"
        fi
    else
        echo -e "    ${YELLOW}⚠️${NC} Lutris not installed"
    fi
}

test_steam_proton_features() {
    echo -e "${BLUE}Testing Steam/Proton Features...${NC}"

    # Test Steam installation
    if command -v steam >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Steam is installed"

        # Test Steam directories
        local steam_dir="$HOME/.steam/steam"
        if [ -d "$steam_dir" ]; then
            echo -e "    ${GREEN}✅${NC} Steam directory exists"

            # Check for Proton installations
            local proton_dir="$steam_dir/steamapps/common"
            if [ -d "$proton_dir" ]; then
                local proton_count=$(find "$proton_dir" -maxdepth 1 -name "*Proton*" -type d 2>/dev/null | wc -l)
                if [ "$proton_count" -gt 0 ]; then
                    echo -e "    ${GREEN}✅${NC} Found $proton_count Proton versions"
                    find "$proton_dir" -maxdepth 1 -name "*Proton*" -type d 2>/dev/null | head -3 | sed 's/.*\//      - /'
                else
                    echo -e "    ${YELLOW}⚠️${NC} No Proton versions found"
                fi
            fi

            # Check compatibility tools directory
            local compat_dir="$steam_dir/compatibilitytools.d"
            if [ -d "$compat_dir" ]; then
                local tool_count=$(find "$compat_dir" -maxdepth 1 -type d | wc -l)
                echo -e "    ${GREEN}✅${NC} Found $((tool_count - 1)) compatibility tools"
            else
                echo -e "    ${YELLOW}⚠️${NC} No custom compatibility tools directory"
            fi

            # Check library folders
            local library_vdf="$steam_dir/steamapps/libraryfolders.vdf"
            if [ -f "$library_vdf" ]; then
                echo -e "    ${GREEN}✅${NC} Steam library configuration found"
            else
                echo -e "    ${YELLOW}⚠️${NC} No library configuration found"
            fi
        else
            echo -e "    ${YELLOW}⚠️${NC} Steam directory not found"
        fi
    else
        echo -e "    ${YELLOW}⚠️${NC} Steam not installed"
    fi
}

test_gaming_performance_tools() {
    echo -e "${BLUE}Testing Gaming Performance Tools...${NC}"

    # Test GameMode
    if command -v gamemoderun >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} GameMode is installed"

        # Check if daemon is running
        if pgrep gamemode >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} GameMode daemon is running"
        else
            echo -e "    ${YELLOW}⚠️${NC} GameMode daemon not running"
        fi
    else
        echo -e "    ${YELLOW}⚠️${NC} GameMode not installed"
    fi

    # Test MangoHud
    if command -v mangohud >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} MangoHud is installed"
    else
        echo -e "    ${YELLOW}⚠️${NC} MangoHud not installed"
    fi

    # Test graphics info tools
    if command -v vulkaninfo >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Vulkan tools available"
        local vulkan_devices=$(vulkaninfo --summary 2>/dev/null | grep -c "GPU" || echo "0")
        echo -e "      Found $vulkan_devices Vulkan devices"
    else
        echo -e "    ${YELLOW}⚠️${NC} Vulkan tools not available"
    fi

    if command -v glxinfo >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} OpenGL tools available"
        local gl_renderer=$(glxinfo 2>/dev/null | grep "OpenGL renderer" | head -1 | cut -d: -f2- | xargs)
        echo -e "      Renderer: $gl_renderer"
    else
        echo -e "    ${YELLOW}⚠️${NC} OpenGL tools not available"
    fi
}

test_dxvk_vkd3d() {
    echo -e "${BLUE}Testing DXVK/VKD3D Support...${NC}"

    # These would normally be installed through Steam or Lutris
    local dxvk_locations=(
        "$HOME/.local/share/lutris/runtime/dxvk"
        "$HOME/.steam/steam/steamapps/common/Proton*/dist/lib/wine/dxvk"
    )

    local dxvk_found=0
    for location in "${dxvk_locations[@]}"; do
        if ls $location >/dev/null 2>&1; then
            dxvk_found=1
            break
        fi
    done

    if [ $dxvk_found -eq 1 ]; then
        echo -e "    ${GREEN}✅${NC} DXVK installation detected"
    else
        echo -e "    ${YELLOW}⚠️${NC} No DXVK installation found"
    fi

    # Check for VKD3D
    local vkd3d_found=0
    for location in "${dxvk_locations[@]/dxvk/vkd3d}"; do
        if ls $location >/dev/null 2>&1; then
            vkd3d_found=1
            break
        fi
    done

    if [ $vkd3d_found -eq 1 ]; then
        echo -e "    ${GREEN}✅${NC} VKD3D installation detected"
    else
        echo -e "    ${YELLOW}⚠️${NC} No VKD3D installation found"
    fi
}

test_wine_tools() {
    echo -e "${BLUE}Testing Advanced Wine Tools...${NC}"

    # Test Wine installation
    if command -v wine >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Wine is available"
        local wine_version=$(wine --version 2>/dev/null)
        echo -e "      Version: $wine_version"
    else
        echo -e "    ${YELLOW}⚠️${NC} Wine not installed"
        return
    fi

    # Test Winetricks
    if command -v winetricks >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Winetricks is available"
    else
        echo -e "    ${YELLOW}⚠️${NC} Winetricks not available"
    fi

    # Test Wine prefix directories
    local wine_prefixes=(
        "$HOME/.wine"
        "$HOME/Games/prefixes"
        "$HOME/.local/share/bottles/bottles"
    )

    local prefix_count=0
    for prefix_dir in "${wine_prefixes[@]}"; do
        if [ -d "$prefix_dir" ]; then
            if [ -d "$prefix_dir/drive_c" ]; then
                prefix_count=$((prefix_count + 1))
            elif [ "$prefix_dir" = "$HOME/Games/prefixes" ] && [ -d "$prefix_dir" ]; then
                # Count subdirectories with drive_c
                local sub_prefixes=$(find "$prefix_dir" -name "drive_c" -type d 2>/dev/null | wc -l)
                prefix_count=$((prefix_count + sub_prefixes))
            fi
        fi
    done

    if [ $prefix_count -gt 0 ]; then
        echo -e "    ${GREEN}✅${NC} Found $prefix_count Wine prefixes"
    else
        echo -e "    ${YELLOW}⚠️${NC} No Wine prefixes found"
    fi

    # Test for common Wine components
    local default_prefix="$HOME/.wine"
    if [ -d "$default_prefix/drive_c/windows/system32" ]; then
        local dll_count=$(find "$default_prefix/drive_c/windows/system32" -name "*.dll" 2>/dev/null | wc -l)
        echo -e "    ${GREEN}✅${NC} Default prefix has $dll_count DLLs"
    fi
}

test_directory_permissions() {
    echo -e "${BLUE}Testing Directory Permissions...${NC}"

    local test_dirs=(
        "$HOME/Games"
        "$HOME/Games/prefixes"
        "$HOME/Games/prefix_backups"
        "$HOME/winetricks_scripts"
        "$HOME/winetricks_profiles"
    )

    for dir in "${test_dirs[@]}"; do
        if mkdir -p "$dir" 2>/dev/null; then
            echo -e "    ${GREEN}✅${NC} Can create/access $dir"
        else
            echo -e "    ${RED}❌${NC} Cannot create $dir"
        fi
    done

    # Test temp file creation
    if touch /tmp/ghostctl_test 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create temporary files"
        rm -f /tmp/ghostctl_test
    else
        echo -e "    ${RED}❌${NC} Cannot create temporary files"
    fi
}

test_script_directories() {
    echo -e "${BLUE}Testing Script Storage...${NC}"

    local script_dirs=(
        "$HOME/winetricks_scripts"
        "$HOME/winetricks_profiles"
        "$HOME/custom_verbs"
        "$HOME/nftables_backups"
    )

    for dir in "${script_dirs[@]}"; do
        if mkdir -p "$dir" 2>/dev/null; then
            # Test script creation
            local test_script="$dir/test_script.sh"
            if echo "#!/bin/bash" > "$test_script" 2>/dev/null; then
                echo -e "    ${GREEN}✅${NC} Can create scripts in $dir"
                rm -f "$test_script"
            else
                echo -e "    ${RED}❌${NC} Cannot create scripts in $dir"
            fi
        else
            echo -e "    ${RED}❌${NC} Cannot create directory $dir"
        fi
    done
}

main() {
    echo -e "${BLUE}Starting Gaming Features Test...${NC}"
    echo

    test_wine_prefix_management
    echo

    test_lutris_integration
    echo

    test_steam_proton_features
    echo

    test_gaming_performance_tools
    echo

    test_dxvk_vkd3d
    echo

    test_wine_tools
    echo

    test_directory_permissions
    echo

    test_script_directories
    echo

    echo -e "${GREEN}🎮 Gaming Features Test Complete!${NC}"
    echo
    echo "Summary:"
    echo "========"
    echo "• Wine Prefix Management: Ready for testing"
    echo "• Lutris Integration: Available for supported configurations"
    echo "• Steam/Proton Features: Works with existing Steam installations"
    echo "• Performance Tools: Depends on system packages"
    echo "• Advanced Wine Tools: Ready with Wine/Winetricks installed"
    echo
    echo "Next Steps:"
    echo "• Install missing gaming packages as needed"
    echo "• Test actual prefix creation in a safe environment"
    echo "• Verify Lutris integration with real games"
    echo "• Test firewall rules in isolated environment"
}

main "$@"