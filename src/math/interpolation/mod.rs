use crate::math::FloatScalable;
use std::fmt::Debug;

/// Trait describing interpolation index requirements.
pub trait InterpolationIndex: Into<f64> + PartialOrd + Debug + Copy + Clone {}
impl<T> InterpolationIndex for T where T: Into<f64> + PartialOrd + Debug + Copy + Clone {}

pub trait Interpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    /// Add a point to the interpolator.
    fn add_point(&mut self, point: (I, V)) -> Result<(), InterpolationError<I, V>>;

    /// Add points to the interpolator.
    fn add_points(&mut self, points: &[(I, V)]);

    /// Remove a point from the interpolator.
    fn remove_point(&mut self, point: I) -> Option<V>;

    /// Remove points from the interpolator.
    fn remove_points(&mut self, points: &[I]) -> &[Option<V>];

    /// Interpolate at a point..
    fn interpolate(&self, point: I) -> InterpolationResult<V>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterpolationResult<V>
where
    V: FloatScalable,
{
    ExistingValue(V),
    InterpolatedValue(V),
    OutOfRange,
}

#[non_exhaustive]
#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterpolationError<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    #[error("Out of range.")]
    OutOfRange,
    #[error("A point already exists at x={0}: ({0}, {1})")]
    AlreadyExists(I, V),
}
// impl From<UninitializedFieldError> for InterpolationError {
//     fn from(_ufe: UninitializedFieldError) -> Self {
//         Self::NoPoints
//     }
// }

// mod linear_interpolator;
// pub use linear_interpolator::LinearInterpolator;
use thiserror::Error;
