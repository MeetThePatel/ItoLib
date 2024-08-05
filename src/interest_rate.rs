use crate::compounding::Compounding;
use crate::types::{CompoundFactor, DiscountFactor, Percentage};

use day_count_conventions::{DayCountFraction, DayCounter};

// TODO: Make Interest Rate generic over currency C.
#[derive(Debug, Copy, Clone)]
pub struct InterestRate<D>
where
    D: DayCounter,
{
    rate: Percentage,
    day_counter: D,
    compounding: Compounding,
}

impl<D> InterestRate<D>
where
    D: DayCounter,
{
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

impl<D> InterestRate<D>
where
    D: DayCounter,
{
    /// Discount factor implied by the rate at time $t$.
    #[must_use]
    pub fn discount_factor(&self, year_fraction: DayCountFraction<D>) -> DiscountFactor {
        1.0 / self.compound_factor(year_fraction)
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
        match self.compounding {
            Compounding::Simple(_) => 1.0 + self.rate * year_fraction.get_fraction(),
            Compounding::Compounding(freq) => (1.0 + self.rate / freq as i32 as f64)
                .powf(freq as i32 as f64 * year_fraction.get_fraction()),
            Compounding::Continuous => f64::exp(self.rate * year_fraction.get_fraction()),
        }
    }
}

impl<D> PartialEq for InterestRate<D>
where
    D: DayCounter,
{
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate && self.compounding == other.compounding
    }
}

pub fn implied_rate_from_compound_factor<D>(
    compound_factor: CompoundFactor,
    day_count_fraction: DayCountFraction<D>,
    day_count_convention: D,
    compounding: Compounding,
) -> Option<InterestRate<D>>
where
    D: DayCounter,
{
    match compounding {
        Compounding::Simple(_) => {
            let r = (compound_factor - 1.0) / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: r,
                day_counter: day_count_convention,
                compounding,
            })
        }
        Compounding::Compounding(f) => {
            let f = f as u32 as f64;
            let r = (compound_factor.powf(1.0 / (f * day_count_fraction.get_fraction())) - 1.0) * f;
            Some(InterestRate {
                rate: r,
                day_counter: day_count_convention,
                compounding,
            })
        }
        Compounding::Continuous => {
            let r = f64::ln(compound_factor) / day_count_fraction.get_fraction();
            Some(InterestRate {
                rate: r,
                day_counter: day_count_convention,
                compounding,
            })
        }
    }
}
