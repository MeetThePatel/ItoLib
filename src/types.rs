#[cfg(feature = "high_precision")]
pub type MonetaryNumber = rust_decimal::Decimal;
pub type MonetaryNumber = f64;

#[cfg(feature = "high_precision")]
pub type Percentage = rust_decimal::Decimal;
pub type Percentage = f64;
