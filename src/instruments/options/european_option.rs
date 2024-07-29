use std::fmt::Display;

use crate::instruments::exercises::{EuropeanExercise, Exercise};
use crate::instruments::options::{Option, OptionType};
use crate::instruments::payoffs::{CallPutPayoff, Payoff, VanillaPayoff};
use crate::money::Currency;

pub struct EuropeanOption<C>
where
    C: Currency,
{
    payoff: VanillaPayoff<C>,
    exercise: EuropeanExercise,
}

impl<C> EuropeanOption<C>
where
    C: Currency,
{
    #[must_use]
    pub const fn new(payoff: VanillaPayoff<C>, exercise: EuropeanExercise) -> Self {
        Self { payoff, exercise }
    }
}

impl<C> Option<C> for EuropeanOption<C>
where
    C: Currency,
{
    fn get_option_type(&self) -> OptionType {
        self.payoff.get_option_type()
    }

    fn get_payoff(&self) -> impl Payoff {
        self.payoff
    }

    fn get_exercise(&self) -> impl Exercise {
        self.exercise
    }
}

impl<C> Display for EuropeanOption<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_letter = match self.get_option_type() {
            OptionType::CALL => "C (E)",
            OptionType::PUT => "P (E)",
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
    use crate::instruments::exercises::EuropeanExercise;
    use crate::instruments::options::{EuropeanOption, OptionType};
    use crate::instruments::payoffs::VanillaPayoff;
    use crate::money::{currency::USD, Money};

    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn test_european_option_display() {
        let strike_price: Money<USD> = Money::new(30.00);
        let option_type = OptionType::CALL;
        let payoff = VanillaPayoff::new(strike_price, option_type);

        let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str("24/07/27 00:00:00", "%y/%m/%d %H:%M:%S").unwrap(),
            Utc,
        );
        let exercise = EuropeanExercise::new(date);

        let option = EuropeanOption::new(payoff, exercise);

        assert_eq!(option.to_string(), "24/07/27 $ 30.00 C (E)");
    }
}
