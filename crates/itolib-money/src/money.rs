use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::{One, Zero};

use itolib_types::MonetaryNumber;

use crate::Currency;

/// Money value.
///
/// The reason for this class is to create a strongly-typed wrapper around different values of
/// different currencies, and to provide for operations between them.
///
/// For example, it doesn't make much sense to allow for two [`Money`]s of different currencies to
/// be trivially added together. (Note: This operation is supported, but has to be made explicity;
/// see [`ExchangeRate`])
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Money<C: Currency> {
    /// The number of units of [`Currency`] C.
    amount: MonetaryNumber,

    /// [`PhantomData`] to allow [`Money`] to be generic over C.
    currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    /// Create a new [`Money`] value.
    /// # Examples
    ///
    /// ```
    /// # use itolib::money::Money;
    /// # use itolib::money::currency::USD;
    /// let x: Money<USD> = Money::new(1.0);
    /// ```
    #[must_use]
    pub fn new(amount: impl Into<MonetaryNumber>) -> Self {
        Self { amount: amount.into(), currency: PhantomData }
    }

    /// Get the amount.
    /// # Examples
    ///
    /// ```
    /// # use ordered_float::OrderedFloat;
    /// # use itolib::money::Money;
    /// # use itolib::money::currency::USD;
    /// let x: Money<USD> = Money::new(1.0);
    /// assert_eq!(x.amount(), 1.0);
    /// ```
    #[must_use]
    pub const fn amount(&self) -> MonetaryNumber {
        self.amount
    }

    /// Get the currency.
    /// # Examples
    ///
    /// ```
    /// # use itolib::money::Money;
    /// # use itolib::money::currency::USD;
    /// let x: Money<USD> = Money::new(1.0);
    /// assert_eq!(x.currency(), USD::default());
    /// ```
    #[must_use]
    #[inline]
    pub fn currency(&self) -> C {
        C::default()
    }
}

impl<C: Currency> Default for Money<C> {
    fn default() -> Self {
        Self::new(MonetaryNumber::new(0.0).unwrap())
    }
}

impl<C: Currency> std::fmt::Display for Money<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add rounding method (according to ISO standards).
        write!(
            f,
            "{} {:.minor$}",
            self.currency().symbol(),
            self.amount,
            minor = self.currency().minor()
        )
    }
}

impl<C: Currency> PartialEq for Money<C> {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl<C: Currency> PartialOrd for Money<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl<C: Currency> Add for Money<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.amount + rhs.amount).expect("Must result in finite money."))
    }
}

impl<C: Currency> Sub for Money<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((self.amount - rhs.amount).expect("Must result in finite money."))
    }
}

impl<C, R> Mul<R> for Money<C>
where
    C: Currency,
    R: Into<MonetaryNumber>,
{
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self::new((self.amount * rhs.into()).expect("Must result in finite money."))
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
        Self::new((self.amount / rhs.into()).expect("Must result in finite money."))
    }
}

impl<C, R> Rem<R> for Money<C>
where
    C: Currency,
    R: Into<MonetaryNumber>,
{
    type Output = Self;

    fn rem(self, rhs: R) -> Self::Output {
        Self::new((self.amount % rhs.into()).expect("Must result in finite money."))
    }
}

impl<C: Currency> One for Money<C> {
    fn one() -> Self {
        Self::new(1.0)
    }
}

impl<C: Currency> Zero for Money<C> {
    fn zero() -> Self {
        Self::new((MonetaryNumber::new(0.0)).expect("Must result in finite money."))
    }

    fn is_zero(&self) -> bool {
        self.amount.value().is_zero()
    }
}

impl<C: Currency> From<Money<C>> for MonetaryNumber {
    fn from(val: Money<C>) -> Self {
        val.amount
    }
}

impl<C: Currency> Neg for Money<C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new((self.amount * -1.0).expect("Must result in finite money."))
    }
}

#[cfg(test)]
mod tests {
    use crate::{currency::USD, Money};

    #[cfg(test)]
    mod arithmetic_operations {
        use crate::{currency::USD, Money};
        use itolib_macros::assert_approx_eq_money;

        #[test]
        fn test_money_add() {
            let m1: Money<USD> = Money::new(5.0);
            let m2 = Money::new(6.3);

            let expected: Money<USD> = Money::new(11.3);
            assert_approx_eq_money!(m1 + m2, expected, 10e-8);
        }

        #[test]
        fn test_money_sub() {
            let m1: Money<USD> = Money::new(12.94);
            let m2 = Money::new(6.3);

            let expected: Money<USD> = Money::new(6.64);
            assert_approx_eq_money!(m1 - m2, expected, 10e-8);
        }

        #[test]
        fn test_money_mul_int() {
            let m: Money<USD> = Money::new(5.0);

            let expected: Money<USD> = Money::new(20.0);
            assert_approx_eq_money!(m * 4.0, expected, 10e-8);
        }

        #[test]
        fn test_money_mul_float() {
            let m: Money<USD> = Money::new(5.0);

            let expected: Money<USD> = Money::new(7.5);
            assert_approx_eq_money!(m * 1.5, expected, 10e-8);
        }

        #[test]
        fn test_money_div_money() {
            let m1: Money<USD> = Money::new(6.0);
            let m2: Money<USD> = Money::new(6.0);

            let expected: Money<USD> = Money::new(1.0);
            assert_approx_eq_money!(m1 / m2, expected, 10e-8);
        }

        #[test]
        fn test_money_div_float() {
            let m: Money<USD> = Money::new(6.0);

            let expected: Money<USD> = Money::new(2.0);
            assert_approx_eq_money!(m / 3.0, expected, 10e-8);
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
