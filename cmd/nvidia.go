// cmd/nvidia.go
package cmd

import (
	"fmt"
	"ghostctl/utils"

	"github.com/spf13/cobra"
)

var nvidiaCmd = &cobra.Command{
	Use:   "nvidia",
	Short: "Rebuild or fix open-source NVIDIA drivers",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Rebuilding NVIDIA DKMS modules...")
		err := utils.RunCommand("sudo", "dkms", "autoinstall")
		if err != nil {
			fmt.Println("Error running dkms autoinstall:", err)
		}
	},
}

func init() {
	rootCmd.AddCommand(nvidiaCmd)
}
