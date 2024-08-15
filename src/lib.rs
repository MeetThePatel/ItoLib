#![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
// Clippy::Pedantic lint modifications
#![allow(clippy::module_name_repetitions)]

pub mod instruments;

pub mod money;
// pub use money::{currency, Currency, ExchangeRate, Money};

#[macro_use]
pub(crate) mod macros;

pub mod interest_rate;
pub use interest_rate::{implied_rate_from_compound_factor, InterestRate};

pub mod term_structures;

pub mod compounding;
pub use compounding::Compounding;

pub mod time;

pub mod types;
pub use types::{CompoundFactor, DiscountFactor, MonetaryNumber, Percentage, Strike, Volatility};

pub mod math;

pub mod pricers;

// TODO: Incorporate https://docs.rs/assert_float_eq/latest/assert_float_eq/ to get coverage for
// float comparisons.
