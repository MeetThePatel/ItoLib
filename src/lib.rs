#![warn(clippy::all)]

pub mod instruments;
pub mod money;

#[macro_use]
pub mod macros;

pub mod interest_rate;
pub mod term_structures;

pub mod compounding;
pub mod time;

pub mod types;

pub mod math;

pub mod pricers;

// TODO: Incorporate https://docs.rs/assert_float_eq/latest/assert_float_eq/ to get coverage for
// float comparisons.
