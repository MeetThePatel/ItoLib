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

#[cfg(feature = "high_precision")]
pub type Volatility = rust_decimal::Decimal;
pub type Volatility = f64;

#[cfg(feature = "high_precision")]
pub type Strike = rust_decimal::Decimal;
pub type Strike = f64;
