use std::fmt::Display;
use std::marker::PhantomData;

use num::Zero;

use crate::instruments::options::OptionType;
use crate::instruments::payoffs::Payoff;
use crate::instruments::payoffs::StrikedPayoff;
use crate::money::{Currency, Money};

#[derive(Debug, Copy, Clone)]
pub struct VanillaPayoff<C>
where
    C: Currency,
{
    strike: Money<C>,
    option_type: OptionType,
    currency: PhantomData<C>,
}

impl<C> VanillaPayoff<C>
where
    C: Currency,
{
    #[must_use]
    #[inline]
    pub const fn new(strike: Money<C>, option_type: OptionType) -> Self {
        Self {
            strike,
            option_type,
            currency: PhantomData,
        }
    }
}

impl<C> Display for VanillaPayoff<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.get_strike(), self.get_option_type())
    }
}

impl<C> Payoff for VanillaPayoff<C>
where
    C: Currency,
{
    type PayoffNumberType = Money<C>;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType {
        match self.option_type {
            OptionType::CALL => {
                if price > self.strike {
                    price - self.strike
                } else {
                    Money::zero()
                }
            }
            OptionType::PUT => {
                if self.strike > price {
                    self.strike - price
                } else {
                    Money::zero()
                }
            }
        }
    }
}

impl<C> StrikedPayoff for VanillaPayoff<C>
where
    C: Currency,
{
    fn get_strike(&self) -> Self::PayoffNumberType {
        self.strike
    }

    fn get_option_type(&self) -> OptionType {
        self.option_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruments::{
            options::OptionType,
            payoffs::{Payoff, StrikedPayoff},
        },
        money::{currency::USD, Money},
    };

    use super::VanillaPayoff;

    #[test]
    fn test_vanilla_payoff() {
        let x: VanillaPayoff<USD> = VanillaPayoff::new(Money::new(100.0), OptionType::CALL);
        assert_eq!(x.get_option_type(), OptionType::CALL);
        assert_eq!(x.get_strike(), Money::new(100.0));
        assert_eq!(x.apply(Money::new(105.0)), Money::new(5.0));
        assert_eq!(x.apply(Money::new(95.0)), Money::new(0.0));
        assert_eq!(x.to_string(), "$ 100.00 CALL");

        let x: VanillaPayoff<USD> = VanillaPayoff::new(Money::new(100.0), OptionType::PUT);
        assert_eq!(x.get_option_type(), OptionType::PUT);
        assert_eq!(x.get_strike(), Money::new(100.0));
        assert_eq!(x.apply(Money::new(105.0)), Money::new(0.0));
        assert_eq!(x.apply(Money::new(95.0)), Money::new(5.0));
        assert_eq!(x.to_string(), "$ 100.00 PUT");
    }
}
