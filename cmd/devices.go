// cmd/devices.go
package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"github.com/GhostKellz/ghostctl/utils"
)

var devicesCmd = &cobra.Command{
	Use:   "devices",
	Short: "List system hardware (PCI, USB, etc.)",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("PCI Devices:")
		out, _ := utils.RunCommandOutput("lspci")
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(devicesCmd)
}}

func init() {
	rootCmd.AddCommand(devicesCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// devicesCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// devicesCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
