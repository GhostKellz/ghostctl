package main

import (
	"fmt"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "app",
	Short: "A brief description of your application",
	Long:  `A longer description that spans multiple lines and likely contains examples
and usage of using your application. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
}

var snapperCmd = &cobra.Command{
	Use:   "snapper",
	Short: "Manage Btrfs snapshots with snapper",
	Long:  `Create, list, and restore Btrfs snapshots using snapper`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("snapper: Btrfs snapshot management (WIP)")
	},
}

func init() {
	rootCmd.AddCommand(snapperCmd)
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
	}
}