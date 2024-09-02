use crate::float::Float;

// TODO: Need to get rid of this and move to using an associated type.

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct Strike(Float);

impl Strike {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let value: f64 = value.into();

        if value.is_finite() {
            Some(Self(Float::new(value)))
        } else {
            None
        }
    }
}

impl num::Bounded for Strike {
    fn min_value() -> Self {
        Self(Float::new(f64::MIN))
    }

    fn max_value() -> Self {
        Self(Float::new(f64::MAX))
    }
}
