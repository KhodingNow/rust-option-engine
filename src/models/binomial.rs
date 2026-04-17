
// PUBLIC API

use crate::types::{Spot, Strike, Rate, Volatility, TimeToMaturity};

/// Cox–Ross–Rubinstein risk-neutral parameters
/// Returns (u, d, p)
pub fn risk_neutral_params(

    rate: f64,
    volatility: f64,
    dt: f64,
    
) -> (f64, f64, f64) {

    let u = (volatility * dt.sqrt()).exp();
    let d = 1.0 / u;
    let p = ((rate * dt).exp() - d) / (u - d);

    (u, d, p)
}

/// Terminal asset prices at maturity (CRR tree)
///
/// Returns N+1 prices for N steps
pub fn terminal_prices(

    spot: Spot,
    u: f64,
    d: f64,
    steps: usize,
    
) -> Vec<f64> {

    let mut prices = Vec::with_capacity(steps + 1);

    // k = number of up moves
    for k in (0..=steps).rev() {
        let up_moves = k;
        let down_moves = steps - k;

        let price =
            spot.0 * u.powi(up_moves as i32) * d.powi(down_moves as i32);

        prices.push(price);
    }

    prices
}

// -------------------------
// Binomial pricing types
// -------------------------
pub struct BinomialParams {
    pub spot: Spot,
    pub strike: Strike,
    pub rate: Rate,
    pub volatility: Volatility,
    pub maturity: TimeToMaturity,
    pub steps: usize,
}


#[derive(Copy, Clone, PartialEq)]
pub enum OptionStyle {
    European,
    American,
}

#[derive(Copy, Clone, PartialEq)]
pub enum OptionType {
    Call,
    Put,

}

// -------------------------
// Payoff logic
// -------------------------
pub fn payoff(price: f64, strike: f64, kind: OptionType) -> f64 {
    match kind {
        OptionType::Call => (price - strike).max(0.0),
        OptionType::Put  => (strike - price).max(0.0),
    }
}

pub fn terminal_payoffs(
    prices: &[f64],
    strike: f64,
    kind: OptionType,

) -> Vec<f64> {
    prices

        .iter()
        .map(|&price| payoff(price, strike, kind))
        .collect()
}

// -------------------------
// Pricing entry point
// -------------------------


/// Prices an option using the Cox-Ross-Rubinstein (CRR) binomial model.
///
/// Supports both European and American options.
///
/// # Arguments
/// - `params` - Model parameters
/// - `style` - European or American
/// - `kind` - Call or Put
///
/// # Returns
/// The option price at time t=0
///
/// # Example
/// ```
/// use rust_option_engine::{price, OptionStyle, OptionType, BinomialParams};
/// use rust_option_engine::types::{Spot, Strike, Rate, Volatility, TimeToMaturity};
///
/// let params = BinomialParams {
///     spot: Spot(100.0),
///     strike: Strike(100.0),
///     rate: Rate(0.05),
///     volatility: Volatility(0.2),
///     maturity: TimeToMaturity(1.0),
///     steps: 100,
/// };
///
/// let price = price(&params, OptionStyle::European, OptionType::Call);
///
/// assert!(price > 0.0);
/// ```
pub fn price(
    params: &BinomialParams,
    style: OptionStyle,
    kind: OptionType,
    
) -> f64  {

    let dt = params.maturity.0 / params.steps as f64;

    let (u, d, p) = risk_neutral_params(
	params.rate.0, params.volatility.0, dt,);

    let discount = (-params.rate.0 * dt).exp();

    // Terminal prices

    let prices = terminal_prices(
        params.spot,

        u,
        d,
        params.steps,
    );
    
    // Terminal payoffs
    
    let mut values: Vec<f64> = prices
        .iter()
        .map(|&s| payoff(s, params.strike.0, kind))
        .collect(); 

    // Backward induction

   for step in (0..params.steps).rev() {

    for i in 0..=step {

        let hold_value = 
        
              (p * values[i] + (1.0 - p) * values[i + 1]) * discount;
        
        if style == OptionStyle::American {

            let stock_price = 
                 params.spot.0 

                 * u.powi(i as i32) 
                 * d.powi((step - i) as i32);

            let intrinsic = match kind {

                OptionType::Call => 
                	(stock_price - params.strike.0).max(0.0),

                OptionType::Put => 
                (params.strike.0 - stock_price).max(0.0),
                
            };

            values[i] = hold_value.max(intrinsic);
            
        } else {

            values[i] = hold_value;
        }
    } 
    
   }       
    
    // Root node value
    values[0]                 

}
