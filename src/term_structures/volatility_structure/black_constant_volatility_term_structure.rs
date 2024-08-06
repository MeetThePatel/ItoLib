use day_count_conventions::DayCounter;

use crate::term_structures::volatility_structure::BlackVolatilityTermStructure;
use crate::term_structures::{TermStructure, TermStructureDateTimeValidity};
use crate::time::DateTime;
use crate::types::{Strike, Volatility};

pub struct BlackConstantVolatilityTermStructure {
    reference_date: DateTime,
    vol: Volatility,
}

impl BlackConstantVolatilityTermStructure {
    #[must_use]
    pub const fn new(reference_date: DateTime, vol: Volatility) -> Self {
        Self {
            reference_date,
            vol,
        }
    }
}

impl<D> TermStructure<D> for BlackConstantVolatilityTermStructure
where
    D: DayCounter,
{
    fn get_reference_date(&self) -> DateTime {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime {
        DateTime::new_from_ymd(9999, 12, 31)
    }

    fn validate_datetime(&self, dt: DateTime) -> TermStructureDateTimeValidity {
        if dt >= self.reference_date {
            TermStructureDateTimeValidity::Valid
        } else {
            TermStructureDateTimeValidity::Invalid
        }
    }
}

impl<D> BlackVolatilityTermStructure<D> for BlackConstantVolatilityTermStructure
where
    D: DayCounter,
{
    fn black_volatility(&self, _maturity: DateTime, _strike: Strike) -> Volatility {
        self.vol
    }

    fn black_forward_volatility(
        &self,
        _start_date: DateTime,
        _end_date: DateTime,
        _strike: Strike,
    ) -> Volatility {
        self.vol
    }
}
