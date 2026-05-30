package dir

import "crypto/tls"

// Credentials abstracts the TLS posture used by the DIR client.
//
// Two stock impls (InsecureCredentials, TlsCredentials) ship in this
// package. SPIFFE-backed credentials live in hop.top/agntcy/spiffe.
type Credentials interface {
	TLS() *tls.Config
}

// InsecureCredentials disables TLS. Dev/test only.
type InsecureCredentials struct{}

func (InsecureCredentials) TLS() *tls.Config { return nil }

// TlsCredentials carries a stdlib *tls.Config.
type TlsCredentials struct {
	Config *tls.Config
}

func (c TlsCredentials) TLS() *tls.Config { return c.Config }
