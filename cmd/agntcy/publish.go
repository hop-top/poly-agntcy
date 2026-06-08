package main

import (
	"fmt"
	"io"
	"os"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newPublishCmd() *cobra.Command {
	var id string
	var payloadPath string
	c := &cobra.Command{
		Use:   "publish",
		Short: "Publish a payload for a registered agent",
		RunE: func(cmd *cobra.Command, _ []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			var payload []byte
			if payloadPath != "" {
				f, err := os.Open(payloadPath)
				if err != nil {
					return err
				}
				defer f.Close()
				payload, err = io.ReadAll(f)
				if err != nil {
					return err
				}
			}
			res, err := client.Publish(cmd.Context(), dir.PublishParams{ID: id, Payload: payload})
			if err != nil {
				return err
			}
			fmt.Fprintln(cmd.OutOrStdout(), res.Receipt)
			return nil
		},
	}
	c.Flags().StringVar(&id, "id", "", "Agent id")
	c.Flags().StringVar(&payloadPath, "payload", "", "Path to payload file")
	return c
}
