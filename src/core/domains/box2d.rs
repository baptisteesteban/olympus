use crate::{
    traits::{Domain, ShapedDomain, SizedDomain},
    Point2d,
};

#[derive(Clone, Copy)]
pub struct Box2d {
    pmin: Point2d,
    pmax: Point2d,
}

impl Box2d {
    pub fn new(pmin: Point2d, pmax: Point2d) -> Box2d {
        Box2d {
            pmin: pmin,
            pmax: pmax,
        }
    }

    pub fn new_from_dimension(width: i32, height: i32) -> Box2d {
        Box2d {
            pmin: Point2d::new(0, 0),
            pmax: Point2d::new(width - 1, height - 1),
        }
    }

    pub fn width(&self) -> i32 {
        self.pmax.x() - self.pmin.x() + 1
    }

    pub fn height(&self) -> i32 {
        self.pmax.y() - self.pmin.y() + 1
    }
}

impl Domain for Box2d {
    type Point = Point2d;

    fn has(&self, p: &Self::Point) -> bool {
        p.x() >= self.pmin.x()
            && p.y() >= self.pmin.y()
            && p.x() <= self.pmax.x()
            && p.y() <= self.pmax.y()
    }
}

impl SizedDomain for Box2d {
    fn size(&self) -> i32 {
        self.width() * self.height()
    }
}

impl ShapedDomain for Box2d {
    const N: usize = 2;

    fn shape(&self, i: usize) -> Result<i32, String> {
        if i == 0 {
            Ok(self.width())
        } else if i == 1 {
            Ok(self.height())
        } else {
            Err(format!(
                "Index i ({}) out of range (should be in [0 - {}[)",
                i,
                Self::N
            ))
        }
    }
}

impl IntoIterator for Box2d {
    type Item = Point2d;
    type IntoIter = Point2dIterator;

    fn into_iter(self) -> Self::IntoIter {
        Point2dIterator::new(self)
    }
}

impl PartialEq for Box2d {
    fn eq(&self, other: &Self) -> bool {
        self.pmin == other.pmin && self.pmax == other.pmax
    }
}

pub struct Point2dIterator {
    cur: Option<Point2d>,
    domain: Box2d,
}

impl Point2dIterator {
    fn new(domain: Box2d) -> Point2dIterator {
        Point2dIterator {
            cur: None,
            domain: domain,
        }
    }
}

impl Iterator for Point2dIterator {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = &self.cur {
            let mut x = cur.x() + 1;
            let mut y = cur.y();
            if x >= self.domain.width() {
                x = 0;
                y += 1;
            }
            if y >= self.domain.height() {
                None
            } else {
                self.cur = Some(Point2d::new(x, y));
                Some(Point2d::new(x, y))
            }
        } else {
            self.cur = Some(Point2d::new(0, 0));
            Some(Point2d::new(0, 0))
        }
    }
}
