use std::fmt::Debug;

use itolib_types::Float;

use crate::FloatLike;

/// Trait describing interpolation index requirements.
#[allow(clippy::module_name_repetitions)]
pub trait InterpolationIndex: Into<Float> + Ord + Clone {}
impl<T> InterpolationIndex for T where T: Into<Float> + Ord + Clone {}

pub trait Interpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatLike,
{
    /// Add a point to the interpolator.
    fn add_point(&mut self, point: (I, V)) -> Option<V>;

    /// Add points to the interpolator.
    fn add_points(&mut self, points: impl IntoIterator<Item = (I, V)>) -> Vec<Option<V>>;

    /// Remove a point from the interpolator.
    fn remove_point(&mut self, point: I) -> Option<V>;

    /// Remove points from the interpolator.
    fn remove_points(&mut self, points: impl IntoIterator<Item = I>) -> Vec<Option<V>>;

    /// Interpolate at a point.
    fn interpolate(&self, point: impl Into<I>) -> InterpolationResult<V>;

    /// Return range of interpolation.
    fn range(&self) -> Option<(I, I)>;
}

#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterpolationResult<V>
where
    V: FloatLike,
{
    ExistingValue(V),
    InterpolatedValue(V),
    OutOfRange,
    NoPoints,
}
