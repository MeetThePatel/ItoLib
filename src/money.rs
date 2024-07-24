use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::currency::Currency;

use num::Num;

// TODO: Attempt making Money generic over `amount` type.
//       For example, if someone wants to use f128 for a high-precision
//       simulation, they can do so without changing much code.
#[derive(Debug)]
pub struct Money<C>
where
    C: Currency,
{
    amount: f64,
    currency: PhantomData<C>,
}

impl<C> Money<C>
where
    C: Currency,
{
    #[must_use]
    pub const fn new(amount: f64) -> Self {
        Self {
            amount,
            currency: PhantomData,
        }
    }

    #[must_use]
    pub fn get_currency_name(self) -> &'static str {
        C::get_name()
    }

    #[must_use]
    pub fn get_currency_symbol(self) -> &'static str {
        C::get_symbol()
    }

    #[must_use]
    pub fn get_currency_alphabetic_code() -> &'static str {
        C::get_alphabetic_code()
    }

    #[must_use]
    pub fn get_currency_numeric_code() -> &'static str {
        C::get_numeric_code()
    }

    #[must_use]
    pub fn get_currency_minor() -> usize {
        C::get_minor()
    }

    #[must_use]
    pub fn get_currency_fractions() -> u16 {
        C::get_fractions()
    }
}

impl<C> Display for Money<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add rounding method (according to ISO standards).
        write!(
            f,
            "{} {:.minor$}",
            C::get_symbol(),
            self.amount,
            minor = C::get_minor()
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
        match self.amount.partial_cmp(&other.amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.currency.partial_cmp(&other.currency)
    }
}

impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money::new(self.amount + rhs.amount)
    }
}

impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Money::new(self.amount - rhs.amount)
    }
}

impl<C, N> Mul<N> for Money<C>
where
    C: Currency,
    N: Num + Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Money::new(self.amount * rhs.into())
    }
}

impl<C> Div for Money<C>
where
    C: Currency,
{
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.amount / rhs.amount
    }
}

impl<C, N> Div<N> for Money<C>
where
    C: Currency,
    N: Num + Into<f64>,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Money::new(self.amount / rhs.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{currency::USD, money::Money};

    #[cfg(test)]
    mod arithmetic_operations {
        use crate::{currency::USD, money::Money};

        #[test]
        fn test_money_add() {
            let m1: Money<USD> = Money::new(5.0);
            let m2 = Money::new(6.3);
            let result = Money::new(11.3);
            assert_eq!(m1 + m2, result);
        }

        #[test]
        fn test_money_sub() {
            let m1: Money<USD> = Money::new(12.94);
            let m2 = Money::new(6.3);
            let result = Money::new(6.64);
            assert_eq!(m1 - m2, result);
        }

        #[test]
        fn test_money_mul_int() {
            let m: Money<USD> = Money::new(5.0);
            let result = Money::new(20.0);
            assert_eq!(m * 4, result);
        }

        #[test]
        fn test_money_mul_float() {
            let m: Money<USD> = Money::new(5.0);
            let result = Money::new(7.5);
            assert_eq!(m * 1.5, result);
        }

        #[test]
        fn test_money_div_money() {
            let m1: Money<USD> = Money::new(6.0);
            let m2: Money<USD> = Money::new(6.0);
            assert_eq!(m1 / m2, 1.0);
        }

        #[test]
        fn test_money_div_int() {
            let m: Money<USD> = Money::new(6.0);
            let result = Money::new(2.0);
            assert_eq!(m / 3, result);
        }

        #[test]
        fn test_money_div_float() {
            let m: Money<USD> = Money::new(6.0);
            let result = Money::new(2.0);
            assert_eq!(m / 3.0, result);
        }
    }
    #[test]
    fn test_money_display() {
        let m: Money<USD> = Money::new(4.32123);
        assert_eq!(m.to_string(), "$ 4.32");
    }

    #[test]
    fn test_money_partial_eq() {
        let m1: Money<USD> = Money::new(157.34);
        let m2: Money<USD> = Money::new(325.3);
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
