use std::collections::BTreeMap;
use std::fmt::Display;

use crate::macros::any_true;
use crate::money::{Currency, ExchangeRate, Money};
use crate::types::MonetaryNumber;

#[derive(Debug, Default, Clone)]
pub struct ExchangeRateManager<'a> {
    exchange_rate_map: BTreeMap<(&'a str, &'a str), MonetaryNumber>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeRateManagerError {
    AlreadyExists,
    ExchangeRateNotFound,
}

impl<'a> ExchangeRateManager<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            exchange_rate_map: BTreeMap::new(),
        }
    }
}

impl<'a> ExchangeRateManager<'a> {
    pub fn insert<B, Q>(
        &mut self,
        rate: &ExchangeRate<B, Q>,
    ) -> Result<(), ExchangeRateManagerError>
    where
        B: Currency,
        Q: Currency,
    {
        let base_currency = rate.get_base_currency();
        let quote_currency = rate.get_quote_currency();

        // Check if already in ExchangeRateManager.
        if self.get(&base_currency, &quote_currency).is_ok() {
            return Err(ExchangeRateManagerError::AlreadyExists);
        }

        let base_code = base_currency.get_alphabetic_code();
        let quote_code = quote_currency.get_alphabetic_code();

        self.exchange_rate_map
            .insert((base_code, quote_code), rate.rate);
        Ok(())
    }

    pub fn get<B, Q>(
        &self,
        base: &B,
        quote: &Q,
    ) -> Result<ExchangeRate<B, Q>, ExchangeRateManagerError>
    where
        B: Currency,
        Q: Currency,
    {
        let base_code = base.get_alphabetic_code();
        let quote_code = quote.get_alphabetic_code();
        self.exchange_rate_map
            .get(&(base_code, quote_code))
            .map_or(Err(ExchangeRateManagerError::ExchangeRateNotFound), |a| {
                Ok(ExchangeRate::new(*a))
            })
    }

    pub fn contains_key<B, Q>(&self, base: &B, quote: &Q) -> Option<(&str, &str)>
    where
        B: Currency,
        Q: Currency,
    {
        let base_code = base.get_alphabetic_code();
        let quote_code = quote.get_alphabetic_code();
        match any_true!(
            self.exchange_rate_map
                .contains_key(&(base_code, quote_code)),
            self.exchange_rate_map
                .contains_key(&(quote_code, base_code))
        ) {
            Some(0) => Some((base_code, quote_code)),
            Some(1) => Some((quote_code, base_code)),
            _ => None,
        }
    }

    pub fn update<B, Q>(&mut self, rate: &ExchangeRate<B, Q>) -> Option<ExchangeRate<B, Q>>
    where
        B: Currency,
        Q: Currency,
    {
        let base_code = rate.get_base_currency().get_alphabetic_code();
        let quote_code = rate.get_quote_currency().get_alphabetic_code();
        self.exchange_rate_map
            .insert((base_code, quote_code), rate.rate)
            .map(ExchangeRate::new)
    }

    pub fn remove<B, Q>(&mut self, base: &B, quote: &Q) -> Option<ExchangeRate<B, Q>>
    where
        B: Currency,
        Q: Currency,
    {
        let base_code = base.get_alphabetic_code();
        let quote_code = quote.get_alphabetic_code();
        self.exchange_rate_map
            .remove(&(base_code, quote_code))
            .map(ExchangeRate::new)
    }

    pub fn clear(&mut self) {
        self.exchange_rate_map.clear();
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.exchange_rate_map.len()
    }

    pub fn convert<C1, C2>(
        &self,
        amount: &Money<C1>,
        _target_currency: &C2,
    ) -> Result<Money<C2>, ExchangeRateManagerError>
    where
        C1: Currency,
        C2: Currency,
    {
        self.get(&C1::default(), &C2::default()).map_or_else(
            |_| {
                self.get(&C2::default(), &C1::default()).map_or(
                    Err(ExchangeRateManagerError::ExchangeRateNotFound),
                    |rate| Ok(rate.convert_to_base(amount)),
                )
            },
            |rate| Ok(rate.convert_to_quote(amount)),
        )
    }
}

