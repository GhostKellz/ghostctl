// cmd/routes.go
package cmd

import (
	"fmt"

	"github.com/GhostKellz/ghostctl/utils"
	"github.com/spf13/cobra"
)

var routesCmd = &cobra.Command{
	Use:   "routes",
	Short: "Advertise or remove Tailscale routes",
	Long:  `Advertise or remove Tailscale routes using tailscale CLI`,
	Run: func(cmd *cobra.Command, args []string) {
		out, err := utils.RunCommandOutput("tailscale", "status", "--json")
		if err != nil {
			fmt.Println("Error running tailscale:", err)
			return
		}
		fmt.Println(out)
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
