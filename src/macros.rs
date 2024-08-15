#[allow(unused_macros)]
macro_rules! assert_approx_equal_money {
    ($x:expr, $y:expr, $d:expr) => {
        assert_eq!($x.currency(), $y.currency());
        assert_approx_eq::assert_approx_eq!($x.amount, $y.amount, $d);
    };
}
#[allow(unused_imports)]
pub(crate) use assert_approx_equal_money;

macro_rules! any_true {
    () => {
        None
    };
    ($single:expr) => {
        if $single {
            Some(0)
        } else {
            None
        }
    };
    ($first:expr, $($rest:expr),*) => {
        if $first {
            Some(0)
        } else {
            match any_true!($($rest),*) {
                Some(index) => Some(index + 1),
                None => None,
            }
        }
    };
}
pub(crate) use any_true;
