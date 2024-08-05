use chrono::{DateTime, Utc};
use day_count_conventions::DayCounter;

pub trait TermStructure<D>
where
    D: DayCounter,
{
    /// The calendar used for the term structure.
    // fn get_calendar(&self) -> Calendar;

    /// The datetime at which the discount factor is equal to 1.0.
    fn get_reference_date(&self) -> DateTime<Utc>;

    /// The latest datetime for which the term structure applies to.
    fn get_max_datetime(&self) -> DateTime<Utc>;

    /// Ensure that the term structure is applicable to the date.
    fn validate_datetime(&self, dt: DateTime<Utc>) -> TermStructureDateTimeValidity;
}

#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
#[derive(Debug)]
pub enum TermStructureError {
    InvalidDateTime,
    T2LessThanT1,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TermStructureDateTimeValidity {
    Invalid = 0,
    Valid = 1,
}
