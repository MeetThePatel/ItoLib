#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(clippy::nursery)]

mod float;
pub use float::Float;

mod financial;
pub use financial::{CompoundFactor, DiscountFactor, Strike, Volatility};

mod domain_error;
pub use domain_error::DomainError;
