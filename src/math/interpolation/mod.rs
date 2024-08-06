use crate::math::FloatScalable;
use std::fmt::Debug;

/// Trait describing interpolation index requirements.
pub trait InterpolationIndex: Into<f64> + PartialOrd + Debug + Copy + Clone + Sized {}
impl<T> InterpolationIndex for T where T: Into<f64> + PartialOrd + Debug + Copy + Clone + Sized {}

pub trait Interpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    fn fit(&mut self) -> Result<(), InterpolationError>;

    fn range(&self) -> Result<(IndexType, IndexType), InterpolationError>;

    fn add_point(&mut self, point: (IndexType, ValueType));

    fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError>;
}

#[non_exhaustive]
pub enum InterpolationError {
    UnequalLength,
    NoPoints,
    OutOfRange,
}

mod linear_interpolator;
pub use linear_interpolator::{LinearInterpolator, LinearInterpolatorStatus};
