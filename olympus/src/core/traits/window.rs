use crate::Point2d;

/// Trait to represent a window object.
pub trait Window {
    /// Returns an iterator which iterates over the window elements.
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d>;
}

/// Trait to represent a weighted window object, meaning that each window object is weighted by a value.
pub trait WeightedWindow: Window {
    /// Weight type of the window object.
    type Weight;

    /// Returns an iterator which iterates over the window elements and their weight.
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = (Self::Weight, Point2d)>;
}
