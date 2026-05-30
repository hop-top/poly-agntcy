package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"hop.top/agntcy/poly-agntcy/internal/version"
)

var rootCmd = &cobra.Command{
	Use:     "poly-agntcy",
	Short:   "Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)",
	Version: version.Version(),
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	cobra.OnInitialize(initConfig)
	rootCmd.PersistentFlags().StringP(
		"format", "f", "text",
		"Output format (text, json, yaml)",
	)
	rootCmd.PersistentFlags().BoolP(
		"verbose", "v", false,
		"Verbose output",
	)
}

func initConfig() {
	viper.SetEnvPrefix("POLY_AGNTCY")
	viper.AutomaticEnv()
	home, err := os.UserHomeDir()
	if err == nil {
		viper.AddConfigPath(
			fmt.Sprintf("%s/.config/poly-agntcy", home),
		)
	}
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	_ = viper.ReadInConfig()
}
