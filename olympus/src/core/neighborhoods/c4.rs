use crate::{Point2d, Window};

/// Implementation of the window representing the 4-connectivity defined over a
/// 2D regular grid.
pub struct C4Connectivity {}

impl C4Connectivity {
    /// The offset points
    const OFFSETS: [Point2d; 4] = [
        Point2d::new(-1, 0),
        Point2d::new(0, -1),
        Point2d::new(1, 0),
        Point2d::new(0, 1),
    ];

    /// Build a new window representing the 4-connectivity
    pub const fn new() -> C4Connectivity {
        C4Connectivity {}
    }
}

/// Default implementation.
impl Default for C4Connectivity {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of the `Window` trait.
impl Window for C4Connectivity {
    /// Iterator that iterates over the 4 neighbors of a point `p`.
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d> {
        Self::OFFSETS.map(|offset| *p + offset).into_iter()
    }
}

/// Constant that instantiate at compile-time a 4-connectivity window, as no
/// runtime information is required.
pub const C4: C4Connectivity = C4Connectivity::new();
