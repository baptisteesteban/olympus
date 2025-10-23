use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// Implementation of a 2D point for a 2D regular grid. The coordinate system
/// used in Olympus is the `(x, y)` coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2d {
    /// The `x` coordinate
    pub x: i32,
    /// The `y` coordinate
    pub y: i32,
}

impl Point2d {
    /// Build a new 2D point.
    pub const fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }
}

impl Default for Point2d {
    /// Default constructor of a `Point2d`. Returns `(0, 0)`.
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Add for Point2d {
    type Output = Point2d;

    fn add(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point2d {
    type Output = Point2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Div<i32> for Point2d {
    type Output = Point2d;

    fn div(self, rhs: i32) -> Self::Output {
        Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<i32> for Point2d {
    type Output = Point2d;

    fn mul(self, rhs: i32) -> Self::Output {
        Point2d::new(self.x * rhs, self.y * rhs)
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.x, self.y).as_str())
    }
}
