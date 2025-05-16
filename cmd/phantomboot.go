package cmd

import (
	"fmt"

	"github.com/GhostKellz/ghostctl/utils"
	"github.com/spf13/cobra"
)

var phantombootCmd = &cobra.Command{
	Use:   "phantomboot",
	Short: "Btrfs recovery ISO and restore tool",
	Long:  `Tools for creating and restoring Btrfs snapshots and recovery ISOs`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("phantomboot: Btrfs recovery ISO tool (WIP)")
	},
}

var btrfsRestoreCmd = &cobra.Command{
	Use:   "restore [snapshot] [mountpoint]",
	Short: "Restore a Btrfs snapshot to a mountpoint",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		snapshot := args[0]
		mountpoint := args[1]
		fmt.Printf("Restoring Btrfs snapshot %s to %s...\n", snapshot, mountpoint)
		out, err := utils.RunCommandOutput("sudo", "btrfs", "send", snapshot, "|", "sudo", "btrfs", "receive", mountpoint)
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(phantombootCmd)
	phantombootCmd.AddCommand(btrfsRestoreCmd)
}
