package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var snapperCmd = &cobra.Command{
	Use:   "snapper",
	Short: "Manage Btrfs snapshots with snapper",
	Long:  `Create, list, and restore Btrfs snapshots using snapper`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("snapper: Btrfs snapshot management (WIP)")
	},
}

func init() {
	rootCmd.AddCommand(snapperCmd)
}
