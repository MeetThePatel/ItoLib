macro_rules! generate_float_impls {
    ($type_name: ident) => {
        impl $type_name {
            #[must_use]
            pub const fn value(&self) -> OrderedFloat<f64> {
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
        impl<T> std::ops::Add<T> for $type_name
        where
            T: Into<f64>,
        {
            type Output = Option<Self>;

            fn add(self, rhs: T) -> Self::Output {
                Self::new(self.value() + rhs.into())
            }
        }

        impl<T> std::ops::Sub<T> for $type_name
        where
            T: Into<f64>,
        {
            type Output = Option<Self>;

            fn sub(self, rhs: T) -> Self::Output {
                Self::new(self.value() - rhs.into())
            }
        }

        impl<T> std::ops::Mul<T> for $type_name
        where
            T: Into<f64>,
        {
            type Output = Option<Self>;

            fn mul(self, rhs: T) -> Self::Output {
                Self::new(self.value() * rhs.into())
            }
        }

        impl<T> std::ops::Div<T> for $type_name
        where
            T: Into<f64>,
        {
            type Output = Option<Self>;

            fn div(self, rhs: T) -> Self::Output {
                Self::new(self.value() / rhs.into())
            }
        }

        impl<T> std::ops::Rem<T> for $type_name
        where
            T: Into<f64>,
        {
            type Output = Option<Self>;

            fn rem(self, rhs: T) -> Self::Output {
                Self::new(self.value() % rhs.into())
            }
        }
    };
}
pub(crate) use generate_float_ops_impls;

macro_rules! generate_float_comparison_impls {
    ($type_name: ident) => {
        impl<T> PartialEq<T> for $type_name
        where
            T: Into<f64> + Copy,
        {
            fn eq(&self, other: &T) -> bool {
                let self_repr: f64 = *(*self).value();
                let other_repr: f64 = (*other).into();

                self_repr == other_repr
            }
        }
        impl Eq for $type_name {}

        impl<T> PartialOrd<T> for $type_name
        where
            T: Into<f64> + Copy,
        {
            fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
                let self_repr: f64 = (*self).into();
                let other_repr: f64 = (*other).into();

                self_repr.partial_cmp(&other_repr)
            }
        }
        impl Ord for $type_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let self_repr: f64 = (*self).into();
                let other_repr: f64 = (*other).into();

                self_repr.partial_cmp(&other_repr).unwrap()
            }
        }
    };
}
pub(crate) use generate_float_comparison_impls;

macro_rules! generate_float_conversions_impls {
    ($type_name: ident) => {
        impl From<$type_name> for f64 {
            fn from(value: $type_name) -> Self {
                value.value().into_inner()
            }
        }
        impl From<$type_name> for ordered_float::OrderedFloat<f64> {
            fn from(value: $type_name) -> Self {
                value.value()
            }
        }
    };
}
pub(crate) use generate_float_conversions_impls;

macro_rules! impl_float {
    ($type_name: ident) => {
        crate::float::macros::generate_float_impls!($type_name);
        crate::float::macros::generate_float_ops_impls!($type_name);
        crate::float::macros::generate_float_comparison_impls!($type_name);
        crate::float::macros::generate_float_conversions_impls!($type_name);
    };
}
pub(crate) use impl_float;
