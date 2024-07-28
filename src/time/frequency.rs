use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
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
    Other(u32),
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Frequency::Once => "Once",
            Frequency::Annual => "Annual",
            Frequency::Semiannual => "SemiAnnual",
            Frequency::EveryFourthMonth => "EveryFourthMonth",
            Frequency::Quarterly => "Quarterly",
            Frequency::Bimonthly => "Bimonthly",
            Frequency::Monthly => "Monthly",
            Frequency::EveryFourthWeek => "EveryFourthWeek",
            Frequency::Biweekly => "Biweekly",
            Frequency::Weekly => "Weekly",
            Frequency::Daily => "Daily",
            Frequency::Other(f) => &format!("Other({})", f),
        };
        write!(f, "{}", output)
    }
}
