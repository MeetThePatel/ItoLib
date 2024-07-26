use std::{fmt::Display, marker::PhantomData};

use crate::money::{Currency, MonetaryNumber, Money};

pub struct ExchangeRate<N, B, Q>
where
    N: MonetaryNumber,
    B: Currency,
    Q: Currency,
{
    pub rate: N,
    base_currency: PhantomData<B>,
    quote_currency: PhantomData<Q>,
}

impl<N, B, Q> ExchangeRate<N, B, Q>
where
    N: MonetaryNumber,
    B: Currency,
    Q: Currency,
{
    #[must_use]
    pub const fn new(rate: N) -> Self {
        Self {
            rate,
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
    pub fn convert_to_base(&self, rhs: &Money<N, Q>) -> Money<N, B> {
        Money::new(rhs.amount / self.rate)
    }
    #[must_use]
    #[inline]
    pub fn convert_to_quote(&self, rhs: &Money<N, B>) -> Money<N, Q> {
        Money::new(rhs.amount * self.rate)
    }
}

impl<N, B, Q> Display for ExchangeRate<N, B, Q>
where
    N: MonetaryNumber,
    B: Currency,
    Q: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}/{}",
            self.rate,
            self.get_base_currency().get_alphabetic_code(),
            self.get_quote_currency().get_alphabetic_code()
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::macros::assert_approx_equal_Money;
    use crate::money::currency::{EUR, USD};
    use crate::money::Money;

    use super::ExchangeRate;

    #[test]
    fn test_quote_to_base() {
        let m1: Money<f64, USD> = Money::new(10.0);
        let rate: ExchangeRate<f64, EUR, USD> = ExchangeRate::new(1.08372);

        let expected = Money::new(9.227_475_731_738_826);
        assert_approx_equal_Money!(rate.convert_to_base(&m1), expected);
    }

    #[test]
    fn test_base_to_quote() {
        let m1: Money<f64, EUR> = Money::new(10.0);
        let rate: ExchangeRate<f64, EUR, USD> = ExchangeRate::new(1.08372);

        let expected = Money::new(10.8372);
        assert_approx_equal_Money!(rate.convert_to_quote(&m1), expected);
    }

    #[test]
    fn test_display() {
        let rate: ExchangeRate<f64, EUR, USD> = ExchangeRate::new(1.08372);
        assert_eq!(rate.to_string(), "1.08372 EUR/USD");
    }
}
