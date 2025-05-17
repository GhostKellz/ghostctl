/*
Copyright © 2025 GhostKellz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/
package cmd

import (
	"bufio"
	"os"
	"os/exec"
	"strings"

	"github.com/spf13/cobra"
)

var cfgFile string

// Helper to run a command and print output/errors
func runAndPrint(cmd *cobra.Command, name string, args ...string) error {
	c := exec.Command(name, args...)
	c.Stderr = os.Stderr
	c.Stdout = os.Stdout
	return c.Run()
}

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "ghostctl",
	Short: "GhostCTL - Manage and control PhantomDNS and related services",
	Long:  `GhostCTL is a CLI tool to manage, control, and query the status of PhantomDNS and related GhostKellz services.`,
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	// Persistent flag for config file
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.ghostctl.yaml)")

	// Version command
	rootCmd.AddCommand(&cobra.Command{
		Use:   "version",
		Short: "Print the version number of GhostCTL",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Println("GhostCTL v0.1.0")
		},
	})

	// Btrfs/Snapper snapshot restore
	rootCmd.AddCommand(&cobra.Command{
		Use:   "restore-snapshot [snapshot] [mountpoint]",
		Short: "Restore a Btrfs/Snapper snapshot in a chroot environment",
		Args:  cobra.ExactArgs(2),
		Run: func(cmd *cobra.Command, args []string) {
			snapshot := args[0]
			mountpoint := args[1]
			cmd.Printf("Restoring snapshot %s to %s...\n", snapshot, mountpoint)
			runAndPrint(cmd, "umount", mountpoint)
			if err := runAndPrint(cmd, "mount", "-o", "subvol="+snapshot, "/", mountpoint); err != nil {
				cmd.Printf("Failed to mount snapshot: %v\n", err)
				return
			}
			cmd.Println("Snapshot restored. You may now chroot into the mountpoint.")
		},
	})

	// Arch system maintenance
	rootCmd.AddCommand(&cobra.Command{
		Use:   "arch-fix",
		Short: "Run common Arch Linux system maintenance and fixups",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Println("Running pacman -Syu...")
			runAndPrint(cmd, "pacman", "-Syu", "--noconfirm")
			cmd.Println("Regenerating initramfs...")
			runAndPrint(cmd, "mkinitcpio", "-P")
			cmd.Println("Checking hooks...")
			cmd.Println("Arch maintenance complete.")
		},
	})

	// Makepkg/dev issues
	rootCmd.AddCommand(&cobra.Command{
		Use:   "fix-makepkg",
		Short: "Attempt to fix common makepkg and dev environment issues",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Println("Cleaning makepkg cache...")
			runAndPrint(cmd, "bash", "-c", "rm -rf $HOME/.cache/pacman/pkg/*")
			cmd.Println("Checking PKGBUILD...")
			cmd.Println("Installing base-devel if missing...")
			runAndPrint(cmd, "pacman", "-S", "--needed", "base-devel", "--noconfirm")
			cmd.Println("Makepkg/dev environment fixes complete.")
		},
	})

	// Tailscale/Headscale advanced subcommands
	tailscaleCmd := &cobra.Command{
		Use:   "tailscale",
		Short: "Tailscale/Headscale troubleshooting and config",
	}
	tailscaleCmd.AddCommand(&cobra.Command{
		Use:   "status",
		Short: "Show Tailscale status",
		Run: func(cmd *cobra.Command, args []string) {
			runAndPrint(cmd, "tailscale", "status")
		},
	})
	tailscaleCmd.AddCommand(&cobra.Command{
		Use:   "up",
		Short: "Re-advertise routes",
		Run: func(cmd *cobra.Command, args []string) {
			runAndPrint(cmd, "tailscale", "up", "--advertise-routes=192.168.0.0/24")
		},
	})
	tailscaleCmd.AddCommand(&cobra.Command{
		Use:   "routes",
		Short: "Show and debug Tailscale routes",
		Run: func(cmd *cobra.Command, args []string) {
			runAndPrint(cmd, "tailscale", "ip", "-4")
		},
	})
	tailscaleCmd.AddCommand(&cobra.Command{
		Use:   "debug",
		Short: "Show Tailscale debug info",
		Run: func(cmd *cobra.Command, args []string) {
			runAndPrint(cmd, "tailscale", "bugreport")
		},
	})
	rootCmd.AddCommand(tailscaleCmd)

	// Recovery menu
	rootCmd.AddCommand(&cobra.Command{
		Use:   "recovery-menu",
		Short: "Interactive Btrfs/system recovery menu",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Println("Recovery Menu:")
			cmd.Println("1) List Btrfs snapshots")
			cmd.Println("2) Restore snapshot")
			cmd.Println("3) Exit")
			cmd.Print("Select option: ")
			scanner := bufio.NewScanner(os.Stdin)
			scanner.Scan()
			choice := strings.TrimSpace(scanner.Text())
			switch choice {
			case "1":
				cmd.Println("Listing snapshots...")
				runAndPrint(cmd, "btrfs", "subvolume", "list", "/")
			case "2":
				cmd.Println("Enter snapshot name:")
				scanner.Scan()
				snap := scanner.Text()
				cmd.Println("Enter mountpoint:")
				scanner.Scan()
				mnt := scanner.Text()
				cmd.Printf("Restoring snapshot %s to %s...\n", snap, mnt)
				runAndPrint(cmd, "umount", mnt)
				if err := runAndPrint(cmd, "mount", "-o", "subvol="+snap, "/", mnt); err != nil {
					cmd.Printf("Failed to mount snapshot: %v\n", err)
					return
				}
				cmd.Println("Snapshot restored.")
			case "3":
				cmd.Println("Exiting recovery menu.")
			default:
				cmd.Println("Invalid option.")
			}
		},
	})

	// Systemd service management
	rootCmd.AddCommand(&cobra.Command{
		Use:   "systemd-service",
		Short: "Create, enable, or disable systemd services",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Println("Systemd Service Management:")
			cmd.Println("1) Enable service")
			cmd.Println("2) Disable service")
			cmd.Println("3) Start service")
			cmd.Println("4) Stop service")
			cmd.Println("5) Status of service")
			cmd.Print("Select option: ")
			scanner := bufio.NewScanner(os.Stdin)
			scanner.Scan()
			choice := strings.TrimSpace(scanner.Text())
			cmd.Print("Enter service name: ")
			scanner.Scan()
			service := scanner.Text()
			switch choice {
			case "1":
				runAndPrint(cmd, "systemctl", "enable", service)
				cmd.Printf("Enabled %s\n", service)
			case "2":
				runAndPrint(cmd, "systemctl", "disable", service)
				cmd.Printf("Disabled %s\n", service)
			case "3":
				runAndPrint(cmd, "systemctl", "start", service)
				cmd.Printf("Started %s\n", service)
			case "4":
				runAndPrint(cmd, "systemctl", "stop", service)
				cmd.Printf("Stopped %s\n", service)
			case "5":
				runAndPrint(cmd, "systemctl", "status", service)
			default:
				cmd.Println("Invalid option.")
			}
		},
	})

	// Interactive main menu
	rootCmd.AddCommand(&cobra.Command{
		Use:   "menu",
		Short: "Interactive main menu for all features",
		Run: func(cmd *cobra.Command, args []string) {
			for {
				cmd.Println("\nGhostCTL Main Menu:")
				cmd.Println("1) Btrfs/Snapper Recovery Menu")
				cmd.Println("2) Arch System Maintenance")
				cmd.Println("3) Makepkg/Dev Fixes")
				cmd.Println("4) Tailscale/Headscale Tools")
				cmd.Println("5) Systemd Service Management")
				cmd.Println("6) Exit")
				cmd.Print("Select option: ")
				scanner := bufio.NewScanner(os.Stdin)
				scanner.Scan()
				choice := strings.TrimSpace(scanner.Text())
				switch choice {
				case "1":
					rootCmd.SetArgs([]string{"recovery-menu"})
					rootCmd.Execute()
				case "2":
					rootCmd.SetArgs([]string{"arch-fix"})
					rootCmd.Execute()
				case "3":
					rootCmd.SetArgs([]string{"fix-makepkg"})
					rootCmd.Execute()
				case "4":
					rootCmd.SetArgs([]string{"tailscale", "status"})
					rootCmd.Execute()
				case "5":
					rootCmd.SetArgs([]string{"systemd-service"})
					rootCmd.Execute()
				case "6":
					cmd.Println("Exiting GhostCTL menu.")
					return
				default:
					cmd.Println("Invalid option.")
				}
			}
		},
	})
}

func initConfig() {
	if cfgFile != "" {
		// TODO: Add config file loading logic here
	}
}
