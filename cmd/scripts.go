package cmd

import (
	"fmt"

	"github.com/GhostKellz/ghostctl/utils"
	"github.com/spf13/cobra"
)

var scriptsCmd = &cobra.Command{
	Use:   "scripts",
	Short: "Collection of useful system scripts",
	Long:  `Run or fetch various system scripts for setup, fixes, and automation`,
}

var fixNvidiaCmd = &cobra.Command{
	Use:   "fix-nvidia-dkms",
	Short: "Fix common NVIDIA DKMS issues on Arch",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Attempting to fix NVIDIA DKMS issues...")
		// Example: sudo mkinitcpio -P && sudo systemctl restart systemd-modules-load.service
		out, err := utils.RunCommandOutput("sudo", "mkinitcpio", "-P")
		if err != nil {
			fmt.Println("mkinitcpio error:", err)
		}
		fmt.Println(out)
		out, err = utils.RunCommandOutput("sudo", "systemctl", "restart", "systemd-modules-load.service")
		if err != nil {
			fmt.Println("systemctl error:", err)
		}
		fmt.Println(out)
	},
}

var mkinitcpioFixCmd = &cobra.Command{
	Use:   "mkinitcpio-fix",
	Short: "Regenerate initramfs and fix common mkinitcpio issues",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Regenerating initramfs with mkinitcpio...")
		out, err := utils.RunCommandOutput("sudo", "mkinitcpio", "-P")
		if err != nil {
			fmt.Println("mkinitcpio error:", err)
		}
		fmt.Println(out)
	},
}

var bootloaderCmd = &cobra.Command{
	Use:   "bootloader",
	Short: "Manage bootloader entries (systemd-boot, GRUB)",
	Long:  `Create, update, or remove bootloader entries for system recovery and multi-boot setups`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("bootloader management: (WIP)")
	},
}

var systemdServiceCmd = &cobra.Command{
	Use:   "systemd-service",
	Short: "Setup or fix systemd services on Arch",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Setting up or fixing systemd services (WIP)")
	},
}

var pullScriptCmd = &cobra.Command{
	Use:   "pull [script]",
	Short: "Pull and display a script from the repository",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		script := args[0]
		fmt.Printf("Fetching script: %s\n", script)
		// Placeholder: In future, fetch from remote repo or local scripts dir
		fmt.Println("(WIP) Script fetch logic goes here.")
	},
}

var archUpdateCmd = &cobra.Command{
	Use:   "update",
	Short: "Update all packages on Arch Linux",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Updating all packages with pacman...")
		out, err := utils.RunCommandOutput("sudo", "pacman", "-Syu", "--noconfirm")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archCleanCmd = &cobra.Command{
	Use:   "clean",
	Short: "Clean package cache on Arch Linux",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Cleaning package cache with pacman...")
		out, err := utils.RunCommandOutput("sudo", "pacman", "-Sc", "--noconfirm")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archMirrorCmd = &cobra.Command{
	Use:   "mirrorlist",
	Short: "Update Arch Linux mirrorlist",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Updating Arch Linux mirrorlist...")
		out, err := utils.RunCommandOutput("sudo", "reflector", "--latest", "20", "--sort", "rate", "--save", "/etc/pacman.d/mirrorlist")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archChrootFixCmd = &cobra.Command{
	Use:   "fix-chroot [mountpoint]",
	Short: "Fix common chroot issues in Arch Linux",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		mountpoint := args[0]
		fmt.Printf("Fixing chroot issues at %s...\n", mountpoint)
		utils.RunCommandOutput("sudo", "mount", "--bind", "/dev", mountpoint+"/dev")
		utils.RunCommandOutput("sudo", "mount", "--bind", "/proc", mountpoint+"/proc")
		utils.RunCommandOutput("sudo", "mount", "--bind", "/sys", mountpoint+"/sys")
		fmt.Println("Chroot environment prepared.")
	},
}

var archKeyringCmd = &cobra.Command{
	Use:   "keyring",
	Short: "Refresh Arch Linux keyring",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Refreshing Arch Linux keyring...")
		out, err := utils.RunCommandOutput("sudo", "pacman-key", "--init")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
		out, err = utils.RunCommandOutput("sudo", "pacman-key", "--populate", "archlinux")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archTimesyncCmd = &cobra.Command{
	Use:   "timesync",
	Short: "Sync system time with NTP",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Syncing system time with NTP...")
		out, err := utils.RunCommandOutput("sudo", "timedatectl", "set-ntp", "true")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archLocaleCmd = &cobra.Command{
	Use:   "locale",
	Short: "Regenerate Arch Linux locales",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Regenerating locales...")
		out, err := utils.RunCommandOutput("sudo", "locale-gen")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archPacmanConfCmd = &cobra.Command{
	Use:   "pacman-conf",
	Short: "Show pacman configuration",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Showing pacman configuration...")
		out, err := utils.RunCommandOutput("cat", "/etc/pacman.conf")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archPacmanLogCmd = &cobra.Command{
	Use:   "pacman-log",
	Short: "Show recent pacman log entries",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Showing recent pacman log entries...")
		out, err := utils.RunCommandOutput("tail", "-n", "50", "/var/log/pacman.log")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var archFsckCmd = &cobra.Command{
	Use:   "fsck [device]",
	Short: "Run fsck on a device",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		device := args[0]
		fmt.Printf("Running fsck on %s...\n", device)
		out, err := utils.RunCommandOutput("sudo", "fsck", "-y", device)
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(scriptsCmd)
	scriptsCmd.AddCommand(fixNvidiaCmd)
	scriptsCmd.AddCommand(mkinitcpioFixCmd)
	scriptsCmd.AddCommand(bootloaderCmd)
	scriptsCmd.AddCommand(systemdServiceCmd)
	scriptsCmd.AddCommand(pullScriptCmd)
	scriptsCmd.AddCommand(archUpdateCmd)
	scriptsCmd.AddCommand(archCleanCmd)
	scriptsCmd.AddCommand(archMirrorCmd)
	scriptsCmd.AddCommand(archChrootFixCmd)
	scriptsCmd.AddCommand(archKeyringCmd)
	scriptsCmd.AddCommand(archTimesyncCmd)
	scriptsCmd.AddCommand(archLocaleCmd)
	scriptsCmd.AddCommand(archPacmanConfCmd)
	scriptsCmd.AddCommand(archPacmanLogCmd)
	scriptsCmd.AddCommand(archFsckCmd)
}
