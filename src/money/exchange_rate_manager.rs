use std::{collections::BTreeMap, fmt::Display};

use crate::money::{Currency, ExchangeRate};

use num::Num;

#[derive(Debug, Default, Clone)]
pub struct ExchangeRateManager<'a, N>
where
    N: Num + PartialOrd + Clone,
{
    exchange_rate_map: BTreeMap<(&'a str, &'a str), N>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeRateManagerError {
    AlreadyExists,
    ExchangeRateNotFound,
}

impl<'a, N> ExchangeRateManager<'a, N>
where
    N: Num + PartialOrd + Clone,
{
    pub fn new() -> Self {
        Self {
            exchange_rate_map: BTreeMap::new(),
        }
    }
}

impl<'a, N> ExchangeRateManager<'a, N>
where
    N: Num + PartialOrd + Copy,
{
    pub fn insert<B, Q>(
        &mut self,
        rate: &ExchangeRate<N, B, Q>,
    ) -> Result<(), ExchangeRateManagerError>
    where
        B: Currency,
        Q: Currency,
    {
        let base_currency = rate.get_base_currency();
        let quote_currency = rate.get_quote_currency();

        // Check if already in ExchangeRateManager.
        if self.lookup(&base_currency, &quote_currency).is_ok() {
            return Err(ExchangeRateManagerError::AlreadyExists);
        }

        let base_code = base_currency.get_alphabetic_code();
        let quote_code = quote_currency.get_alphabetic_code();

        self.exchange_rate_map
            .insert((base_code, quote_code), rate.rate);
        Ok(())
    }

    pub fn lookup<B, Q>(
        &self,
        base: &B,
        quote: &Q,
    ) -> Result<ExchangeRate<N, B, Q>, ExchangeRateManagerError>
    where
        B: Currency,
        Q: Currency,
    {
        let base_code = base.get_alphabetic_code();
        let quote_code = quote.get_alphabetic_code();
        match self.exchange_rate_map.get(&(base_code, quote_code)) {
            Some(a) => Ok(ExchangeRate::new(*a)),
            None => Err(ExchangeRateManagerError::ExchangeRateNotFound),
        }
    }

    pub fn update<B, Q>(&mut self, rate: &ExchangeRate<N, B, Q>) -> Option<ExchangeRate<N, B, Q>>
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

    pub fn remove<B, Q>(&mut self, base: &B, quote: &Q) -> Option<ExchangeRate<N, B, Q>>
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

    // TODO: Implement `manager::convert(&self, amount, curr1) -> curr2` with logic to
    //       auto-select exchange rate to use, and direction to go.
}

impl<'a, N> Display for ExchangeRateManager<'a, N>
where
    N: Num + PartialOrd + Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "+-----------+----------------------+");
        let _ = writeln!(f, "| {:^9} | {:>20} |", "Pair", "Rate");
        let _ = writeln!(f, "+-----------+----------------------+");
        for ((base_currency, quote_currency), rate) in &self.exchange_rate_map {
            let _ = writeln!(
                f,
                "| {:^3} / {:^3} | {:>20} |",
                base_currency, quote_currency, rate
            );
        }
        let _ = writeln!(f, "+-----------+----------------------+");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{
        currency::{EUR, GBP, JPY, USD},
        ExchangeRate, ExchangeRateManagerError,
    };

    use super::ExchangeRateManager;

    #[test]
    fn test_exchange_rate_manager() {
        let gbpusd: ExchangeRate<f64, GBP, USD> = ExchangeRate::new(1.28510);
        let mut manager: ExchangeRateManager<f64> = ExchangeRateManager::new();

        // Test insert.
        assert!(manager.insert(&gbpusd).is_ok());
        assert_eq!(
            manager.insert(&gbpusd),
            Err(ExchangeRateManagerError::AlreadyExists)
        );

        // Test lookup.
        let rate = manager.lookup(&GBP::default(), &USD::default()).unwrap();
        assert_eq!(rate.rate, 1.28510);

        // Test update.
        let gbpusd: ExchangeRate<f64, GBP, USD> = ExchangeRate::new(1.28512);
        manager.update(&gbpusd);
        let rate = manager.lookup(&GBP::default(), &USD::default()).unwrap();
        assert_eq!(rate.rate, 1.28512);

        // Test remove.
        let rate = manager.remove(&GBP::default(), &USD::default()).unwrap();
        assert_eq!(rate.rate, 1.28512);
    }

    #[test]
    fn test_exchange_rate_manager_display() {
        let gbpusd: ExchangeRate<f64, GBP, USD> = ExchangeRate::new(1.28510);
        let eurusd: ExchangeRate<f64, EUR, USD> = ExchangeRate::new(1.08485);
        let gbpeur: ExchangeRate<f64, GBP, EUR> = ExchangeRate::new(1.1872);
        let usdjpy: ExchangeRate<f64, USD, JPY> = ExchangeRate::new(153.6380);

        let mut manager: ExchangeRateManager<f64> = ExchangeRateManager::new();
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
