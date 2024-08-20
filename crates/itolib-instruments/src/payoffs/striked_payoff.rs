use crate::options::OptionType;
use crate::payoffs::Payoff;

pub trait StrikedPayoff: Payoff {
    #[must_use]
    fn get_strike(&self) -> Self::PayoffNumberType;

    #[must_use]
    fn get_option_type(&self) -> OptionType;
}
