package main

import (
	"fmt"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newRegisterCmd() *cobra.Command {
	var name string
	var endpointURL string
	c := &cobra.Command{
		Use:   "register",
		Short: "Register an agent descriptor",
		RunE: func(cmd *cobra.Command, _ []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			res, err := client.Register(cmd.Context(), dir.RegisterParams{
				Agent: &dir.AgentDescriptor{Name: name, Endpoint: endpointURL},
			})
			if err != nil {
				return err
			}
			fmt.Fprintln(cmd.OutOrStdout(), res.ID)
			return nil
		},
	}
	c.Flags().StringVar(&name, "name", "", "Agent name")
	c.Flags().StringVar(&endpointURL, "agent-endpoint", "", "Agent endpoint URL")
	return c
}
