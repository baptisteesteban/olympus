use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub const fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }
}

impl Default for Point2d {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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
