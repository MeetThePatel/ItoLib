pub mod vanilla_payoff;
pub use vanilla_payoff::*;

use crate::instruments::options::OptionType;
use crate::money::{Currency, MonetaryNumber};

pub trait Payoff<N>
where
    N: MonetaryNumber,
{
    // Type most likely should be Money<N, C>.
    // However, in the future, other option types will be implemented, such as percentage strikes.
    type PayoffNumberType;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType;
}

pub trait CallPutPayoff<N, C>: Payoff<N>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn get_option_type(&self) -> OptionType;
}
