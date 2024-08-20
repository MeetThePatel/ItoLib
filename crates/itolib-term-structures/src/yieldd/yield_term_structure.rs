use day_count_conventions::DayCounter;

use itolib_money::{Currency, InterestRate};
use itolib_time::DateTime;
use itolib_types::DiscountFactor;

use crate::{TermStructure, TermStructureError};

pub trait YieldTermStructure<C, D>: TermStructure<D>
where
    C: Currency,
    D: DayCounter,
{
    /// Get the discount factor for a given datetime.
    fn discount_factor(&self, t: DateTime) -> Result<DiscountFactor, TermStructureError>;

    /// Get the implied zero-yield rate for a given datetime.
    fn zero_rate(&self, t: DateTime) -> Result<InterestRate<C, D>, TermStructureError>;

    /// Get the forward interest rate between two dates.
    fn forward_rate(
        &self,
        t1: DateTime,
        t2: DateTime,
    ) -> Result<InterestRate<C, D>, TermStructureError>;
}
