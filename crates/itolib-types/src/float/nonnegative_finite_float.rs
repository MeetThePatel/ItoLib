use crate::float::macros::impl_float;
use crate::float::IntoFloat;

// =============================================================================
// Definition
// =============================================================================

/// Non-negative finite floating point numbers.
///
/// Elements of $\mathbb{R}_+ \coloneqq [0, \infty)$.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
// #[derive(Eq, Ord)]
pub struct NonNegativeFiniteFloat(f64);

// =============================================================================
// Implementations
// =============================================================================

impl NonNegativeFiniteFloat {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let float_repr: f64 = value.into();
        if float_repr.is_sign_negative() || float_repr.is_nan() || float_repr.is_infinite() {
            None
        } else {
            Some(Self(float_repr))
        }
    }
}

impl IntoFloat for NonNegativeFiniteFloat {
    fn as_f64(&self) -> f64 {
        self.0
    }
}

impl_float!(NonNegativeFiniteFloat);

#[cfg(test)]
mod tests {
    use core::f64;
    use std::cmp::Ordering;

    use super::NonNegativeFiniteFloat;

    #[test]
    fn domain() {
        // Check positivity requirement of zeroes.
        assert!(NonNegativeFiniteFloat::new(0.0).is_some());
        assert!(NonNegativeFiniteFloat::new(-0.0).is_none());

        // Check positivity requirement of nonzero integers.
        assert!(NonNegativeFiniteFloat::new(1.0).is_some());
        assert!(NonNegativeFiniteFloat::new(-1.0).is_none());

        // Assert no infinities.
        assert!(NonNegativeFiniteFloat::new(f64::INFINITY).is_none());
        assert!(NonNegativeFiniteFloat::new(f64::NEG_INFINITY).is_none());

        // Assert no NaNs.
        assert!(NonNegativeFiniteFloat::new(f64::NAN).is_none());
    }

    #[test]
    fn equality() {
        let zero = NonNegativeFiniteFloat::new(0.0).unwrap();
        let positive_num = NonNegativeFiniteFloat::new(1.0).unwrap();

        // Zero as LHS
        assert_eq!(zero, zero);
        assert_ne!(zero, positive_num);

        // PositiveNum as LHS
        assert_ne!(positive_num, zero);
        assert_eq!(positive_num, positive_num);
    }

    #[test]
    fn ordering() {
        let zero = NonNegativeFiniteFloat::new(0.0).unwrap();
        let positive_num = NonNegativeFiniteFloat::new(1.0).unwrap();

        // Reflexivity of equality.
        assert_eq!(zero.partial_cmp(&zero).unwrap(), Ordering::Equal);
        assert_eq!(positive_num.partial_cmp(&positive_num).unwrap(), Ordering::Equal);

        // Less.
        assert_eq!(zero.partial_cmp(&positive_num).unwrap(), Ordering::Less);

        // Greater
        assert_eq!(positive_num.partial_cmp(&zero).unwrap(), Ordering::Greater);
    }
}
