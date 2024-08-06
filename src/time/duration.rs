use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

pub use hifitime::Unit::*;
use hifitime::{Duration as hifidur, Unit};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Duration(hifidur);

impl Duration {
    pub const fn new_from_duration(dur: &hifidur) -> Self {
        Self(*dur)
    }

    pub fn new_from_millis(millis: f64) -> Self {
        Self(hifidur::from_f64(millis, Millisecond))
    }
    pub fn new_from_seconds(seconds: f64) -> Self {
        Self(hifidur::from_f64(seconds, Second))
    }
    pub fn new_from_minutes(minutes: f64) -> Self {
        Self(hifidur::from_f64(minutes, Minute))
    }
    pub fn new_from_hours(hours: f64) -> Self {
        Self(hifidur::from_f64(hours, Hour))
    }
    pub fn new_from_days(days: f64) -> Self {
        Self(hifidur::from_f64(days, Day))
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
impl Add<Duration> for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
/// Duration + Unit
impl Add<Unit> for Duration {
    type Output = Duration;

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
impl Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
/// Duration - Unit
impl Sub<Unit> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Unit) -> Self::Output {
        Self(self.0 - rhs)
    }
}

/// Duration * f64
impl Mul<f64> for Duration {
    type Output = Duration;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
/// f64 * Duration
impl Mul<Duration> for f64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        Duration(rhs.0 * self)
    }
}
/// Duration * i64
impl Mul<i64> for Duration {
    type Output = Duration;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
/// i64 * Duration
impl Mul<Duration> for i64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        Duration(rhs.0 * self)
    }
}

/// Duration / f64
impl Div<f64> for Duration {
    type Output = Duration;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
/// Duration / i64
impl Div<i64> for Duration {
    type Output = Duration;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
