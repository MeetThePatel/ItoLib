use itolib_money::{Currency, Money};

pub trait Instrument<C>
where
    C: Currency,
{
    fn npv(&self) -> Money<C>;

    // TODO:set_pricing_engine(Rc<PricingEngine>)
}
