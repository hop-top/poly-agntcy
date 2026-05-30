//! AGNTCY Directory Service SDK.

pub mod client;
pub mod credentials;
pub mod proto;

pub use client::{Client, Error, Options};
pub use credentials::{Credentials, InsecureCredentials, TlsCredentials};
