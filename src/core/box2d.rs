use crate::Point2d;

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

    pub fn has(&self, p: &Point2d) -> bool {
        p.x() >= 0 && p.y() >= 0 && p.x() < self.width && p.y() < self.height()
    }
}

impl IntoIterator for Box2d {
    type Item = Point2d;
    type IntoIter = Point2dIterator;

    fn into_iter(self) -> Self::IntoIter {
        Point2dIterator::new(self)
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

impl PartialEq for Box2d {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
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

#[cfg(test)]
mod tests {
    use crate::Box2d;

    #[test]
    fn test_creation_and_attributes() {
        let domain = Box2d::new(7, 3);
        assert_eq!(domain.width(), 7);
        assert_eq!(domain.height(), 3);
    }

    #[test]
    fn test_iter() {
        let ref_p = [
            "(0, 0)", "(1, 0)", "(2, 0)", "(3, 0)", "(0, 1)", "(1, 1)", "(2, 1)", "(3, 1)",
        ];

        let domain = Box2d::new(4, 2);
        let mut i = 0;
        for p in domain {
            let formatted = String::from(format!("{}", p).as_str());
            assert_eq!(formatted, ref_p[i]);
            i += 1;
        }
    }
}
