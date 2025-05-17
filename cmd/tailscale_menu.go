package cmd

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

var tailscaleMenuCmd = &cobra.Command{
	Use:   "menu",
	Short: "Interactive menu for Tailscale commands",
	Run: func(cmd *cobra.Command, args []string) {
		tsCmds := []string{"acls", "routes", "status"}
		desc := []string{
			"Manage Tailscale/Headscale ACLs",
			"Advertise or remove Tailscale routes",
			"Show Tailscale status",
		}
		for {
			fmt.Println("\nTailscale Menu:")
			for i, s := range tsCmds {
				fmt.Printf("%d) %s - %s\n", i+1, s, desc[i])
			}
			fmt.Println("0) Back")
			fmt.Print("Select a Tailscale command: ")
			r := bufio.NewReader(os.Stdin)
			input, _ := r.ReadString('\n')
			input = strings.TrimSpace(input)
			choice, _ := strconv.Atoi(input)
			if choice == 0 {
				return
			}
			if choice > 0 && choice <= len(tsCmds) {
				fmt.Printf("Running: %s\n", tsCmds[choice-1])
				cmd.Root().SetArgs([]string{tsCmds[choice-1]})
				cmd.Root().Execute()
			} else {
				fmt.Println("Invalid option.")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(&cobra.Command{
		Use:   "tailscale",
		Short: "Tailscale/Headscale management",
	})
	rootCmd.AddCommand(tailscaleMenuCmd)
}
