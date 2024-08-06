use chrono::{DateTime, Utc};
use day_count_conventions::DayCounter;

use crate::types::{Strike, Volatility};

use super::TermStructure;

pub trait BlackVolatilityTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    fn black_volatility(&self, maturity: DateTime<Utc>, strike: Strike) -> Volatility;

    fn black_forward_volatility(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        strike: Strike,
    ) -> Volatility;
}
