use day_count_conventions::DayCounter;
use num::Bounded;

use crate::math::interpolation::{InterpolationResult, Interpolator};
use crate::term_structures::volatility_structure::{
    BlackVolatilityTermStructure, VolatilityTermStructure,
};
use crate::term_structures::{TermStructure, TermStructureStrikeValidity};
use crate::time::DateTime;
use crate::types::{Strike, Volatility};

use super::black_volatility_term_structure::BlackVolatilityTermStructureResult;

//  ------------------------------------------------------------------------------------------------
//  Definition
//  ------------------------------------------------------------------------------------------------

pub struct BlackVolatilityCurve<I, D>
where
    I: Interpolator<DateTime, Volatility>,
    D: DayCounter,
{
    interpolator: I,

    reference_date: DateTime,

    day_counter: D,
}

//  ------------------------------------------------------------------------------------------------
//  Builder
//  ------------------------------------------------------------------------------------------------

#[allow(clippy::module_name_repetitions)]
pub struct BlackVolatilityCurveBuilder<I, D>
where
    I: Interpolator<DateTime, Volatility>,
    D: DayCounter,
{
    interpolator: I,
    reference_date: Option<DateTime>,
    day_count_convention: Option<D>,
}

impl<I, D> BlackVolatilityCurveBuilder<I, D>
where
    I: Interpolator<DateTime, Volatility>,
    D: DayCounter,
{
    /// Add a point to the volatility curve.
    pub fn add_point(&mut self, point: (DateTime, Volatility)) -> &mut Self {
        let _ = self.interpolator.add_point(point);
        self
    }

    /// Add points to the volatility curve.
    pub fn add_points(&mut self, points: &[(DateTime, Volatility)]) -> &mut Self {
        let _ = self.interpolator.add_points(points.to_vec());
        self
    }

    /// Set the reference date for the volatility curve.
    pub fn reference_date(&mut self, reference_date: DateTime) -> &mut Self {
        self.reference_date = Some(reference_date);
        self
    }

    pub fn day_count_convention(&mut self, day_count_convention: D) -> &mut Self {
        self.day_count_convention = Some(day_count_convention);
        self
    }

    /// Build the volatility curve.
    pub fn build(self) -> BlackVolatilityCurve<I, D> {
        let reference_date = self.reference_date.map_or_else(DateTime::now, |d| d);
        let day_counter = self.day_count_convention.unwrap_or_default();
        BlackVolatilityCurve {
            interpolator: self.interpolator,
            // Safe to unwrap, because we gave default value above.
            reference_date,
            day_counter,
        }
    }
}

//  ------------------------------------------------------------------------------------------------
//  Trait implementations
//  ------------------------------------------------------------------------------------------------

impl<D, I> TermStructure<D> for BlackVolatilityCurve<I, D>
where
    D: DayCounter,
    I: Interpolator<DateTime, Volatility>,
{
    fn get_reference_date(&self) -> DateTime {
        self.reference_date
    }

    fn get_max_datetime(&self) -> DateTime {
        self.interpolator.range().unwrap().1
    }

    fn get_day_counter(&self) -> D {
        self.day_counter
    }

    fn is_datetime_valid(&self, dt: DateTime) -> bool {
        dt >= <Self as TermStructure<D>>::get_reference_date(self)
            && dt <= <Self as TermStructure<D>>::get_max_datetime(self)
    }
}

impl<D, I> VolatilityTermStructure<D> for BlackVolatilityCurve<I, D>
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

impl<D, I> BlackVolatilityTermStructure<D> for BlackVolatilityCurve<I, D>
where
    D: DayCounter,
    I: Interpolator<DateTime, Volatility>,
{
    fn black_volatility(
        &self,
        maturity: DateTime,
        _strike: Strike,
    ) -> BlackVolatilityTermStructureResult {
        use BlackVolatilityTermStructureResult::{
            ExistingValue, InterpolatedValue, NoPoints, OutOfRange,
        };
        match self.interpolator.interpolate(maturity) {
            InterpolationResult::ExistingValue(v) => ExistingValue(v),
            InterpolationResult::InterpolatedValue(v) => InterpolatedValue(v),
            InterpolationResult::OutOfRange => OutOfRange,
            InterpolationResult::NoPoints => NoPoints,
        }
    }

    fn black_forward_volatility(
        &self,
        _start_date: DateTime,
        _end_date: DateTime,
        _strike: Strike,
    ) -> BlackVolatilityTermStructureResult {
        todo!()
    }
}
