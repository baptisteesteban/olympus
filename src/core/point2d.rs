use std::{
    fmt::{Debug, Display, Result},
    ops::{Add, Sub},
};

#[derive(Clone, Copy)]
pub struct Point2d {
    x: i32,
    y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d { x: x, y: y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl Default for Point2d {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self)
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

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
