package cmd

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

var menuCmd = &cobra.Command{
	Use:   "menu",
	Short: "Interactive menu for ghostctl",
	Run: func(cmd *cobra.Command, args []string) {
		for {
			fmt.Println("\nGhostctl Interactive Menu:")
			fmt.Println("1) System Info")
			fmt.Println("2) Scripts")
			fmt.Println("3) Tailscale")
			fmt.Println("4) Backups")
			fmt.Println("5) Services")
			fmt.Println("6) Checkup/Doctor")
			fmt.Println("0) Exit")
			fmt.Print("Select an option: ")
			r := bufio.NewReader(os.Stdin)
			input, _ := r.ReadString('\n')
			input = strings.TrimSpace(input)
			choice, _ := strconv.Atoi(input)
			switch choice {
			case 1:
				fmt.Println("\n--- System Info ---")
				// Call system-info command
				cmd.Root().SetArgs([]string{"system-info"})
				cmd.Root().Execute()
			case 2:
				fmt.Println("\n--- Scripts ---")
				// Call scripts menu
				cmd.Root().SetArgs([]string{"scripts", "menu"})
				cmd.Root().Execute()
			case 3:
				fmt.Println("\n--- Tailscale ---")
				cmd.Root().SetArgs([]string{"tailscale", "menu"})
				cmd.Root().Execute()
			case 4:
				fmt.Println("\n--- Backups ---")
				cmd.Root().SetArgs([]string{"backups", "menu"})
				cmd.Root().Execute()
			case 5:
				fmt.Println("\n--- Services ---")
				cmd.Root().SetArgs([]string{"services", "menu"})
				cmd.Root().Execute()
			case 6:
				fmt.Println("\n--- Checkup/Doctor ---")
				cmd.Root().SetArgs([]string{"check"})
				cmd.Root().Execute()
			case 0:
				fmt.Println("Exiting.")
				return
			default:
				fmt.Println("Invalid option.")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(menuCmd)
}
