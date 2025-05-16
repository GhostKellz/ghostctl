package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Show and edit ghostctl configuration",
	Long:  `Display and modify ghostctl configuration file`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("ghostctl config (WIP)")
	},
}

func init() {
	rootCmd.AddCommand(configCmd)
}
