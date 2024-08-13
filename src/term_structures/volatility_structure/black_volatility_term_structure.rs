use day_count_conventions::DayCounter;

use crate::{
    time::DateTime,
    types::{Strike, Volatility},
};

use super::VolatilityTermStructure;

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

#[derive(Debug, Copy, Clone)]
pub enum BlackVolatilityTermStructureResult {
    InterpolatedValue(Volatility),
    ExistingValue(Volatility),
    OutOfRange,
    NoPoints,
}
