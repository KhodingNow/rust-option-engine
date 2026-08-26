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

pub mod greeks;
pub mod models;
pub mod types;

pub use greeks::*;
pub use models::binomial::*;
pub use models::black_scholes::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Calculate the Black-Scholes European call option price.
#[cfg(feature = "python")]
#[pyfunction(name = "call_price")]
fn call_price_py(
    spot: f64,
    strike: f64,
    rate: f64,
    volatility: f64,
    time_to_maturity: f64,
) -> PyResult<f64> {
    if volatility <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "volatility must be greater than zero",
        ));
    }

    if time_to_maturity < 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "time to maturity must not be negative",
        ));
    }

    if spot <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "spot price must be greater than zero",
        ));
    }

    if strike <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "strike price must be greater than zero",
        ));
    }

    Ok(call_price(
        types::Spot(spot),
        types::Strike(strike),
        types::Rate(rate),
        types::Volatility(volatility),
        types::TimeToMaturity(time_to_maturity),
    ))
}

/// Calculate the Black-Scholes European put option price.
#[cfg(feature = "python")]
#[pyfunction(name = "put_price")]
fn put_price_py(
    spot: f64,
    strike: f64,
    rate: f64,
    volatility: f64,
    time_to_maturity: f64,
) -> PyResult<f64> {
    if volatility <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "volatility must be greater than zero",
        ));
    }

    if time_to_maturity < 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "time to maturity must not be negative",
        ));
    }

    if spot <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "spot price must be greater than zero",
        ));
    }

    if strike <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "strike price must be greater than zero",
        ));
    }

    Ok(put_price(
        types::Spot(spot),
        types::Strike(strike),
        types::Rate(rate),
        types::Volatility(volatility),
        types::TimeToMaturity(time_to_maturity),
    ))
}

/// Calculate the Black-Scholes call option delta.
#[cfg(feature = "python")]
#[pyfunction(name = "delta_call")]
fn delta_call_py(
    spot: f64,
    strike: f64,
    rate: f64,
    volatility: f64,
    time_to_maturity: f64,
) -> PyResult<f64> {
    if volatility <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "volatility must be greater than zero",
        ));
    }

    if time_to_maturity < 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "time to maturity must not be negative",
        ));
    }

    if spot <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "spot price must be greater than zero",
        ));
    }

    if strike <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "strike price must be greater than zero",
        ));
    }

    Ok(delta_call(
        types::Spot(spot),
        types::Strike(strike),
        types::Rate(rate),
        types::Volatility(volatility),
        types::TimeToMaturity(time_to_maturity),
    ))
}

#[cfg(feature = "python")]
#[pymodule]
fn rust_option_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(call_price_py, m)?)?;
    m.add_function(wrap_pyfunction!(put_price_py, m)?)?;
    m.add_function(wrap_pyfunction!(delta_call_py, m)?)?;
    Ok(())
}
