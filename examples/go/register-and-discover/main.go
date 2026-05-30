package main

import (
	"context"
	"flag"
	"fmt"
	"log"

	"hop.top/agntcy/dir"
)

func main() {
	endpoint := flag.String("endpoint", "http://localhost:8888", "DIR endpoint")
	capability := flag.String("capability", "inventory", "capability to publish + discover")
	flag.Parse()

	ctx := context.Background()

	client, err := dir.NewClient(dir.Options{
		Endpoint:    *endpoint,
		Credentials: dir.InsecureCredentials{},
	})
	if err != nil {
		log.Fatalf("client: %v", err)
	}

	desc := &dir.AgentDescriptor{
		Name:         "inventory-agent",
		Capabilities: []string{*capability},
	}

	reg, err := client.Register(ctx, dir.RegisterParams{Agent: desc})
	if err != nil {
		log.Fatalf("register: %v", err)
	}
	fmt.Printf("registered id=%s\n", reg.ID)

	dis, err := client.Discover(ctx, dir.DiscoverParams{Capability: *capability})
	if err != nil {
		log.Fatalf("discover: %v", err)
	}
	for _, a := range dis.Agents {
		fmt.Printf("discovered %s\n", a.GetName())
	}

	if _, err := client.Publish(ctx, dir.PublishParams{ID: reg.ID, Payload: []byte(`{"msg":"hello"}`)}); err != nil {
		log.Fatalf("publish: %v", err)
	}
	fmt.Println("published signed payload")
}
