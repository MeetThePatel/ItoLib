use crate::math::{
    interpolation::{InterpolationError, InterpolationIndex, Interpolator, InterpolatorStatus},
    FloatScalable,
};

#[derive(Debug, Default)]
pub struct LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    pub xs: Vec<IndexType>,
    pub ys: Vec<ValueType>,
    pub status: InterpolatorStatus,
}

impl<IndexType, ValueType> LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    pub fn new() -> Self {
        Self {
            xs: Vec::new(),
            ys: Vec::new(),
            status: InterpolatorStatus::Unfitted,
        }
    }

    pub fn new_with_pairs(xs: &[IndexType], ys: &[ValueType]) -> Result<Self, InterpolationError> {
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
            status: InterpolatorStatus::Unfitted,
        })
    }
}

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: FloatScalable,
{
    fn new() -> Self {
        Self {
            xs: Vec::new(),
            ys: Vec::new(),
            status: InterpolatorStatus::Unfitted,
        }
    }

    fn set_xs(&mut self, xs: &[IndexType]) {
        self.xs = xs.to_vec();
    }

    fn set_ys(&mut self, ys: &[ValueType]) {
        self.ys = ys.to_vec();
    }

    fn add_point(&mut self, point: (IndexType, ValueType)) {
        let idx = self.xs.partition_point(|&x| x < point.0);
        self.xs.insert(idx, point.0);
        self.ys.insert(idx, point.1);
    }

    fn fit(&mut self) -> Result<(), InterpolationError> {
        self.status = InterpolatorStatus::Fitted;
        Ok(())
    }

    fn get_status(&self) -> InterpolatorStatus {
        self.status
    }

    fn range(&self) -> Result<(IndexType, IndexType), InterpolationError> {
        if self.xs.is_empty() {
            Err(InterpolationError::NoPoints)
        } else {
            // Safe to unwrap, because Vec::first and Vec::last only return None if the vec is empty
            Ok((*self.xs.first().unwrap(), *self.xs.last().unwrap()))
        }
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

#[cfg(test)]
mod tests {
    use crate::{
        math::interpolation::*,
        money::{currency::USD, Money},
    };

    #[test]
    fn test_new_interpolator() {
        let xs = [1.0, 3.0, 2.0];
        let ys = [10.0, 30.0, 20.0];

        let interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();

        assert_eq!(interpolator.xs, vec![1.0, 2.0, 3.0]);
        assert_eq!(interpolator.ys, vec![10.0, 20.0, 30.0]);
        assert_eq!(interpolator.status, InterpolatorStatus::Unfitted);
    }

    #[test]
    fn test_fit() {
        let xs = [1.0, 2.0];
        let ys = [10.0, 20.0];

        let mut interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();
        interpolator.fit().unwrap();

        assert_eq!(interpolator.status, InterpolatorStatus::Fitted);
    }

    #[test]
    fn test_add_point() {
        let mut interpolator = LinearInterpolator::new_with_pairs(&[1.0], &[10.0]).unwrap();

        interpolator.add_point((2.0, 20.0));
        assert_eq!(interpolator.xs, vec![1.0, 2.0]);
        assert_eq!(interpolator.ys, vec![10.0, 20.0]);
    }
    #[test]
    fn test_interpolate_f64() {
        let xs = [1.0, 2.0];
        let ys = [10.0, 20.0];

        let interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();
        let result = interpolator.interpolate(1.5).unwrap();

        assert_approx_equal_f64!(result, 15.0);
    }

    #[test]
    fn test_existing_point() {
        let xs = [1.0, 2.0];
        let ys = [10.0, 20.0];

        let interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();
        let result = interpolator.interpolate(1.0).unwrap();

        assert_approx_equal_f64!(result, 10.0);
    }

    #[test]
    fn test_interpolate_money() {
        let xs = [1.0, 2.0];
        let ys = vec![Money::new(10.0), Money::new(20.0)];

        let interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();
        let result = interpolator.interpolate(1.5).unwrap();

        assert_approx_equal_Money!(result, Money::<USD>::new(15.0));
    }
    #[test]
    fn test_interpolate_out_of_range() {
        let xs = [1.0, 2.0];
        let ys = [10.0, 20.0];

        let interpolator = LinearInterpolator::new_with_pairs(&xs, &ys).unwrap();
        let result = interpolator.interpolate(3.0);

        assert_eq!(result, Err(InterpolationError::OutOfRange));
    }

    #[test]
    fn test_interpolate_no_points() {
        let result = LinearInterpolator::<f64, Money<USD>>::new_with_pairs(&[], &[])
            .unwrap()
            .range()
            .unwrap_err();

        assert_eq!(result, InterpolationError::NoPoints);
    }

    #[test]
    fn test_uneven_points() {
        let result =
            LinearInterpolator::<f64, f64>::new_with_pairs(&[1.0, 2.0], &[1.0]).unwrap_err();

        assert_eq!(result, InterpolationError::UnequalLength);
    }
}
