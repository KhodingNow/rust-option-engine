# Rust Option Pricing Engine

A correctness-focused option pricing library in Rust implementing:

- Black-Scholes (closed-form)
- Binomial Tree (CRR)

The goal of this project is not trading performance, but **model correctness, numerical stability, and financial invariants**.

---

## Implemented Models

### Black-Scholes (European Options)

- Closed-form pricing for European calls and puts
- Assumptions:
  - Log-normal underlying
  - Constant volatility
  - Constant risk-free rate
  - No dividends

Implementation:

src/models/black_scholes.rs


---

### Binomial Tree (CRR)

- Discrete-time lattice model
- Backward induction pricing
- Foundation for American options and early exercise extensions

Implementation:

src/models/binomial.rs


---

## Example Usage

```rust
use option_pricing_rs::call_price;

let price = call_price(100.0, 100.0, 0.05, 0.2, 1.0);

```

# Getting Started

Clone the repository and run tests:

git clone https://github.com/<your-username>/rust-option-engine.git
cd rust-option-engine
cargo test

(The test suite is the primary interface for validating correctness)

# Testing Strategy

Unit Tests

- Positivity of option prices
- Behaviour near zero time to maturity
- Sanity checks on closed-form solutions
- Property-Based Tests (via proptest)

Located in:

tests/known_values.rs

Examples:

- call_price_is_never_below_intrinsic_value
- european_put_respects_discounted_lower_bound
- put_call_parity_holds

These tests validate correctness across wide ranges of inputs, not just fixed scenarios.

Numerical Sensitivity & Greeks

- Central-difference Delta approximation
- Bounds enforced on Delta for call options

Floating-point tolerances (epsilon) are used to distinguish numerical noise from genuine violations of financial principles.

# Project Structure

src/
├── lib.rs
├── types.rs
├── greeks.rs
└── models/
    ├── mod.rs
    ├── black_scholes.rs
    └── binomial.rs

tests/
└── known_values.rs

benches/
└── pricing_bench.rs

# Benchmarks (Optional)

Criterion benchmarks are included to explore performance characteristics after correctness is established.

Correct first. Fast later.

Key Learnings

- Financial models benefit from executable invariants, not just formulas
- Property-based testing is a natural fit for quantitative finance
- Strong typing helps prevent conceptual errors (rates vs time vs price)
- Binomial models are easier to validate through structural guarantees
- Rust encourages correctness before optimisation

# Background

Postgraduate Certificate in Econometrics
University of the Witwatersrand

This project represents a software-engineering-driven re-entry into derivative pricing using Rust.

The focus is:

- model correctness
- validation
- disciplined reasoning

—not trading strategies.

# Ecosystem Context

This crate focuses on correctness and clarity.

For production-grade pricing systems with automatic differentiation, see ongoing work in
quantsupport.

# References:

Quantitative Finance

- Hull — Options, Futures, and Other Derivatives
- Natenberg — Option Volatility & Pricing
- Glasserman — Monte Carlo Methods in Financial Engineering

Software Engineering

- Beck — Test-Driven Development (primary influence)
- Ken Youens-Clark => Command Line Rust (modern Rust application 2024)

Disclaimer:

This project is for educational purposes only.

It is not intended for:

- production trading
- investment decisions
- financial advice

Why This Project Exists?

To explore how:

- disciplined testing
- strong typing
- explicit assumptions

can make complex financial models easier to reason about — even when entering a new domain.
