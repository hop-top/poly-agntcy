package dir_test

import (
	"testing"

	"hop.top/agntcy/dir"
)

func TestNewClient_RequiresEndpoint(t *testing.T) {
	_, err := dir.NewClient(dir.Options{})
	if err == nil {
		t.Fatalf("expected error when Endpoint is empty")
	}
}

func TestNewClient_OK(t *testing.T) {
	c, err := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if c == nil {
		t.Fatalf("client must not be nil")
	}
}
