// Definition of traits

pub trait Value {}
pub trait Number: Value {}
pub trait Integer: Number {}
pub trait Float: Number {}
pub trait UnsignedInteger: Integer {}

// Implementation of traits
impl<T> Value for T {}

macro_rules! impl_value_traits {
    // Pattern: type followed by one or more traits
    ($type:ident, $($trait:ident),+ $(,)?) => {
        $(
            impl $trait for $type {}
        )*
    };
}

// Implementation of the traits for the numeric types
impl_value_traits!(u8, Number, Integer, UnsignedInteger);
impl_value_traits!(u16, Number, Integer, UnsignedInteger);
impl_value_traits!(u32, Number, Integer, UnsignedInteger);
impl_value_traits!(u64, Number, Integer, UnsignedInteger);
impl_value_traits!(usize, Number, Integer, UnsignedInteger);

impl_value_traits!(i8, Number, Integer);
impl_value_traits!(i16, Number, Integer);
impl_value_traits!(i32, Number, Integer);
impl_value_traits!(i64, Number, Integer);

impl_value_traits!(f32, Number, Float);
impl_value_traits!(f64, Number, Float);

pub trait BoundedValueSet {
    fn inf() -> Self;
    fn sup() -> Self;
}

macro_rules! impl_bounded_valueset_trait {
    ($type:ident) => {
        impl BoundedValueSet for $type {
            fn inf() -> $type {
                $type::MIN
            }

            fn sup() -> $type {
                $type::MAX
            }
        }
    };
}

impl_bounded_valueset_trait!(u8);
impl_bounded_valueset_trait!(u16);
impl_bounded_valueset_trait!(u32);
impl_bounded_valueset_trait!(u64);
impl_bounded_valueset_trait!(usize);

impl_bounded_valueset_trait!(i8);
impl_bounded_valueset_trait!(i16);
impl_bounded_valueset_trait!(i32);
impl_bounded_valueset_trait!(i64);

impl_bounded_valueset_trait!(f32);
impl_bounded_valueset_trait!(f64);

impl BoundedValueSet for bool {
    fn inf() -> bool {
        false
    }

    fn sup() -> bool {
        true
    }
}
