pub mod vanilla_payoff;
pub use vanilla_payoff::*;

use crate::instruments::options::OptionType;
use crate::money::MonetaryNumber;

pub trait Payoff<N>
where
    N: MonetaryNumber,
{
    fn apply(&self, price: N) -> N;
}

pub trait CallPutPayoff<N>: Payoff<N>
where
    N: MonetaryNumber,
{
    fn get_option_type(&self) -> OptionType;
}
