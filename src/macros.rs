#[macro_export]
macro_rules! assert_approx_equal_f64 {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            (ordered_float::OrderedFloat::<f64>::from($x)
                - ordered_float::OrderedFloat::<f64>::from($y)
                <= ordered_float::OrderedFloat::<f64>::from($d))
                && (ordered_float::OrderedFloat::<f64>::from($y)
                    - ordered_float::OrderedFloat::<f64>::from($x)
                    <= ordered_float::OrderedFloat::<f64>::from($d)),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        );
    };
}
pub use assert_approx_equal_f64;

#[macro_export]
macro_rules! assert_approx_equal_f32 {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            (ordered_float::OrderedFloat::<f32>::from($x)
                - ordered_float::OrderedFloat::<f32>::from($y)
                <= ordered_float::OrderedFloat::<f32>::from($d))
                && (ordered_float::OrderedFloat::<f32>::from($y)
                    - ordered_float::OrderedFloat::<f32>::from($x)
                    <= ordered_float::OrderedFloat::<f32>::from($d)),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        );
    };
}
pub use assert_approx_equal_f32;

#[macro_export]
macro_rules! assert_approx_equal_Money {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            (ordered_float::OrderedFloat::<f64>::from(($x - $y).amount)
                <= ordered_float::OrderedFloat::<f64>::from($d))
                && (ordered_float::OrderedFloat::<f64>::from(($y - $x).amount)
                    <= ordered_float::OrderedFloat::<f64>::from($d)),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        );
        assert_eq!($x.get_currency_name(), $y.get_currency_name());
    };
}
pub use assert_approx_equal_Money;

#[macro_export]
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
pub use any_true;
