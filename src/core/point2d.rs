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

#[cfg(test)]
mod tests {
    use crate::Point2d;

    #[test]
    fn test_creation_and_attributes() {
        let p1 = Point2d::new(10, 3);
        assert_eq!(p1.x(), 10);
        assert_eq!(p1.y, 3);

        let p2 = Point2d::default();
        assert_eq!(p2.x(), 0);
        assert_eq!(p2.y(), 0);
    }

    #[test]
    fn test_display() {
        let p = Point2d::new(10, 78);
        let formatted = String::from(format!("{}", p).as_str());
        assert_eq!(formatted, "(10, 78)");
    }
}
