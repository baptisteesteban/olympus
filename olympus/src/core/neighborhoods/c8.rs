use crate::{Point2d, Window};

pub struct C8Connectivity {}

impl C8Connectivity {
    const OFFSETS: [Point2d; 8] = [
        Point2d::new(-1, 0),
        Point2d::new(-1, -1),
        Point2d::new(0, -1),
        Point2d::new(1, -1),
        Point2d::new(1, 0),
        Point2d::new(1, 1),
        Point2d::new(0, 1),
        Point2d::new(-1, 1),
    ];

    pub const fn new() -> C8Connectivity {
        C8Connectivity {}
    }
}

impl Default for C8Connectivity {
    fn default() -> Self {
        Self::new()
    }
}

impl Window for C8Connectivity {
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d> {
        Self::OFFSETS.map(|offset| *p + offset).into_iter()
    }
}

pub const C8: C8Connectivity = C8Connectivity::new();
