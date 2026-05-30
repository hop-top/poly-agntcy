package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"

	"hop.top/agntcy/dir"
)

func main() {
	endpoint := flag.String("endpoint", "http://localhost:8888", "DIR endpoint")
	flag.Parse()
	verb := flag.Arg(0)
	if verb == "" {
		log.Fatal("usage: go-agent [--endpoint URL] {register|discover|describe}")
	}

	ctx := context.Background()
	client, err := dir.NewClient(dir.Options{
		Endpoint:    *endpoint,
		Credentials: dir.InsecureCredentials{},
	})
	if err != nil {
		log.Fatalf("client: %v", err)
	}

	switch verb {
	case "register":
		agent := &dir.AgentDescriptor{
			Name:         "inventory-agent",
			Capabilities: []string{"inventory"},
		}
		res, err := client.Register(ctx, dir.RegisterParams{Agent: agent})
		if err != nil {
			log.Fatalf("register: %v", err)
		}
		fmt.Printf("go-agent registered id=%s\n", res.ID)
	case "discover":
		res, err := client.Discover(ctx, dir.DiscoverParams{Capability: "inventory"})
		if err != nil {
			log.Fatalf("discover: %v", err)
		}
		for _, a := range res.Agents {
			fmt.Printf("go-agent discovered %s\n", a.GetName())
		}
	default:
		log.Fatalf("unknown verb %q", verb)
		os.Exit(2)
	}
}
