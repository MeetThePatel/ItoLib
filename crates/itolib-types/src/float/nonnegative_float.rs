use crate::float::macros::impl_float;
use crate::float::{
    FiniteFloat, IntoFloat, NonNegativeFiniteFloat, PositiveFiniteFloat, PositiveFloat,
};
use crate::generate_fallible_conversion_impls;
// =============================================================================
// Definition
// =============================================================================

/// Non-negative floating point numbers.
///
/// Elements of $\overline{\mathbb{R}}_+ \coloneqq [0, \infty) \cup \lbrace \infty \rbrace$.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct NonNegativeFloat(f64);

// =============================================================================
// Implementations
// =============================================================================

impl NonNegativeFloat {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let float_repr: f64 = value.into();
        if float_repr.is_sign_negative() || float_repr.is_nan() {
            None
        } else {
            Some(Self(float_repr))
        }
    }
}

impl IntoFloat for NonNegativeFloat {
    fn as_f64(&self) -> f64 {
        self.0
    }
}

impl_float!(NonNegativeFloat);

generate_fallible_conversion_impls!(
    NonNegativeFloat,
    FiniteFloat,
    NonNegativeFiniteFloat,
    PositiveFiniteFloat,
    PositiveFloat
);

#[cfg(test)]
mod tests {
    use core::f64;
    use std::cmp::Ordering;

    use super::NonNegativeFloat;

    #[test]
    fn domain() {
        // Check positivity requirement of zeroes.
        assert!(NonNegativeFloat::new(0.0).is_some());
        assert!(NonNegativeFloat::new(-0.0).is_none());

        // Check positivity requirement of nonzero integers.
        assert!(NonNegativeFloat::new(1.0).is_some());
        assert!(NonNegativeFloat::new(-1.0).is_none());

        // Check positivity requirement of infinities.
        assert!(NonNegativeFloat::new(f64::INFINITY).is_some());
        assert!(NonNegativeFloat::new(f64::NEG_INFINITY).is_none());

        // Assert no NaNs.
        assert!(NonNegativeFloat::new(f64::NAN).is_none());
    }

    #[test]
    fn equality() {
        let zero = NonNegativeFloat::new(0.0).unwrap();
        let positive_num = NonNegativeFloat::new(1.0).unwrap();
        let inf = NonNegativeFloat::new(f64::INFINITY).unwrap();

        // Zero as LHS
        assert_eq!(zero, zero);
        assert_ne!(zero, positive_num);
        assert_ne!(zero, inf);

        // PositiveNum as LHS
        assert_ne!(positive_num, zero);
        assert_eq!(positive_num, positive_num);
        assert_ne!(positive_num, inf);

        // Infinity as LHS
        assert_ne!(inf, zero);
        assert_ne!(inf, positive_num);
        assert_eq!(inf, inf);
    }

    #[test]
    fn ordering() {
        let zero = NonNegativeFloat::new(0.0).unwrap();
        let positive_num = NonNegativeFloat::new(1.0).unwrap();
        let inf = NonNegativeFloat::new(f64::INFINITY).unwrap();

        // Reflexivity of equality.
        assert_eq!(zero.partial_cmp(&zero).unwrap(), Ordering::Equal);
        assert_eq!(positive_num.partial_cmp(&positive_num).unwrap(), Ordering::Equal);
        assert_eq!(inf.partial_cmp(&inf).unwrap(), Ordering::Equal);

        // Less.
        assert_eq!(zero.partial_cmp(&positive_num).unwrap(), Ordering::Less);
        assert_eq!(zero.partial_cmp(&inf).unwrap(), Ordering::Less);
        assert_eq!(positive_num.partial_cmp(&inf).unwrap(), Ordering::Less);

        // Greater
        assert_eq!(inf.partial_cmp(&positive_num).unwrap(), Ordering::Greater);
        assert_eq!(inf.partial_cmp(&zero).unwrap(), Ordering::Greater);
        assert_eq!(positive_num.partial_cmp(&zero).unwrap(), Ordering::Greater);
    }
}
