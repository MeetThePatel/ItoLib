use crate::math::{
    interpolation::{InterpolationError, InterpolationIndex},
    FloatScalable,
};

use super::Interpolator;

pub struct LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    pub xs: Vec<IndexType>,
    pub ys: Vec<ValueType>,
    pub status: LinearInterpolatorStatus,
}

impl<IndexType, ValueType> LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    pub fn new(xs: &[IndexType], ys: &[ValueType]) -> Result<Self, InterpolationError> {
        if xs.len() != ys.len() {
            return Err(InterpolationError::UnequalLength);
        }

        let mut tmp = xs.iter().zip(ys).collect::<Vec<_>>();

        // Safe to unwrap, because f64::partial_cmp fails if one of the values is NaN or INFINITY.
        tmp.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        let (xs, ys) = tmp.into_iter().unzip();

        Ok(Self {
            xs,
            ys,
            status: LinearInterpolatorStatus::Unfitted,
        })
    }
}

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    fn fit(&mut self) -> Result<(), InterpolationError> {
        self.status = LinearInterpolatorStatus::Fitted;
        Ok(())
    }

    fn range(&self) -> Result<(IndexType, IndexType), InterpolationError> {
        if self.xs.is_empty() {
            Err(InterpolationError::NoPoints)
        } else {
            // Safe to unwrap, because Vec::first and Vec::last only return None if the vec is empty
            Ok((*self.xs.first().unwrap(), *self.xs.last().unwrap()))
        }
    }

    fn add_point(&mut self, point: (IndexType, ValueType)) {
        let idx = self.xs.partition_point(|&x| x < point.0);
        self.xs.insert(idx, point.0);
        self.ys.insert(idx, point.1);
    }

    fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError> {
        // Check if point is in the range of the interpolator.
        let (x_min, x_max) = self.range()?;
        if point < x_min || point > x_max {
            return Err(InterpolationError::OutOfRange);
        }

        // Check if the point is already provided. If so, no need to interpolate.
        if let Ok(idx) = self
            .xs
            .binary_search_by(|x| x.partial_cmp(&point).expect("Cannot compare values."))
        {
            return Ok(self.ys[idx]);
        }

        let idx_r = self.xs.partition_point(|&x| x < point);
        let idx_l = idx_r - 1;

        let x_l: f64 = self.xs[idx_l].into();
        let x_r: f64 = self.xs[idx_r].into();
        let point: f64 = point.into();

        let y_l = self.ys[idx_l];
        let y_r = self.ys[idx_r];

        let val = y_l + (y_r - y_l) * (point - x_l) / (x_r - x_l);

        Ok(val)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LinearInterpolatorStatus {
    Unfitted = 0,
    Fitted = 1,
}

// TODO: Write tests.
