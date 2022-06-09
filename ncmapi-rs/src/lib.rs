//! NetEase Cloud Music API For Rust.

mod api;
mod client;
mod crypto;
pub mod types;

pub use api::{NcmApi, ResourceType, SearchType};

type TResult<T> = std::result::Result<T, TError>;
type TError = Box<dyn std::error::Error>;
