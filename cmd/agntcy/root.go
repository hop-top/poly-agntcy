package main

import "github.com/spf13/cobra"

func newRootCmd() *cobra.Command {
	root := &cobra.Command{
		Use:           "agntcy",
		Short:         "AGNTCY Directory Service CLI",
		SilenceUsage:  true,
		SilenceErrors: true,
	}
	root.PersistentFlags().String("endpoint", "", "DIR endpoint URL")
	root.AddCommand(
		newRegisterCmd(),
		newDiscoverCmd(),
		newDescribeCmd(),
		newPublishCmd(),
		newVerifyCmd(),
	)
	return root
}
