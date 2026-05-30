//! Generated protobuf bindings.
//!
//! Real bindings live under `src/proto/` (gitignored) and are produced
//! by `mise run gen` via prost + tonic against the upstream BSR
//! module. Until codegen runs, this hand-rolled shim provides the
//! minimum surface area required for the rest of the crate to compile
//! and for downstream consumers to wire mocks. Replace wholesale once
//! real stubs land.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentDescriptor {
    pub name: String,
    pub capability: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscoverRequest {
    pub capability: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscoverResponse {
    pub agents: Vec<AgentDescriptor>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegisterRequest {
    pub descriptor: Option<AgentDescriptor>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegisterResponse {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DescribeRequest {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DescribeResponse {
    pub descriptor: Option<AgentDescriptor>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PublishRequest {
    pub descriptor: Option<AgentDescriptor>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PublishResponse {
    pub digest: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerifyRequest {
    pub envelope: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerifyResponse {
    pub valid: bool,
}
