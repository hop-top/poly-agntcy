package main

import (
	"os"

	"hop.top/agntcy/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
