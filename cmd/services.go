package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Add a services command to list running systemd services
var servicesCmd = &cobra.Command{
	Use:   "services",
	Short: "List running systemd services",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Running systemd services:")
		// Example: systemctl list-units --type=service --state=running
	},
}

func init() {
	rootCmd.AddCommand(servicesCmd)
}
