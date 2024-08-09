use std::fmt::Debug;
use std::ops::{Div, Mul};

use num::traits::NumOps;
use ordered_float::OrderedFloat;

pub trait FloatScalable:
    NumOps
    + Mul<OrderedFloat<f64>, Output = Self>
    + Div<OrderedFloat<f64>, Output = Self>
    + Into<OrderedFloat<f64>>
    + Debug
    + Default
    + Clone
{
}

impl<T> FloatScalable for T where
    T: NumOps
        + Mul<OrderedFloat<f64>, Output = Self>
        + Div<OrderedFloat<f64>, Output = Self>
        + Into<OrderedFloat<f64>>
        + Debug
        + Default
        + Clone
{
}
