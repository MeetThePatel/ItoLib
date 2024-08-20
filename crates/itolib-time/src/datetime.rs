use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;

use hifitime::prelude::*;
use ordered_float::OrderedFloat;

use crate::Duration;

/// This is a thin wrapper around `HifiTime`'s Epoch.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DateTime(Epoch);

impl DateTime {
    /// Create a new `DateTime` from an Epoch.
    #[must_use]
    pub const fn new_from_epoch(dt: &Epoch) -> Self {
        Self(*dt)
    }

    /// Create a new `DateTime` from year-month-day.
    #[must_use]
    pub fn new_from_ymd(year: i32, month: u8, day: u8) -> Self {
        Self(Epoch::from_gregorian_utc_at_midnight(year, month, day))
    }

    /// Create a new `DateTime` from year-month-day-hour-minute-second.
    ///
    /// This sets the nanoseconds equal to zero.
    #[must_use]
    pub fn new_from_ymd_hms(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Self {
        Self(Epoch::from_gregorian_utc_hms(year, month, day, hour, minute, second))
    }

    /// Returns current `DateTime`
    ///
    /// # Panics
    /// if there was an issue initializing system time.
    #[must_use]
    pub fn now() -> Self {
        Self(Epoch::now().unwrap())
    }

    /// Format the datetime using a format string.
    ///
    /// # Errors
    /// Will return `ParsingErrors` if there was an issue parsing the format string.
    pub fn format(&self, s: &str) -> Result<String, hifitime::ParsingErrors> {
        Ok(Formatter::new(**self, format(s)?).to_string())
    }

    #[must_use]
    pub fn format_ymd(&self) -> String {
        Formatter::new(**self, format_ymd()).to_string()
    }
}

/// Create a format for formating datetimes.
///
/// # Errors
/// Will return `ParsingErrors` if there was an issue parsing the format string.
pub fn format(s: &str) -> Result<Format, hifitime::ParsingErrors> {
    Format::from_str(s)
}

/// Create a formatter that outputs Y/M/D.
#[must_use]
pub fn format_ymd() -> Format {
    Format::from_str("%y/%m/%d").unwrap_or_default()
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl Deref for DateTime {
    type Target = Epoch;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Epoch> for DateTime {
    fn from(value: Epoch) -> Self {
        Self::new_from_epoch(&value)
    }
}
impl From<DateTime> for i64 {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: DateTime) -> Self {
        value.0.to_utc_seconds() as Self
    }
}
impl From<DateTime> for f64 {
    fn from(value: DateTime) -> Self {
        value.0.to_utc_seconds()
    }
}
impl From<DateTime> for OrderedFloat<f64> {
    fn from(value: DateTime) -> Self {
        Self(value.0.to_utc_seconds())
    }
}

/// `DateTime` + `Duration`
impl Add<Duration> for DateTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0.add(*rhs))
    }
}
/// `Duration` + `DateTime`
impl Add<DateTime> for Duration {
    type Output = DateTime;

    fn add(self, rhs: DateTime) -> Self::Output {
        DateTime(*rhs + *self)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use hifitime::Epoch;
    use num::ToPrimitive;
    use ordered_float::OrderedFloat;

    use crate::{DateTime, Duration};

    #[test]
    fn test_datetime_new() {
        let e = Epoch::from_gregorian(2024, 1, 1, 0, 0, 0, 0, hifitime::TimeScale::UTC);
        let dt = DateTime::new_from_epoch(&e);
        assert_eq!(e, *dt);

        let dt_from_cast = DateTime::from(e);
        assert_eq!(dt, dt_from_cast);

        let dt2 = DateTime::new_from_ymd_hms(2024, 1, 1, 0, 0, 0);
        assert_eq!(dt, dt2);
    }

    #[test]
    fn test_print() {
        let dt = DateTime::new_from_ymd(2024, 1, 1);
        assert_eq!(dt.to_string(), "2024-01-01T00:00:00 UTC");

        assert_eq!(dt.format("%y/%m/%d").unwrap(), "2024/01/01");
    }

    #[test]
    fn test_casts() {
        let mut dt = DateTime::default();

        let float_repr = f64::from(dt);
        assert_eq!(dt.to_utc_seconds(), float_repr);

        dt = DateTime::new_from_ymd(2025, 1, 1);

        let int_repr = i64::from(dt);
        assert_eq!(dt.to_utc_seconds().to_i64().unwrap(), int_repr);

        let ordered_float_repr = OrderedFloat::<f64>::from(dt);
        assert_eq!(dt.to_utc_seconds(), *ordered_float_repr);
    }

    #[test]
    fn test_ops() {
        let mut dt = DateTime::new_from_ymd(2024, 1, 1);

        assert_eq!(dt + Duration::new_from_days(1.0), DateTime::new_from_ymd(2024, 1, 2));

        assert_eq!(Duration::new_from_days(1.0) + dt, DateTime::new_from_ymd(2024, 1, 2));

        *dt = *DateTime::now();
    }
}
