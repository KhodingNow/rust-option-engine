use statrs::distribution::{Normal, ContinuousCDF};
use crate::types::*;
/// Computes the Delta of a European call option under Black-Scholes.
///
/// Delta represents the sensitivity of the option price to changes in the underlying price.
///
/// # Example
/// ```
/// use rust_option_engine::delta_call;
/// use rust_option_engine::types::{Spot, Strike, Rate, Volatility, TimeToMaturity};
///
/// let delta = delta_call(
///     Spot(100.0),
///     Strike(100.0),
///     Rate(0.05),
///     Volatility(0.2),
///     TimeToMaturity(1.0),
/// );
///
/// assert!(delta > 0.0 && delta < 1.0);
/// ```
pub fn delta_call(
    spot: Spot,
    strike: Strike,
    rate: Rate,
    vol: Volatility,
    time: TimeToMaturity,
    
) -> f64 {
    let normal = Normal::new(0.0, 1.0).expect("Standard normal should always construct");

    let d1 = (
        (spot.0 / strike.0).ln() + (rate.0 + 0.5 * vol.0 * vol.0) * time.0) / (vol.0 * time.0.sqrt());

     normal.cdf(d1)
}
