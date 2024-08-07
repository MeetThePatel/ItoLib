use day_count_conventions::DayCounter;

use crate::time::DateTime;

pub trait TermStructure<D>
where
    D: DayCounter,
{
    /// The calendar used for the term structure.
    // fn get_calendar(&self) -> Calendar;

    /// The datetime at which the discount factor is equal to 1.0.
    fn get_reference_date(&self) -> DateTime;

    /// The latest datetime for which the term structure applies to.
    fn get_max_datetime(&self) -> DateTime;

    /// Ensure that the term structure is applicable to the date.
    fn validate_datetime(&self, dt: DateTime) -> TermStructureDateTimeValidity;
}

#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TermStructureStrikeValidity {
    Invalid = 0,
    Valid = 1,
}
