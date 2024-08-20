use crate::float::PositiveFiniteFloat;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct DiscountFactor(PositiveFiniteFloat);

impl DiscountFactor {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        Some(Self(PositiveFiniteFloat::new(value.into())?))
    }

    #[must_use]
    pub const fn value(&self) -> PositiveFiniteFloat {
        self.0
    }
}
