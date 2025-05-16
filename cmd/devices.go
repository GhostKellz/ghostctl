// cmd/devices.go
package cmd

import (
	"fmt"

	"github.com/GhostKellz/ghostctl/utils"
	"github.com/spf13/cobra"
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
}
