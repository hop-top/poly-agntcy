package main

import (
	"context"
	"fmt"
	"io"
	"os"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newVerifyCmd() *cobra.Command {
	var id string
	var sigPath string
	c := &cobra.Command{
		Use:   "verify",
		Short: "Verify a signed agent payload",
		RunE: func(cmd *cobra.Command, _ []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			var sig []byte
			if sigPath != "" {
				f, err := os.Open(sigPath)
				if err != nil {
					return err
				}
				defer f.Close()
				sig, err = io.ReadAll(f)
				if err != nil {
					return err
				}
			}
			res, err := client.Verify(context.Background(), dir.VerifyParams{ID: id, Signature: sig})
			if err != nil {
				return err
			}
			fmt.Fprintln(cmd.OutOrStdout(), res.Valid)
			return nil
		},
	}
	c.Flags().StringVar(&id, "id", "", "Agent id")
	c.Flags().StringVar(&sigPath, "signature", "", "Path to signature file")
	return c
}
