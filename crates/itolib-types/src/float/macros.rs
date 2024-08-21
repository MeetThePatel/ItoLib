macro_rules! generate_float_impls {
    ($type_name: ident) => {
        impl $type_name {
            #[must_use]
            pub const fn value(&self) -> f64 {
                self.0
            }
        }

        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value())
            }
        }
    };
}
pub(crate) use generate_float_impls;

macro_rules! generate_float_ops_impls {
    ($type_name: ident) => {
        // Cannot implement operations of two Option<$type_name> because of orphan rule.
        // TODO: Investigate using custom Option wrapper to circumvent this. However, the tricky
        // part will be exposing a std::Option in the API, and just using custom Option wrapper for
        // library internals.

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
pub(crate) use generate_float_ops_impls;

macro_rules! generate_float_comparison_impls {
    ($type_name: ident) => {
        impl<T> PartialEq<T> for $type_name
        where
            T: $crate::float::IntoFloat + Copy,
        {
            fn eq(&self, other: &T) -> bool {
                let self_repr: f64 = self.as_f64();
                let other_repr: f64 = other.as_f64();

                self_repr == other_repr
            }
        }
        impl Eq for $type_name {}

        impl<T> PartialOrd<T> for $type_name
        where
            T: $crate::float::IntoFloat + Copy,
        {
            fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                let self_repr: f64 = (*self).as_f64();
                let other_repr: f64 = (*other).as_f64();

                self_repr.partial_cmp(&other_repr)
            }
        }
        impl Ord for $type_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let self_repr: f64 = (*self).as_f64();
                let other_repr: f64 = (*other).as_f64();

                self_repr.partial_cmp(&other_repr).unwrap()
            }
        }
    };
}
pub(crate) use generate_float_comparison_impls;

macro_rules! generate_fallible_conversion_impls {
    ($type_name: ident) => {
        impl TryFrom<f64> for $type_name {
            type Error = &'static str;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::new(value).ok_or("Value not in domain.")
            }
        }
        impl TryFrom<f32> for $type_name {
            type Error = &'static str;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::new(value).ok_or("Value not in domain.")
            }
        }
    };
}
pub(crate) use generate_fallible_conversion_impls;

// macro_rules! generate_float_conversions_impls {
//     ($type_name: ident) => {
//         impl From<$type_name> for f64 {
//             fn from(value: $type_name) -> Self {
//                 value.value()
//             }
//         }
//     };
// }
// pub(crate) use generate_float_conversions_impls;

macro_rules! impl_float {
    ($type_name: ident) => {
        crate::float::macros::generate_float_impls!($type_name);
        crate::float::macros::generate_float_ops_impls!($type_name);
        crate::float::macros::generate_float_comparison_impls!($type_name);
        crate::float::macros::generate_fallible_conversion_impls!($type_name);
        // crate::float::macros::generate_float_conversions_impls!($type_name);
    };
}
pub(crate) use impl_float;
