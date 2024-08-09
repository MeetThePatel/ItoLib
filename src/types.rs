#[cfg(feature = "high_precision")]
pub type MonetaryNumber = rust_decimal::Decimal;
pub type MonetaryNumber = ordered_float::OrderedFloat<f64>;

#[cfg(feature = "high_precision")]
pub type Percentage = rust_decimal::Decimal;
pub type Percentage = ordered_float::OrderedFloat<f64>;

#[cfg(feature = "high_precision")]
pub type DiscountFactor = rust_decimal::Decimal;
pub type DiscountFactor = ordered_float::OrderedFloat<f64>;

#[cfg(feature = "high_precision")]
pub type CompoundFactor = rust_decimal::Decimal;
pub type CompoundFactor = ordered_float::OrderedFloat<f64>;

#[cfg(feature = "high_precision")]
pub type Volatility = rust_decimal::Decimal;
pub type Volatility = ordered_float::OrderedFloat<f64>;

#[cfg(feature = "high_precision")]
pub type Strike = rust_decimal::Decimal;
pub type Strike = ordered_float::OrderedFloat<f64>;
