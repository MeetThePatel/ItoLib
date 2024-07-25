#[macro_export]
macro_rules! assert_approx_equal_f64 {
    ($x:expr, $y:expr) => {
        assert!(
            ($x - $y <= f64::EPSILON) && ($y - $x <= f64::EPSILON),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            f64::EPSILON
        );
    };
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        );
    };
}

#[macro_export]
macro_rules! assert_approx_equal_f32 {
    ($x:expr, $y:expr) => {
        assert!(
            ($x - $y <= f32::EPSILON) && ($y - $x <= f32::EPSILON),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            f32::EPSILON
        );
    };
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        );
    };
}

#[macro_export]
macro_rules! assert_approx_equal_Money {
    ($x:expr, $y:expr) => {
        assert!(
            (($x - $y).amount <= f64::EPSILON) && (($y - $x).amount <= f64::EPSILON),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            f64::EPSILON
        );
        assert_eq!($x.get_currency_name(), $y.get_currency_name());
    };
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            (($x - $y).amount <= $d) && (($y - $x).amount <= $d),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        )
        assert_eq!($x.get_currency_name(), $y.get_currency_name());
    };
}

pub use assert_approx_equal_Money;
pub use assert_approx_equal_f32;
pub use assert_approx_equal_f64;
