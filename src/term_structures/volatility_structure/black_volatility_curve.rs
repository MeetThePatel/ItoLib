use day_count_conventions::DayCounter;
use derive_builder::{Builder, UninitializedFieldError};
use num::Bounded;

use crate::math::interpolation::Interpolator;
use crate::term_structures::volatility_structure::{
    BlackVolatilityTermStructure, VolatilityTermStructure,
};
use crate::term_structures::{
    TermStructure, TermStructureDateTimeValidity, TermStructureStrikeValidity,
};
use crate::time::DateTime;
use crate::types::{Strike, Volatility};

//  ------------------------------------------------------------------------------------------------
//  Definition
//  ------------------------------------------------------------------------------------------------

#[derive(Builder)]
#[builder(build_fn(
    validate = "Self::validate",
    error = "BlackVolatilityCurveBuilderError"
))]
pub struct BlackVolatilityCurve<I>
where
    I: Interpolator<DateTime, Volatility>,
{
    interpolator: I,

    #[builder(default = "DateTime::now()")]
    reference_date: DateTime,
}

//  ------------------------------------------------------------------------------------------------
//  Builder
//  ------------------------------------------------------------------------------------------------

impl<I> BlackVolatilityCurveBuilder<I>
where
    I: Interpolator<DateTime, Volatility>,
{
    pub fn add_points(&mut self, points: &[(DateTime, Volatility)]) -> &mut Self {
        todo!();
        // let mut pts = points.to_vec();
        // pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // let (mut datetimes, mut volatilities) = pts.into_iter().unzip();

        // if self.datetimes.is_none() {
        //     self.datetimes = Some(datetimes);
        // } else {
        //     self.datetimes.as_mut().unwrap().append(&mut datetimes);
        // }
        // if self.volatilities.is_none() {
        //     self.volatilities = Some(volatilities);
        // } else {
        //     self.volatilities
        //         .as_mut()
        //         .unwrap()
        //         .append(&mut volatilities);
        // }

        self
    }

    pub fn validate(&self) -> Result<(), BlackVolatilityCurveBuilderError> {
        // Check if empty datetimes.
        // if self.datetimes.is_none() {
        //     return Err(BlackVolatilityCurveBuilderError::NoDateTimesProvided);
        // }

        // Check if empty volatilities.
        // if self.volatilities.is_none() {
        //     return Err(BlackVolatilityCurveBuilderError::NoVolatilitiesProvided);
        // }

        // Check if even lengths.
        // let datetimes_len = self.datetimes.as_ref().unwrap().len();
        // let volatilities_len = self.volatilities.as_ref().unwrap().len();
        // if datetimes_len != volatilities_len {
        //     return Err(BlackVolatilityCurveBuilderError::UnevenLengths);
        // }

        Ok(())
    }

    // pub fn build(&self) -> Result<BlackVolatilityCurve<I>, BlackVolatilityCurveBuilderError> {
    //     todo!();
    //     // let interpolator = todo!();
    //     // Ok(BlackVolatilityCurve {
    //     //     datetimes: self.datetimes.as_ref().unwrap().to_vec(),
    //     //     volatilities: self.volatilities.as_ref().unwrap().to_vec(),
    //     //     interpolator,
    //     //     reference_date: self.reference_date.unwrap(),
    //     // })
    // }
}

pub enum BlackVolatilityCurveBuilderError {
    NoDateTimesProvided,
    NoVolatilitiesProvided,
    NoInterpolatorProvided,
    UnevenLengths,
}
impl From<UninitializedFieldError> for BlackVolatilityCurveBuilderError {
    fn from(_value: UninitializedFieldError) -> Self {
        Self::NoInterpolatorProvided
    }
}

//  ------------------------------------------------------------------------------------------------
//  Trait implementations
//  ------------------------------------------------------------------------------------------------

impl<D, I> TermStructure<D> for BlackVolatilityCurve<I>
where
    D: DayCounter,
    I: Interpolator<DateTime, Volatility>,
{
    fn get_reference_date(&self) -> DateTime {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime {
        todo!()
        // self.datetimes.last().copied().unwrap()
    }

    fn validate_datetime(&self, dt: DateTime) -> TermStructureDateTimeValidity {
        if dt >= <Self as TermStructure<D>>::get_reference_date(self)
            && dt <= <Self as TermStructure<D>>::get_max_datetime(self)
        {
            TermStructureDateTimeValidity::Valid
        } else {
            TermStructureDateTimeValidity::Invalid
        }
    }
}

impl<D, I> VolatilityTermStructure<D> for BlackVolatilityCurve<I>
where
    D: DayCounter,
    I: Interpolator<DateTime, Volatility>,
{
    fn get_min_max_strike(&self) -> (Strike, Strike) {
        (Strike::min_value(), Strike::max_value())
    }

    fn validate_strike(&self, _strike: Strike) -> TermStructureStrikeValidity {
        TermStructureStrikeValidity::Valid
    }
}

impl<D, I> BlackVolatilityTermStructure<D> for BlackVolatilityCurve<I>
where
    D: DayCounter,
    I: Interpolator<DateTime, Volatility>,
{
    fn black_volatility(&self, maturity: DateTime, strike: Strike) -> Volatility {
        todo!()
    }

    fn black_forward_volatility(
        &self,
        start_date: DateTime,
        end_date: DateTime,
        strike: Strike,
    ) -> Volatility {
        todo!()
    }
}
