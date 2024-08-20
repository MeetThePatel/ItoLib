use ordered_float::OrderedFloat;

use crate::float::FiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
// Must be nonNAN
pub struct Percentage(FiniteFloat);

impl Percentage {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Self {
        Self(FiniteFloat::new(value.into()).unwrap())
    }

    #[must_use]
    pub const fn value(&self) -> FiniteFloat {
        self.0
    }
}
