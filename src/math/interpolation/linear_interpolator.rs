use std::collections::BTreeMap;

use crate::math::{
    interpolation::{InterpolationError, InterpolationIndex, Interpolator},
    FloatScalable,
};

use super::InterpolationResult;

//  ------------------------------------------------------------------------------------------------
//  Definition.
//  ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    pub points: BTreeMap<I, V>,
}

//  ------------------------------------------------------------------------------------------------
//  Trait implementations.
//  ------------------------------------------------------------------------------------------------

impl<I, V> Interpolator<I, V> for LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    fn add_point(&mut self, point: (I, V)) -> Result<(), InterpolationError<I, V>> {
        self.points.insert(point);
        Ok(())
    }

    fn add_points(&mut self, points: &[(I, V)]) {
        // Push new points to end.
        self.points.extend_from_slice(points);
        // Sort all points.
        self.points
            .sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());
    }

    fn remove_point(&mut self, point: I) -> Option<V> {
        match self
            .points
            .binary_search_by(|x| x.0.partial_cmp(&point).unwrap())
        {
            // If the point exists, remove it and return the y-value.
            Ok(index) => Some(self.points.remove(index).1),
            // If a point does not exist, do nothing.
            Err(_) => None,
        }
    }

    fn remove_points(&mut self, points: &[I]) -> &[Option<V>] {
        todo!()
    }

    fn interpolate(&self, point: I) -> InterpolationResult<V> {
        // Check if point is in the range of the interpolator.
        let x_min = self.points.first().unwrap().0;
        let x_max = self.points.last().unwrap().0;

        if point < x_min || point > x_max {
            return InterpolationResult::OutOfRange;
        }

        // Check if the point is already provided. If so, no need to interpolate.
        let idx_r = match self
            .points
            .binary_search_by(|probe| probe.0.partial_cmp(&point).unwrap())
        {
            Ok(index) => return InterpolationResult::ExistingValue(self.points[index].1),
            Err(index) => index,
        };

        let idx_l = idx_r - 1;

        let (x_l, y_l) = self.points[idx_l];
        let (x_r, y_r) = self.points[idx_r];

        let x_l: f64 = x_l.into();
        let x_r: f64 = x_r.into();
        let point: f64 = point.into();

        let val = y_l + (y_r - y_l) * (point - x_l) / (x_r - x_l);

        InterpolationResult::InterpolatedValue(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::interpolation::*,
        money::{currency::USD, Money},
    };

    #[test]
    fn test_linear_interpolator() {
        let data = [(1.0, 10.0), (2.0, 20.0), (3.0, 30.0)];

        let mut interpolator = LinearInterpolator::default();

        // Test add points
        interpolator.add_points(&data);
        assert_eq!(interpolator.points.len(), 3);

        assert!(interpolator.add_point((4.0, 40.0)).is_ok());
        assert_eq!(interpolator.points.len(), 4);

        assert_eq!(
            interpolator.add_point((4.0, 50.0)),
            Err(InterpolationError::AlreadyExists(4.0, 40.0))
        );
        assert_eq!(interpolator.points.len(), 4);

        assert_eq!(
            interpolator.interpolate(4.0),
            InterpolationResult::ExistingValue(40.0)
        );
        assert_eq!(
            interpolator.interpolate(3.5),
            InterpolationResult::InterpolatedValue(35.0)
        );
        assert_eq!(
            interpolator.interpolate(6.0),
            InterpolationResult::OutOfRange
        );
    }

    // #[test]
    // fn test_new_interpolator() {
    //     let xs = vec![1.0, 3.0, 2.0];
    //     let ys = vec![10.0, 30.0, 20.0];

    //     let interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();

    //     assert_eq!(interpolator.xs, vec![1.0, 3.0, 2.0]);
    //     assert_eq!(interpolator.ys, vec![10.0, 30.0, 20.0]);
    //     assert_eq!(interpolator.status, InterpolatorStatus::Unfitted);
    // }

    // #[test]
    // fn test_fit() {
    //     let xs = vec![1.0, 2.0];
    //     let ys = vec![10.0, 20.0];

    //     let mut interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();
    //     interpolator.fit().unwrap();

    //     assert_eq!(interpolator.status, InterpolatorStatus::Fitted);
    // }

    // #[test]
    // fn test_add_point() {
    //     let xs = vec![1.0];
    //     let ys = vec![10.0];

    //     let mut interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();

    //     interpolator.add_point((2.0, 20.0));
    //     assert_eq!(interpolator.xs, vec![1.0, 2.0]);
    //     assert_eq!(interpolator.ys, vec![10.0, 20.0]);
    // }
    // #[test]
    // fn test_interpolate_f64() {
    //     let xs = vec![1.0, 2.0];
    //     let ys = vec![10.0, 20.0];

    //     let interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();
    //     let result = interpolator.interpolate(1.5).unwrap();

    //     assert_approx_equal_f64!(result, 15.0);
    // }

    // #[test]
    // fn test_existing_point() {
    //     let xs = vec![1.0, 2.0];
    //     let ys = vec![10.0, 20.0];

    //     let interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();
    //     let result = interpolator.interpolate(1.0).unwrap();

    //     assert_approx_equal_f64!(result, 10.0);
    // }

    // #[test]
    // fn test_interpolate_money() {
    //     let xs = vec![1.0, 2.0];
    //     let ys = vec![Money::new(10.0), Money::new(20.0)];

    //     let interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();
    //     let result = interpolator.interpolate(1.5).unwrap();

    //     assert_approx_equal_Money!(result, Money::<USD>::new(15.0));
    // }
    // #[test]
    // fn test_interpolate_out_of_range() {
    //     let xs = vec![1.0, 2.0];
    //     let ys = vec![10.0, 20.0];

    //     let interpolator = LinearInterpolatorBuilder::default()
    //         .xs(xs)
    //         .ys(ys)
    //         .build()
    //         .unwrap();
    //     let result = interpolator.interpolate(3.0);

    //     assert_eq!(result, Err(InterpolationError::OutOfRange));
    // }

    // #[test]
    // fn test_interpolate_no_points() {
    //     let interpolator: LinearInterpolator<f64, f64> =
    //         LinearInterpolatorBuilder::default().build().unwrap();
    //     let result = interpolator.range().unwrap_err();

    //     assert_eq!(result, InterpolationError::NoPoints);
    // }
}
