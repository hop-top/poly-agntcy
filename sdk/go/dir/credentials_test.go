package dir_test

import (
	"crypto/tls"
	"testing"

	"hop.top/agntcy/dir"
)

func TestInsecureCredentials(t *testing.T) {
	c := dir.InsecureCredentials{}
	if c.TLS() != nil {
		t.Fatalf("InsecureCredentials.TLS() must return nil")
	}
}

func TestTlsCredentials(t *testing.T) {
	c := dir.TlsCredentials{Config: &tls.Config{ServerName: "example"}}
	got := c.TLS()
	if got == nil || got.ServerName != "example" {
		t.Fatalf("TlsCredentials.TLS() must return underlying config")
	}
}
