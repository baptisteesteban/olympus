pub trait Point {}

pub trait UndefinedPoint: Point {
    const UNDEF: Self;
}
