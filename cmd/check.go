package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var checkCmd = &cobra.Command{
	Use:   "check",
	Short: "Run system and config checks (doctor)",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Running system and config checks (WIP)...")
		// Add checks for config, binaries, services, etc.
	},
}

func init() {
	rootCmd.AddCommand(checkCmd)
}
