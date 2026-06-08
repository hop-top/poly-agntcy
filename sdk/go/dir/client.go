package dir

import (
	"errors"
	"net/http"

	"connectrpc.com/connect"
)

// Options configures a Client.
type Options struct {
	Endpoint    string
	Credentials Credentials
	HTTPClient  *http.Client
}

// Client is a DIR API client.
type Client struct {
	opts Options
	http *http.Client
}

// NewClient constructs a DIR client.
func NewClient(opts Options) (*Client, error) {
	if opts.Endpoint == "" {
		return nil, errors.New("dir: Endpoint required")
	}
	if opts.Credentials == nil {
		opts.Credentials = InsecureCredentials{}
	}
	httpc := opts.HTTPClient
	if httpc == nil {
		httpc = &http.Client{}
	}
	if tlsCfg := opts.Credentials.TLS(); tlsCfg != nil {
		if t, ok := httpc.Transport.(*http.Transport); ok {
			t.TLSClientConfig = tlsCfg
		} else {
			httpc.Transport = &http.Transport{TLSClientConfig: tlsCfg}
		}
	}
	_ = connect.Code(0)
	return &Client{opts: opts, http: httpc}, nil
}

// HTTPClient returns the underlying *http.Client used for transport.
func (c *Client) HTTPClient() *http.Client { return c.http }

// Endpoint returns the configured DIR endpoint.
func (c *Client) Endpoint() string { return c.opts.Endpoint }
