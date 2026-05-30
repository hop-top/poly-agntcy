package spiffe_test

import (
	"testing"

	"hop.top/agntcy/dir"
	"hop.top/agntcy/spiffe"
)

func TestCredentials_ImplementsInterface(t *testing.T) {
	var _ dir.Credentials = (*spiffe.Credentials)(nil)
}

func TestCredentials_NilReturnsNilConfig(t *testing.T) {
	var c *spiffe.Credentials
	if c.TLS() != nil {
		t.Fatalf("nil receiver must return nil TLS config")
	}
	c2 := &spiffe.Credentials{}
	if c2.TLS() != nil {
		t.Fatalf("empty Source must return nil TLS config")
	}
}
