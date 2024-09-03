use std::marker::PhantomData;

use day_count_conventions::{DayCountFraction, DayCounter};

use itolib_types::{CompoundFactor, DiscountFactor, Float};

use crate::{Compounding, Currency};

/// Interest Rate
///
/// An interest rate has an associated currency, day counting convention, and compounding method.
#[derive(Debug, Clone)]
pub struct InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    rate: Float,
    day_counter: D,
    compounding: Compounding,
    _marker: PhantomData<C>,
}

impl<C, D> InterestRate<C, D>
where
    C: Currency,
    D: DayCounter,
{
    /// Create a new interest rate.
    #[must_use]
    pub fn new(rate: impl Into<f64>, day_counter: D, compounding: Compounding) -> Self {
        Self { rate: Float::new(rate.into()), day_counter, compounding, _marker: PhantomData }
    }

    /// Get the rate.
    #[inline]
    #[must_use]
    pub const fn get_rate(&self) -> Float {
        self.rate
    }

    /// Get the day counting convention used.
    #[inline]
    #[must_use]
    pub const fn get_day_counter(&self) -> D {
        self.day_counter
    }

    /// Get the compounding method.
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
    ///
    /// To calculate the discount factor:
    /// $$
    ///     DF \coloneqq \frac{1}{CF}
    /// $$
    /// where $CF$ is the compound factor.
    #[must_use]
    pub fn discount_factor(&self, year_fraction: &DayCountFraction<D>) -> DiscountFactor {
        DiscountFactor::new(1.0 / self.compound_factor(year_fraction).value()).unwrap()
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
        let rate: f64 = *self.rate;
        let tau: f64 = year_fraction.get_fraction();

        CompoundFactor::new(match self.compounding {
            Compounding::Simple(_) => f64::mul_add(rate, tau, 1.0),
            Compounding::Compounding(freq) => {
                let freq = freq as i32 as f64;
                f64::powf(1.0 + rate / freq, tau * freq)
            }
            Compounding::Continuous => f64::exp(rate * tau),
        })
        .unwrap()
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

/// Calculate the interest rate implied by a compound factor.
///
/// Simple:
/// $$
///     CF = (1 + r \tau) \implies r = \frac{CF - 1}{\tau}
/// $$
///
/// Compounding:
/// $$
///     CF = \left(1 + \frac{r}{n}\right)^{n \tau} \implies r = n (CF^{\frac{1}{n \tau}} - 1)
/// $$
///
/// Continuous:
/// $$
///     CF = e^{r \tau} \implies r = \frac{\ln{C}}{\tau}
/// $$
pub fn implied_rate_from_compound_factor<C, D>(
    compound_factor: impl Into<CompoundFactor>,
    day_count_fraction: &DayCountFraction<D>,
    day_count_convention: D,
    compounding: Compounding,
) -> Option<InterestRate<C, D>>
where
    C: Currency,
    D: DayCounter,
{
    let tau = day_count_fraction.get_fraction();
    let compound_factor: f64 = *compound_factor.into().value();

    let implied_rate: f64 = match compounding {
        Compounding::Simple(_) => (compound_factor - 1.0) / tau,
        Compounding::Compounding(freq) => {
            let freq = freq as i32 as f64;

            freq * (f64::powf(compound_factor, f64::recip(freq * tau)) - 1.0)
        }
        Compounding::Continuous => f64::ln(compound_factor) / tau,
    };
    Some(InterestRate {
        rate: Float::new(implied_rate),
        day_counter: day_count_convention,
        compounding,
        _marker: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use day_count_conventions::{Actual360, DayCountFraction};

    use crate::{
        currency::USD, interest_rate::implied_rate_from_compound_factor, Compounding, InterestRate,
    };
    use itolib_time::Frequency;
    use itolib_types::CompoundFactor;

    #[test]
    fn test_interest_rates() {
        let simple_rate: InterestRate<USD, Actual360> =
            InterestRate::new(0.05, Actual360, Compounding::Simple(Frequency::Annual));
        let compound_rate: InterestRate<USD, Actual360> =
            InterestRate::new(0.05, Actual360, Compounding::Compounding(Frequency::Quarterly));
        let continuous_rate: InterestRate<USD, Actual360> =
            InterestRate::new(0.05, Actual360, Compounding::Continuous);

        // Getters
        assert_eq!(simple_rate.get_compounding(), Compounding::Simple(Frequency::Annual));
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
        let dcf1 = DayCountFraction::new(1.0);
        // Discount Factor
        assert_approx_eq!(simple_rate.discount_factor(&dcf1).value(), 0.952_380_95, 10e-8);
        assert_approx_eq!(compound_rate.discount_factor(&dcf1).value(), 0.951_524_27, 10e-8);
        assert_approx_eq!(continuous_rate.discount_factor(&dcf1).value(), 0.951_229_42, 10e-8);

        // Implied Interest Rate from Compound Factor
        assert_approx_eq!(
            implied_rate_from_compound_factor::<USD, _>(
                CompoundFactor::new(1.5).unwrap(),
                &dcf1,
                Actual360,
                Compounding::Simple(Frequency::Annual)
            )
            .unwrap()
            .get_rate(),
            0.5,
            10e-8
        );
        assert_approx_eq!(
            implied_rate_from_compound_factor::<USD, Actual360>(
                CompoundFactor::new(1.5).unwrap(),
                &DayCountFraction::new(1.0),
                Actual360,
                Compounding::Compounding(Frequency::Quarterly)
            )
            .unwrap()
            .get_rate(),
            0.426_727_67,
            10e-8
        );
        assert_approx_eq!(
            implied_rate_from_compound_factor::<USD, Actual360>(
                CompoundFactor::new(1.5).unwrap(),
                &DayCountFraction::new(1.0),
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
