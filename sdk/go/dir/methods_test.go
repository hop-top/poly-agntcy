package dir_test

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"testing"

	"hop.top/agntcy/dir"
)

type fakeRT struct {
	body []byte
}

func (f fakeRT) RoundTrip(_ *http.Request) (*http.Response, error) {
	return &http.Response{
		StatusCode: 200,
		Body:       io.NopCloser(bytes.NewReader(f.body)),
		Header:     http.Header{"Content-Type": []string{"application/proto"}},
	}, nil
}

func TestClient_Discover_DoesNotPanic(t *testing.T) {
	c, err := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{body: []byte{}}},
	})
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	_, _ = c.Discover(context.Background(), dir.DiscoverParams{})
}

func TestClient_Register_DoesNotPanic(t *testing.T) {
	c, _ := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{}},
	})
	_, _ = c.Register(context.Background(), dir.RegisterParams{})
}

func TestClient_Describe_DoesNotPanic(t *testing.T) {
	c, _ := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{}},
	})
	_, _ = c.Describe(context.Background(), dir.DescribeParams{})
}

func TestClient_Publish_DoesNotPanic(t *testing.T) {
	c, _ := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{}},
	})
	_, _ = c.Publish(context.Background(), dir.PublishParams{})
}

func TestClient_Verify_DoesNotPanic(t *testing.T) {
	c, _ := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{}},
	})
	_, _ = c.Verify(context.Background(), dir.VerifyParams{})
}
