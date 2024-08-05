use crate::interest_rate::{implied_rate_from_compound_factor, InterestRate};
use crate::money::Currency;
use crate::term_structures::{TermStructure, TermStructureError, YieldTermStructure};
use crate::types::DiscountFactor;

use chrono::{DateTime, Utc};
use day_count_conventions::{DayCountFraction, DayCounter};

pub struct FlatForwardTermStructure<C, D>
where
    C: Currency,
    D: DayCounter,
{
    pub reference_date: DateTime<Utc>,
    pub rate: InterestRate<C, D>,
}

impl<C, D> FlatForwardTermStructure<C, D>
where
    C: Currency,
    D: DayCounter,
{
    pub fn new(reference_date: DateTime<Utc>, rate: InterestRate<C, D>) -> Self {
        Self {
            reference_date,
            rate,
        }
    }
}

impl<C, D> TermStructure<D> for FlatForwardTermStructure<C, D>
where
    C: Currency,
    D: DayCounter,
{
    fn get_reference_date(&self) -> DateTime<Utc> {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::MAX_UTC
    }

    fn validate_datetime(&self, dt: DateTime<Utc>) -> bool {
        dt >= self.reference_date && dt <= self.get_max_datetime()
    }
}

impl<C, D> YieldTermStructure<C, D> for FlatForwardTermStructure<C, D>
where
    C: Currency,
    D: DayCounter,
{
    fn discount_factor(&self, t: DateTime<Utc>) -> Result<DiscountFactor, TermStructureError> {
        if self.validate_datetime(t) {
            return Err(TermStructureError::InvalidDateTime);
        }

        let year_fraction = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.reference_date.date_naive(), &t.date_naive());

        Ok(self.rate.discount_factor(year_fraction))
    }

    fn zero_rate(&self, t: DateTime<Utc>) -> Result<InterestRate<C, D>, TermStructureError> {
        if self.validate_datetime(t) {
            return Err(TermStructureError::InvalidDateTime);
        }

        let day_count_fraction = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.get_reference_date().date_naive(), &t.date_naive());

        let day_count_convention = self.rate.get_day_counter();

        let compound_factor = 1.0 / self.discount_factor(t).unwrap();
        let compounding = self.rate.get_compounding();

        Ok(implied_rate_from_compound_factor(
            compound_factor,
            day_count_fraction,
            *day_count_convention,
            compounding,
        )
        .unwrap())
    }

    fn forward_rate(
        &self,
        t1: DateTime<Utc>,
        t2: DateTime<Utc>,
    ) -> Result<InterestRate<C, D>, TermStructureError> {
        if self.validate_datetime(t1) {
            return Err(TermStructureError::InvalidDateTime);
        }
        if self.validate_datetime(t2) {
            return Err(TermStructureError::InvalidDateTime);
        }
        if t2 < t1 {
            return Err(TermStructureError::T2LessThanT1);
        }

        let compound_factor = self.discount_factor(t1).unwrap() / self.discount_factor(t2).unwrap();

        let yf1 = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.reference_date.date_naive(), &t1.date_naive());
        let yf2 = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.reference_date.date_naive(), &t2.date_naive());

        let day_count_fraction = DayCountFraction::new(yf2.get_fraction() - yf1.get_fraction());

        let day_count_convention = *self.rate.get_day_counter();

        Ok(implied_rate_from_compound_factor(
            compound_factor,
            day_count_fraction,
            day_count_convention,
            self.rate.get_compounding(),
        )
        .unwrap())
    }
}
