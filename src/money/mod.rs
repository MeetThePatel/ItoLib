pub mod currency;
pub use currency::Currency;

mod exchange_rate;
pub use exchange_rate::ExchangeRate;

mod exchange_rate_manager;
pub use exchange_rate_manager::{ExchangeRateManager, ExchangeRateManagerError};

use num::Zero;

use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::types::MonetaryNumber;

#[derive(Copy, Clone, Debug)]
pub struct Money<C>
where
    C: Currency,
{
    pub amount: MonetaryNumber,
    currency: PhantomData<C>,
}

impl<C> Money<C>
where
    C: Currency,
{
    #[must_use]
    pub const fn new(amount: MonetaryNumber) -> Self {
        Self {
            amount,
            currency: PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub fn get_currency(&self) -> C {
        Default::default()
    }

    #[must_use]
    #[inline]
    pub fn get_currency_name(&self) -> &'static str {
        self.get_currency().get_name()
    }
    #[must_use]
    #[inline]
    pub fn get_currency_symbol(&self) -> &'static str {
        self.get_currency().get_symbol()
    }

    #[must_use]
    #[inline]
    pub fn get_currency_alphabetic_code(&self) -> &'static str {
        self.get_currency().get_alphabetic_code()
    }

    #[must_use]
    #[inline]
    pub fn get_currency_numeric_code(&self) -> &'static str {
        self.get_currency().get_numeric_code()
    }

    #[must_use]
    #[inline]
    pub fn get_currency_minor(&self) -> usize {
        self.get_currency().get_minor()
    }

    #[must_use]
    #[inline]
    pub fn get_currency_fractions(&self) -> u16 {
        self.get_currency().get_fractions()
    }
}

impl<C> Display for Money<C>
where
    C: Currency + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add rounding method (according to ISO standards).
        write!(
            f,
            "{} {:.minor$}",
            self.get_currency_symbol(),
            self.amount,
            minor = self.get_currency_minor()
        )
    }
}

impl<C> PartialEq for Money<C>
where
    C: Currency,
{
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl<C> PartialOrd for Money<C>
where
    C: Currency,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.amount + rhs.amount)
    }
}

impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.amount - rhs.amount)
    }
}

impl<C, R> Mul<R> for Money<C>
where
    C: Currency,
    R: Into<MonetaryNumber>,
{
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self::new(self.amount * rhs.into())
    }
}

impl<C, R> Div<R> for Money<C>
where
    C: Currency,
    R: Into<MonetaryNumber>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: R) -> Self::Output {
        Money::new(self.amount / rhs.into())
    }
}

impl<C> Zero for Money<C>
where
    C: Currency,
{
    fn zero() -> Self {
        Self::new(MonetaryNumber::zero())
    }

    fn is_zero(&self) -> bool {
        (self.amount - MonetaryNumber::zero()).abs() < MonetaryNumber::EPSILON
    }
}

impl<C> From<Money<C>> for MonetaryNumber
where
    C: Currency,
{
    fn from(val: Money<C>) -> Self {
        val.amount
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{currency::USD, Money};

    #[cfg(test)]
    mod arithmetic_operations {
        use crate::{
            macros::assert_approx_equal_Money,
            money::{currency::USD, Money},
        };

        #[test]
        fn test_money_add() {
            let m1: Money<USD> = Money::new(5.0);
            let m2 = Money::new(6.3);

            let expected = Money::new(11.3);
            assert_approx_equal_Money!(m1 + m2, expected);
        }

        #[test]
        fn test_money_sub() {
            let m1: Money<USD> = Money::new(12.94);
            let m2 = Money::new(6.3);

            let expected = Money::new(6.64);
            assert_approx_equal_Money!(m1 - m2, expected);
        }

        #[test]
        fn test_money_mul_int() {
            let m: Money<USD> = Money::new(5.0);

            let expected = Money::new(20.0);
            assert_approx_equal_Money!(m * 4.0, expected);
        }

        #[test]
        fn test_money_mul_float() {
            let m: Money<USD> = Money::new(5.0);

            let expected = Money::new(7.5);
            assert_approx_equal_Money!(m * 1.5, expected);
        }

        #[test]
        fn test_money_div_money() {
            let m1: Money<USD> = Money::new(6.0);
            let m2: Money<USD> = Money::new(6.0);

            let expected: Money<USD> = Money::new(1.0);
            assert_approx_equal_Money!(m1 / m2, expected);
        }

        #[test]
        fn test_money_div_float() {
            let m: Money<USD> = Money::new(6.0);

            let expected = Money::new(2.0);
            assert_approx_equal_Money!(m / 3.0, expected);
        }
    }
    #[test]
    fn test_money_display() {
        let m: Money<USD> = Money::new(4.32123);

        let expected = "$ 4.32";
        assert_eq!(m.to_string(), expected);
    }

    #[test]
    fn test_money_partial_eq() {
        let m1: Money<USD> = Money::new(157.34);
        let m2: Money<USD> = Money::new(3.0);
        assert_ne!(m1, m2);
        assert_eq!(m1, m1);
    }

    #[test]
    fn test_money_partial_ord() {
        let m1: Money<USD> = Money::new(41.34);
        let m2: Money<USD> = Money::new(324.3);
        assert!(m2 > m1);
        assert!(m2 >= m1);
        assert!(m1 < m2);
        assert!(m1 <= m2);
    }
}
