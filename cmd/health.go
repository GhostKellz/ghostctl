package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
)

// Add a health command to check system health
var healthCmd = &cobra.Command{
	Use:   "health",
	Short: "Check system health and status",
	Long:  `Run a series of checks for disk, memory, network, and service health`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("System health check (WIP)")
	},
}

func init() {
	rootCmd.AddCommand(healthCmd)
}