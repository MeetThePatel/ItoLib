use crate::float::Float;

/// Discount Factors
///
/// # Domain:
/// [`DiscountFactor`]s must reside in $(0, 1]$.
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct DiscountFactor(Float);

impl DiscountFactor {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let value: f64 = value.into();

        if value <= 1.0 && value > 0.0 {
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
