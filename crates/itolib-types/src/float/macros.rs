/// This macro generates `Display` for the type.
#[macro_export]
macro_rules! generate_display {
    ($type_name: ident) => {
        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_f64())
            }
        }
    };
}

pub use generate_display;

/// This macro generates numeric operations.
///
/// The operations generated are: `Add`, `Sub`, `Mul`, `Div`, and `Rem`. These operations are
/// defined for the following LHS-RHS pairs for the type `T`:
/// - (`T`, `T`)
/// - (`T`, `Option<T>`)
/// - (`Option<T>`, `T`)
///
/// Due to the orphan rule, we would require a wrapper around `Option` to generate ops for the
/// LHS-RHS pair, (`Option<T>`, `Option<T>`).
///
/// The reason for these additional LHS-RHS is because it is quite inconvenient to have to write
/// `.unwrap()` after each operation in an operation chain. For example, if we want to do `a + b +
/// c`, we would need to write: `((a + b).unwrap() + c).unwrap()`. With these additional LHS-RHS
/// pairs, we can write `(a.unwrap() + b + c).unwrap()`. This way, we only have to unwrap at start
/// of the operation chain, and the end of the operation chain.
#[macro_export]
macro_rules! generate_float_ops_impls {
    ($type_name: ident) => {
        // Cannot implement operations of two Option<$type_name> because of orphan rule.
        // TODO: Investigate using custom Option wrapper to circumvent this. However, the tricky
        //       part will be exposing a std::Option in the API, and just using custom Option
        //       wrapper for library internals.

        // Implement addition for any type that can turn into a f64.
        impl<T> std::ops::Add<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            type Output = Option<Self>;

            fn add(self, rhs: T) -> Self::Output {
                Self::new(self.0 + rhs.as_f64())
            }
        }
        // Implement addition for: $type_name + Option<$type_name>
        impl std::ops::Add<Option<Self>> for $type_name {
            type Output = Option<Self>;

            fn add(self, rhs: Option<Self>) -> Self::Output {
                rhs.and_then(|rhs_val| Self::new(self.0 + rhs_val.0))
            }
        }
        // Implement addition for: Option<$type_name> + $type_name
        impl std::ops::Add<$type_name> for Option<$type_name> {
            type Output = Self;

            fn add(self, rhs: $type_name) -> Self::Output {
                self.and_then(|self_val| $type_name::new(self_val.0 + rhs.0))
            }
        }

        // Implement subtraction for any type that can turn into a f64.
        impl<T> std::ops::Sub<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            type Output = Option<Self>;

            fn sub(self, rhs: T) -> Self::Output {
                Self::new(self.0 - rhs.as_f64())
            }
        }
        // Implement subtraction for: $type_name - Option<$type_name>
        impl std::ops::Sub<Option<Self>> for $type_name {
            type Output = Option<Self>;

            fn sub(self, rhs: Option<Self>) -> Self::Output {
                rhs.and_then(|rhs_val| Self::new(self.0 - rhs_val.0))
            }
        }
        // Implement subtraction for: Option<$type_name> - $type_name
        impl std::ops::Sub<$type_name> for Option<$type_name> {
            type Output = Self;

            fn sub(self, rhs: $type_name) -> Self::Output {
                self.and_then(|self_val| $type_name::new(self_val.0 - rhs.0))
            }
        }

        // Implement multiplication for any type that can turn into a f64.
        impl<T> std::ops::Mul<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            type Output = Option<Self>;

            fn mul(self, rhs: T) -> Self::Output {
                Self::new(self.0 * rhs.as_f64())
            }
        }
        // Implement multiplication for: $type_name * Option<$type_name>
        impl std::ops::Mul<Option<Self>> for $type_name {
            type Output = Option<Self>;

            fn mul(self, rhs: Option<Self>) -> Self::Output {
                rhs.and_then(|rhs_val| Self::new(self.0 * rhs_val.0))
            }
        }
        // Implement multiplication for: Option<$type_name> * $type_name
        impl std::ops::Mul<$type_name> for Option<$type_name> {
            type Output = Self;

            fn mul(self, rhs: $type_name) -> Self::Output {
                self.and_then(|self_val| $type_name::new(self_val.0 * rhs.0))
            }
        }

        // Implement division for any type that can turn into a f64.
        impl<T> std::ops::Div<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            type Output = Option<Self>;

            fn div(self, rhs: T) -> Self::Output {
                Self::new(self.0 / rhs.as_f64())
            }
        }
        // Implement division for: $type_name / Option<$type_name>
        impl std::ops::Div<Option<Self>> for $type_name {
            type Output = Option<Self>;

            fn div(self, rhs: Option<Self>) -> Self::Output {
                rhs.and_then(|rhs_val| Self::new(self.0 / rhs_val.0))
            }
        }
        // Implement division for: Option<$type_name> / $type_name
        impl std::ops::Div<$type_name> for Option<$type_name> {
            type Output = Self;

            fn div(self, rhs: $type_name) -> Self::Output {
                self.and_then(|self_val| $type_name::new(self_val.0 / rhs.0))
            }
        }

        // Implement remainder for any type that can turn into a f64.
        impl<T> std::ops::Rem<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            type Output = Option<Self>;

            fn rem(self, rhs: T) -> Self::Output {
                Self::new(self.0 % rhs.as_f64())
            }
        }
        // Implement remainder for: $type_name % Option<$type_name>
        impl std::ops::Rem<Option<Self>> for $type_name {
            type Output = Option<Self>;

            fn rem(self, rhs: Option<Self>) -> Self::Output {
                rhs.and_then(|rhs_val| Self::new(self.0 % rhs_val.0))
            }
        }
        // Implement remainder for: Option<$type_name> % $type_name
        impl std::ops::Rem<$type_name> for Option<$type_name> {
            type Output = Self;

            fn rem(self, rhs: $type_name) -> Self::Output {
                self.and_then(|self_val| $type_name::new(self_val.0 % rhs.0))
            }
        }
    };
}
pub use generate_float_ops_impls;

