use num::traits::NumOps;
use ordered_float::OrderedFloat;

use crate::float::NonNegativeFiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Volatility(NonNegativeFiniteFloat);

impl Volatility {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Self {
        Self(NonNegativeFiniteFloat::new(value.into()).unwrap())
    }

    #[must_use]
    pub const fn value(&self) -> NonNegativeFiniteFloat {
        self.0
    }
}

impl Default for Volatility {
    fn default() -> Self {
        Self::new(0.0)
    }
}
impl std::ops::Add for Volatility {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value().value() + rhs.value().value())
    }
}

impl std::ops::Sub for Volatility {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value().value() - rhs.value().value())
    }
}

impl std::ops::Mul for Volatility {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value().value() * rhs.value().value())
    }
}

impl std::ops::Div for Volatility {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value().value() / rhs.value().value())
    }
}

impl std::ops::Rem for Volatility {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.value().value() % rhs.value().value())
    }
}

impl From<Volatility> for OrderedFloat<f64> {
    fn from(val: Volatility) -> Self {
        val.value().value()
    }
}

impl std::ops::Mul<OrderedFloat<f64>> for Volatility {
    type Output = Self;

    fn mul(self, rhs: OrderedFloat<f64>) -> Self::Output {
        Self::new(self.value().value() * *rhs)
    }
}

impl std::ops::Div<OrderedFloat<f64>> for Volatility {
    type Output = Self;

    fn div(self, rhs: OrderedFloat<f64>) -> Self::Output {
        Self::new(self.value().value() / *rhs)
    }
}
