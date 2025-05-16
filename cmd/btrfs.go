// cmd/btrfs.go
package cmd

import (
	"fmt"
	"ghostctl/utils"

	"github.com/spf13/cobra"
)

var btrfsCmd = &cobra.Command{
	Use:   "btrfs",
	Short: "Show Btrfs filesystem info",
	Long:  `Display Btrfs filesystem usage and status`,
	Run: func(cmd *cobra.Command, args []string) {
		out, err := utils.RunCommandOutput("btrfs", "filesystem", "usage", "/")
		if err != nil {
			fmt.Println("Error running btrfs:", err)
			return
		}
		fmt.Println(out)
	},
}

var btrfsSnapshotCmd = &cobra.Command{
	Use:   "snapshot [subvol] [dest]",
	Short: "Create a Btrfs snapshot",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		subvol := args[0]
		dest := args[1]
		fmt.Printf("Creating Btrfs snapshot of %s to %s...\n", subvol, dest)
		out, err := utils.RunCommandOutput("sudo", "btrfs", "subvolume", "snapshot", subvol, dest)
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

var btrfsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Btrfs subvolumes",
	Run: func(cmd *cobra.Command, args []string) {
		out, err := utils.RunCommandOutput("sudo", "btrfs", "subvolume", "list", "/")
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(btrfsCmd)
	btrfsCmd.AddCommand(btrfsSnapshotCmd)
	btrfsCmd.AddCommand(btrfsListCmd)
}
