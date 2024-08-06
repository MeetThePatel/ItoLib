use std::fmt::Debug;
use std::ops::{Div, Mul};

use num::traits::NumOps;

pub trait FloatScalable:
    NumOps
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + Into<f64>
    + Debug
    + Copy
    + Clone
    + Sized
{
}

impl<T> FloatScalable for T where
    T: NumOps
        + Mul<f64, Output = Self>
        + Div<f64, Output = Self>
        + Into<f64>
        + Debug
        + Copy
        + Clone
        + Sized
{
}
