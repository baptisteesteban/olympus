use crate::{
    traits::{Domain, ShapedDomain, SizedDomain},
    Point2d,
};

pub struct Box2d {
    width: i32,
    height: i32,
}

impl Box2d {
    pub fn new(width: i32, height: i32) -> Box2d {
        Box2d {
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

impl Domain for Box2d {
    type Point = Point2d;

    fn has(&self, p: &Self::Point) -> bool {
        p.x() >= 0 && p.y() >= 0 && p.x() < self.width && p.y() < self.height()
    }
}

impl SizedDomain for Box2d {
    fn size(&self) -> i32 {
        self.width * self.height
    }
}

impl ShapedDomain for Box2d {
    const N: usize = 2;

    fn shape(&self, i: usize) -> Result<i32, String> {
        if i == 0 {
            Ok(self.width)
        } else if i == 1 {
            Ok(self.height)
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
        self.width == other.width && self.height == other.height
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
