//! # Twelve Data API Wrapper
//!
//! This crate provides a wrappers for the [Twelve Data API](https://twelvedata.com/docs).
//!
//! The Twelve Data API provides financial market data for developers.
//!
//! ## Quick Start
//!
//! - First subscribe to the [Twelve Data API](https://twelvedata.com). You can use the free tier to get started. Or don't, most of the endpoints in reference do not require an API key.
//! - Then create a new client with your API key.
//! - Now you can start using the wrappers to get data from the Twelve Data API.
//!
//! ```rust
//! use twelvedata::core;
//!
//! #[tokio::main]
//! async fn main() {
//!     let end_of_day = core::EndOfDay::builder()
//!         .symbol("AAPL")
//!         .apikey("your_api_key")
//!         .execute()
//!         .await;
//!
//!     match end_of_day {
//!         Ok(eod) => println!("{:?}", eod),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## Notes
//! - You need an async runtime to use this crate. I recommend [tokio](https://tokio.rs/). It is actually what to run async tests.
//! - This crate is not yet complete. I will be adding more endpoints in the future.
//! - Paid endpoints/features are not fully supported. They are in the library, but are not neccessarily guaranteed to work. This is because I do not have a paid subscription to test them.

/// Core Data
pub mod core;
// Internal Library Tooling
mod internal;
/// Reference Twelve Data
pub mod reference;
// Mutal Funds Data
pub mod mutualfunds;
