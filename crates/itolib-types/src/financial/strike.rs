use crate::float::FiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Strike(FiniteFloat);

impl num::Bounded for Strike {
    fn min_value() -> Self {
        Self(FiniteFloat::new(f64::MIN).unwrap())
    }

    fn max_value() -> Self {
        Self(FiniteFloat::new(f64::MAX).unwrap())
    }
}
