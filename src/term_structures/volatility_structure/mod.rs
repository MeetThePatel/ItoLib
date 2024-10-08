use day_count_conventions::DayCounter;

use crate::term_structures::{TermStructure, TermStructureStrikeValidity};
use crate::types::Strike;

pub trait VolatilityTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    fn get_min_max_strike(&self) -> (Strike, Strike);

    fn validate_strike(&self, strike: Strike) -> TermStructureStrikeValidity;
}

mod black_volatility_term_structure;
pub use black_volatility_term_structure::{
    BlackVolatilityTermStructure, BlackVolatilityTermStructureResult,
};

mod constant_vol_term_structure;
pub use constant_vol_term_structure::{
    ConstantVolTermStructure, ConstantVolTermStructureBuilder, ConstantVolTermStructureBuilderError,
};

mod black_volatility_curve;
pub use black_volatility_curve::{BlackVolatilityCurve, BlackVolatilityCurveBuilder};
