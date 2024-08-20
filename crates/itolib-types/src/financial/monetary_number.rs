use ordered_float::OrderedFloat;

use crate::{
    financial::macros::{
        impl_add_self, impl_div_f64_like, impl_mul_f64_like, impl_rem_self, impl_sub_self,
    },
    float::FiniteFloat,
};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Hash)]
pub struct MonetaryNumber(FiniteFloat);

impl MonetaryNumber {
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        Some(Self(FiniteFloat::new(value)?))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0.value().into_inner()
    }
}

impl_add_self!(MonetaryNumber);
impl_sub_self!(MonetaryNumber);
impl_rem_self!(MonetaryNumber);
impl_mul_f64_like!(MonetaryNumber);
impl_div_f64_like!(MonetaryNumber);

impl From<f64> for MonetaryNumber {
    fn from(value: f64) -> Self {
        Self::new(value).unwrap()
    }
}
impl From<OrderedFloat<f64>> for MonetaryNumber {
    fn from(value: OrderedFloat<f64>) -> Self {
        Self::new(*value).unwrap()
    }
}

impl From<MonetaryNumber> for f64 {
    fn from(value: MonetaryNumber) -> Self {
        value.0.into()
    }
}

impl From<MonetaryNumber> for OrderedFloat<f64> {
    fn from(value: MonetaryNumber) -> Self {
        value.0.into()
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