impl<'a> Display for ExchangeRateManager<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "+-----------+----------------------+");
        let _ = writeln!(f, "| {:^9} | {:>20} |", "Pair", "Rate");
        let _ = writeln!(f, "+-----------+----------------------+");
        for ((base_currency, quote_currency), rate) in &self.exchange_rate_map {
            let _ = writeln!(
                f,
                "| {base_currency:^3} / {quote_currency:^3} | {rate:>20} |",
            );
        }
        let _ = writeln!(f, "+-----------+----------------------+");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal_f64;
    use crate::macros::assert_approx_equal_Money;
    use crate::money::{
        currency::{EUR, GBP, JPY, USD},
        ExchangeRate, ExchangeRateManagerError, Money,
    };

    use super::ExchangeRateManager;

    #[test]
    fn test_exchange_rate_manager_operations() {
        let mut manager: ExchangeRateManager = ExchangeRateManager::new();
        let gbpusd: ExchangeRate<GBP, USD> = ExchangeRate::new(1.28510);
        let eurusd: ExchangeRate<EUR, USD> = ExchangeRate::new(1.08564);

        // Test insert.
        assert!(manager.insert(&gbpusd).is_ok());
        assert_eq!(
            manager.insert(&gbpusd),
            Err(ExchangeRateManagerError::AlreadyExists)
        );

        // Test contains_key.
        assert_eq!(
            manager.contains_key(&GBP::default(), &USD::default()),
            Some(("GBP", "USD"))
        );
        assert_eq!(
            manager.contains_key(&USD::default(), &GBP::default()),
            Some(("GBP", "USD"))
        );
        assert_eq!(manager.contains_key(&EUR::default(), &USD::default()), None);

        // Test size
        manager.insert(&eurusd).unwrap();
        assert_eq!(manager.size(), 2);

        // Test get.
        let rate = manager.get(&GBP::default(), &USD::default()).unwrap();
        assert_approx_equal_f64!(rate.rate, 1.28510, 10e-8);

        // Test convert to base.
        let m1: Money<USD> = Money::new(1.0);
        let expected: Money<GBP> = Money::new(0.778_149_56);
        assert_approx_equal_Money!(
            manager.convert(&m1, &GBP::default()).unwrap(),
            expected,
            10e-7
        );

        // Test convert to quote.
        let m2: Money<GBP> = Money::new(1.0);
        let expected: Money<USD> = Money::new(1.28510);
        assert_approx_equal_Money!(
            manager.convert(&m2, &USD::default()).unwrap(),
            expected,
            10e-7
        );

        // Test update.
        let gbpusd: ExchangeRate<GBP, USD> = ExchangeRate::new(1.28512);
        manager.update(&gbpusd);
        let rate = manager.get(&GBP::default(), &USD::default()).unwrap();
        assert_approx_equal_f64!(rate.rate, 1.28512_f64, 10e-8);

        // Test remove.
        let rate = manager.remove(&GBP::default(), &USD::default()).unwrap();
        assert_approx_equal_f64!(rate.rate, 1.28512_f64, 10e-8);

        // Test clear.
        manager.clear();
        assert_eq!(manager.size(), 0);
    }

    #[test]
    fn test_exchange_rate_manager_display() {
        let gbpusd: ExchangeRate<GBP, USD> = ExchangeRate::new(1.28510);
        let eurusd: ExchangeRate<EUR, USD> = ExchangeRate::new(1.08485);
        let gbpeur: ExchangeRate<GBP, EUR> = ExchangeRate::new(1.1872);
        let usdjpy: ExchangeRate<USD, JPY> = ExchangeRate::new(153.6380);

        let mut manager: ExchangeRateManager = ExchangeRateManager::new();
        let _ = manager.insert(&gbpusd);
        let _ = manager.insert(&gbpeur);
        let _ = manager.insert(&eurusd);
        let _ = manager.insert(&usdjpy);

        let expected = "+-----------+----------------------+
|   Pair    |                 Rate |
+-----------+----------------------+
| EUR / USD |              1.08485 |
| GBP / EUR |               1.1872 |
| GBP / USD |               1.2851 |
| USD / JPY |              153.638 |
+-----------+----------------------+\n";

        assert_eq!(manager.to_string(), expected);
    }
}
