use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use hifitime::{Duration as hifidur, Unit};

/// This is a thin wrapper around `HifiTime`'s Duration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Duration(hifidur);

impl Duration {
    #[must_use]
    pub const fn new_from_duration(dur: &hifidur) -> Self {
        Self(*dur)
    }

    #[must_use]
    pub fn new_from_millis(millis: f64) -> Self {
        Self(hifidur::from_f64(millis, Unit::Millisecond))
    }
    #[must_use]
    pub fn new_from_seconds(seconds: f64) -> Self {
        Self(hifidur::from_f64(seconds, Unit::Second))
    }
    #[must_use]
    pub fn new_from_minutes(minutes: f64) -> Self {
        Self(hifidur::from_f64(minutes, Unit::Minute))
    }
    #[must_use]
    pub fn new_from_hours(hours: f64) -> Self {
        Self(hifidur::from_f64(hours, Unit::Hour))
    }
    #[must_use]
    pub fn new_from_days(days: f64) -> Self {
        Self(hifidur::from_f64(days, Unit::Day))
    }
}

impl Deref for Duration {
    type Target = hifidur;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Duration + Duration
impl Add<Self> for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
/// Duration + Unit
impl Add<Unit> for Duration {
    type Output = Self;

    fn add(self, rhs: Unit) -> Self::Output {
        Self(self.0 + rhs)
    }
}
/// Unit + Duration
impl Add<Duration> for Unit {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration(rhs.0 + self)
    }
}

/// Duration - Duration
impl Sub<Self> for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
/// Duration - Unit
impl Sub<Unit> for Duration {
    type Output = Self;

    fn sub(self, rhs: Unit) -> Self::Output {
        Self(self.0 - rhs)
    }
}

/// Duration * `Into<f64>`
impl<T> Mul<T> for Duration
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.into())
    }
}
/// f64 * Duration
impl Mul<Duration> for f64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        Duration(rhs.0 * self)
    }
}
/// i64 * Duration
impl Mul<Duration> for i64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        Duration(rhs.0 * self)
    }
}

/// Duration / Duration
impl Div<Self> for Duration {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_unit(Unit::Millisecond) / rhs.to_unit(Unit::Millisecond)
    }
}

/// Duration / f64
impl Div<f64> for Duration {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
/// Duration / i64
impl Div<i64> for Duration {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use hifitime::Unit;

    use crate::Duration;

    #[test]
    fn test_constructors() {
        assert!(Duration::new_from_days(1.0) > Duration::new_from_hours(1.0));
        assert_eq!(
            Duration::new_from_duration(&hifitime::Duration::from_milliseconds(1.0)),
            Duration::new_from_millis(1.0)
        );
        assert_ne!(Duration::new_from_seconds(1.0), Duration::new_from_minutes(1.0));
    }

    #[test]
    fn test_deref() {
        let dur = Duration::new_from_minutes(1.0);
        assert_ne!((*dur).to_parts(), (1, 1));

        let mut dur2 = Duration::new_from_minutes(2.0);
        *dur2 = *Duration::new_from_minutes(3.0);

        dur2 = Duration::new_from_minutes(1.0);
        assert_eq!(dur, dur2);
    }

    #[test]
    fn test_ops() {
        let dur = Duration::new_from_minutes(1.0);

        assert_eq!(dur - dur, Duration::new_from_minutes(0.0));

        assert_eq!(dur + Unit::Minute, dur + dur);
        assert_eq!(Unit::Minute + dur, dur + dur);
        assert_eq!(dur - Unit::Minute, Duration::new_from_minutes(0.0));

        assert_eq!(dur * 2.0, Duration::new_from_minutes(2.0));
        assert_eq!(2.0 * dur, Duration::new_from_minutes(2.0));

        assert_eq!(dur * 2, Duration::new_from_minutes(2.0));
        assert_eq!(2 * dur, Duration::new_from_minutes(2.0));

        assert_eq!(dur / 2.0, Duration::new_from_minutes(0.5));
        assert_eq!(dur / 2, Duration::new_from_minutes(0.5));

        assert_eq!(dur / Duration::new_from_millis(500.0), 120.0);
    }
}
