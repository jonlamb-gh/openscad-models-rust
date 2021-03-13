#[macro_export]
macro_rules! newtype_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_from {
    ($name:ident, $t:ty) => {
        impl std::convert::From<$t> for $name {
            fn from(val: $t) -> Self {
                $name(val)
            }
        }
    };
}

#[macro_export]
macro_rules! from_newtype {
    ($name:ident, $t:ty) => {
        impl std::convert::From<$name> for $t {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_deref {
    ($name:ident, $t:ty) => {
        impl std::ops::Deref for $name {
            type Target = $t;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_deref_mut {
    ($name:ident, $t:ty) => {
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_mul {
    ($name:ident) => {
        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<f32> for $name {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                $name(self.0 * rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_div {
    ($name:ident) => {
        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                $name(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<f32> for $name {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                $name(self.0 / rhs)
            }
        }
        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, modulus: Self) -> Self::Output {
                $name(self.0.rem(modulus.0))
            }
        }
        impl std::ops::Rem<f32> for $name {
            type Output = Self;
            fn rem(self, modulus: f32) -> Self::Output {
                $name(self.0.rem(modulus))
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_add {
    ($name:ident) => {
        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                $name(self.0 + other.0)
            }
        }
        impl std::ops::Add<f32> for $name {
            type Output = Self;
            fn add(self, other: f32) -> Self::Output {
                $name(self.0 + other)
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_sub {
    ($name:ident) => {
        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                $name(self.0 - other.0)
            }
        }
        impl std::ops::Sub<f32> for $name {
            type Output = Self;
            fn sub(self, other: f32) -> Self::Output {
                $name(self.0 - other)
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_helper_impls {
    ($name:ident) => {
        impl $name {
            pub const fn new(val: f32) -> Self {
                $name(val)
            }
            pub const fn as_f32(&self) -> f32 {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_approx_impls {
    ($name:ident) => {
        impl PartialEq<f32> for $name {
            fn eq(&self, other: &f32) -> bool {
                self.0.eq(other)
            }
        }
        impl PartialOrd<f32> for $name {
            fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }
        impl approx::AbsDiffEq for $name {
            type Epsilon = f32;

            fn default_epsilon() -> Self::Epsilon {
                <f32 as approx::AbsDiffEq>::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.0, &other.0, epsilon)
            }
        }
        impl approx::AbsDiffEq<f32> for $name {
            type Epsilon = f32;

            fn default_epsilon() -> Self::Epsilon {
                <f32 as approx::AbsDiffEq>::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &f32, epsilon: Self::Epsilon) -> bool {
                <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.0, &other, epsilon)
            }
        }
        impl approx::RelativeEq for $name {
            fn default_max_relative() -> Self::Epsilon {
                <f32 as approx::RelativeEq>::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <f32 as approx::RelativeEq>::relative_eq(&self.0, &other.0, epsilon, max_relative)
            }
        }
        impl approx::RelativeEq<f32> for $name {
            fn default_max_relative() -> Self::Epsilon {
                <f32 as approx::RelativeEq>::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &f32,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <f32 as approx::RelativeEq>::relative_eq(&self.0, other, epsilon, max_relative)
            }
        }
    };
}
