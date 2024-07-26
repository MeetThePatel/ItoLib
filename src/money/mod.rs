pub mod currency;
pub use currency::Currency;

pub mod exchange_rate;
pub use exchange_rate::*;

pub mod exchange_rate_manager;
pub use exchange_rate_manager::*;

use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use num::traits::NumOps;

#[derive(Clone, Copy, Debug)]
pub struct Money<N, C>
where
    N: NumOps + PartialOrd + Copy,
    C: Currency,
{
    pub amount: N,
    currency: PhantomData<C>,
}

impl<N, C> Money<N, C>
where
    N: NumOps + PartialOrd + Copy,
    C: Currency,
{
    #[must_use]
    pub const fn new(amount: N) -> Self {
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

impl<N, C> Display for Money<N, C>
where
    N: NumOps + PartialOrd + Display + Copy,
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

impl<N, C, R> PartialEq<Money<R, C>> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps + PartialOrd + Copy + Clone,
{
    fn eq(&self, other: &Money<R, C>) -> bool {
        self.amount == other.amount.into()
    }
}

impl<N, C, R> PartialOrd<Money<R, C>> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps + PartialOrd + Copy + Clone,
{
    fn partial_cmp(&self, other: &Money<R, C>) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&(other.amount.into()))
    }
}

impl<N, C, R> Add<Money<R, C>> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps + PartialOrd + Copy,
{
    type Output = Self;

    fn add(self, rhs: Money<R, C>) -> Self::Output {
        Self::new(self.amount + rhs.amount.into())
    }
}

impl<N, C, R> Sub<Money<R, C>> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps + PartialOrd + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Money<R, C>) -> Self::Output {
        Self::new(self.amount - rhs.amount.into())
    }
}

impl<N, C, R> Mul<R> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps<N> + PartialOrd + Copy,
{
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self::new(self.amount * rhs.into())
    }
}

impl<N, C, R> Div<Money<R, C>> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy + From<R>,
    R: NumOps + PartialOrd + Copy,
{
    type Output = N;

    fn div(self, rhs: Money<R, C>) -> Self::Output {
        self.amount / rhs.amount.into()
    }
}

impl<N, C> Div<N> for Money<N, C>
where
    C: Currency,
    N: NumOps + PartialOrd + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Self::new(self.amount / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{currency::USD, Money};

    #[cfg(test)]
    mod arithmetic_operations {
        use crate::{
            macros::{assert_approx_equal_Money, assert_approx_equal_f64},
            money::{currency::USD, Money},
        };

        #[test]
        fn test_money_add() {
            let m1: Money<f64, USD> = Money::new(5.0);
            let m2 = Money::new(6.3);

            let expected = Money::new(11.3);
            assert_approx_equal_Money!(m1 + m2, expected);
        }

        #[test]
        fn test_money_sub() {
            let m1: Money<f64, USD> = Money::new(12.94);
            let m2 = Money::new(6.3);

            let expected = Money::new(6.64);
            assert_approx_equal_Money!(m1 - m2, expected);
        }

        #[test]
        fn test_money_mul_int() {
            let m: Money<f64, USD> = Money::new(5.0);

            let expected = Money::new(20.0);
            assert_approx_equal_Money!(m * 4.0, expected);
        }

        #[test]
        fn test_money_mul_float() {
            let m: Money<f64, USD> = Money::new(5.0);

            let expected = Money::new(7.5);
            assert_approx_equal_Money!(m * 1.5, expected);
        }

        #[test]
        fn test_money_div_money() {
            let m1: Money<f64, USD> = Money::new(6.0);
            let m2: Money<f64, USD> = Money::new(6.0);

            let expected = 1.0;
            assert_approx_equal_f64!(m1 / m2, expected);
        }

        #[test]
        fn test_money_div_int() {
            let m: Money<f64, USD> = Money::new(6.0);

            let expected = Money::new(2.0);
            assert_approx_equal_Money!(m / 3.0, expected);
        }

        #[test]
        fn test_money_div_float() {
            let m: Money<f64, USD> = Money::new(6.0);

            let expected = Money::new(2.0);
            assert_approx_equal_Money!(m / 3.0, expected);
        }
    }
    #[test]
    fn test_money_display() {
        let m: Money<f64, USD> = Money::new(4.32123);

        let expected = "$ 4.32";
        assert_eq!(m.to_string(), expected);
    }

    #[test]
    fn test_money_partial_eq() {
        let m1: Money<f64, USD> = Money::new(157.34);
        let m2: Money<i32, USD> = Money::new(3);
        assert_ne!(m1, m2);
        assert_eq!(m1, m1);
    }

    #[test]
    fn test_money_partial_ord() {
        let m1: Money<f64, USD> = Money::new(41.34);
        let m2: Money<f64, USD> = Money::new(324.3);
        assert!(m2 > m1);
        assert!(m2 >= m1);
        assert!(m1 < m2);
        assert!(m1 <= m2);
    }
}