pub mod currency;
pub use currency::Currency;

mod exchange_rate;
pub use exchange_rate::ExchangeRate;

mod exchange_rate_manager;
pub use exchange_rate_manager::{ExchangeRateManager, ExchangeRateManagerError};

// -------------------------------------------------------------------------------------------------
// Money
// -------------------------------------------------------------------------------------------------

use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::{Float, Num, NumCast, One, ToPrimitive, Zero};
use ordered_float::OrderedFloat;

use crate::types::MonetaryNumber;

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
        Self {
            amount: amount.into(),
            currency: PhantomData,
        }
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
    /// assert_eq!(x.get_currency(), USD::default());
    /// ```
    #[must_use]
    #[inline]
    pub fn currency(&self) -> C {
        C::default()
    }
}

impl<C: Currency> Default for Money<C> {
    fn default() -> Self {
        Self::new(0.0)
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
        Self::new(self.amount + rhs.amount)
    }
}

impl<C: Currency> Sub for Money<C> {
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
        Self::new(self.amount / rhs.into())
    }
}

impl<C, R> Rem<R> for Money<C>
where
    C: Currency,
    R: Into<MonetaryNumber>,
{
    type Output = Self;

    fn rem(self, rhs: R) -> Self::Output {
        Self::new(self.amount % rhs.into())
    }
}

impl<C: Currency> One for Money<C> {
    fn one() -> Self {
        Self::new(1.0)
    }
}

impl<C: Currency> Zero for Money<C> {
    fn zero() -> Self {
        Self::new(MonetaryNumber::zero())
    }

    fn is_zero(&self) -> bool {
        <MonetaryNumber as Float>::abs(self.amount) < <MonetaryNumber as Float>::epsilon()
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
        Self::new(self.amount * -1.0)
    }
}

impl<C: Currency> ToPrimitive for Money<C> {
    fn to_i64(&self) -> Option<i64> {
        self.amount.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.amount.to_u64()
    }
}

impl<C: Currency> NumCast for Money<C> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        <MonetaryNumber as NumCast>::from(n).map(Self::new)
    }
}

impl<C: Currency> Num for Money<C> {
    type FromStrRadixErr = <OrderedFloat<f64> as num::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        OrderedFloat::<f64>::from_str_radix(str, radix).map(Self::new)
    }
}
impl<C: Currency> num::Float for Money<C> {
    fn nan() -> Self {
        Self::new(<MonetaryNumber as Float>::nan())
    }

    fn infinity() -> Self {
        Self::new(<MonetaryNumber as Float>::infinity())
    }

    fn neg_infinity() -> Self {
        Self::new(<MonetaryNumber as Float>::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::new(<MonetaryNumber as Float>::neg_zero())
    }

    fn min_value() -> Self {
        Self::new(<MonetaryNumber as Float>::min_value())
    }

    fn min_positive_value() -> Self {
        Self::new(<MonetaryNumber as Float>::min_positive_value())
    }

    fn max_value() -> Self {
        Self::new(<MonetaryNumber as Float>::max_value())
    }

    fn is_nan(self) -> bool {
        self.amount.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.amount.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.amount.is_finite()
    }

    fn is_normal(self) -> bool {
        self.amount.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.amount.classify()
    }

    fn floor(self) -> Self {
        Self::new(self.amount.floor())
    }

    fn ceil(self) -> Self {
        Self::new(self.amount.floor())
    }

    fn round(self) -> Self {
        Self::new(self.amount.round())
    }

    fn trunc(self) -> Self {
        Self::new(self.amount.trunc())
    }

    fn fract(self) -> Self {
        Self::new(self.amount.fract())
    }

    fn abs(self) -> Self {
        Self::new(self.amount.abs())
    }

    fn signum(self) -> Self {
        Self::new(self.amount.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.amount.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.amount.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.amount.mul_add(a.amount, b.amount))
    }

    fn recip(self) -> Self {
        Self::new(self.amount.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.amount.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self::new(self.amount.powf(n.into()))
    }

    fn sqrt(self) -> Self {
        Self::new(self.amount.sqrt())
    }

    fn exp(self) -> Self {
        Self::new(self.amount.exp())
    }

    fn exp2(self) -> Self {
        Self::new(self.amount.exp2())
    }

    fn ln(self) -> Self {
        Self::new(self.amount.ln())
    }

    fn log(self, base: Self) -> Self {
        Self::new(self.amount.log(base.amount()))
    }

    fn log2(self) -> Self {
        Self::new(self.amount.log2())
    }

    fn log10(self) -> Self {
        Self::new(self.amount.log10())
    }

    fn max(self, other: Self) -> Self {
        match self.amount.cmp(&other.amount) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => self,
            std::cmp::Ordering::Less => other,
        }
    }

    fn min(self, other: Self) -> Self {
        match self.amount.cmp(&other.amount) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => self,
            std::cmp::Ordering::Greater => other,
        }
    }

