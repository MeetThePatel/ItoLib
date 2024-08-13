pub trait Pricer<C>
where
    C: Currency,
{
    fn price(&self, option: EuropeanOption<C>) -> Money<C>;
}

pub mod analytic_black_scholes_merton;
pub use analytic_black_scholes_merton::AnalyticBlackScholesMerton;

use crate::{
    instruments::options::EuropeanOption,
    money::{Currency, Money},
};
