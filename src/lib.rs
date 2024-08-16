#![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
// Clippy::Pedantic lint modifications
#![allow(clippy::module_name_repetitions)]

pub mod instruments;

pub mod money;
pub use money::{currency, Currency, ExchangeRate, Money};

#[macro_use]
pub(crate) mod macros;

mod interest_rate;
pub use interest_rate::{implied_rate_from_compound_factor, InterestRate};

pub mod term_structures;

mod compounding;
pub use compounding::Compounding;

pub mod time;

pub mod types;
pub use types::{CompoundFactor, DiscountFactor, MonetaryNumber, Percentage, Strike, Volatility};

pub mod math;

pub mod pricers;
