use itolib_instruments::EuropeanOption;
use itolib_money::{Currency, Money};

pub trait Pricer<C>
where
    C: Currency,
{
    fn price(&self, option: &EuropeanOption<C>) -> Money<C>;

    fn price_vec(&self, options: &[EuropeanOption<C>]) -> Vec<Money<C>>;
}
