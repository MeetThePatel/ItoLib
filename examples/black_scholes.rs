use day_count_conventions::Thirty360;
use itolib::{
    compounding::Compounding,
    instruments::{
        exercises::EuropeanExercise,
        options::{EuropeanOption, OptionType},
        payoffs::VanillaPayoff,
    },
    interest_rate::InterestRate,
    money::{currency::USD, Money},
    term_structures::{
        volatility_structure::constant_vol_term_structure::ConstantVolTermStructureBuilder,
        yield_structure::flat_forward_term_structure::FlatForwardTermStructureBuilder,
    },
    time::DateTime,
};

fn main() {
    let strike_price: Money<USD> = Money::new(105.0);
    let spot_rate: InterestRate<USD, Thirty360> =
        InterestRate::new(0.02, Thirty360, Compounding::Continuous);

    let payoff = VanillaPayoff::new(strike_price, OptionType::CALL);
    let exercise = EuropeanExercise::new(DateTime::new_from_ymd(2024, 9, 1));

    let call = EuropeanOption::new(payoff, exercise);

    let vol_curve = ConstantVolTermStructureBuilder::default()
        .reference_date(DateTime::now())
        .volatility(0.05)
        .build()
        .unwrap();

    let yield_curve = FlatForwardTermStructureBuilder::default()
        .reference_date(DateTime::now())
        .rate(spot_rate)
        .build()
        .unwrap();
}
