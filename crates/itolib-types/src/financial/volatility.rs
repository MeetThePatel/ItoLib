use crate::financial::macros::impl_ops_self;
use crate::float::Float;

use super::macros::{impl_ops_f64_like, impl_try_from_float};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct Volatility(Float);

impl Volatility {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let value: f64 = value.into();

        if value >= 0.0 && value.is_finite() {
            Some(Self(Float::new(value)))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(&self) -> Float {
        self.0
    }
}

impl Default for Volatility {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl_ops_self!(Volatility);
impl_ops_f64_like!(Volatility);

impl_try_from_float!(Volatility);
