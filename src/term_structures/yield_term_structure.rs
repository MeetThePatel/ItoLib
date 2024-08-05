use chrono::{DateTime, Utc};
use day_count_conventions::DayCounter;

use crate::interest_rate::InterestRate;
use crate::term_structures::term_structure::{TermStructure, TermStructureError};
use crate::types::DiscountFactor;

pub trait YieldTermStructure<D>: TermStructure<D>
where
    D: DayCounter,
{
    /// Get the discount factor for a given datetime.
    fn discount_factor(&self, t: DateTime<Utc>) -> Result<DiscountFactor, TermStructureError>;

    /// Get the implied zero-yield rate for a given datetime.
    fn zero_rate(&self, t: DateTime<Utc>) -> Result<InterestRate<D>, TermStructureError>;

    /// Get the forward interest rate between two dates.
    fn forward_rate(
        &self,
        t1: DateTime<Utc>,
        t2: DateTime<Utc>,
    ) -> Result<InterestRate<D>, TermStructureError>;
}
