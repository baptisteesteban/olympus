use crate::{Domain, Point2d, ShapeDomain, SizedDomain};

/// This structure represents a domain defined on a 2D regular grid.
#[derive(Debug, Clone, Copy)]
pub struct Box2d {
    /// Width of the grid
    width: i32,
    /// Height of the grid
    height: i32,
}

impl Box2d {
    /// Construct a new domain object from the width and the height of the grid.
    pub fn new(width: i32, height: i32) -> Box2d {
        Box2d { width, height }
    }

    /// Returns the width of the grid
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Returns the height of the grid
    pub fn height(&self) -> i32 {
        self.height
    }
}

impl Domain for Box2d {
    type Point = Point2d;

    /// Check if a point `p` belongs to the domain.
    fn has(&self, p: &Self::Point) -> bool {
        self.width > 0
            && self.height > 0
            && p.x >= 0
            && p.y >= 0
            && p.x < self.width
            && p.y < self.height
    }
}

impl SizedDomain for Box2d {
    fn size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

impl ShapeDomain for Box2d {
    fn shape(&self, i: usize) -> Option<usize> {
        match i {
            0 => Some(self.width as usize),
            1 => Some(self.height as usize),
            _ => None,
        }
    }
}

/// Default construction of the `Box2d` object.
impl Default for Box2d {
    /// Returns an empty 2D regular grid.
    fn default() -> Self {
        Box2d::new(0, 0)
    }
}

impl IntoIterator for Box2d {
    /// The item type of a 2D regular grid is a `Point2d`.
    type Item = Point2d;
    type IntoIter = Box2dIterator;

    /// Returns an interator to the first element of a `Box2d` (typically the Point2d set to `(0, 0)`).
    fn into_iter(self) -> Self::IntoIter {
        Box2dIterator::new(&self)
    }
}

/// Iterator type to iterate over a 2D regular grid.
pub struct Box2dIterator {
    /// The domain that the iterator is traversing.
    domain: Box2d,
    /// The current element pointer by the iterator. It is set to `None` if the
    /// iterator has traversed the whole domain.
    cur: Option<Point2d>,
}

impl Box2dIterator {
    /// Build a new iterator that traverses a 2D regular grid. The current
    /// element is initially set to the first point of the grid (from now on the
    /// point at `(0, 0)`).
    pub fn new(domain: &Box2d) -> Box2dIterator {
        Box2dIterator {
            domain: *domain,
            cur: Some(Point2d::default()),
        }
    }
}

impl Iterator for Box2dIterator {
    type Item = Point2d;

    /// Go to the next point of the grid. If the current point is the last point
    /// of the grid, returns `None`.
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
