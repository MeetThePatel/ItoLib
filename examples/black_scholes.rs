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
    pricers::{AnalyticBlackScholesMerton, Pricer},
    term_structures::{
        volatility_structure::ConstantVolTermStructureBuilder,
        yield_structure::FlatForwardTermStructureBuilder,
    },
    time::DateTime,
};

fn main() {
    let underlying_spot_price: Money<USD> = Money::new(100.0);

    let strike_price = Money::new(105.0);
    let spot_rate: InterestRate<USD, Thirty360> =
        InterestRate::new(0.1, Thirty360, Compounding::Continuous);

    let payoff = VanillaPayoff::new(strike_price, OptionType::CALL);
    let exercise = EuropeanExercise::new(DateTime::new_from_ymd(2025, 1, 1));

    let call = EuropeanOption::new(payoff, exercise);

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

    println!("{}", bsm_pricer.price(call));
}
