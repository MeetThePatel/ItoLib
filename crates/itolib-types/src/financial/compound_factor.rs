use crate::float::NonNegativeFiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct CompoundFactor(NonNegativeFiniteFloat);

impl CompoundFactor {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        Some(Self(NonNegativeFiniteFloat::new(value.into())?))
    }

    #[must_use]
    pub const fn value(&self) -> NonNegativeFiniteFloat {
        self.0
    }
}
