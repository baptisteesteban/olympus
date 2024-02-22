use crate::{traits::Window, Box2d, Point2d};

pub struct C8 {
    offsets: [Point2d; 8],
}

impl C8 {
    pub fn new() -> C8 {
        C8 {
            offsets: [
                Point2d::new(-1, 0),
                Point2d::new(-1, 1),
                Point2d::new(0, 1),
                Point2d::new(1, 1),
                Point2d::new(1, 0),
                Point2d::new(1, -1),
                Point2d::new(0, -1),
                Point2d::new(-1, -1),
            ],
        }
    }
}

impl Window for C8 {
    type Domain = Box2d;

    fn apply(&self, p: &Point2d) -> Vec<Point2d> {
        let mut res = Vec::<Point2d>::with_capacity(4);
        for dp in self.offsets {
            res.push(*p + dp);
        }
        res
    }
}
