package cmd

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

var backupsMenuCmd = &cobra.Command{
	Use:   "menu",
	Short: "Interactive menu for backup commands",
	Run: func(cmd *cobra.Command, args []string) {
		bkCmds := []string{"restic-setup", "restic-backup", "restic-restore", "rclone-setup", "rclone-backup", "rclone-restore"}
		desc := []string{
			"Setup restic with Minio/systemd",
			"Trigger a restic backup",
			"Restore from a restic backup",
			"Setup rclone for cloud backups",
			"Trigger a rclone backup",
			"Restore from a rclone backup",
		}
		for {
			fmt.Println("\nBackups Menu:")
			for i, s := range bkCmds {
				fmt.Printf("%d) %s - %s\n", i+1, s, desc[i])
			}
			fmt.Println("0) Back")
			fmt.Print("Select a backup command: ")
			r := bufio.NewReader(os.Stdin)
			input, _ := r.ReadString('\n')
			input = strings.TrimSpace(input)
			choice, _ := strconv.Atoi(input)
			if choice == 0 {
				return
			}
			if choice > 0 && choice <= len(bkCmds) {
				fmt.Printf("Running: %s\n", bkCmds[choice-1])
				cmd.Root().SetArgs([]string{"backups", bkCmds[choice-1]})
				cmd.Root().Execute()
			} else {
				fmt.Println("Invalid option.")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(&cobra.Command{
		Use:   "backups",
		Short: "Backup and restore management",
	})
	rootCmd.AddCommand(backupsMenuCmd)
}
