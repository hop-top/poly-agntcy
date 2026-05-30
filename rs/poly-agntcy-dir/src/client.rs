use std::sync::Arc;

use crate::credentials::Credentials;

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
}

pub struct Options {
    pub endpoint: String,
    pub credentials: Arc<dyn Credentials>,
}

pub struct Client {
    pub(crate) opts: Options,
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
}
