use chrono::{DateTime, Utc};
use day_count_conventions::DayCounter;

use crate::{
    term_structures::{
        term_structure::TermStructureStrikeValidity, TermStructure, TermStructureDateTimeValidity,
    },
    types::{Strike, Volatility},
};

use super::VolailityTermStructure;

pub struct BlackVolatilityCurve {
    datetimes: Vec<DateTime<Utc>>,
    volatilities: Vec<Volatility>,
    reference_date: DateTime<Utc>,
}

impl BlackVolatilityCurve {
    pub fn new(
        datetimes: &[DateTime<Utc>],
        volatilities: &[Volatility],
        reference_date: DateTime<Utc>,
    ) -> Self {
        Self {
            datetimes: datetimes.to_vec(),
            volatilities: volatilities.to_vec(),
            reference_date,
        }
    }
    pub fn new_with_pairs(
        pairs: &[(DateTime<Utc>, Volatility)],
        reference_date: DateTime<Utc>,
    ) -> Self {
        let (datetimes, volatilities) = pairs.iter().cloned().unzip();
        Self {
            datetimes,
            volatilities,
            reference_date,
        }
    }
}

impl<D> TermStructure<D> for BlackVolatilityCurve
where
    D: DayCounter,
{
    fn get_reference_date(&self) -> DateTime<Utc> {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime<Utc> {
        self.datetimes.last().copied().unwrap()
    }

    fn validate_datetime(&self, dt: DateTime<Utc>) -> TermStructureDateTimeValidity {
        if dt >= <BlackVolatilityCurve as TermStructure<D>>::get_reference_date(self)
            && dt <= <BlackVolatilityCurve as TermStructure<D>>::get_max_datetime(self)
        {
            TermStructureDateTimeValidity::Valid
        } else {
            TermStructureDateTimeValidity::Invalid
        }
    }
}

impl<D> VolailityTermStructure<D> for BlackVolatilityCurve
where
    D: DayCounter,
{
    fn get_min_max_strike(&self) -> (Strike, Strike) {
        (Strike::MIN, Strike::MAX)
    }

    fn validate_strike(&self, _strike: Strike) -> TermStructureStrikeValidity {
        TermStructureStrikeValidity::Valid
    }
}
