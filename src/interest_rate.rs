use std::marker::PhantomData;

use crate::compounding::Compounding;
use crate::money::Currency;
use crate::types::{CompoundFactor, DiscountFactor, Percentage};

use day_count_conventions::{DayCountFraction, DayCounter};
use ordered_float::OrderedFloat;

#[derive(Debug, Copy, Clone)]
pub struct InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    rate: Percentage,
    day_counter: D,
    compounding: Compounding,
    _marker: PhantomData<C>,
}

impl<C, D> InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    #[must_use]
    pub fn new(rate: impl Into<Percentage>, day_counter: D, compounding: Compounding) -> Self {
        Self {
            rate: rate.into(),
            day_counter,
            compounding,
            _marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn get_rate(&self) -> Percentage {
        self.rate
    }

    #[inline]
    #[must_use]
    pub const fn get_day_counter(&self) -> D {
        self.day_counter
    }

    #[inline]
    #[must_use]
    pub const fn get_compounding(&self) -> Compounding {
        self.compounding
    }
}

impl<C, D> InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    /// Discount factor implied by the rate at time $t$.
    #[must_use]
    pub fn discount_factor(&self, year_fraction: DayCountFraction<D>) -> DiscountFactor {
        OrderedFloat(1.0) / self.compound_factor(year_fraction)
    }

    /// Compound factor implied by the rate at time $t$.
    ///
    /// Simple: $1 + rt$
    ///
    /// Compounding: $(1 + \frac{r}{n})^{nt}$
    ///
    /// Continuous: $e^{rt}$
    #[must_use]
    pub fn compound_factor(&self, year_fraction: DayCountFraction<D>) -> CompoundFactor {
        OrderedFloat(match self.compounding {
            Compounding::Simple(_) => self.rate.mul_add(year_fraction.get_fraction(), 1.0),
            Compounding::Compounding(freq) => (OrderedFloat(1.0)
                + self.rate / f64::from(freq as i32))
            .powf(f64::from(freq as i32) * year_fraction.get_fraction()),
            Compounding::Continuous => (self.rate * year_fraction.get_fraction()).exp(),
        })
    }
}

impl<C, D> PartialEq for InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate && self.compounding == other.compounding
    }
}

pub fn implied_rate_from_compound_factor<C, D>(
    compound_factor: impl Into<CompoundFactor>,
    day_count_fraction: DayCountFraction<D>,
    day_count_convention: D,
    compounding: Compounding,
) -> Option<InterestRate<C, D>>
where
    C: Currency,
    D: DayCounter,
{
    match compounding {
        Compounding::Simple(_) => {
            let r = (compound_factor.into() - 1.0) / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: r,
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
        Compounding::Compounding(f) => {
            let f = f64::from(f as u32);
            let r = (compound_factor
                .into()
                .powf(1.0 / (f * day_count_fraction.get_fraction()))
                - 1.0)
                * f;
            Some(InterestRate {
                rate: OrderedFloat(r),
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
        Compounding::Continuous => {
            let r = compound_factor.into().ln() / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: OrderedFloat(r),
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use day_count_conventions::Actual360;

    use super::*;
    use crate::{money::currency::USD, time::Frequency};

    #[test]
    fn test_interest_rates() {
        let simple_rate: InterestRate<USD, Actual360> =
            InterestRate::new(0.05, Actual360, Compounding::Simple(Frequency::Annual));
        let compound_rate: InterestRate<USD, Actual360> = InterestRate::new(
            0.05,
            Actual360,
            Compounding::Compounding(Frequency::Quarterly),
        );
        let continuous_rate: InterestRate<USD, Actual360> =
            InterestRate::new(0.05, Actual360, Compounding::Continuous);

        // Getters
        assert_eq!(
            simple_rate.get_compounding(),
            Compounding::Simple(Frequency::Annual)
        );
        assert_eq!(simple_rate.get_day_counter(), Actual360);
        assert_eq!(simple_rate.get_rate(), 0.05);

        // PartialEq
        assert_eq!(
            simple_rate,
            InterestRate::<USD, Actual360>::new(
                0.05,
                Actual360,
                Compounding::Simple(Frequency::Annual)
            )
        );

        // Discount Factor
        assert_approx_equal_f64!(
            simple_rate.discount_factor(DayCountFraction::new(1.0)),
            0.952_380_95,
            10e-8
        );
        assert_approx_equal_f64!(
            compound_rate.discount_factor(DayCountFraction::new(1.0)),
            0.951_524_27,
            10e-8
        );
        assert_approx_equal_f64!(
            continuous_rate.discount_factor(DayCountFraction::new(1.0)),
            0.951_229_42,
            10e-8
        );

        // Implied Interest Rate from Compound Factor
        assert_approx_equal_f64!(
            implied_rate_from_compound_factor::<USD, _>(
                1.5,
                DayCountFraction::new(1.0),
                Actual360,
                Compounding::Simple(Frequency::Annual)
            )
            .unwrap()
            .get_rate(),
            0.5,
            10e-8
        );
        assert_approx_equal_f64!(
            implied_rate_from_compound_factor::<USD, Actual360>(
                1.5,
                DayCountFraction::new(1.0),
                Actual360,
                Compounding::Compounding(Frequency::Quarterly)
            )
            .unwrap()
            .get_rate(),
            0.426_727_67,
            10e-8
        );
        assert_approx_equal_f64!(
            implied_rate_from_compound_factor::<USD, Actual360>(
                1.5,
                DayCountFraction::new(1.0),
                Actual360,
                Compounding::Continuous
            )
            .unwrap()
            .get_rate(),
            0.405_465_10,
            10e-8
        );
    }
}
