use crate::{Point2d, Window};

pub struct C4Connectivity {}

impl C4Connectivity {
    const OFFSETS: [Point2d; 4] = [
        Point2d::new(-1, 0),
        Point2d::new(0, -1),
        Point2d::new(1, 0),
        Point2d::new(0, 1),
    ];

    pub const fn new() -> C4Connectivity {
        C4Connectivity {}
    }
}

impl Default for C4Connectivity {
    fn default() -> Self {
        Self::new()
    }
}

impl Window for C4Connectivity {
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d> {
        Self::OFFSETS.map(|offset| *p + offset).into_iter()
    }
}

pub const C4: C4Connectivity = C4Connectivity::new();