    fn abs_sub(self, other: Self) -> Self {
        Self::new(self.amount.abs_sub(other.amount))
    }

    fn cbrt(self) -> Self {
        Self::new(self.amount.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self::new(self.amount.hypot(other.amount))
    }

    fn sin(self) -> Self {
        Self::new(self.amount.sin())
    }

    fn cos(self) -> Self {
        Self::new(self.amount.cos())
    }

    fn tan(self) -> Self {
        Self::new(self.amount.tan())
    }

    fn asin(self) -> Self {
        Self::new(self.amount.asin())
    }

    fn acos(self) -> Self {
        Self::new(self.amount.acos())
    }

    fn atan(self) -> Self {
        Self::new(self.amount.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self::new(self.amount.atan2(other.amount))
    }

    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    fn exp_m1(self) -> Self {
        Self::new(self.amount.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::new(self.amount.ln_1p())
    }

    fn sinh(self) -> Self {
        Self::new(self.amount.sinh())
    }

    fn cosh(self) -> Self {
        Self::new(self.amount.cosh())
    }

    fn tanh(self) -> Self {
        Self::new(self.amount.tanh())
    }

    fn asinh(self) -> Self {
        Self::new(self.amount.asinh())
    }

    fn acosh(self) -> Self {
        Self::new(self.amount.acosh())
    }

    fn atanh(self) -> Self {
        Self::new(self.amount.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.amount.integer_decode()
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{currency::USD, Money};

    #[cfg(test)]
    mod arithmetic_operations {
        use crate::{
            macros::assert_approx_equal_money,
            money::{currency::USD, Money},
        };

        #[test]
        fn test_money_add() {
            let m1: Money<USD> = Money::new(5.0);
            let m2 = Money::new(6.3);

            let expected: Money<USD> = Money::new(11.3);
            assert_approx_equal_money!(m1 + m2, expected, 10e-8);
        }

        #[test]
        fn test_money_sub() {
            let m1: Money<USD> = Money::new(12.94);
            let m2 = Money::new(6.3);

            let expected: Money<USD> = Money::new(6.64);
            assert_approx_equal_money!(m1 - m2, expected, 10e-8);
        }

        #[test]
        fn test_money_mul_int() {
            let m: Money<USD> = Money::new(5.0);

            let expected: Money<USD> = Money::new(20.0);
            assert_approx_equal_money!(m * 4.0, expected, 10e-8);
        }

        #[test]
        fn test_money_mul_float() {
            let m: Money<USD> = Money::new(5.0);

            let expected: Money<USD> = Money::new(7.5);
            assert_approx_equal_money!(m * 1.5, expected, 10e-8);
        }

        #[test]
        fn test_money_div_money() {
            let m1: Money<USD> = Money::new(6.0);
            let m2: Money<USD> = Money::new(6.0);

            let expected: Money<USD> = Money::new(1.0);
            assert_approx_equal_money!(m1 / m2, expected, 10e-8);
        }

        #[test]
        fn test_money_div_float() {
            let m: Money<USD> = Money::new(6.0);

            let expected: Money<USD> = Money::new(2.0);
            assert_approx_equal_money!(m / 3.0, expected, 10e-8);
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
