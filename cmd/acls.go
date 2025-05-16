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

// aclsListCmd represents the list subcommand of acls
var aclsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List current Tailscale/Headscale ACLs",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Listing current ACLs (WIP)")
	},
}

// aclsApplyCmd represents the apply subcommand of acls
var aclsApplyCmd = &cobra.Command{
	Use:   "apply [file]",
	Short: "Apply ACLs from a file",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		file := args[0]
		fmt.Printf("Applying ACLs from file: %s (WIP)\n", file)
	},
}

// versionCmd represents the version command
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show the version and build info",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("ghostctl version 0.1.0") // You can automate this with ldflags later
	},
}

// completionCmd represents the completion command
var completionCmd = &cobra.Command{
	Use:   "completion [bash|zsh]",
	Short: "Generate shell completion scripts",
	Long: `To load completions:

Bash:
  $ source <(ghostctl completion bash)

  # To load completions for each session, execute once:
  # Linux:
  $ ghostctl completion bash > /etc/bash_completion.d/ghostctl

Zsh:
  $ ghostctl completion zsh > "${fpath[1]}/_ghostctl"
`,
	Args:      cobra.ExactValidArgs(1),
	ValidArgs: []string{"bash", "zsh"},
	Run: func(cmd *cobra.Command, args []string) {
		switch args[0] {
		case "bash":
			cmd.Root().GenBashCompletion(cmd.OutOrStdout())
		case "zsh":
			cmd.Root().GenZshCompletion(cmd.OutOrStdout())
		}
	},
}

func init() {
	rootCmd.AddCommand(aclsCmd)
	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(completionCmd)

	aclsCmd.AddCommand(aclsListCmd)
	aclsCmd.AddCommand(aclsApplyCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// aclsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// aclsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
