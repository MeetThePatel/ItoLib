use day_count_conventions::DayCounter;
use itolib_types::Strike;
use statrs::distribution::{ContinuousCDF, Normal};

use itolib_instruments::{EuropeanOption, Exercise, Option, OptionType};
use itolib_money::{Currency, Money};
use itolib_term_structures::{
    volatility::{BlackVolatilityTermStructure, BlackVolatilityTermStructureResult},
    yieldd::YieldTermStructure,
};

use crate::Pricer;

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
        Self { underlying_spot, volatility_curve, yield_curve }
    }
}

impl<'a, C, D> Pricer<C> for AnalyticBlackScholesMerton<'a, C, D>
where
    C: Currency,
    D: DayCounter,
{
    #[allow(clippy::many_single_char_names)]
    fn price(&self, option: &EuropeanOption<C>) -> Money<C> {
        use BlackVolatilityTermStructureResult::{
            ExistingValue, InterpolatedValue, NoPoints, OutOfRange,
        };

        let dcc = D::default();

        let t = option.get_exercise().get_last_date();

        let k = option.get_strike();

        let s = self.underlying_spot;

        let sigma = match self.volatility_curve.black_volatility(
            self.volatility_curve.get_reference_date(),
            Strike::new(s.amount()).unwrap(),
        ) {
            ExistingValue(v) | InterpolatedValue(v) => v,
            OutOfRange => panic!("Out of range."),
            NoPoints => panic!("No points."),
        };

        // Discount factor.
        let d = self.yield_curve.discount_factor(t).unwrap();
        // Forward price of underlying.
        let f: Money<C> = s / d.value().value();

        let tau =
            dcc.day_count_fraction(&self.volatility_curve.get_reference_date(), &t).get_fraction();
        let d_plus = (0.5 * *sigma.value().value() * *sigma.value().value())
            .mul_add(tau, (f / k).amount().value().ln())
            / *(sigma.value().value() * tau.sqrt());
        let d_minus = (*sigma.value().value()).mul_add(-tau.sqrt(), d_plus);

        let norm = Normal::standard();
        match option.get_option_type() {
            OptionType::CALL => {
                let call_price = d.value().value()
                    * norm
                        .cdf(d_plus)
                        .mul_add(f.amount().into(), -(norm.cdf(d_minus) * k.amount().value()));
                Money::new(call_price)
            }
            OptionType::PUT => {
                let put_price = d.value().value()
                    * norm.cdf(-1.0 * d_minus).mul_add(
                        k.amount().into(),
                        -(norm.cdf(-1.0 * d_plus) * f.amount().value()),
                    );
                Money::new(put_price)
            }
        }
    }

    fn price_vec(&self, options: &[EuropeanOption<C>]) -> Vec<Money<C>> {
        options.iter().map(|option| self.price(option)).collect()
    }
}
