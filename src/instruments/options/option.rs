use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OptionType {
    CALL,
    PUT,
}

impl Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionType::CALL => write!(f, "CALL"),
            OptionType::PUT => write!(f, "PUT"),
        }
    }
}
