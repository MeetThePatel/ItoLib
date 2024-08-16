use day_count_conventions::Thirty360;

use itolib::{
    currency::USD,
    instruments::{EuropeanExercise, EuropeanOption, OptionType, VanillaPayoff},
    pricers::{AnalyticBlackScholesMerton, Pricer},
    term_structures::{ConstantVolTermStructureBuilder, FlatForwardTermStructureBuilder},
    time::{DateTime, Duration},
    Compounding, InterestRate, Money,
};

fn main() {
    let mut exercises = Vec::new();
    for i in 1..=12 {
        let tmp = EuropeanExercise::new(DateTime::now() + Duration::new_from_days(30.0) * i);
        exercises.push(tmp);
    }

    let underlying_spot_price: Money<USD> = Money::new(100.0);

    let strike_price = Money::new(105.0);
    let spot_rate: InterestRate<USD, Thirty360> =
        InterestRate::new(0.1, Thirty360, Compounding::Continuous);

    let payoff = VanillaPayoff::new(strike_price, OptionType::CALL);

    let mut calls = Vec::new();
    for exercise in exercises {
        let tmp = EuropeanOption::new(payoff, exercise);
        calls.push(tmp)
    }

    let vol_curve = ConstantVolTermStructureBuilder::new()
        .reference_date(DateTime::now())
        .volatility(0.05)
        .build()
        .unwrap();

    let yield_curve = FlatForwardTermStructureBuilder::new()
        .reference_date(DateTime::now())
        .rate(spot_rate)
        .build()
        .unwrap();

    let bsm_pricer =
        AnalyticBlackScholesMerton::new(underlying_spot_price, &vol_curve, &yield_curve);

    dbg!(bsm_pricer.price_vec(&calls));
}
