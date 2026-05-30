// Package spiffe provides SPIFFE-backed Credentials for the DIR client.
package spiffe

import (
	"crypto/tls"
	"crypto/x509"

	"github.com/spiffe/go-spiffe/v2/workloadapi"
)

// Credentials carries a SPIFFE X509Source and produces a TLS config that
// presents the workload SVID as the client certificate. Server verification
// is left to caller-supplied roots — callers needing SPIFFE-aware
// authentication should compose with go-spiffe spiffetls helpers.
type Credentials struct {
	Source *workloadapi.X509Source
}

// TLS returns a *tls.Config that uses the SPIFFE source for client certs.
func (c *Credentials) TLS() *tls.Config {
	if c == nil || c.Source == nil {
		return nil
	}
	return &tls.Config{
		GetClientCertificate: func(*tls.CertificateRequestInfo) (*tls.Certificate, error) {
			svid, err := c.Source.GetX509SVID()
			if err != nil {
				return nil, err
			}
			return &tls.Certificate{
				Certificate: rawChain(svid.Certificates),
				PrivateKey:  svid.PrivateKey,
				Leaf:        leafOf(svid.Certificates),
			}, nil
		},
		MinVersion: tls.VersionTLS12,
	}
}

func rawChain(certs []*x509.Certificate) [][]byte {
	out := make([][]byte, len(certs))
	for i, c := range certs {
		out[i] = c.Raw
	}
	return out
}

func leafOf(certs []*x509.Certificate) *x509.Certificate {
	if len(certs) == 0 {
		return nil
	}
	return certs[0]
}
