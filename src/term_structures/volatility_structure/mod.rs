use day_count_conventions::DayCounter;

use crate::types::Strike;

use super::{term_structure::TermStructureStrikeValidity, TermStructure};

pub trait VolailityTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    fn get_min_max_strike(&self) -> (Strike, Strike);

    fn validate_strike(&self, strike: Strike) -> TermStructureStrikeValidity;
}

pub mod black_volatility_term_structure;
pub use black_volatility_term_structure::BlackVolatilityTermStructure;

pub mod black_constant_volatility_term_structure;
pub use black_constant_volatility_term_structure::BlackConstantVolatilityTermStructure;
