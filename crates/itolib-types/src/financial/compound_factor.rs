use crate::float::Float;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct CompoundFactor(Float);

impl CompoundFactor {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let value: f64 = value.into();

        if value >= 1.0 && value.is_finite() {
            Some(Self(Float::new(value)))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn value(&self) -> Float {
        self.0
    }
}
