use day_count_conventions::DayCounter;

use crate::{
    time::DateTime,
    types::{Strike, Volatility},
};

use super::TermStructure;

pub trait BlackVolatilityTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    fn black_volatility(&self, maturity: DateTime, strike: Strike) -> Volatility;

    fn black_forward_volatility(
        &self,
        start_date: DateTime,
        end_date: DateTime,
        strike: Strike,
    ) -> Volatility;
}
