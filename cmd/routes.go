// cmd/routes.go
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var routesCmd = &cobra.Command{
	Use:   "routes",
	Short: "Advertise or remove Tailscale routes",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Routes tool placeholder — todo: advertise/remove routes")
	},
}

func init() {
	rootCmd.AddCommand(routesCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// routesCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// routesCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
