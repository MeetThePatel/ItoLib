use day_count_conventions::DayCounter;

use itolib_types::Strike;

use crate::{TermStructure, TermStructureStrikeValidity};

pub trait VolatilityTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    fn get_min_max_strike(&self) -> (Strike, Strike);

    fn validate_strike(&self, strike: Strike) -> TermStructureStrikeValidity;
}
