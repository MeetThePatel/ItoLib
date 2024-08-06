use crate::interest_rate::{implied_rate_from_compound_factor, InterestRate};
use crate::money::Currency;
use crate::term_structures::yield_structure::YieldTermStructure;
use crate::term_structures::{TermStructure, TermStructureDateTimeValidity, TermStructureError};
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
    #[must_use]
    pub const fn new(reference_date: DateTime<Utc>, rate: InterestRate<C, D>) -> Self {
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

    fn validate_datetime(&self, dt: DateTime<Utc>) -> TermStructureDateTimeValidity {
        if dt >= self.reference_date {
            TermStructureDateTimeValidity::Valid
        } else {
            TermStructureDateTimeValidity::Invalid
        }
    }
}

impl<C, D> YieldTermStructure<C, D> for FlatForwardTermStructure<C, D>
where
    C: Currency,
    D: DayCounter,
{
    fn discount_factor(&self, t: DateTime<Utc>) -> Result<DiscountFactor, TermStructureError> {
        if self.validate_datetime(t) == TermStructureDateTimeValidity::Invalid {
            return Err(TermStructureError::InvalidDateTime);
        }

        let mut year_fraction = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.reference_date.date_naive(), &t.date_naive());
        if year_fraction.get_fraction() == 0.0 {
            year_fraction = DayCountFraction::new(10e-8);
        }

        Ok(self.rate.discount_factor(&year_fraction))
    }

    fn zero_rate(&self, t: DateTime<Utc>) -> Result<InterestRate<C, D>, TermStructureError> {
        if self.validate_datetime(t) == TermStructureDateTimeValidity::Invalid {
            return Err(TermStructureError::InvalidDateTime);
        }

        let day_count_convention = self.rate.get_day_counter();

        let mut day_count_fraction = self
            .rate
            .get_day_counter()
            .day_count_fraction(&self.get_reference_date().date_naive(), &t.date_naive());

        if day_count_fraction.get_fraction() == 0.0 {
            day_count_fraction = DayCountFraction::new(10e-8);
        }
        let compound_factor = 1.0 / self.discount_factor(t).unwrap();

        let compounding = self.rate.get_compounding();

        Ok(implied_rate_from_compound_factor(
            compound_factor,
            &day_count_fraction,
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
        if self.validate_datetime(t1) == TermStructureDateTimeValidity::Invalid {
            return Err(TermStructureError::InvalidDateTime);
        }
        if self.validate_datetime(t2) == TermStructureDateTimeValidity::Invalid {
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
            &day_count_fraction,
            day_count_convention,
            self.rate.get_compounding(),
        )
        .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
    use day_count_conventions::Actual365Fixed;

    use crate::term_structures::yield_structure::{FlatForwardTermStructure, YieldTermStructure};
    use crate::{compounding::Compounding, interest_rate::InterestRate, money::currency::USD};

    #[test]
    fn test_flat_forward_term_structure_discount_factor() {
        let dcc = Actual365Fixed;
        let comp = Compounding::Continuous;
        let flat_rate: InterestRate<USD, _> = InterestRate::new(0.045, dcc, comp);

        let ref_date = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year_7_month: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 8, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let term_structure = FlatForwardTermStructure::new(ref_date, flat_rate);

        // Test Discount Factor
        assert_approx_equal_f64!(
            term_structure.discount_factor(ref_date).unwrap(),
            1.0,
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .discount_factor(ref_date_plus_1_year)
                .unwrap(),
            0.955_879_62,
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .discount_factor(ref_date_plus_1_year_7_month)
                .unwrap(),
            0.931_219_48,
            10e-8
        );
    }

    #[test]
    fn test_flat_forward_term_structure_zero_rate() {
        let dcc = Actual365Fixed;
        let comp = Compounding::Continuous;
        let flat_rate: InterestRate<USD, _> = InterestRate::new(0.045, dcc, comp);

        let ref_date = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year_7_month: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 8, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let term_structure = FlatForwardTermStructure::new(ref_date, flat_rate);

        // Test Zero Rate
        assert_approx_equal_f64!(
            term_structure.zero_rate(ref_date).unwrap().get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .zero_rate(ref_date_plus_1_year)
                .unwrap()
                .get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .zero_rate(ref_date_plus_1_year_7_month)
                .unwrap()
                .get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
    }

    #[test]
    fn test_flat_forward_term_structure_forward_rate() {
        let dcc = Actual365Fixed;
        let comp = Compounding::Continuous;
        let flat_rate: InterestRate<USD, _> = InterestRate::new(0.045, dcc, comp);

        let ref_date = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let ref_date_plus_1_year_7_month: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2025, 8, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Utc,
        );

        let term_structure = FlatForwardTermStructure::new(ref_date, flat_rate);

        // Test forward rate
        assert_approx_equal_f64!(
            term_structure
                .forward_rate(ref_date, ref_date_plus_1_year)
                .unwrap()
                .get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .forward_rate(ref_date_plus_1_year, ref_date_plus_1_year_7_month)
                .unwrap()
                .get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
        assert_approx_equal_f64!(
            term_structure
                .forward_rate(ref_date, ref_date_plus_1_year_7_month)
                .unwrap()
                .get_rate(),
            flat_rate.get_rate(),
            10e-8
        );
    }
}
