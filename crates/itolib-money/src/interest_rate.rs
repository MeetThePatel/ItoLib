use std::marker::PhantomData;

use day_count_conventions::{DayCountFraction, DayCounter};
use ordered_float::{Float, OrderedFloat};

use itolib_types::{CompoundFactor, DiscountFactor, Percentage};

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
    /// Create a new interest rate.
    #[must_use]
    pub fn new(rate: impl Into<Percentage>, day_counter: D, compounding: Compounding) -> Self {
        Self { rate: rate.into(), day_counter, compounding, _marker: PhantomData }
    }

    /// Get the rate.
    #[inline]
    #[must_use]
    pub const fn get_rate(&self) -> Percentage {
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
        DiscountFactor::new(1.0 / *self.compound_factor(year_fraction).value().value()).unwrap()
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
        CompoundFactor::new(match self.compounding {
            Compounding::Simple(_) => {
                (*self.rate.value().value()).mul_add(year_fraction.get_fraction(), 1.0)
            }
            Compounding::Compounding(freq) => (1.0
                + *(self.rate.value() / f64::from(freq as i32)).unwrap().value())
            .powf(*OrderedFloat(f64::from(freq as i32) * year_fraction.get_fraction())),
            Compounding::Continuous => {
                *(self.rate.value() * year_fraction.get_fraction()).unwrap().value().exp()
            }
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
    let implied_rate: Percentage = match compounding {
        Compounding::Simple(_) => Percentage::new(
            ((compound_factor.into().value() - 1.0).unwrap() / day_count_fraction.get_fraction())
                .unwrap(),
        ),
        Compounding::Compounding(freq) => {
            let freq = f64::from(freq as u32);
            Percentage::new(
                (freq)
                    * *(compound_factor
                        .into()
                        .value()
                        .value()
                        .powf(OrderedFloat(1.0 / (freq * day_count_fraction.get_fraction())))
                        - 1.0),
            )
        }
        Compounding::Continuous => Percentage::new(
            (compound_factor.into().value() / day_count_fraction.get_fraction()).unwrap(),
        ),
    };
    Some(InterestRate {
        rate: implied_rate,
        day_counter: day_count_convention,
        compounding,
        _marker: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use day_count_conventions::{Actual360, DayCountFraction};

    use itolib_time::Frequency;
    use itolib_types::{CompoundFactor, Percentage};

    use crate::{
        currency::USD, interest_rate::implied_rate_from_compound_factor, Compounding, InterestRate,
    };

    #[test]
    fn test_interest_rates() {
        let simple_rate: InterestRate<USD, Actual360> = InterestRate::new(
            Percentage::new(0.05),
            Actual360,
            Compounding::Simple(Frequency::Annual),
        );
        let compound_rate: InterestRate<USD, Actual360> = InterestRate::new(
            Percentage::new(0.05),
            Actual360,
            Compounding::Compounding(Frequency::Quarterly),
        );
        let continuous_rate: InterestRate<USD, Actual360> =
            InterestRate::new(Percentage::new(0.05), Actual360, Compounding::Continuous);

        // Getters
        assert_eq!(simple_rate.get_compounding(), Compounding::Simple(Frequency::Annual));
        assert_eq!(simple_rate.get_day_counter(), Actual360);
        assert_eq!(simple_rate.get_rate().value(), 0.05);

        // PartialEq
        assert_eq!(
            simple_rate,
            InterestRate::<USD, Actual360>::new(
                Percentage::new(0.05),
                Actual360,
                Compounding::Simple(Frequency::Annual)
            )
        );
        let dcf1 = DayCountFraction::new(1.0);
        // Discount Factor
        assert_approx_eq!(simple_rate.discount_factor(&dcf1).value().value(), 0.952_380_95, 10e-8);
        assert_approx_eq!(
            compound_rate.discount_factor(&dcf1).value().value(),
            0.951_524_27,
            10e-8
        );
        assert_approx_eq!(
            continuous_rate.discount_factor(&dcf1).value().value(),
            0.951_229_42,
            10e-8
        );

        // Implied Interest Rate from Compound Factor
        assert_approx_eq!(
            implied_rate_from_compound_factor::<USD, _>(
                CompoundFactor::new(1.5),
                &dcf1,
                Actual360,
                Compounding::Simple(Frequency::Annual)
            )
            .unwrap()
            .get_rate()
            .value()
            .value(),
            0.5,
            10e-8
        );
        assert_approx_eq!(
            *implied_rate_from_compound_factor::<USD, Actual360>(
                CompoundFactor::new(1.5),
                &DayCountFraction::new(1.0),
                Actual360,
                Compounding::Compounding(Frequency::Quarterly)
            )
            .unwrap()
            .get_rate()
            .value()
            .value(),
            0.426_727_67,
            10e-8
        );
        assert_approx_eq!(
            *implied_rate_from_compound_factor::<USD, Actual360>(
                CompoundFactor::new(1.5),
                &DayCountFraction::new(1.0),
                Actual360,
                Compounding::Continuous
            )
            .unwrap()
            .get_rate()
            .value()
            .value(),
            0.405_465_10,
            10e-8
        );
    }
}