use crate::{
    financial::macros::{
        impl_add_self, impl_div_f64_like, impl_mul_f64_like, impl_rem_self, impl_sub_self,
    },
    float::{FiniteFloat, IntoFloat},
};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MonetaryNumber(FiniteFloat);

impl MonetaryNumber {
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        Some(Self(FiniteFloat::new(value)?))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0.as_f64()
    }
}

impl_add_self!(MonetaryNumber);
impl_sub_self!(MonetaryNumber);
impl_rem_self!(MonetaryNumber);
impl_mul_f64_like!(MonetaryNumber);
impl_div_f64_like!(MonetaryNumber);

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
