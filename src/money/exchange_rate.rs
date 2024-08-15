use std::{fmt::Display, marker::PhantomData};

use crate::money::{Currency, Money};
use crate::types::MonetaryNumber;

pub struct ExchangeRate<B, Q>
where
    B: Currency,
    Q: Currency,
{
    pub rate: MonetaryNumber,
    base_currency: PhantomData<B>,
    quote_currency: PhantomData<Q>,
}

impl<B, Q> ExchangeRate<B, Q>
where
    B: Currency,
    Q: Currency,
{
    #[must_use]
    pub fn new(rate: impl Into<MonetaryNumber>) -> Self {
        Self {
            rate: rate.into(),
            quote_currency: PhantomData,
            base_currency: PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub fn get_base_currency(&self) -> B {
        Default::default()
    }
    #[must_use]
    #[inline]
    pub fn get_quote_currency(&self) -> Q {
        Default::default()
    }

    #[must_use]
    #[inline]
    pub fn convert_to_base(&self, rhs: &Money<Q>) -> Money<B> {
        Money::new(rhs.amount / self.rate)
    }
    #[must_use]
    #[inline]
    pub fn convert_to_quote(&self, rhs: &Money<B>) -> Money<Q> {
        Money::new(rhs.amount * self.rate)
    }
}

impl<B, Q> Display for ExchangeRate<B, Q>
where
    B: Currency,
    Q: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}/{}",
            self.rate,
            self.get_base_currency().alphabetic_code(),
            self.get_quote_currency().alphabetic_code()
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::macros::assert_approx_equal_money;
    use crate::money::currency::{EUR, USD};
    use crate::money::Money;

    use super::ExchangeRate;

    #[test]
    fn test_quote_to_base() {
        let m1: Money<USD> = Money::new(10.0);
        let rate: ExchangeRate<EUR, USD> = ExchangeRate::new(1.08372);

        let expected: Money<EUR> = Money::new(9.227_475_731_738_826);
        assert_approx_equal_money!(rate.convert_to_base(&m1), expected, 10e-8);
    }

    #[test]
    fn test_base_to_quote() {
        let m1: Money<EUR> = Money::new(10.0);
        let rate: ExchangeRate<EUR, USD> = ExchangeRate::new(1.08372);

        let expected: Money<USD> = Money::new(10.8372);
        assert_approx_equal_money!(rate.convert_to_quote(&m1), expected, 10e-8);
    }

    #[test]
    fn test_display() {
        let rate: ExchangeRate<EUR, USD> = ExchangeRate::new(1.08372);
        assert_eq!(rate.to_string(), "1.08372 EUR/USD");
    }
}
