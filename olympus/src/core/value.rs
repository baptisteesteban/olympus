/// The trait `Value` represents the set of some values
pub trait Value {}

/// The trait `Number` is implemented by the set of value types that represent a
/// Number (typically in ℝ).
pub trait Number: Value {}
/// The trait `Integer` is implemented by integer value types.
pub trait Integer: Number {}
/// The trait `Float` is implemented by floating-points value types.
pub trait Float: Number {}
/// The trait `UnsignedInteger` is implemented by the set of value types that
/// represents unsigned integers.
pub trait UnsignedInteger: Integer {}

/// We consider that all types are values
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

/// The trait `BoundedValueSet` is implemented by value types that have infimum
/// and supremum values. *TODO*: Think about algebraic data structure.
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
