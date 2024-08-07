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
    fn new() -> Self
    where
        Self: Sized;

    fn set_xs(&mut self, xs: &[IndexType]);

    fn set_ys(&mut self, ys: &[ValueType]);

    fn add_point(&mut self, point: (IndexType, ValueType));

    fn fit(&mut self) -> Result<(), InterpolationError>;

    fn get_status(&self) -> InterpolatorStatus;

    fn range(&self) -> Result<(IndexType, IndexType), InterpolationError>;

    fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum InterpolatorStatus {
    #[default]
    Unfitted = 0,
    Fitted = 1,
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterpolationError {
    UnequalLength,
    NoPoints,
    OutOfRange,
}

mod linear_interpolator;
pub use linear_interpolator::LinearInterpolator;
