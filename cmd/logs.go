package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Add a logs command to show recent system logs
var logsCmd = &cobra.Command{
	Use:   "logs",
	Short: "Show recent system logs",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Recent system logs:")
		// Example: journalctl -n 50
	},
}

func init() {
	rootCmd.AddCommand(logsCmd)
}
