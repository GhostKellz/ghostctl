package cmd

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

var servicesMenuCmd = &cobra.Command{
	Use:   "menu",
	Short: "Interactive menu for systemd service management",
	Run: func(cmd *cobra.Command, args []string) {
		svcCmds := []string{"list", "status", "enable", "disable", "start", "stop", "restart", "watchlist"}
		desc := []string{
			"List all systemd services",
			"Show status for a service",
			"Enable a service",
			"Disable a service",
			"Start a service",
			"Stop a service",
			"Restart a service",
			"Manage service watchlist",
		}
		for {
			fmt.Println("\nServices Menu:")
			for i, s := range svcCmds {
				fmt.Printf("%d) %s - %s\n", i+1, s, desc[i])
			}
			fmt.Println("0) Back")
			fmt.Print("Select a service command: ")
			r := bufio.NewReader(os.Stdin)
			input, _ := r.ReadString('\n')
			input = strings.TrimSpace(input)
			choice, _ := strconv.Atoi(input)
			if choice == 0 {
				return
			}
			if choice > 0 && choice <= len(svcCmds) {
				fmt.Printf("Running: %s\n", svcCmds[choice-1])
				cmd.Root().SetArgs([]string{"services", svcCmds[choice-1]})
				cmd.Root().Execute()
			} else {
				fmt.Println("Invalid option.")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(servicesMenuCmd)
}
