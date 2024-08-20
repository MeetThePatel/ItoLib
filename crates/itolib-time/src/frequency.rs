use std::fmt::Display;

/// Commonly defined frequencies.
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Frequency {
    Once = 0,
    Annual = 1,
    Semiannual = 2,
    EveryFourthMonth = 3,
    Quarterly = 4,
    Bimonthly = 6,
    Monthly = 12,
    EveryFourthWeek = 13,
    Biweekly = 26,
    Weekly = 52,
    Daily = 365,
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Once => "Once",
            Self::Annual => "Annual",
            Self::Semiannual => "SemiAnnual",
            Self::EveryFourthMonth => "EveryFourthMonth",
            Self::Quarterly => "Quarterly",
            Self::Bimonthly => "Bimonthly",
            Self::Monthly => "Monthly",
            Self::EveryFourthWeek => "EveryFourthWeek",
            Self::Biweekly => "Biweekly",
            Self::Weekly => "Weekly",
            Self::Daily => "Daily",
        };
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_frequency() {
        let test_cases = [
            (Frequency::Once, "Once"),
            (Frequency::Annual, "Annual"),
            (Frequency::Semiannual, "SemiAnnual"),
            (Frequency::EveryFourthMonth, "EveryFourthMonth"),
            (Frequency::Quarterly, "Quarterly"),
            (Frequency::Bimonthly, "Bimonthly"),
            (Frequency::Monthly, "Monthly"),
            (Frequency::EveryFourthWeek, "EveryFourthWeek"),
            (Frequency::Biweekly, "Biweekly"),
            (Frequency::Weekly, "Weekly"),
            (Frequency::Daily, "Daily"),
        ];

        for (freq, expected_output) in test_cases {
            assert_eq!(format!("{freq}"), expected_output);
        }
    }
}
