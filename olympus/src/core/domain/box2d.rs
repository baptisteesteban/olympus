use crate::Point2d;

#[derive(Debug, Clone, Copy)]
pub struct Box2d {
    width: i32,
    height: i32,
}

impl Box2d {
    pub fn new(width: i32, height: i32) -> Box2d {
        Box2d { width, height }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn has(&self, p: &Point2d) -> bool {
        self.width > 0
            && self.height > 0
            && p.x >= 0
            && p.y >= 0
            && p.x < self.width
            && p.y < self.height
    }
}

impl Default for Box2d {
    fn default() -> Self {
        Box2d::new(0, 0)
    }
}

impl IntoIterator for Box2d {
    type Item = Point2d;
    type IntoIter = Box2dIterator;

    fn into_iter(self) -> Self::IntoIter {
        Box2dIterator::new(&self)
    }
}

pub struct Box2dIterator {
    domain: Box2d,
    cur: Option<Point2d>,
}

impl Box2dIterator {
    pub fn new(domain: &Box2d) -> Box2dIterator {
        Box2dIterator {
            domain: *domain,
            cur: Some(Point2d::default()),
        }
    }
}

impl Iterator for Box2dIterator {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.cur;

        if let Some(mut cur) = self.cur {
            cur.x += 1;
            if cur.x >= self.domain.width() {
                cur.x = 0;
                cur.y += 1;
            }
            self.cur = if cur.y >= self.domain.height {
                None
            } else {
                Some(cur)
            };
        }

        res
    }
}
