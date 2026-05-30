package main

import (
	"context"
	"fmt"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newDescribeCmd() *cobra.Command {
	var id string
	c := &cobra.Command{
		Use:   "describe",
		Short: "Describe an agent by id",
		RunE: func(cmd *cobra.Command, _ []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			res, err := client.Describe(context.Background(), dir.DescribeParams{ID: id})
			if err != nil {
				return err
			}
			if res.Agent != nil {
				fmt.Fprintf(cmd.OutOrStdout(), "%s\t%s\n", res.Agent.GetName(), res.Agent.GetEndpoint())
			}
			return nil
		},
	}
	c.Flags().StringVar(&id, "id", "", "Agent id")
	return c
}
