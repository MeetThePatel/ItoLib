#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(clippy::nursery)]

pub mod float;

mod financial;
pub use financial::{CompoundFactor, DiscountFactor, MonetaryNumber, Strike, Volatility};

mod math;
pub use math::Percentage;
