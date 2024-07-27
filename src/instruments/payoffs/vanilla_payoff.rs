use std::fmt::Display;

use crate::instruments::options::OptionType;
use crate::instruments::payoffs::{CallPutPayoff, Payoff};
use crate::money::MonetaryNumber;

pub struct VanillaPayoff<N>
where
    N: MonetaryNumber,
{
    strike: N,
    option_type: OptionType,
}

impl<N> VanillaPayoff<N>
where
    N: MonetaryNumber,
{
    #[must_use]
    #[inline]
    pub const fn new(strike: N, option_type: OptionType) -> Self {
        Self {
            strike,
            option_type,
        }
    }

    #[must_use]
    #[inline]
    pub const fn get_strike(&self) -> N {
        self.strike
    }
}

impl<N> Display for VanillaPayoff<N>
where
    N: MonetaryNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: When reworking ISO rounding, update this.
        write!(f, "{:.4} {}", self.get_strike(), self.get_option_type())
    }
}

impl<N> Payoff<N> for VanillaPayoff<N>
where
    N: MonetaryNumber,
{
    fn apply(&self, price: N) -> N {
        match self.option_type {
            OptionType::CALL => {
                if (price - self.strike) > N::zero() {
                    price - self.strike
                } else {
                    N::zero()
                }
            }
            OptionType::PUT => {
                if (self.strike - price) > N::zero() {
                    self.strike - price
                } else {
                    N::zero()
                }
            }
        }
    }
}

impl<N> CallPutPayoff<N> for VanillaPayoff<N>
where
    N: MonetaryNumber,
{
    fn get_option_type(&self) -> OptionType {
        self.option_type
    }
}

#[cfg(test)]
mod tests {
    use crate::instruments::{
        options::OptionType,
        payoffs::{CallPutPayoff, Payoff},
    };

    use super::VanillaPayoff;

    #[test]
    fn test_vanilla_payoff() {
        let x: VanillaPayoff<f64> = VanillaPayoff::new(100.0, OptionType::CALL);
        assert_eq!(x.get_option_type(), OptionType::CALL);
        assert_eq!(x.get_strike(), 100.0);
        assert_eq!(x.apply(105.0), 5.0);
        assert_eq!(x.apply(95.0), 0.0);
        assert_eq!(x.to_string(), "100.0000 CALL");

        let x: VanillaPayoff<f64> = VanillaPayoff::new(100.0, OptionType::PUT);
        assert_eq!(x.get_option_type(), OptionType::PUT);
        assert_eq!(x.get_strike(), 100.0);
        assert_eq!(x.apply(105.0), 0.0);
        assert_eq!(x.apply(95.0), 5.0);
        assert_eq!(x.to_string(), "100.0000 PUT");
    }
}
