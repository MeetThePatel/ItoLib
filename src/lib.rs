#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

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
