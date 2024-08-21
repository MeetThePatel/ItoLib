use crate::float::FiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
// Must be nonNAN
pub struct Percentage(FiniteFloat);

impl Percentage {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        Some(Self(FiniteFloat::new(value.into())?))
    }

    #[must_use]
    pub const fn value(&self) -> FiniteFloat {
        self.0
    }
}
