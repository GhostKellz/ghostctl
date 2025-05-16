package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Add a network command to show network interfaces and status
var networkCmd = &cobra.Command{
	Use:   "network",
	Short: "Show network interfaces and status",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Network Interfaces:")
		// Example: ip addr
	},
}

func init() {
	rootCmd.AddCommand(networkCmd)
}
