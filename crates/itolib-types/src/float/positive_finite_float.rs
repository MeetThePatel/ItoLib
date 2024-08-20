use ordered_float::OrderedFloat;

use crate::float::macros::impl_float;

// =============================================================================
// Definition
// =============================================================================

/// Positive finite floating point numbers.
///
/// Elements of $\mathbb{R}^*_+ \coloneqq (0, \infty)$.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Hash)]
pub struct PositiveFiniteFloat(OrderedFloat<f64>);

// =============================================================================
// Implementations
// =============================================================================

impl PositiveFiniteFloat {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let float_repr: f64 = value.into();
        if float_repr > 0.0 && float_repr.is_finite() {
            Some(Self(OrderedFloat(float_repr)))
        } else {
            None
        }
    }
}

impl_float!(PositiveFiniteFloat);

#[cfg(test)]
mod tests {
    use core::f64;
    use std::cmp::Ordering;

    use super::PositiveFiniteFloat;

    #[test]
    fn domain() {
        // Assert no zeros.
        assert!(PositiveFiniteFloat::new(0.0).is_none());
        assert!(PositiveFiniteFloat::new(-0.0).is_none());

        // Check positivity requirement of nonzero integers.
        assert!(PositiveFiniteFloat::new(1.0).is_some());
        assert!(PositiveFiniteFloat::new(-1.0).is_none());

        // Assert no infinities.
        assert!(PositiveFiniteFloat::new(f64::INFINITY).is_none());
        assert!(PositiveFiniteFloat::new(f64::NEG_INFINITY).is_none());

        // Assert no NaNs.
        assert!(PositiveFiniteFloat::new(f64::NAN).is_none());
    }

    #[test]
    fn equality() {
        let positive_num = PositiveFiniteFloat::new(1.0).unwrap();

        // PositiveNum as LHS
        assert_eq!(positive_num, positive_num);
    }

    #[test]
    fn ordering() {
        let positive_num = PositiveFiniteFloat::new(1.0).unwrap();
        let positive_num2 = PositiveFiniteFloat::new(2.0).unwrap();

        // Reflexivity of equality.
        assert_eq!(positive_num.partial_cmp(&positive_num).unwrap(), Ordering::Equal);

        // Expect normal behavior.
        assert_eq!(positive_num.partial_cmp(&positive_num2).unwrap(), Ordering::Less);
        assert_eq!(positive_num2.partial_cmp(&positive_num).unwrap(), Ordering::Greater);
    }
}
