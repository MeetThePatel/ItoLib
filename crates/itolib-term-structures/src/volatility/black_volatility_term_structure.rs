use day_count_conventions::DayCounter;

use itolib_time::DateTime;
use itolib_types::{Strike, Volatility};

use crate::volatility::VolatilityTermStructure;

pub trait BlackVolatilityTermStructure<D>: VolatilityTermStructure<D>
where
    D: DayCounter,
{
    fn black_volatility(
        &self,
        maturity: DateTime,
        strike: Strike,
    ) -> BlackVolatilityTermStructureResult;

    fn black_forward_volatility(
        &self,
        start_date: DateTime,
        end_date: DateTime,
        strike: Strike,
    ) -> BlackVolatilityTermStructureResult;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone)]
pub enum BlackVolatilityTermStructureResult {
    InterpolatedValue(Volatility),
    ExistingValue(Volatility),
    OutOfRange,
    NoPoints,
}
