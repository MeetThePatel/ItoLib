use crate::float::FiniteFloat;

// TODO: Need to get rid of this and move to using an associated type.

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Strike(FiniteFloat);

impl Strike {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        Some(Self(FiniteFloat::new(value.into())?))
    }
}

impl num::Bounded for Strike {
    fn min_value() -> Self {
        Self(FiniteFloat::new(f64::MIN).unwrap())
    }

    fn max_value() -> Self {
        Self(FiniteFloat::new(f64::MAX).unwrap())
    }
}
