//! # Rust Option Pricing Engine
//!
//! A correctness-focused option pricing library implementing:
//! - Black-Scholes (closed-form)
//! - Binomial CRR model
//!
//! ## Example
//!
//! ```
//! use rust_option_engine::{call_price, delta_call};
//! use rust_option_engine::types::{Spot, Strike, Rate, Volatility, TimeToMaturity};
//!
//! let price = call_price(
//!     Spot(100.0),
//!     Strike(100.0),
//!     Rate(0.05),
//!     Volatility(0.2),
//!     TimeToMaturity(1.0),
//! );
//!
//! let delta = delta_call(
//!     Spot(100.0),
//!     Strike(100.0),
//!     Rate(0.05),
//!     Volatility(0.2),
//!     TimeToMaturity(1.0),
//! );
//!
//! assert!(price > 0.0);
//! assert!(delta > 0.0);
//! ```

pub mod models;
pub mod greeks;
pub mod types;

pub use greeks::*;
pub use models::black_scholes::*;
pub use models::binomial::*;



