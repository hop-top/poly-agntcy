// Package protostub provides minimal placeholder types that mirror the
// shape of generated connect-go bindings for the AGNTCY DIR service.
//
// These types exist only so the dir package compiles before the proto
// codegen pipeline can resolve the upstream agntcy/dir protos. The real
// types live under internal/proto/ once buf generate succeeds; at that
// point this file is replaced by the import path swap noted in the plan.
package protostub

import (
	"context"
	"net/http"

	"connectrpc.com/connect"
)

type AgentDescriptor struct {
	Name         string
	Capabilities []string
	Endpoint     string
}

func (a *AgentDescriptor) GetName() string           { return a.Name }
func (a *AgentDescriptor) GetCapabilities() []string { return a.Capabilities }
func (a *AgentDescriptor) GetEndpoint() string       { return a.Endpoint }

type RegisterRequest struct {
	Agent *AgentDescriptor
}

type RegisterResponse struct {
	Id string
}

func (r *RegisterResponse) GetId() string { return r.Id }

type DiscoverRequest struct {
	Capability string
}

type DiscoverResponse struct {
	Agents []*AgentDescriptor
}

func (r *DiscoverResponse) GetAgents() []*AgentDescriptor { return r.Agents }

type DescribeRequest struct {
	Id string
}

type DescribeResponse struct {
	Agent *AgentDescriptor
}

func (r *DescribeResponse) GetAgent() *AgentDescriptor { return r.Agent }

type PublishRequest struct {
	Id      string
	Payload []byte
}

type PublishResponse struct {
	Receipt string
}

func (r *PublishResponse) GetReceipt() string { return r.Receipt }

type VerifyRequest struct {
	Id        string
	Signature []byte
}

type VerifyResponse struct {
	Valid bool
}

func (r *VerifyResponse) GetValid() bool { return r.Valid }

// DirectoryServiceClient is the shape we expect from the generated
// connect-go client; we only need the methods called by the dir package.
type DirectoryServiceClient interface {
	Register(context.Context, *connect.Request[RegisterRequest]) (*connect.Response[RegisterResponse], error)
	Discover(context.Context, *connect.Request[DiscoverRequest]) (*connect.Response[DiscoverResponse], error)
	Describe(context.Context, *connect.Request[DescribeRequest]) (*connect.Response[DescribeResponse], error)
	Publish(context.Context, *connect.Request[PublishRequest]) (*connect.Response[PublishResponse], error)
	Verify(context.Context, *connect.Request[VerifyRequest]) (*connect.Response[VerifyResponse], error)
}

// NewDirectoryServiceClient returns a stub client that errors on every
// call. The real generated constructor replaces this once codegen lands.
func NewDirectoryServiceClient(_ connect.HTTPClient, _ string, _ ...connect.ClientOption) DirectoryServiceClient {
	return &stubClient{}
}

type stubClient struct{}

func notImplemented(rpc string) error {
	return connect.NewError(connect.CodeUnimplemented,
		errStub{rpc: rpc})
}

type errStub struct{ rpc string }

func (e errStub) Error() string {
	return "dir: " + e.rpc + " not implemented (proto stub)"
}

func (s *stubClient) Register(_ context.Context, _ *connect.Request[RegisterRequest]) (*connect.Response[RegisterResponse], error) {
	return nil, notImplemented("Register")
}

func (s *stubClient) Discover(_ context.Context, _ *connect.Request[DiscoverRequest]) (*connect.Response[DiscoverResponse], error) {
	return nil, notImplemented("Discover")
}

func (s *stubClient) Describe(_ context.Context, _ *connect.Request[DescribeRequest]) (*connect.Response[DescribeResponse], error) {
	return nil, notImplemented("Describe")
}

func (s *stubClient) Publish(_ context.Context, _ *connect.Request[PublishRequest]) (*connect.Response[PublishResponse], error) {
	return nil, notImplemented("Publish")
}

func (s *stubClient) Verify(_ context.Context, _ *connect.Request[VerifyRequest]) (*connect.Response[VerifyResponse], error) {
	return nil, notImplemented("Verify")
}

// keep http import referenced when no real client wraps it yet
var _ = http.MethodGet
