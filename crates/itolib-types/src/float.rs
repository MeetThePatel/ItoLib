use std::ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub};

// =============================================================================
// Definition
// =============================================================================

/// Floating point numbers.
#[repr(transparent)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct Float(f64);

// =============================================================================
// Implementations
// =============================================================================

impl Float {
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Self {
        Self(value.into())
    }
}

impl Deref for Float {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Ops: (Float, Float)
impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Rem for Float {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

// Ops: Float, f64
impl Add<f64> for Float {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl Sub<f64> for Float {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}
impl Mul<f64> for Float {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Div<f64> for Float {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl Rem<f64> for Float {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        Self(self.0 % rhs)
    }
}

// Ops: f64, Float
impl Add<Float> for f64 {
    type Output = Float;

    fn add(self, rhs: Float) -> Self::Output {
        Float::new(self + rhs.0)
    }
}
impl Sub<Float> for f64 {
    type Output = Float;

    fn sub(self, rhs: Float) -> Self::Output {
        Float::new(self - rhs.0)
    }
}
impl Mul<Float> for f64 {
    type Output = Float;

    fn mul(self, rhs: Float) -> Self::Output {
        Float::new(self * rhs.0)
    }
}
impl Div<Float> for f64 {
    type Output = Float;

    fn div(self, rhs: Float) -> Self::Output {
        Float::new(self / rhs.0)
    }
}
impl Rem<Float> for f64 {
    type Output = Float;

    fn rem(self, rhs: Float) -> Self::Output {
        Float::new(self % rhs.0)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }
        self.0 == other.0
    }
}
impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }
        self.0 == *other
    }
}
impl PartialEq<Float> for f64 {
    fn eq(&self, other: &Float) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }
        *self == other.0
    }
}

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<f64> for Float {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&Self::new(*other)))
    }
}
impl PartialOrd<Float> for f64 {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        match (self.is_nan(), other.is_nan()) {
            (true, true) => Some(std::cmp::Ordering::Equal),
            (true, false) => Some(std::cmp::Ordering::Greater),
            (false, true) => Some(std::cmp::Ordering::Less),
            (false, false) => self.partial_cmp(&other.0),
        }
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.is_nan(), other.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.0
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp, clippy::cognitive_complexity)]

    use super::Float;

    #[test]
    fn ops() {
        let float_pos = Float::new(123.4);
        let float_neg = Float::new(-1.12);
        let float_zero = Float::new(0.0);
        let float_pos_inf = Float::new(f64::INFINITY);
        let float_neg_inf = Float::new(f64::NEG_INFINITY);
        let float_nan = Float::new(f64::NAN);

        assert_eq!(float_pos + float_neg, 122.28);
        assert_eq!(float_pos + *float_neg, 122.28);
        assert_eq!(*float_pos + float_neg, 122.28);

        assert_eq!(float_nan + float_pos, float_nan);
        assert_eq!(float_pos + float_nan, float_nan);
        assert_eq!(float_pos_inf + float_pos, float_pos_inf);
        assert_eq!(float_pos + float_pos_inf, float_pos_inf);
        assert_eq!(float_neg_inf + float_pos, float_neg_inf);
        assert_eq!(float_pos + float_neg_inf, float_neg_inf);

        assert_eq!(float_nan + float_pos, float_nan);
        assert_eq!(float_pos + float_nan, float_nan);
        assert_eq!(float_pos_inf + float_pos, float_pos_inf);
        assert_eq!(float_pos + float_pos_inf, float_pos_inf);
        assert_eq!(float_neg_inf + float_pos, float_neg_inf);
        assert_eq!(float_pos + float_neg_inf, float_neg_inf);

        assert_eq!(float_nan - float_pos, float_nan);
        assert_eq!(float_pos - float_nan, float_nan);
        assert_eq!(float_pos_inf - float_pos, float_pos_inf);
        assert_eq!(float_pos - float_pos_inf, float_neg_inf);
        assert_eq!(float_neg_inf - float_pos, float_neg_inf);
        assert_eq!(float_pos - float_neg_inf, float_pos_inf);

        assert_eq!(float_nan * float_pos, float_nan);
        assert_eq!(float_pos * float_nan, float_nan);
        assert_eq!(float_pos_inf * float_pos, float_pos_inf);
        assert_eq!(float_pos * float_pos_inf, float_pos_inf);
        assert_eq!(float_neg_inf * float_pos, float_neg_inf);
        assert_eq!(float_pos * float_neg_inf, float_neg_inf);

        assert_eq!(float_nan / float_pos, float_nan);
        assert_eq!(float_pos / float_nan, float_nan);
        assert_eq!(float_pos_inf / float_pos, float_pos_inf);
        assert_eq!(float_pos / float_pos_inf, float_zero);
        assert_eq!(float_neg_inf / float_pos, float_neg_inf);
        assert_eq!(float_pos / float_neg_inf, float_zero);
    }

    #[test]
    fn equality() {
        let float_pos = Float::new(123.4);
        let float_neg = Float::new(-1.12);
        let float_zero = Float::new(0.0);
        let float_pos_inf = Float::new(f64::INFINITY);
        let float_neg_inf = Float::new(f64::NEG_INFINITY);
        let float_nan = Float::new(f64::NAN);

        // Float, Float
        assert_eq!(float_pos, float_pos);
        assert_eq!(float_neg, float_neg);
        assert_eq!(float_zero, float_zero);
        assert_eq!(float_pos_inf, float_pos_inf);
        assert_eq!(float_neg_inf, float_neg_inf);
        assert_eq!(float_nan, float_nan);

        // Float, f64
        assert_eq!(float_pos, 123.4);
        assert_eq!(float_neg, -1.12);
        assert_eq!(float_zero, 0.0);
        assert_eq!(float_pos_inf, f64::INFINITY);
        assert_eq!(float_neg_inf, f64::NEG_INFINITY);
        assert_eq!(float_nan, f64::NAN);

        // f64, Float
        assert_eq!(123.4, float_pos);
        assert_eq!(-1.12, float_neg);
        assert_eq!(0.0, float_zero);
        assert_eq!(f64::INFINITY, float_pos_inf);
        assert_eq!(f64::NEG_INFINITY, float_neg_inf);
        assert_eq!(f64::NAN, float_nan);
    }

    #[test]
    fn ordering() {
        let float_neg_inf = Float::new(f64::NEG_INFINITY);
        let float_neg = Float::new(-1.12);
        let float_zero = Float::new(0.0);
        let float_pos = Float::new(12.34);
        let float_pos_inf = Float::new(f64::INFINITY);
        let float_nan = Float::new(f64::NAN);

        // Float, Float
        assert!(float_neg_inf < float_neg);
        assert!(float_neg < float_zero);
        assert!(float_zero < float_pos);
        assert!(float_pos < float_pos_inf);
        assert!(float_pos_inf < float_nan);

        // Float, f64
        assert!(float_neg_inf < *float_neg);
        assert!(float_neg < *float_zero);
        assert!(float_zero < *float_pos);
        assert!(float_pos < *float_pos_inf);
        assert!(float_pos_inf < *float_nan);

        // f64, Float
        assert!(*float_neg_inf < float_neg);
        assert!(*float_neg < float_zero);
        assert!(*float_zero < float_pos);
        assert!(*float_pos < float_pos_inf);
        assert!(*float_pos_inf < float_nan);
    }
}
