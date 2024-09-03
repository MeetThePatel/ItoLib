use itolib_types::Float;

use crate::interpolation::{InterpolationIndex, InterpolationResult, Interpolator};
use crate::FloatLike;

//  ------------------------------------------------------------------------------------------------
//  Definition.
//  ------------------------------------------------------------------------------------------------

// TODO: Swap from BTreeMap -> Vec. Doesn't seem like insertions will be priority, and it adds the
// requirement of Ord, while Vec implementation only requires PartialOrd.

/// Linear Interpolator.
///
/// For more details on the mathematics of linear interpolation, see
/// [Wikipedia](https://en.wikipedia.org/wiki/Linear_interpolation).
#[derive(Debug)]
#[derive(Clone)]
pub struct LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatLike,
{
    points: Vec<(I, V)>,
}

impl<I, V> LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatLike,
{
    /// Create a new `LinearInterpolator`.
    #[must_use]
    pub const fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Create a new `LinearInterpolator` from a set of points.
    #[must_use]
    pub fn new_from_points(points: Vec<(I, V)>) -> Self {
        let mut ret = Self { points: Vec::new() };
        ret.add_points(points);
        ret
    }
}

impl<I, V> Default for LinearInterpolator<I, V>
where
    I: InterpolationIndex,
    V: FloatLike,
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
    V: FloatLike,
{
    /// Adds a point to the `LinearInterpolator`.
    ///
    /// If the interpolator did not have a point at this index present, `None` is returned.
    /// If the interpolator already had a point at this index, the value is updated and the old
    /// value is returned.
    fn add_point(&mut self, point: (I, V)) -> Option<V> {
        match self.points.binary_search_by(|(k, _)| k.cmp(&point.0)) {
            Ok(pos) => {
                let old_val = std::mem::replace(&mut self.points[pos].1, point.1);
                Some(old_val)
            }
            Err(pos) => {
                self.points.insert(pos, point);
                None
            }
        }
    }

    /// Adds multiple points to the `LinearInterpolator`.
    ///
    /// Each insertion returns a value as described in [`LinearInterpolator::add_point`], and is
    /// returned as a `Vec`.
    fn add_points(&mut self, points: impl IntoIterator<Item = (I, V)>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.add_point(pt)).collect()
    }

    /// Removes a point from the `LinearInterpolator`.
    ///
    /// If the interpolator contained the point, it returns the value at the point.
    /// If the key was not in the interpolator, `None` is returned.
    fn remove_point(&mut self, point: I) -> Option<V> {
        match self.points.binary_search_by(|(k, _)| k.cmp(&point)) {
            Ok(pos) => {
                let (_, value) = self.points.remove(pos);
                Some(value)
            }
            Err(_) => None,
        }
    }

    /// Removes multiple points from the `LinearInterpolator`.
    ///
    /// Each deletion returns a value as described by [`LinearInterpolator::remove_point`], and is
    /// returned as a `Vec`.
    fn remove_points(&mut self, points: impl IntoIterator<Item = I>) -> Vec<Option<V>> {
        points.into_iter().map(|pt| self.remove_point(pt)).collect()
    }

    /// Interpolate at a point.
    ///
    /// Returns [`InterpolationResult::OutOfRange`] if the point is out of the range of the
    /// interpolator. Returns [`InterpolationResult::ExistingValue`] if the point was one of the
    /// given points (no interpolation necessary).
    /// Returns [`InterpolationResult::InterpolatedValue`] is the point required interpolation.
    /// Returns [`InterpolationResult::NoPoints`] if the interpolator does not have any points.
    fn interpolate(&self, point: impl Into<I>) -> InterpolationResult<V> {
        let point: I = point.into();

        let range = self.range();

        if range.is_none() {
            return InterpolationResult::NoPoints;
        }
        let range = range.unwrap();

        // Check if point is in the range of the interpolator.
        if point < range.0 || point > range.1 {
            return InterpolationResult::OutOfRange;
        }

        // If point already exists, no need to interpolate.
        let pos = self.points.binary_search_by(|(k, _)| k.cmp(&point));

        if let Ok(idx) = pos {
            return InterpolationResult::ExistingValue(self.points[idx].1.clone());
        }
        let pos = pos.err().unwrap();

        let (x_l, y_l) = &self.points[pos - 1];
        let (x_r, y_r) = &self.points[pos];

        let x_l: Float = (x_l).clone().into();
        let x_r: Float = (x_r).clone().into();
        let point: Float = point.into();

        let val: V = y_l.clone() + (y_r.clone() - y_l.clone()) * (point - x_l) / (x_r - x_l);

        InterpolationResult::InterpolatedValue(val)
    }

    /// Returns the effective interpolation range of the `LinearInterpolator`.
    fn range(&self) -> Option<(I, I)> {
        if self.points.is_empty() {
            None
        } else {
            Some((
                // Safe to unwrap here, because we checked to make sure there were points in the
                // interpolator.
                self.points.first().unwrap().0.clone(),
                self.points.last().unwrap().0.clone(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use itolib_types::Float;

    use super::*;

    #[test]
    fn test_linear_interpolator() {
        let interpolator: LinearInterpolator<Float, Float> = LinearInterpolator::new();
        assert_eq!(interpolator.interpolate(1.0), InterpolationResult::NoPoints);

        let interpolator: LinearInterpolator<Float, Float> = LinearInterpolator::default();
        assert_eq!(interpolator.range(), None);

        let data: Vec<(f64, f64)> = vec![(1.0, 10.0), (2.0, 20.0), (3.0, 30.0)];
        let data: Vec<(Float, Float)> =
            data.into_iter().map(|(i, j)| (i.into(), j.into())).collect();

        // Test add points
        let mut interpolator: LinearInterpolator<Float, Float> =
            LinearInterpolator::new_from_points(data);
        assert_eq!(interpolator.points.len(), 3);

        assert!(interpolator.add_point((Float::new(4.0), Float::new(40.0))).is_none());
        assert_eq!(interpolator.points.len(), 4);

        assert!(interpolator.add_point((Float::new(4.0), Float::new(50.0))).is_some());
        assert_eq!(interpolator.points.len(), 4);

        assert_eq!(interpolator.range().unwrap(), (Float::new(1.0), Float::new(4.0)));

        // Test interpolation.
        assert_eq!(
            interpolator.interpolate(2.0),
            InterpolationResult::ExistingValue(Float::new(20.0))
        );
        assert_eq!(
            interpolator.interpolate(1.5),
            InterpolationResult::InterpolatedValue(Float::new(15.0))
        );
        assert_eq!(interpolator.interpolate(6.0), InterpolationResult::OutOfRange);

        // Test remove points.
        let _ = interpolator.remove_points([Float::new(4.0), Float::new(1.0)].to_vec());
        assert_eq!(interpolator.interpolate(1.0), InterpolationResult::OutOfRange);
    }
}
