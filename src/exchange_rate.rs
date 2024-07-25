use std::marker::PhantomData;

use crate::{currency::Currency, money::Money};

use num::Num;

pub struct ExchangeRate<N, B, Q>
where
    N: Num + PartialOrd + Clone,
    B: Currency,
    Q: Currency,
{
    rate: N,
    base_currency: PhantomData<B>,
    quote_currency: PhantomData<Q>,
}

impl<N, B, Q> ExchangeRate<N, B, Q>
where
    N: Num + PartialOrd + Copy,
    B: Currency,
    Q: Currency,
{
    pub fn new(rate: N) -> Self {
        Self {
            rate,
            quote_currency: PhantomData,
            base_currency: PhantomData,
        }
    }

    pub fn convert_to_base(&self, rhs: &Money<N, Q>) -> Money<N, B> {
        Money::new(rhs.amount / self.rate)
    }
    pub fn convert_to_quote(&self, rhs: &Money<N, B>) -> Money<N, Q> {
        Money::new(rhs.amount * self.rate)
    }
}

#[cfg(test)]
mod tests {

    use crate::currency::{EUR, USD};
    use crate::macros::assert_approx_equal_Money;
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
    fn test_quote_to_quote() {
        let m1: Money<f64, EUR> = Money::new(10.0);
        let rate: ExchangeRate<f64, EUR, USD> = ExchangeRate::new(1.08372);

        let expected = Money::new(10.8372);
        assert_approx_equal_Money!(rate.convert_to_quote(&m1), expected);
    }
}
