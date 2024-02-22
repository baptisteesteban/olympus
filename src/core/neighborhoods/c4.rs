use crate::{traits::Window, Box2d, Point2d};

pub struct C4 {
    offsets: [Point2d; 4],
}

impl C4 {
    pub fn new() -> C4 {
        C4 {
            offsets: [
                Point2d::new(-1, 0),
                Point2d::new(0, 1),
                Point2d::new(1, 0),
                Point2d::new(0, -1),
            ],
        }
    }
}

impl Window for C4 {
    type Domain = Box2d;

    fn apply(&self, p: &Point2d) -> Vec<Point2d> {
        let mut res = Vec::<Point2d>::with_capacity(4);
        for dp in self.offsets {
            res.push(*p + dp);
        }
        res
    }
}
