#[macro_export]
macro_rules! assert_approx_eq_money {
    ($x:expr, $y:expr, $d:expr) => {
        assert_eq!($x.currency(), $y.currency());
        assert_approx_eq::assert_approx_eq!($x.amount().value(), $y.amount().value(), $d);
    };
}