/// This macro generates comparison operations.
///
/// The traits generated are: [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`].
///
/// For the equality operations, we define `NaN` as being equal to `NaN`, so that we can have
/// total ordering.
///
/// For the ordering operations, we define the following order:
/// $$
/// -\infty < \text{Negative Numbers} < 0 < \text{Positive Numbers} < +\infty < \text{NaN}
/// $$
#[macro_export]
macro_rules! generate_float_comparison_impls {
    ($type_name: ident) => {
        impl<T> PartialEq<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            fn eq(&self, other: &T) -> bool {
                let self_repr: f64 = self.as_f64();
                let other_repr: f64 = other.as_f64();

                // If both are NaN, return True. Otherwise, use f64 comparison.
                if self_repr.is_nan() && other_repr.is_nan() {
                    true
                } else {
                    self_repr == other_repr
                }
            }
        }
        impl Eq for $type_name {}

        impl<T> PartialOrd<T> for $type_name
        where
            T: $crate::float::IntoFloat,
        {
            fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                let self_repr: f64 = self.as_f64();
                let other_repr: f64 = other.as_f64();

                match (self_repr.is_nan(), other_repr.is_nan()) {
                    (true, true) => return Some(std::cmp::Ordering::Equal),
                    (true, false) => return Some(std::cmp::Ordering::Greater),
                    (false, true) => return Some(std::cmp::Ordering::Less),
                    _ => (),
                };

                self_repr.partial_cmp(&other_repr)
            }
        }
        impl Ord for $type_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
    };
}
pub use generate_float_comparison_impls;

/// This macro generates `TryFrom` for `f64` and `f32`.
#[macro_export]
macro_rules! generate_tryfrom_std_float_impls {
    ($type_name: ident) => {
        impl TryFrom<f64> for $type_name {
            type Error = $crate::float::ConversionDomainError;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::new(value).ok_or($crate::float::ConversionDomainError)
            }
        }
        impl TryFrom<f32> for $type_name {
            type Error = $crate::float::ConversionDomainError;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::new(value).ok_or($crate::float::ConversionDomainError)
            }
        }
    };
}
pub use generate_tryfrom_std_float_impls;

/// This macro generates infallible conversion operations.
///
/// On the flowchart found in the README.md for the module `float`, these conversion operations are
/// ones in the direction of the directed arrows.
#[macro_export]
macro_rules! generate_infallible_conversion_impls {
    ($from_type:ty, ) => {};

    ($from_type:ident, $to_type:ident $(, $rest:ident)*) => {
        impl From<$from_type> for $to_type {
            fn from(value: $from_type) -> Self {
                $to_type::new(value.as_f64()).unwrap()
            }
        }
        $crate::float::macros::generate_infallible_conversion_impls!($from_type,$($rest),*);
    };
}
pub use generate_infallible_conversion_impls;

/// This macro generates fallible conversion operations.
///
/// For all conversions not included in `generate_infallible_conversion_impls`, this operation is
/// implemented here.
#[macro_export]
macro_rules! generate_fallible_conversion_impls {
    ($from_type: ty, ) => {};

    ($from_type:ident, $to_type:ident $(, $rest:ident)*) => {
        impl TryFrom<$from_type> for $to_type {
            type Error = $crate::float::ConversionDomainError;

            fn try_from(value: $from_type) -> Result<Self, Self::Error> {
                $to_type::new(value.as_f64()).ok_or($crate::float::ConversionDomainError)
            }
        }
        $crate::float::macros::generate_fallible_conversion_impls!($from_type,$($rest),*);
    };
}
pub use generate_fallible_conversion_impls;

/// This macro calls the other configuration-less macros.
///
/// The configuration-less macros are:
/// - [`generate_display`]
/// - [`generate_float_ops_impls`]
/// - [`generate_float_comparison_impls`]
/// - [`generate_tryfrom_std_float_impls`]
///
/// The macros that require configuration (which this macro does not call) are:
/// - [`generate_fallible_conversion_impls`]
/// - [`generate_infallible_conversion_impls`]
#[macro_export]
macro_rules! impl_float {
    ($type_name: ident) => {
        $crate::float::macros::generate_display!($type_name);
        $crate::float::macros::generate_float_ops_impls!($type_name);
        $crate::float::macros::generate_float_comparison_impls!($type_name);
        $crate::float::macros::generate_tryfrom_std_float_impls!($type_name);
    };
}
pub use impl_float;
