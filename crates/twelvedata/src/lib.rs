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
//!
//! ```rust
//! use twelvedata::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("TOKEN_HERE");
//!     let price = client.price("AAPL").await;
//!     println!("{}", price);
//! }
//! ```
//! - Run the program. It should return the current stock price of AAPL.
//! - Now you can start using the client to get data from the Twelve Data API.
//!
//! ## Notes
//! - You need an async runtime to use this crate. I recommend [tokio](https://tokio.rs/) (as used above).
//! - This crate is not yet complete. I will be adding more endpoints in the future.
//! - Paid endpoints are not yet supported. But will be put behind a feature flag when they are.

/// Core Twelve Data
pub mod core;
/// Reference Twelve Data
pub mod reference;

/// Client for the Twelve Data API
///
/// # Example
/// ```rust
/// let client = Client::new("TOKEN_HERE");
/// let price = client.price("AAPL").await;
/// println!("{}", price.price);
/// ```
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
