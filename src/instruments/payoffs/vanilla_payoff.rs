use std::fmt::Display;
use std::marker::PhantomData;

use crate::instruments::options::OptionType;
use crate::instruments::payoffs::{CallPutPayoff, Payoff};
use crate::money::{Currency, MonetaryNumber, Money};

#[derive(Debug, Copy, Clone)]
pub struct VanillaPayoff<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    strike: Money<N, C>,
    option_type: OptionType,
    currency: PhantomData<C>,
}

impl<N, C> VanillaPayoff<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    #[must_use]
    #[inline]
    pub const fn new(strike: Money<N, C>, option_type: OptionType) -> Self {
        Self {
            strike,
            option_type,
            currency: PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub const fn get_strike(&self) -> Money<N, C> {
        self.strike
    }
}

impl<N, C> Display for VanillaPayoff<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.get_strike(), self.get_option_type())
    }
}

impl<N, C> Payoff<N> for VanillaPayoff<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    type PayoffNumberType = Money<N, C>;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType {
        match self.option_type {
            OptionType::CALL => {
                if price > self.strike {
                    price - self.strike
                } else {
                    Money::new(N::zero())
                }
            }
            OptionType::PUT => {
                if self.strike > price {
                    self.strike - price
                } else {
                    Money::new(N::zero())
                }
            }
        }
    }
}

impl<N, C> CallPutPayoff<N, C> for VanillaPayoff<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn get_option_type(&self) -> OptionType {
        self.option_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruments::{
            options::OptionType,
            payoffs::{CallPutPayoff, Payoff},
        },
        money::{currency::USD, Money},
    };

    use super::VanillaPayoff;

    #[test]
    fn test_vanilla_payoff() {
        let x: VanillaPayoff<f64, USD> = VanillaPayoff::new(Money::new(100.0), OptionType::CALL);
        assert_eq!(x.get_option_type(), OptionType::CALL);
        assert_eq!(x.get_strike(), Money::new(100.0));
        assert_eq!(x.apply(Money::new(105.0)), Money::new(5.0));
        assert_eq!(x.apply(Money::new(95.0)), Money::new(0.0));
        assert_eq!(x.to_string(), "$ 100.00 CALL");

        let x: VanillaPayoff<f64, USD> = VanillaPayoff::new(Money::new(100.0), OptionType::PUT);
        assert_eq!(x.get_option_type(), OptionType::PUT);
        assert_eq!(x.get_strike(), Money::new(100.0));
        assert_eq!(x.apply(Money::new(105.0)), Money::new(0.0));
        assert_eq!(x.apply(Money::new(95.0)), Money::new(5.0));
        assert_eq!(x.to_string(), "$ 100.00 PUT");
    }
}
