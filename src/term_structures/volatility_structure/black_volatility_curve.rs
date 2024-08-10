use day_count_conventions::DayCounter;
use num::Bounded;

use crate::math::interpolation::Interpolator;
use crate::term_structures::volatility_structure::{
    BlackVolatilityTermStructure, VolatilityTermStructure,
};
use crate::term_structures::{TermStructure, TermStructureStrikeValidity};
use crate::time::DateTime;
use crate::types::{Strike, Volatility};

//  ------------------------------------------------------------------------------------------------
//  Definition
//  ------------------------------------------------------------------------------------------------

pub struct BlackVolatilityCurve<I>
where
    I: Interpolator<DateTime, Volatility>,
{
    interpolator: I,

    reference_date: DateTime,
}

//  ------------------------------------------------------------------------------------------------
//  Builder
//  ------------------------------------------------------------------------------------------------

pub struct BlackVolatilityCurveBuilder<I>
where
    I: Interpolator<DateTime, Volatility>,
{
    interpolator: I,
    reference_date: Option<DateTime>,
}

impl<I> BlackVolatilityCurveBuilder<I>
where
    I: Interpolator<DateTime, Volatility>,
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

    /// Build the volatility curve.
    pub fn build(mut self) -> BlackVolatilityCurve<I> {
        if self.reference_date.is_none() {
            self.reference_date = Some(DateTime::now());
        }
        BlackVolatilityCurve {
            interpolator: self.interpolator,
            // Safe to unwrap, because we gave default value above.
            reference_date: self.reference_date.unwrap(),
        }
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
        self.interpolator.range().unwrap().1
    }

    fn is_datetime_valid(&self, dt: DateTime) -> bool {
        dt >= <Self as TermStructure<D>>::get_reference_date(self)
            && dt <= <Self as TermStructure<D>>::get_max_datetime(self)
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
