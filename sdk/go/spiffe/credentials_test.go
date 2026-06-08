package spiffe_test

import (
	"testing"

	"github.com/spiffe/go-spiffe/v2/spiffeid"
	"github.com/spiffe/go-spiffe/v2/workloadapi"

	"hop.top/agntcy-go-spiffe"
	"hop.top/agntcy/dir"
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
	c3 := &spiffe.Credentials{Source: &workloadapi.X509Source{}}
	if c3.TLS() != nil {
		t.Fatalf("zero TrustDomain must return nil TLS config")
	}
}

func TestCredentials_TLSConfigEnforcesPeerVerification(t *testing.T) {
	td := spiffeid.RequireTrustDomainFromString("example.org")
	c := &spiffe.Credentials{
		Source:      &workloadapi.X509Source{},
		TrustDomain: td,
	}
	cfg := c.TLS()
	if cfg == nil {
		t.Fatal("TLS() returned nil")
	}
	// MTLSClientConfig wires SPIFFE peer verification via VerifyPeerCertificate
	// against the X509 bundle, bypassing stdlib chain validation. RootCAs stays
	// nil by design; bundle authorization happens in VerifyPeerCertificate.
	if cfg.VerifyPeerCertificate == nil {
		t.Fatal("VerifyPeerCertificate must be set")
	}
	if cfg.GetClientCertificate == nil {
		t.Fatal("GetClientCertificate must be set")
	}
	if !cfg.InsecureSkipVerify {
		t.Fatal("InsecureSkipVerify must be true (SPIFFE mTLS owns verification)")
	}
}
