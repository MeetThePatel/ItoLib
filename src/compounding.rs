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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_compounding() {
        let test_cases = [
            (Compounding::Simple(Frequency::Annual), "Simple(Annual)"),
            (
                Compounding::Compounding(Frequency::Quarterly),
                "Compounding(Quarterly)",
            ),
            (Compounding::Continuous, "Continuous"),
        ];

        for (compounding, expected_output) in test_cases {
            assert_eq!(format!("{}", compounding), expected_output);
        }
    }
}
