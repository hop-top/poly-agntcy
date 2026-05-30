use std::sync::Arc;

use tonic::transport::{Channel, ClientTlsConfig, Endpoint};

use crate::credentials::Credentials;
use crate::proto::{
    AgentDescriptor, DescribeRequest, DescribeResponse, DiscoverRequest, DiscoverResponse,
    PublishRequest, PublishResponse, RegisterRequest, RegisterResponse, VerifyRequest,
    VerifyResponse,
};

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("endpoint required")]
    EndpointRequired,
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(String),
    #[error("transport: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("status: {0}")]
    Status(#[from] tonic::Status),
    #[error("not implemented: codegen pending")]
    NotImplemented,
}

pub struct Options {
    pub endpoint: String,
    pub credentials: Arc<dyn Credentials>,
}

pub struct Client {
    pub(crate) opts: Options,
}

pub struct DiscoverParams {
    pub capability: String,
}

pub struct DiscoverResult {
    pub agents: Vec<AgentDescriptor>,
}

pub struct RegisterParams {
    pub descriptor: AgentDescriptor,
}

pub struct RegisterResult {
    pub name: String,
}

pub struct DescribeParams {
    pub name: String,
}

pub struct DescribeResult {
    pub descriptor: Option<AgentDescriptor>,
}

pub struct PublishParams {
    pub descriptor: AgentDescriptor,
}

pub struct PublishResult {
    pub digest: String,
}

pub struct VerifyParams {
    pub envelope: Vec<u8>,
}

pub struct VerifyResult {
    pub valid: bool,
}

impl Client {
    pub fn new(opts: Options) -> Result<Self, Error> {
        if opts.endpoint.is_empty() {
            return Err(Error::EndpointRequired);
        }
        Ok(Self { opts })
    }

    pub fn endpoint(&self) -> &str {
        &self.opts.endpoint
    }

    pub fn credentials(&self) -> &Arc<dyn Credentials> {
        &self.opts.credentials
    }

    async fn channel(&self) -> Result<Channel, Error> {
        let mut endpoint: Endpoint = Channel::from_shared(self.opts.endpoint.clone())
            .map_err(|e| Error::InvalidEndpoint(e.to_string()))?;
        if self.opts.credentials.tls().is_some() {
            let tls = ClientTlsConfig::new().with_native_roots();
            endpoint = endpoint.tls_config(tls)?;
        }
        Ok(endpoint.connect().await?)
    }

    pub async fn register(&self, p: RegisterParams) -> Result<RegisterResult, Error> {
        let _ch = self.channel().await?;
        let _req = RegisterRequest {
            descriptor: Some(p.descriptor),
        };
        let _resp: RegisterResponse = Default::default();
        Err(Error::NotImplemented)
    }

    pub async fn discover(&self, p: DiscoverParams) -> Result<DiscoverResult, Error> {
        let _ch = self.channel().await?;
        let _req = DiscoverRequest {
            capability: p.capability,
        };
        let _resp: DiscoverResponse = Default::default();
        Err(Error::NotImplemented)
    }

    pub async fn describe(&self, p: DescribeParams) -> Result<DescribeResult, Error> {
        let _ch = self.channel().await?;
        let _req = DescribeRequest { name: p.name };
        let _resp: DescribeResponse = Default::default();
        Err(Error::NotImplemented)
    }

    pub async fn publish(&self, p: PublishParams) -> Result<PublishResult, Error> {
        let _ch = self.channel().await?;
        let _req = PublishRequest {
            descriptor: Some(p.descriptor),
        };
        let _resp: PublishResponse = Default::default();
        Err(Error::NotImplemented)
    }

    pub async fn verify(&self, p: VerifyParams) -> Result<VerifyResult, Error> {
        let _ch = self.channel().await?;
        let _req = VerifyRequest {
            envelope: p.envelope,
        };
        let _resp: VerifyResponse = Default::default();
        Err(Error::NotImplemented)
    }
}
