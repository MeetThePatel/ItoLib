use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;

use hifitime::prelude::*;

use super::Duration;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DateTime(Epoch);

impl DateTime {
    #[must_use]
    pub const fn new_from_epoch(dt: &Epoch) -> Self {
        Self(*dt)
    }

    #[must_use]
    pub fn new_from_ymd(year: i32, month: u8, day: u8) -> Self {
        Self(Epoch::from_gregorian_utc_at_midnight(year, month, day))
    }

    #[must_use]
    pub fn new_from_ymd_hms(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Self {
        Self(Epoch::from_gregorian_utc_hms(
            year, month, day, hour, minute, second,
        ))
    }

    #[must_use]
    pub fn now() -> Self {
        Self(Epoch::now().unwrap())
    }

    pub fn format(&self, s: &str) -> Result<String, hifitime::ParsingErrors> {
        Ok(Formatter::new(**self, format(s)?).to_string())
    }

    #[must_use]
    pub fn format_ymd(&self) -> String {
        Formatter::new(**self, format_ymd()).to_string()
    }
}

pub fn format(s: &str) -> Result<Format, hifitime::ParsingErrors> {
    Format::from_str(s)
}

#[must_use]
pub fn format_ymd() -> Format {
    Format::from_str("%y/%m/%d").unwrap()
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
