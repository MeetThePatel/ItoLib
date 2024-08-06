use std::fmt::Display;

use crate::instruments::exercises::{AmericanExercise, Exercise};
use crate::instruments::options::{Option, OptionType};
use crate::instruments::payoffs::{CallPutPayoff, Payoff, VanillaPayoff};
use crate::money::Currency;

pub struct AmericanOption<C>
where
    C: Currency,
{
    payoff: VanillaPayoff<C>,
    exercise: AmericanExercise,
}

impl<C> AmericanOption<C>
where
    C: Currency,
{
    #[must_use]
    pub const fn new(payoff: VanillaPayoff<C>, exercise: AmericanExercise) -> Self {
        Self { payoff, exercise }
    }
}

impl<C> Option<C> for AmericanOption<C>
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

impl<C> Display for AmericanOption<C>
where
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
            self.exercise.get_last_date().format_ymd(),
            self.payoff.get_strike(),
            type_letter,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruments::exercises::AmericanExercise;
    use crate::instruments::options::{AmericanOption, OptionType};
    use crate::instruments::payoffs::VanillaPayoff;
    use crate::money::{currency::USD, Money};
    use crate::time::DateTime;

    #[test]
    fn test_american_option_display() {
        let strike_price: Money<USD> = Money::new(30.00);
        let call_payoff = VanillaPayoff::new(strike_price, OptionType::CALL);
        let put_payoff = VanillaPayoff::new(strike_price, OptionType::PUT);

        let date: DateTime = DateTime::new_from_ymd(2024, 7, 27);
        let exercise = AmericanExercise::new(date);

        let call = AmericanOption::new(call_payoff, exercise);
        let put = AmericanOption::new(put_payoff, exercise);

        assert_eq!(call.to_string(), "2024/07/27 $ 30.00 C (A)");
        assert_eq!(put.to_string(), "2024/07/27 $ 30.00 P (A)");
        assert_eq!(call.get_payoff().to_string(), "$ 30.00 CALL");
        assert_eq!(
            call.get_exercise().get_dates(),
            &[DateTime::new_from_ymd(2024, 7, 27)]
        );
    }
}
