use ordered_float::OrderedFloat;

use crate::float::macros::impl_float;

// =============================================================================
// Definition
// =============================================================================

/// Positive floating point numbers.
///
/// Elements of $\overline{\mathbb{R}}^*_+ \coloneqq (0, \infty) \cup \lbrace \infty \rbrace$.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Hash)]
pub struct PositiveFloat(OrderedFloat<f64>);

// =============================================================================
// Implementations
// =============================================================================

impl PositiveFloat {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let float_repr: f64 = value.into();
        if float_repr > 0.0 {
            Some(Self(OrderedFloat(float_repr)))
        } else {
            None
        }
    }
}

impl_float!(PositiveFloat);

#[cfg(test)]
mod tests {
    use core::f64;
    use std::cmp::Ordering;

    use super::PositiveFloat;

    #[test]
    fn domain() {
        // Assert no zeros.
        assert!(PositiveFloat::new(0.0).is_none());
        assert!(PositiveFloat::new(-0.0).is_none());

        // Check positivity requirement of nonzero integers.
        assert!(PositiveFloat::new(1.0).is_some());
        assert!(PositiveFloat::new(-1.0).is_none());

        // Check positivity requirement of infinities.
        assert!(PositiveFloat::new(f64::INFINITY).is_some());
        assert!(PositiveFloat::new(f64::NEG_INFINITY).is_none());

        // Assert no NaNs.
        assert!(PositiveFloat::new(f64::NAN).is_none());
    }

    #[test]
    fn equality() {
        let positive_num = PositiveFloat::new(1.0).unwrap();
        let inf = PositiveFloat::new(f64::INFINITY).unwrap();

        // PositiveNum as LHS
        assert_eq!(positive_num, positive_num);
        assert_ne!(positive_num, inf);

        // Infinity as LHS
        assert_ne!(inf, positive_num);
        assert_eq!(inf, inf);
    }

    #[test]
    fn ordering() {
        let positive_num = PositiveFloat::new(1.0).unwrap();
        let inf = PositiveFloat::new(f64::INFINITY).unwrap();

        // Reflexivity of equality.
        assert_eq!(positive_num.partial_cmp(&positive_num).unwrap(), Ordering::Equal);
        assert_eq!(inf.partial_cmp(&inf).unwrap(), Ordering::Equal);

        // Less.
        assert_eq!(positive_num.partial_cmp(&inf).unwrap(), Ordering::Less);

        // Greater
        assert_eq!(inf.partial_cmp(&positive_num).unwrap(), Ordering::Greater);
    }
}
