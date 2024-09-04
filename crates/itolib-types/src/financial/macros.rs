macro_rules! impl_add_self {
    ($type_name: ident) => {
        impl std::ops::Add<Self> for $type_name {
            type Output = Option<Self>;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.0 + rhs.0)
            }
        }
    };
}
pub(crate) use impl_add_self;

macro_rules! impl_sub_self {
    ($type_name: ident) => {
        impl std::ops::Sub<Self> for $type_name {
            type Output = Option<Self>;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.0 - rhs.0)
            }
        }
    };
}
pub(crate) use impl_sub_self;

macro_rules! impl_mul_self {
    ($type_name: ident) => {
        impl std::ops::Mul<Self> for $type_name {
            type Output = Option<Self>;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.0 * rhs.0)
            }
        }
    };
}
pub(crate) use impl_mul_self;

macro_rules! impl_div_self {
    ($type_name: ident) => {
        impl std::ops::Div<Self> for $type_name {
            type Output = Option<Self>;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.0 / rhs.0)
            }
        }
    };
}
pub(crate) use impl_div_self;

macro_rules! impl_rem_self {
    ($type_name: ident) => {
        impl std::ops::Rem<Self> for $type_name {
            type Output = Option<Self>;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::new(self.0 % rhs.0)
            }
        }
    };
}
pub(crate) use impl_rem_self;

macro_rules! impl_ops_self {
    ($type_name: ident) => {
        crate::financial::macros::impl_add_self!($type_name);
        crate::financial::macros::impl_sub_self!($type_name);
        crate::financial::macros::impl_mul_self!($type_name);
        crate::financial::macros::impl_div_self!($type_name);
        crate::financial::macros::impl_rem_self!($type_name);
    };
}
pub(crate) use impl_ops_self;

macro_rules! impl_add_f64_like {
    ($type_name: ident) => {
        impl<T: Into<f64>> std::ops::Add<T> for $type_name {
            type Output = Option<Self>;

            fn add(self, rhs: T) -> Self::Output {
                Self::new(self.0 + rhs.into())
            }
        }
    };
}
pub(crate) use impl_add_f64_like;

macro_rules! impl_sub_f64_like {
    ($type_name: ident) => {
        impl<T: Into<f64>> std::ops::Sub<T> for $type_name {
            type Output = Option<Self>;

            fn sub(self, rhs: T) -> Self::Output {
                Self::new(self.0 - rhs.into())
            }
        }
    };
}
pub(crate) use impl_sub_f64_like;

macro_rules! impl_mul_f64_like {
    ($type_name: ident) => {
        impl<T: Into<f64>> std::ops::Mul<T> for $type_name {
            type Output = Option<Self>;

            fn mul(self, rhs: T) -> Self::Output {
                Self::new(self.0 * rhs.into())
            }
        }
    };
}
pub(crate) use impl_mul_f64_like;

macro_rules! impl_div_f64_like {
    ($type_name: ident) => {
        impl<T: Into<f64>> std::ops::Div<T> for $type_name {
            type Output = Option<Self>;

            fn div(self, rhs: T) -> Self::Output {
                Self::new(self.0 / rhs.into())
            }
        }
    };
}
pub(crate) use impl_div_f64_like;

macro_rules! impl_rem_f64_like {
    ($type_name: ident) => {
        impl<T: Into<f64>> std::ops::Rem<T> for $type_name {
            type Output = Option<Self>;

            fn rem(self, rhs: T) -> Self::Output {
                Self::new(self.0 % rhs.into())
            }
        }
    };
}
pub(crate) use impl_rem_f64_like;

macro_rules! impl_ops_f64_like {
    ($type_name: ident) => {
        $crate::financial::macros::impl_add_f64_like!($type_name);
        $crate::financial::macros::impl_sub_f64_like!($type_name);
        $crate::financial::macros::impl_mul_f64_like!($type_name);
        $crate::financial::macros::impl_div_f64_like!($type_name);
        $crate::financial::macros::impl_rem_f64_like!($type_name);
    };
}
pub(crate) use impl_ops_f64_like;

macro_rules! impl_try_from_float {
    ($type_name: ident) => {
        impl TryFrom<$crate::Float> for $type_name {
            type Error = $crate::DomainError;

            fn try_from(value: $crate::Float) -> Result<Self, Self::Error> {
                match Self::new(value) {
                    Some(v) => Ok(v),
                    None => Err($crate::DomainError(value)),
                }
            }
        }
    };
}
pub(crate) use impl_try_from_float;
