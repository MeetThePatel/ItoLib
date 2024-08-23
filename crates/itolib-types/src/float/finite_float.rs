use num::Bounded;

use crate::float::{
    IntoFloat, NonNegativeFiniteFloat, NonNegativeFloat, PositiveFiniteFloat, PositiveFloat,
};
use crate::{generate_fallible_conversion_impls, impl_float};
// =============================================================================
// Definition
// =============================================================================

/// Finite floating point numbers.
///
/// Elements of $\overline{\mathbb{R}} \coloneqq (-\infty), \infty)$.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct FiniteFloat(f64);

// =============================================================================
// Implementations
// =============================================================================

impl FiniteFloat {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Option<Self> {
        let float_repr: f64 = value.into();
        if float_repr.is_infinite() || float_repr.is_nan() {
            None
        } else {
            Some(Self(float_repr))
        }
    }
}

impl IntoFloat for FiniteFloat {
    fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Bounded for FiniteFloat {
    fn min_value() -> Self {
        Self(f64::MIN)
    }

    fn max_value() -> Self {
        Self(f64::MAX)
    }
}

impl_float!(FiniteFloat);

generate_fallible_conversion_impls!(
    FiniteFloat,
    NonNegativeFloat,
    NonNegativeFiniteFloat,
    PositiveFloat,
    PositiveFiniteFloat
);

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::float::FiniteFloat;

    #[test]
    fn domain() {
        // Check positivity requirement of zeroes.
        assert!(FiniteFloat::new(0.0).is_some());
        assert!(FiniteFloat::new(-0.0).is_some());

        // Check positivity requirement of nonzero integers.
        assert!(FiniteFloat::new(1.0).is_some());
        assert!(FiniteFloat::new(-1.0).is_some());

        // Assert no infinities.
        assert!(FiniteFloat::new(f64::INFINITY).is_none());
        assert!(FiniteFloat::new(f64::NEG_INFINITY).is_none());

        // Assert no NaNs.
        assert!(FiniteFloat::new(f64::NAN).is_none());
    }

    #[test]
    fn equality() {
        let zero = FiniteFloat::new(0.0).unwrap();
        let positive_num = FiniteFloat::new(1.0).unwrap();
        let negative_num = FiniteFloat::new(-1.0).unwrap();

        // Zero as LHS
        assert_eq!(zero, zero);
        assert_ne!(zero, positive_num);
        assert_ne!(zero, negative_num);

        // PositiveNum as LHS
        assert_ne!(positive_num, zero);
        assert_eq!(positive_num, positive_num);
        assert_ne!(positive_num, negative_num);

        // Infinity as LHS
        assert_ne!(negative_num, zero);
        assert_ne!(negative_num, positive_num);
        assert_eq!(negative_num, negative_num);
    }

    #[test]
    fn ordering() {
        let zero = FiniteFloat::new(0.0).unwrap();
        let positive_num = FiniteFloat::new(1.0).unwrap();
        let negative_num = FiniteFloat::new(-1.0).unwrap();

        // Reflexivity of equality.
        assert_eq!(zero.partial_cmp(&zero).unwrap(), Ordering::Equal);
        assert_eq!(positive_num.partial_cmp(&positive_num).unwrap(), Ordering::Equal);
        assert_eq!(negative_num.partial_cmp(&negative_num).unwrap(), Ordering::Equal);

        // Less.
        assert_eq!(zero.partial_cmp(&positive_num).unwrap(), Ordering::Less);
        assert_eq!(negative_num.partial_cmp(&zero).unwrap(), Ordering::Less);
        assert_eq!(negative_num.partial_cmp(&positive_num).unwrap(), Ordering::Less);

        // Greater
        assert_eq!(zero.partial_cmp(&negative_num).unwrap(), Ordering::Greater);
        assert_eq!(positive_num.partial_cmp(&negative_num).unwrap(), Ordering::Greater);
        assert_eq!(positive_num.partial_cmp(&zero).unwrap(), Ordering::Greater);
    }

    #[test]
    fn operations() {
        // ===========================================================
        // Operations that stay in the domain.
        // ===========================================================
        assert_eq!(
            FiniteFloat::new(1.0) + FiniteFloat::new(1.0).unwrap(),
            Some(FiniteFloat::new(2.0).unwrap())
        );
        assert_eq!(
            FiniteFloat::new(1.0).unwrap() - FiniteFloat::new(1.0).unwrap(),
            Some(FiniteFloat::new(0.0).unwrap())
        );

        // ===========================================================
        // Operations that leave the domain.
        // ===========================================================
        assert!((FiniteFloat::new(1.0).unwrap() + FiniteFloat::new(0.0)).is_some());
    }
}
