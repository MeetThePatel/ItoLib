use crate::{
    // financial::macros::{impl_ops_f64_like, impl_ops_self},
    float::Float,
};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct MonetaryNumber(Float);

impl MonetaryNumber {
    /// Create a new [`MonetaryNumber`] with bounds-checking.
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let value: f64 = value.into();

        if value.is_finite() {
            Some(Self(Float::new(value)))
        } else {
            None
        }
    }

    /// Get the value contained inside.
    #[must_use]
    pub fn value(self) -> f64 {
        *self.0
    }
}

// impl_ops_self!(MonetaryNumber);
// impl_ops_f64_like!(MonetaryNumber);

pub struct DomainError;

impl TryInto<MonetaryNumber> for f64 {
    type Error = DomainError;

    fn try_into(self) -> Result<MonetaryNumber, Self::Error> {
        MonetaryNumber::new(self).ok_or(DomainError)
    }
}

impl Default for MonetaryNumber {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl std::fmt::Display for MonetaryNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
