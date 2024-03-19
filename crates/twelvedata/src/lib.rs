//! # Twelve Data API Client
//!
//! This crate provides a client for the [Twelve Data API](https://twelvedata.com/docs).
//!
//! The Twelve Data API provides financial market data for developers.
//!
//! ## Quick Start
//!
//! - First subscribe to the [Twelve Data API](https://rapidapi.com/twelvedata/api/twelve-data1).
//! - Then create a new client with your API key.
//! - Now you can start using the wrapper to get data from the Twelve Data API.
//!
//! ## Notes
//! - You need an async runtime to use this crate. I recommend [tokio](https://tokio.rs/). It is actually what to run async tests.
//! - This crate is not yet complete. I will be adding more endpoints in the future.
//! - Paid endpoints are not yet supported. But will be put behind a feature flag when they are. But this will be difficult as I don't have a paid account to test with.

/// Core Twelve Data
pub mod core;
// Internal Library Tooling
mod internal;
/// Reference Twelve Data
pub mod reference;

/// Client for the Twelve Data API, slowly being phased out
pub struct Client {
    pub api_key: String,
}

impl Client {
    /// Create a new client
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
        }
    }
}
