use day_count_conventions::DayCounter;
use num::Bounded;
use ordered_float::OrderedFloat;

use crate::term_structures::volatility_structure::{
    BlackVolatilityTermStructure, VolatilityTermStructure,
};
use crate::term_structures::TermStructure;
use crate::term_structures::TermStructureStrikeValidity;
use crate::time::DateTime;
use crate::types::{Strike, Volatility};

use super::black_volatility_term_structure::BlackVolatilityTermStructureResult;

//  ------------------------------------------------------------------------------------------------
//  Definition
//  ------------------------------------------------------------------------------------------------

pub struct ConstantVolTermStructure<D>
where
    D: DayCounter,
{
    volatility: Volatility,

    reference_date: DateTime,

    day_count_convention: D,
}

#[derive(Debug)]
pub struct ConstantVolTermStructureBuilder<D>
where
    D: DayCounter,
{
    volatility: Option<Volatility>,
    reference_date: Option<DateTime>,
    day_count_convention: Option<D>,
}

#[derive(Debug, Copy, Clone)]
pub enum ConstantVolTermStructureBuilderError {
    NoVolatilityProvided,
    NegativeVolatility,
}

impl<D> ConstantVolTermStructureBuilder<D>
where
    D: DayCounter,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            volatility: None,
            reference_date: None,
            day_count_convention: None,
        }
    }

    pub fn volatility(&mut self, volatility: impl Into<Volatility>) -> &mut Self {
        self.volatility = Some(volatility.into());
        self
    }

    pub fn reference_date(&mut self, reference_date: DateTime) -> &mut Self {
        self.reference_date = Some(reference_date);
        self
    }

    pub fn day_count_convention(&mut self, day_count_convention: D) -> &mut Self {
        self.day_count_convention = Some(day_count_convention);
        self
    }

    pub fn build(
        &self,
    ) -> Result<ConstantVolTermStructure<D>, ConstantVolTermStructureBuilderError> {
        self.volatility.map_or(
            Err(ConstantVolTermStructureBuilderError::NoVolatilityProvided),
            |volatility| {
                if volatility < OrderedFloat(0.0) {
                    Err(ConstantVolTermStructureBuilderError::NegativeVolatility)
                } else {
                    Ok(ConstantVolTermStructure {
                        volatility,
                        reference_date: self.reference_date.unwrap_or_else(DateTime::now),
                        day_count_convention: self.day_count_convention.unwrap_or_default(),
                    })
                }
            },
        )
    }
}

impl<D> Default for ConstantVolTermStructureBuilder<D>
where
    D: DayCounter,
{
    fn default() -> Self {
        Self::new()
    }
}

//  ------------------------------------------------------------------------------------------------
//  Trait implementations
//  ------------------------------------------------------------------------------------------------

impl<D> TermStructure<D> for ConstantVolTermStructure<D>
where
    D: DayCounter,
{
    fn get_reference_date(&self) -> DateTime {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime {
        DateTime::new_from_ymd(9999, 12, 31)
    }

    fn is_datetime_valid(&self, dt: DateTime) -> bool {
        dt >= self.reference_date
    }

    fn get_day_counter(&self) -> D {
        self.day_count_convention
    }
}

impl<D> VolatilityTermStructure<D> for ConstantVolTermStructure<D>
where
    D: DayCounter,
{
    fn get_min_max_strike(&self) -> (Strike, Strike) {
        (Strike::min_value(), Strike::max_value())
    }

    fn validate_strike(&self, _strike: Strike) -> TermStructureStrikeValidity {
        TermStructureStrikeValidity::Valid
    }
}

impl<D> BlackVolatilityTermStructure<D> for ConstantVolTermStructure<D>
where
    D: DayCounter,
{
    fn black_volatility(
        &self,
        _maturity: DateTime,
        _strike: Strike,
    ) -> BlackVolatilityTermStructureResult {
        BlackVolatilityTermStructureResult::ExistingValue(self.volatility)
    }

    fn black_forward_volatility(
        &self,
        _start_date: DateTime,
        _end_date: DateTime,
        _strike: Strike,
    ) -> BlackVolatilityTermStructureResult {
        BlackVolatilityTermStructureResult::ExistingValue(self.volatility)
    }
}
