use crate::{Point2d, Window};

#[derive(Clone)]
pub struct Mask2d {
    offsets: Vec<Point2d>,
}

impl Mask2d {
    pub fn new(v: Vec<bool>, width: i32, height: i32) -> Result<Mask2d, String> {
        let mut res = Mask2d {
            offsets: Vec::<Point2d>::new(),
        };

        if (width * height) as usize != v.len() {
            return Err(String::from("Invalid width for mask2d"));
        }

        if width % 2 == 0 || height % 2 == 0 {
            return Err(String::from("Mask shapes must be even"));
        }

        let (x_center, y_center) = (width / 2, height / 2);

        for dy in 0..height {
            for dx in 0..width {
                if *v.get((dy * width + dx) as usize).unwrap() {
                    res.offsets.push(Point2d::new(x_center - dx, y_center - dy));
                }
            }
        }

        Ok(res)
    }

    pub fn cross(width: i32, height: i32) -> Result<Mask2d, String> {
        let (x_center, y_center) = (width / 2, height / 2);

        let mut mask: Vec<bool> = vec![false; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                if x == x_center || y == y_center {
                    *mask.get_mut((y * width + x) as usize).unwrap() = true;
                }
            }
        }

        Mask2d::new(mask, width, height)
    }

    pub fn rect(width: i32, height: i32) -> Result<Mask2d, String> {
        let mask: Vec<bool> = vec![true; (width * height) as usize];
        Mask2d::new(mask, width, height)
    }
}

impl Window for Mask2d {
    fn apply(&self, p: &Point2d) -> impl Iterator<Item = Point2d> {
        Mask2dApplyIterator::new(*p, self)
    }
}

pub struct Mask2dIterator {
    cur: usize,
    mask: Mask2d,
}

impl Mask2dIterator {
    pub fn new(mask: Mask2d) -> Mask2dIterator {
        Mask2dIterator { cur: 0, mask }
    }
}

impl Iterator for Mask2dIterator {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.mask.offsets.len() {
            None
        } else {
            let p = *self.mask.offsets.get(self.cur).unwrap();
            self.cur += 1;
            Some(p)
        }
    }
}

impl IntoIterator for Mask2d {
    type Item = Point2d;

    type IntoIter = Mask2dIterator;

    fn into_iter(self) -> Self::IntoIter {
        Mask2dIterator::new(self)
    }
}

pub struct Mask2dApplyIterator {
    p_ref: Point2d,
    it: Mask2dIterator,
}

impl Mask2dApplyIterator {
    pub fn new(p: Point2d, mask: &Mask2d) -> Mask2dApplyIterator {
        Mask2dApplyIterator {
            p_ref: p,
            it: mask.clone().into_iter(),
        }
    }
}

impl Iterator for Mask2dApplyIterator {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.it.next();
        if let Some(p) = cur {
            Some(p + self.p_ref)
        } else {
            None
        }
    }
}
