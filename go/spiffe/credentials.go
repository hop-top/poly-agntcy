// Package spiffe provides SPIFFE-backed Credentials for the DIR client.
package spiffe

import (
	"context"
	"crypto/tls"
	"errors"

	"github.com/spiffe/go-spiffe/v2/spiffeid"
	"github.com/spiffe/go-spiffe/v2/spiffetls/tlsconfig"
	"github.com/spiffe/go-spiffe/v2/workloadapi"
)

// Credentials carries a SPIFFE X509Source plus the trust domain peers must
// belong to. TLS() returns an mTLS *tls.Config built via
// spiffetls/tlsconfig.MTLSClientConfig so the workload SVID is presented as
// the client certificate and the server SVID is verified and authorized
// against TrustDomain.
type Credentials struct {
	Source      *workloadapi.X509Source
	TrustDomain spiffeid.TrustDomain
}

// TLS returns a *tls.Config wired for SPIFFE mTLS. Returns nil when the
// receiver, Source, or TrustDomain is unset — callers treat nil as "no
// credentials available" and either fall back or fail closed.
func (c *Credentials) TLS() *tls.Config {
	if c == nil || c.Source == nil || c.TrustDomain.IsZero() {
		return nil
	}
	return tlsconfig.MTLSClientConfig(
		c.Source,
		c.Source,
		tlsconfig.AuthorizeMemberOf(c.TrustDomain),
	)
}

// FromDefaultSocket constructs Credentials backed by a workloadapi X509Source
// dialed at socketPath. Caller owns the returned Source and must Close it.
func FromDefaultSocket(ctx context.Context, socketPath string, td spiffeid.TrustDomain) (*Credentials, error) {
	if socketPath == "" {
		return nil, errors.New("spiffe: socketPath required")
	}
	if td.IsZero() {
		return nil, errors.New("spiffe: TrustDomain required")
	}
	src, err := workloadapi.NewX509Source(
		ctx,
		workloadapi.WithClientOptions(workloadapi.WithAddr(socketPath)),
	)
	if err != nil {
		return nil, err
	}
	return &Credentials{Source: src, TrustDomain: td}, nil
}
