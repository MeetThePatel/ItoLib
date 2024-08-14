mod vanilla_payoff;
pub use vanilla_payoff::VanillaPayoff;

use crate::instruments::options::OptionType;

pub trait Payoff: std::fmt::Display {
    // Type most likely should be Money<N, C>.
    // However, in the future, other option types will be implemented, such as percentage strikes.
    type PayoffNumberType;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType;
}

pub trait StrikedPayoff: Payoff {
    #[must_use]
    fn get_strike(&self) -> Self::PayoffNumberType;

    #[must_use]
    fn get_option_type(&self) -> OptionType;
}
