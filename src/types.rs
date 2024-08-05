#[cfg(feature = "high_precision")]
pub type MonetaryNumber = rust_decimal::Decimal;
pub type MonetaryNumber = f64;

#[cfg(feature = "high_precision")]
pub type Percentage = rust_decimal::Decimal;
pub type Percentage = f64;

#[cfg(feature = "high_precision")]
pub type DiscountFactor = rust_decimal::Decimal;
pub type DiscountFactor = f64;

#[cfg(feature = "high_precision")]
pub type CompoundFactor = rust_decimal::Decimal;
pub type CompoundFactor = f64;
