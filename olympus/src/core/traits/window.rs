/// Trait to represent a window object.
pub trait Window {
    /// The associated point type of the window. The points are the element of the window.
    type Point;

    /// Returns an iterator which iterates over the window elements.
    fn apply(&self, p: &Self::Point) -> impl Iterator<Item = Self::Point>;
}

/// Trait to represent a weighted window object, meaning that each window object is weighted by a value.
pub trait WeightedWindow: Window {
    /// Weight type of the window object.
    type Weight;

    /// Returns an iterator which iterates over the window elements and their weight.
    fn apply(&self, p: &Self::Point) -> impl Iterator<Item = (Self::Weight, Self::Point)>;
}
