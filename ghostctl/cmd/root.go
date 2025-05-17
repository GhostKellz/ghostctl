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

// Helper to run a command and print output/errors, capturing output for verbose mode
func runAndPrint(cmd *cobra.Command, name string, args ...string) error {
	c := exec.Command(name, args...)
	c.Stderr = os.Stderr
	c.Stdout = os.Stdout
	if verbose, _ := cmd.Flags().GetBool("verbose"); verbose {
		cmd.Printf("Running: %s %s\n", name, strings.Join(args, " "))
	}
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
	rootCmd.PersistentFlags().BoolP("verbose", "v", false, "Enable verbose output")

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

	// Backup menu
	rootCmd.AddCommand(&cobra.Command{
		Use:   "backup-menu",
		Short: "Interactive backup menu (Restic, Snapper, more in future)",
		Run: func(cmd *cobra.Command, args []string) {
			for {
				cmd.Println("\nBackup Menu:")
				cmd.Println("1) Setup Restic backup (systemd + .env)")
				cmd.Println("2) Setup Snapper configs (home/root)")
				cmd.Println("3) Exit")
				cmd.Print("Select option: ")
				scanner := bufio.NewScanner(os.Stdin)
				scanner.Scan()
				choice := strings.TrimSpace(scanner.Text())
				switch choice {
				case "1":
					cmd.Println("Installing restic if needed...")
					runAndPrint(cmd, "bash", "-c", "command -v restic || sudo pacman -S --noconfirm restic")
					cmd.Println("Prompting for backup source directory:")
					cmd.Print("Enter source directory to backup (default: /home): ")
					scanner.Scan()
					source := scanner.Text()
					if source == "" {
						source = "/home"
					}
					cmd.Print("Enter backup destination (e.g. s3:s3.amazonaws.com/bucket, /mnt/backup, etc): ")
					scanner.Scan()
					dest := scanner.Text()
					cmd.Print("Enter schedule (e.g. daily, weekly): ")
					scanner.Scan()
					sched := scanner.Text()
					cmd.Println("Writing restic-backup.service and restic-backup.timer...")
					// Write systemd unit
					os.WriteFile("/etc/systemd/system/restic-backup.service", []byte(`[Unit]
Description=Restic Backup

[Service]
Type=oneshot
EnvironmentFile=/etc/restic.env
ExecStart=/usr/bin/restic backup `+source+` --repo `+dest+`
ExecStartPost=/usr/bin/restic forget --prune --keep-last 7 --repo `+dest+`
`), 0644)
					os.WriteFile("/etc/systemd/system/restic-backup.timer", []byte(`[Unit]
Description=Restic Backup Timer

[Timer]
OnCalendar=`+sched+`
Persistent=true

[Install]
WantedBy=timers.target
`), 0644)
					cmd.Println("Writing /etc/restic.env (edit this file for credentials)")
					os.WriteFile("/etc/restic.env", []byte(`# RESTIC_PASSWORD=yourpassword
# AWS_ACCESS_KEY_ID=yourkey
# AWS_SECRET_ACCESS_KEY=yoursecret
# RESTIC_REPOSITORY=s3:s3.amazonaws.com/bucket
`), 0644)
					cmd.Println("Enabling and starting restic-backup.timer...")
					runAndPrint(cmd, "systemctl", "daemon-reload")
					runAndPrint(cmd, "systemctl", "enable", "--now", "restic-backup.timer")
					cmd.Println("Restic backup setup complete! Edit /etc/restic.env as needed.")
				case "2":
					cmd.Println("Configuring Snapper for home and root...")
					runAndPrint(cmd, "snapper", "-c", "root", "create-config", "/")
					runAndPrint(cmd, "snapper", "-c", "home", "create-config", "/home")
					cmd.Println("Snapper configs created. You may want to edit /etc/snapper/configs/root and /etc/snapper/configs/home.")
				case "3":
					cmd.Println("Exiting backup menu.")
					return
				default:
					cmd.Println("Invalid option.")
				}
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
				cmd.Println("6) Backup Menu")
				cmd.Println("7) Exit")
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
					rootCmd.SetArgs([]string{"backup-menu"})
					rootCmd.Execute()
				case "7":
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
		// Example: Load YAML config and print loaded config if verbose
		if _, err := os.Stat(cfgFile); err == nil {
			f, err := os.Open(cfgFile)
			if err == nil {
				scanner := bufio.NewScanner(f)
				for scanner.Scan() {
					if strings.TrimSpace(scanner.Text()) != "" {
						if verbose, _ := rootCmd.Flags().GetBool("verbose"); verbose {
							rootCmd.Printf("Config: %s\n", scanner.Text())
						}
					}
				}
				f.Close()
			}
		}
	}
}
