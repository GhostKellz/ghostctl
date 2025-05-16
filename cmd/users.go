package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Add a users command to list system users
var usersCmd = &cobra.Command{
	Use:   "users",
	Short: "List system users",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("System Users:")
		// Example: parse /etc/passwd
	},
}

func init() {
	rootCmd.AddCommand(usersCmd)
}
