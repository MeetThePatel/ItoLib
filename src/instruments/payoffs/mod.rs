mod vanilla_payoff;
pub use vanilla_payoff::VanillaPayoff;

use crate::instruments::options::OptionType;
use crate::money::Currency;

pub trait Payoff {
    // Type most likely should be Money<N, C>.
    // However, in the future, other option types will be implemented, such as percentage strikes.
    type PayoffNumberType;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType;
}

pub trait CallPutPayoff<C>: Payoff
where
    C: Currency,
{
    fn get_option_type(&self) -> OptionType;
}
