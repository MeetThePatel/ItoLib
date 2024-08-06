use chrono::{DateTime, Utc};
use day_count_conventions::DayCounter;

use crate::term_structures::volatility_structure::BlackVolatilityTermStructure;
use crate::term_structures::{TermStructure, TermStructureDateTimeValidity};
use crate::types::{Strike, Volatility};

pub struct BlackConstantVolatilityTermStructure {
    reference_date: DateTime<Utc>,
    vol: Volatility,
}

impl BlackConstantVolatilityTermStructure {
    pub const fn new(reference_date: DateTime<Utc>, vol: Volatility) -> Self {
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
    fn get_reference_date(&self) -> DateTime<Utc> {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::MAX_UTC
    }

    fn validate_datetime(&self, dt: DateTime<Utc>) -> TermStructureDateTimeValidity {
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
    fn black_volatility(&self, _maturity: DateTime<Utc>, _strike: Strike) -> Volatility {
        self.vol
    }

    fn black_forward_volatility(
        &self,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
        _strike: Strike,
    ) -> Volatility {
        self.vol
    }
}
