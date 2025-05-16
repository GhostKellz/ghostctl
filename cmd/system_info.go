package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Add a system-info command to show basic system info
var sysInfoCmd = &cobra.Command{
	Use:   "system-info",
	Short: "Show basic system information",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("System Info:")
		// Example: show kernel, CPU, memory, etc.
	},
}

func init() {
	rootCmd.AddCommand(sysInfoCmd)
}
