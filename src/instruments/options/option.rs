use std::fmt::Display;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OptionType {
    CALL,
    PUT,
}

impl Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CALL => write!(f, "CALL"),
            Self::PUT => write!(f, "PUT"),
        }
    }
}
