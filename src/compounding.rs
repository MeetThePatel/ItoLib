use std::fmt::Display;

use crate::time::Frequency;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compounding {
    Simple(Frequency),
    Compounding(Frequency),
    Continuous,
}

impl Display for Compounding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Compounding::Simple(f) => &format!("Simple({f})"),
            Compounding::Compounding(f) => &format!("Compounding({f})"),
            Compounding::Continuous => "Continuous",
        };
        write!(f, "{output}")
    }
}
