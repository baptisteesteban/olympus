use crate::{BoundedValueSet, Float, Integer, Number, UnsignedInteger, Value};

fn check_unsigned_integer<T: Value + Number + Integer + UnsignedInteger>(_t: T) {}
fn check_integer<T: Value + Number + Integer>(_t: T) {}
fn check_float<T: Value + Number + Float>(_t: T) {}
fn check_any<T: Value>(_t: T) {}

struct Foo {}

#[test]
fn test_value_traits() {
    check_unsigned_integer(1u8);
    check_unsigned_integer(1u16);
    check_unsigned_integer(1u32);
    check_unsigned_integer(1u64);

    check_integer(1i8);
    check_integer(1i16);
    check_integer(1i32);
    check_integer(1i64);

    check_float(1f32);
    check_float(1f64);

    check_any(Foo {});

    assert!(true);
}

macro_rules! generate_check_bounded_valueset {
    ($type:ident) => {
        assert_eq!($type::MIN, <$type as BoundedValueSet>::inf());
        assert_eq!($type::MAX, <$type as BoundedValueSet>::sup());
    };
}

#[test]
fn test_bounded_valueset() {
    generate_check_bounded_valueset!(u8);
    generate_check_bounded_valueset!(u16);
    generate_check_bounded_valueset!(u32);
    generate_check_bounded_valueset!(u64);

    generate_check_bounded_valueset!(i8);
    generate_check_bounded_valueset!(i16);
    generate_check_bounded_valueset!(i32);
    generate_check_bounded_valueset!(i64);

    generate_check_bounded_valueset!(f32);
    generate_check_bounded_valueset!(f64);

    assert!(!<bool as BoundedValueSet>::inf());
    assert!(<bool as BoundedValueSet>::sup());
}
