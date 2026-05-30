//! AGNTCY Directory Service SDK.

pub mod client;
pub mod credentials;
pub mod proto;

pub use client::{
    Client, DescribeParams, DescribeResult, DiscoverParams, DiscoverResult, Error, Options,
    PublishParams, PublishResult, RegisterParams, RegisterResult, VerifyParams, VerifyResult,
};
pub use credentials::{Credentials, InsecureCredentials, TlsCredentials};
