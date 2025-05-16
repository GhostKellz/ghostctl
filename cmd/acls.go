/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// aclsCmd represents the acls command
var aclsCmd = &cobra.Command{
	Use:   "acls",
	Short: "Manage Tailscale/Headscale ACLs",
	Long:  `Manage Tailscale/Headscale ACLs`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("acls called")
	},
}

func init() {
	rootCmd.AddCommand(aclsCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// aclsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// aclsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
