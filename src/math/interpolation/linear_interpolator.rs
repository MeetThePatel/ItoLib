use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::math::{
    interpolation::{InterpolationIndex, Interpolator},
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
    fn add_point(&mut self, point: (I, V)) -> Option<V> {
        self.points.insert(point.0, point.1)
    }

    fn add_points(&mut self, points: Vec<(I, V)>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.add_point(pt)).collect()
    }

    fn remove_point(&mut self, point: I) -> Option<V> {
        self.points.remove(&point)
    }

    fn remove_points(&mut self, points: Vec<I>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.remove_point(pt)).collect()
    }

    fn interpolate(&self, point: I) -> InterpolationResult<V> {
        // Check if point is in the range of the interpolator.
        let x_min = *self.points.first_key_value().unwrap().0;
        let x_max = *self.points.last_key_value().unwrap().0;

        if point < x_min || point > x_max {
            return InterpolationResult::OutOfRange;
        }

        // Check if the point is already provided. If so, no need to interpolate.
        if let Some(y) = self.points.get(&point) {
            return InterpolationResult::ExistingValue(y.clone());
        }

        let mut iter = self.points.range(..=point);
        let (x_l, y_l) = iter.next_back().unwrap();

        let mut iter = self.points.range(point..);
        let (x_r, y_r) = iter.next().unwrap();

        let x_l: OrderedFloat<f64> = (*x_l).into();
        let x_r: OrderedFloat<f64> = (*x_r).into();
        let point: OrderedFloat<f64> = point.into();

        let val: V = y_l.clone() + (y_r.clone() - y_l.clone()) * (point - x_l) / (x_r - x_l);

        InterpolationResult::InterpolatedValue(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolator() {
        let data: Vec<(OrderedFloat<f64>, OrderedFloat<f64>)> = vec![
            (OrderedFloat(1.0), OrderedFloat(10.0)),
            (OrderedFloat(2.0), OrderedFloat(20.0)),
            (OrderedFloat(3.0), OrderedFloat(30.0)),
        ];

        let mut interpolator = LinearInterpolator::default();

        // Test add points
        interpolator.add_points(data);
        assert_eq!(interpolator.points.len(), 3);

        assert!(interpolator
            .add_point((OrderedFloat(4.0), OrderedFloat(40.0)))
            .is_none());
        assert_eq!(interpolator.points.len(), 4);

        assert!(interpolator
            .add_point((OrderedFloat(4.0), OrderedFloat(50.0)))
            .is_some());
        assert_eq!(interpolator.points.len(), 4);

        assert_eq!(
            interpolator.interpolate(OrderedFloat(2.0)),
            InterpolationResult::ExistingValue(OrderedFloat(20.0))
        );
        assert_eq!(
            interpolator.interpolate(OrderedFloat(1.5)),
            InterpolationResult::InterpolatedValue(OrderedFloat(15.0))
        );
        assert_eq!(
            interpolator.interpolate(OrderedFloat(6.0)),
            InterpolationResult::OutOfRange
        );
    }
}
