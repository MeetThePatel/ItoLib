use day_count_conventions::DayCounter;
use ordered_float::OrderedFloat;
use statrs::distribution::{ContinuousCDF, Normal};

use crate::{
    instruments::{
        exercises::Exercise,
        options::{EuropeanOption, Option, OptionType},
    },
    money::{Currency, Money},
    pricers::Pricer,
    term_structures::{
        volatility_structure::{
            black_volatility_term_structure::BlackVolatilityTermStructureResult,
            BlackVolatilityTermStructure,
        },
        yield_structure::YieldTermStructure,
    },
};

pub struct AnalyticBlackScholesMerton<'a, C, D>
where
    C: Currency,
    D: DayCounter,
{
    underlying_spot: Money<C>,
    volatility_curve: &'a dyn BlackVolatilityTermStructure<D>,
    yield_curve: &'a dyn YieldTermStructure<C, D>,
}

impl<'a, C, D> AnalyticBlackScholesMerton<'a, C, D>
where
    C: Currency,
    D: DayCounter,
{
    pub fn new(
        underlying_spot: Money<C>,
        volatility_curve: &'a impl BlackVolatilityTermStructure<D>,
        yield_curve: &'a impl YieldTermStructure<C, D>,
    ) -> Self {
        Self {
            underlying_spot,
            volatility_curve,
            yield_curve,
        }
    }
}

impl<'a, C, D> Pricer<C> for AnalyticBlackScholesMerton<'a, C, D>
where
    C: Currency,
    D: DayCounter,
{
    fn price(&self, option: EuropeanOption<C>) -> Money<C> {
        let dcc = D::default();

        let t = option.get_exercise().get_last_date();

        let k = option.get_strike();

        let s = self.underlying_spot;
        let sigma = match self
            .volatility_curve
            .black_volatility(self.volatility_curve.get_reference_date(), s.into())
        {
            BlackVolatilityTermStructureResult::InterpolatedValue(v) => v,
            BlackVolatilityTermStructureResult::ExistingValue(v) => v,
            BlackVolatilityTermStructureResult::OutOfRange => panic!("Out of range."),
            BlackVolatilityTermStructureResult::NoPoints => panic!("No points."),
        };

        // Discount factor.
        let d = self.yield_curve.discount_factor(t).unwrap();
        // Forward price of underlying.
        let f: Money<C> = s / d;

        let tau = dcc
            .day_count_fraction(&self.volatility_curve.get_reference_date(), &t)
            .get_fraction();
        let d_plus = ((f / k).amount.ln() + 0.5 * *sigma * *sigma * tau) / *(sigma * tau.sqrt());
        let d_minus = d_plus - *sigma * tau.sqrt();

        let norm = Normal::standard();
        match option.get_option_type() {
            OptionType::CALL => {
                let call_price = d * (norm.cdf(d_plus) * *f.amount - norm.cdf(d_minus) * *k.amount);
                Money::new(call_price)
            }
            OptionType::PUT => {
                let put_price = d
                    * (norm.cdf(-1.0 * d_minus) * *k.amount - norm.cdf(-1.0 * d_plus) * *f.amount);
                Money::new(put_price)
            }
        }
    }
}
