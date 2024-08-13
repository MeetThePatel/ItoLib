use std::marker::PhantomData;

use day_count_conventions::DayCounter;
use derive_builder::{Builder, UninitializedFieldError};
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

#[derive(Builder)]
#[builder(build_fn(
    validate = "Self::validate",
    error = "ConstantVolTermStructureBuilderError"
))]
pub struct ConstantVolTermStructure<D>
where
    D: DayCounter,
{
    #[builder(setter(into))]
    volatility: Volatility,

    #[builder(default = "DateTime::now()")]
    reference_date: DateTime,

    #[builder(default = "D::default()")]
    day_count_convention: D,
}

//  ------------------------------------------------------------------------------------------------
//  Builder
//  ------------------------------------------------------------------------------------------------

impl<D> ConstantVolTermStructureBuilder<D>
where
    D: DayCounter,
{
    fn validate(&self) -> Result<(), ConstantVolTermStructureBuilderError> {
        // Check if volatility is negative.
        if let Some(vol) = self.volatility {
            if vol < OrderedFloat(0.0) {
                return Err(ConstantVolTermStructureBuilderError::NegativeVolatility);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConstantVolTermStructureBuilderError {
    NoVolatilityProvided,
    NegativeVolatility,
}
impl From<UninitializedFieldError> for ConstantVolTermStructureBuilderError {
    fn from(_value: UninitializedFieldError) -> Self {
        Self::NoVolatilityProvided
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
