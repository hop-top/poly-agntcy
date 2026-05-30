package dir

import (
	"context"

	"connectrpc.com/connect"

	pb "hop.top/agntcy/internal/protostub"
)

// AgentDescriptor describes a DIR-registered agent.
type AgentDescriptor = pb.AgentDescriptor

// RegisterParams is the input to Register.
type RegisterParams struct {
	Agent *AgentDescriptor
}

// RegisterResult is the output of Register.
type RegisterResult struct {
	ID string
}

// Register publishes an agent descriptor to the DIR.
func (c *Client) Register(ctx context.Context, p RegisterParams) (*RegisterResult, error) {
	svc := pb.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&pb.RegisterRequest{Agent: p.Agent})
	resp, err := svc.Register(ctx, req)
	if err != nil {
		return nil, err
	}
	return &RegisterResult{ID: resp.Msg.GetId()}, nil
}

// DiscoverParams is the input to Discover.
type DiscoverParams struct {
	Capability string
}

// DiscoverResult is the output of Discover.
type DiscoverResult struct {
	Agents []*AgentDescriptor
}

// Discover queries the DIR for agents matching a capability.
func (c *Client) Discover(ctx context.Context, p DiscoverParams) (*DiscoverResult, error) {
	svc := pb.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&pb.DiscoverRequest{Capability: p.Capability})
	resp, err := svc.Discover(ctx, req)
	if err != nil {
		return nil, err
	}
	return &DiscoverResult{Agents: resp.Msg.GetAgents()}, nil
}

// DescribeParams is the input to Describe.
type DescribeParams struct {
	ID string
}

// DescribeResult is the output of Describe.
type DescribeResult struct {
	Agent *AgentDescriptor
}

// Describe fetches the full descriptor for a registered agent.
func (c *Client) Describe(ctx context.Context, p DescribeParams) (*DescribeResult, error) {
	svc := pb.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&pb.DescribeRequest{Id: p.ID})
	resp, err := svc.Describe(ctx, req)
	if err != nil {
		return nil, err
	}
	return &DescribeResult{Agent: resp.Msg.GetAgent()}, nil
}

// PublishParams is the input to Publish.
type PublishParams struct {
	ID      string
	Payload []byte
}

// PublishResult is the output of Publish.
type PublishResult struct {
	Receipt string
}

// Publish pushes a signed payload for an existing agent.
func (c *Client) Publish(ctx context.Context, p PublishParams) (*PublishResult, error) {
	svc := pb.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&pb.PublishRequest{Id: p.ID, Payload: p.Payload})
	resp, err := svc.Publish(ctx, req)
	if err != nil {
		return nil, err
	}
	return &PublishResult{Receipt: resp.Msg.GetReceipt()}, nil
}

// VerifyParams is the input to Verify.
type VerifyParams struct {
	ID        string
	Signature []byte
}

// VerifyResult is the output of Verify.
type VerifyResult struct {
	Valid bool
}

// Verify checks a signature against a stored descriptor.
func (c *Client) Verify(ctx context.Context, p VerifyParams) (*VerifyResult, error) {
	svc := pb.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&pb.VerifyRequest{Id: p.ID, Signature: p.Signature})
	resp, err := svc.Verify(ctx, req)
	if err != nil {
		return nil, err
	}
	return &VerifyResult{Valid: resp.Msg.GetValid()}, nil
}
