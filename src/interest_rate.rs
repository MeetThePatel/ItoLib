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
    pub const fn get_day_counter(&self) -> &D {
        &self.day_counter
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
    pub fn discount_factor(&self, year_fraction: &DayCountFraction<D>) -> DiscountFactor {
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
    pub fn compound_factor(&self, year_fraction: &DayCountFraction<D>) -> CompoundFactor {
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
    compound_factor: CompoundFactor,
    day_count_fraction: &DayCountFraction<D>,
    day_count_convention: D,
    compounding: Compounding,
) -> Option<InterestRate<C, D>>
where
    C: Currency,
    D: DayCounter,
{
    match compounding {
        Compounding::Simple(_) => {
            let r = (compound_factor - 1.0) / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: r,
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
        Compounding::Compounding(f) => {
            let f = f64::from(f as u32);
            let r = (compound_factor.powf(1.0 / (f * day_count_fraction.get_fraction())) - 1.0) * f;
            Some(InterestRate {
                rate: OrderedFloat(r),
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
        Compounding::Continuous => {
            let r = compound_factor.ln() / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: OrderedFloat(r),
                day_counter: day_count_convention,
                compounding,
                _marker: PhantomData,
            })
        }
    }
}
