use std::fmt::Display;

use crate::instruments::exercises::{AmericanExercise, Exercise};
use crate::instruments::options::{Option, OptionType};
use crate::instruments::payoffs::{CallPutPayoff, Payoff, VanillaPayoff};
use crate::money::{Currency, MonetaryNumber};

pub struct AmericanOption<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    payoff: VanillaPayoff<N, C>,
    exercise: AmericanExercise,
}

impl<N, C> AmericanOption<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    #[must_use]
    pub const fn new(payoff: VanillaPayoff<N, C>, exercise: AmericanExercise) -> Self {
        Self { payoff, exercise }
    }
}

impl<N, C> Option<N, C> for AmericanOption<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn get_option_type(&self) -> OptionType {
        self.payoff.get_option_type()
    }

    fn get_payoff(&self) -> impl Payoff<N> {
        self.payoff
    }

    fn get_exercise(&self) -> impl Exercise {
        self.exercise
    }
}

impl<N, C> Display for AmericanOption<N, C>
where
    N: MonetaryNumber,
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_letter = match self.get_option_type() {
            OptionType::CALL => "C (A)",
            OptionType::PUT => "P (A)",
        };
        write!(
            f,
            "{} {} {}",
            self.exercise.get_last_date().format("%y/%m/%d"),
            self.payoff.get_strike(),
            type_letter,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instruments::exercises::AmericanExercise;
    use crate::instruments::options::{AmericanOption, OptionType};
    use crate::instruments::payoffs::VanillaPayoff;
    use crate::money::{currency::USD, Money};

    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn test_american_option_display() {
        let strike_price: Money<f64, USD> = Money::new(30.00);
        let option_type = OptionType::CALL;
        let payoff = VanillaPayoff::new(strike_price, option_type);

        let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str("24/07/27 00:00:00", "%y/%m/%d %H:%M:%S").unwrap(),
            Utc,
        );
        let exercise = AmericanExercise::new(date);

        let option = AmericanOption::new(payoff, exercise);

        assert_eq!(option.to_string(), "24/07/27 $ 30.00 C (A)");
    }
}
