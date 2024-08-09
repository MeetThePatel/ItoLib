use crate::math::FloatScalable;
use std::fmt::Debug;

/// Trait describing interpolation index requirements.
pub trait InterpolationIndex:
    Into<OrderedFloat<f64>> + PartialOrd + Ord + Debug + Copy + Clone
{
}
impl<T> InterpolationIndex for T where
    T: Into<OrderedFloat<f64>> + PartialOrd + Ord + Debug + Copy + Clone
{
}

pub trait Interpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    /// Add a point to the interpolator.
    fn add_point(&mut self, point: (I, V)) -> Option<V>;

    /// Add points to the interpolator.
    fn add_points(&mut self, points: Vec<(I, V)>) -> Vec<Option<V>>;

    /// Remove a point from the interpolator.
    fn remove_point(&mut self, point: I) -> Option<V>;

    /// Remove points from the interpolator.
    fn remove_points(&mut self, points: Vec<I>) -> Vec<Option<V>>;

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

mod linear_interpolator;
pub use linear_interpolator::LinearInterpolator;
use ordered_float::OrderedFloat;
use thiserror::Error;
