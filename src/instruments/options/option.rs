use std::fmt::Display;

use crate::instruments::{Exercise, Payoff};
use crate::money::{Currency, MonetaryNumber};

// TODO: Make this subtrait of Instrument.
pub trait Option<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn get_option_type(&self) -> OptionType;

    fn get_payoff(&self) -> impl Payoff<N>;

    fn get_exercise(&self) -> impl Exercise;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OptionType {
    CALL,
    PUT,
}

impl Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CALL => write!(f, "CALL"),
            Self::PUT => write!(f, "PUT"),
        }
    }
}
