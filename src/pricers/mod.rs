mod analytic_black_scholes_merton;
pub use analytic_black_scholes_merton::AnalyticBlackScholesMerton;

use crate::{
    instruments::EuropeanOption,
    money::{Currency, Money},
};

pub trait Pricer<C>
where
    C: Currency,
{
    fn price(&self, option: &EuropeanOption<C>) -> Money<C>;

    fn price_vec(&self, options: &[EuropeanOption<C>]) -> Vec<Money<C>>;
}
