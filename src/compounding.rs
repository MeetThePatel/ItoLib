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
            Self::Simple(f) => &format!("Simple({f})"),
            Self::Compounding(f) => &format!("Compounding({f})"),
            Self::Continuous => "Continuous",
        };
        write!(f, "{output}")
    }
}
