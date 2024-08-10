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

/// Linear Interpolator.
///
/// For more details on the mathematics of linear interpolation, see [Wikipedia](https://en.wikipedia.org/wiki/Linear_interpolation).
#[derive(Debug, Clone)]
pub struct LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    points: BTreeMap<I, V>,
}

impl<I, V> LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    /// Create a new `LinearInterpolator`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            points: BTreeMap::new(),
        }
    }

    /// Create a new `LinearInterpolator` from a set of points.
    pub fn new_from_points(points: Vec<(I, V)>) -> Self {
        let mut ret = Self {
            points: BTreeMap::new(),
        };
        ret.add_points(points);
        ret
    }
}

impl<I, V> Default for LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    /// Create a new `LinearInterpolator`.
    fn default() -> Self {
        Self::new()
    }
}

//  ------------------------------------------------------------------------------------------------
//  Trait implementations.
//  ------------------------------------------------------------------------------------------------

impl<I, V> Interpolator<I, V> for LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatScalable,
{
    /// Adds a point to the LinearInterpolator.
    ///
    /// If the interpolator did not have this index present, `None` is returned.
    /// If the interpolator already had a point at this index, the value is updated
    /// and the old value is returned.
    fn add_point(&mut self, point: (I, V)) -> Option<V> {
        self.points.insert(point.0, point.1)
    }

    /// Adds multiple points to the LinearInterpolator.
    ///
    /// Each insertion returns a value as described in [LinearInterpolator::add_point], and is returned as a `Vec`.
    fn add_points(&mut self, points: Vec<(I, V)>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.add_point(pt)).collect()
    }

    /// Removes a point from the LinearInterpolator.
    ///
    /// If the interpolator contained the point, it returns the value at the point.
    /// If the key was not in the interpolator, `None` is returned.
    fn remove_point(&mut self, point: I) -> Option<V> {
        self.points.remove(&point)
    }

    /// Removes multiple points from the LinearInterpolator.
    ///
    /// Each deletion returns a value as described by [LinearInterpolator::remove_point], and is returned as a `Vec`.
    fn remove_points(&mut self, points: Vec<I>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.remove_point(pt)).collect()
    }

    /// Interpolate at a point.
    ///
    /// Returns [InterpolationResult::OutOfRange] if the point is out of the range of the interpolator.
    /// Returns [InterpolationResult::ExistingValue] if the point was one of the given points (no interpolation necessary).
    /// Returns [InterpolationResult::InterpolatedValue] is the point required interpolation.
    /// Returns [InterpolationResult::NoPoints] if the interpolator does not have any points.
    fn interpolate(&self, point: I) -> InterpolationResult<V> {
        // Check if the interpolator is empty.
        if self.range().is_none() {
            return InterpolationResult::NoPoints;
        }

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

    /// Returns the effective interpolation range of the LinearInterpolator.
    fn range(&self) -> Option<(I, I)> {
        if self.points.is_empty() {
            None
        } else {
            Some((
                // Safe to unwrap here, because we checked to make sure there were points in the interpolator.
                *self.points.first_key_value().unwrap().0,
                *self.points.last_key_value().unwrap().0,
            ))
        }
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

        // Test add points
        let mut interpolator = LinearInterpolator::new_from_points(data);
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
            interpolator.range().unwrap(),
            (OrderedFloat(1.0), OrderedFloat(4.0))
        );

        // Test interpolation.
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

        // Test remove points.
        let _ = interpolator.remove_points([OrderedFloat(4.0), OrderedFloat(1.0)].to_vec());
        assert_eq!(
            interpolator.interpolate(OrderedFloat(1.0)),
            InterpolationResult::OutOfRange
        );
    }
}
