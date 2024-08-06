use day_count_conventions::DayCounter;

use crate::{
    term_structures::{
        term_structure::TermStructureStrikeValidity, TermStructure, TermStructureDateTimeValidity,
    },
    time::DateTime,
    types::{Strike, Volatility},
};

use super::VolailityTermStructure;

pub struct BlackVolatilityCurve {
    datetimes: Vec<DateTime>,
    volatilities: Vec<Volatility>,
    reference_date: DateTime,
}

impl BlackVolatilityCurve {
    #[must_use]
    pub fn new(
        datetimes: &[DateTime],
        volatilities: &[Volatility],
        reference_date: DateTime,
    ) -> Self {
        Self {
            datetimes: datetimes.to_vec(),
            volatilities: volatilities.to_vec(),
            reference_date,
        }
    }
    #[must_use]
    pub fn new_with_pairs(pairs: &[(DateTime, Volatility)], reference_date: DateTime) -> Self {
        let (datetimes, volatilities) = pairs.iter().copied().unzip();
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
    fn get_reference_date(&self) -> DateTime {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime {
        self.datetimes.last().copied().unwrap()
    }

    fn validate_datetime(&self, dt: DateTime) -> TermStructureDateTimeValidity {
        if dt >= <Self as TermStructure<D>>::get_reference_date(self)
            && dt <= <Self as TermStructure<D>>::get_max_datetime(self)
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
