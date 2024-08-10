pub mod volatility_structure;
pub mod yield_structure;

use day_count_conventions::DayCounter;

use crate::time::DateTime;

// TODO: Think more about if this is even a useful trait to have.
pub trait TermStructure<D>
where
    D: DayCounter,
{
    /// The calendar used for the term structure.
    ///
    /// TODO: This will be added when `ItoLib` gets support for international calendars.
    // fn get_calendar(&self) -> Calendar;

    /// The datetime at which the discount factor is equal to 1.0.
    ///
    /// This is the date that the curve refers to as "spot".
    fn get_reference_date(&self) -> DateTime;

    /// The latest datetime for which the term structure applies to.
    ///
    /// This is the last date that the term structure can make predictions on.
    fn get_max_datetime(&self) -> DateTime;

    /// Ensure that the term structure is applicable to the date.
    ///
    /// This is a utility method that makes sure that the term structure can make predictions about a given date.
    fn is_datetime_valid(&self, dt: DateTime) -> bool;
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
