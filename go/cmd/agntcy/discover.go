package main

import (
	"fmt"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newDiscoverCmd() *cobra.Command {
	var capability string
	c := &cobra.Command{
		Use:   "discover",
		Short: "Discover agents by capability",
		RunE: func(cmd *cobra.Command, _ []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			res, err := client.Discover(cmd.Context(), dir.DiscoverParams{Capability: capability})
			if err != nil {
				return err
			}
			for _, a := range res.Agents {
				fmt.Fprintln(cmd.OutOrStdout(), a.GetName())
			}
			return nil
		},
	}
	c.Flags().StringVar(&capability, "capability", "", "Capability to search for")
	return c
}
