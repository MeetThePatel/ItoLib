use std::ops::{Div, Mul};

use itolib_types::Float;
use num::traits::NumOps;

pub trait FloatLike:
    NumOps + Mul<Float, Output = Self> + Div<Float, Output = Self> + Into<Float> + Clone
{
}

impl<T> FloatLike for T where
    T: NumOps + Mul<Float, Output = Self> + Div<Float, Output = Self> + Into<Float> + Clone
{
}
