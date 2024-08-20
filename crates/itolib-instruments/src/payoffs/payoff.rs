pub trait Payoff: std::fmt::Display {
    // Type most likely should be Money<N, C>.
    // However, in the future, other option types will be implemented, such as percentage strikes.
    type PayoffNumberType;

    fn apply(&self, price: Self::PayoffNumberType) -> Self::PayoffNumberType;
}
