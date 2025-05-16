// cmd/nvidia.go
package cmd

import (
	"fmt"
	"github.com/GhostKellz/ghostctl/utils"
	"github.com/spf13/cobra"
)

var nvidiaCmd = &cobra.Command{
	Use:   "nvidia",
	Short: "Show NVIDIA GPU status and info",
	Long:  `Display NVIDIA GPU information using nvidia-smi`,
	Run: func(cmd *cobra.Command, args []string) {
		out, err := utils.RunCommandOutput("nvidia-smi")
		if err != nil {
			fmt.Println("Error running nvidia-smi:", err)
			return
		}
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(nvidiaCmd)
}
