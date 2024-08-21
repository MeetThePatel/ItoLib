use crate::financial::macros::impl_ops_self;
use crate::float::{IntoFloat, NonNegativeFiniteFloat};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Volatility(NonNegativeFiniteFloat);

impl Volatility {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        Some(Self(NonNegativeFiniteFloat::new(value)?))
    }

    #[must_use]
    pub const fn value(&self) -> NonNegativeFiniteFloat {
        self.0
    }
}

impl Default for Volatility {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl_ops_self!(Volatility);

impl From<Volatility> for f64 {
    fn from(val: Volatility) -> Self {
        val.0.as_f64()
    }
}

// impl std::ops::Mul<f64> for Volatility {
//     type Output = Self;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Self::new(self.value().value() * rhs)
//     }
// }

// impl std::ops::Div<f64> for Volatility {
//     type Output = Self;

//     fn div(self, rhs: f64) -> Self::Output {
//         Self::new(self.value().value() / *rhs)
//     }
// }
