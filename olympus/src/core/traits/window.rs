use crate::Point2d;

pub trait Window {
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d>;
}

pub trait WeightedWindow: Window {
    type Value;

    fn apply(&self, p: &Point2d) -> impl Iterator<Item = (Self::Value, Point2d)>;
}
