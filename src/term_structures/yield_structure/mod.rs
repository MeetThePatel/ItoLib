use day_count_conventions::DayCounter;

use crate::interest_rate::InterestRate;
use crate::money::Currency;
use crate::term_structures::term_structure::{TermStructure, TermStructureError};
use crate::time::DateTime;
use crate::types::DiscountFactor;

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

pub mod flat_forward_term_structure;
pub use flat_forward_term_structure::FlatForwardTermStructure;
