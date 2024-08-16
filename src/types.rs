// Must be nonNAN
pub type MonetaryNumber = ordered_float::OrderedFloat<f64>;

// Must be nonNAN
pub type Percentage = ordered_float::OrderedFloat<f64>;

// Must be positive, nonNAN
pub type DiscountFactor = ordered_float::OrderedFloat<f64>;

// Must be positive, nonNAN
pub type CompoundFactor = ordered_float::OrderedFloat<f64>;

// Must be positive, nonNAN
pub type Volatility = ordered_float::OrderedFloat<f64>;

// TODO: Get rid of this and use associated type on the exercise/payoff.
pub type Strike = ordered_float::OrderedFloat<f64>;
