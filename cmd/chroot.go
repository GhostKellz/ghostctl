package cmd

import (
	"fmt"
	"myproject/utils"

	"github.com/spf13/cobra"
)

var chrootCmd = &cobra.Command{
	Use:   "chroot [mountpoint]",
	Short: "Chroot into a mounted Linux system",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		mountpoint := args[0]
		fmt.Printf("Preparing chroot environment at %s...\n", mountpoint)
		// Mount necessary filesystems
		utils.RunCommandOutput("sudo", "mount", "--bind", "/dev", mountpoint+"/dev")
		utils.RunCommandOutput("sudo", "mount", "--bind", "/proc", mountpoint+"/proc")
		utils.RunCommandOutput("sudo", "mount", "--bind", "/sys", mountpoint+"/sys")
		fmt.Println("Launching chroot shell...")
		out, err := utils.RunCommandOutput("sudo", "chroot", mountpoint)
		if err != nil {
			fmt.Println("Error:", err)
		}
		fmt.Println(out)
	},
}

func init() {
	rootCmd.AddCommand(chrootCmd)
}
