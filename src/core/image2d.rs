use crate::{Box2d, Point2d};

pub struct Image2d<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Image2d<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        self.data.get(y * self.width + x).expect("Invalid index")
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data
            .get_mut(y * self.width + x)
            .expect("Invalid index")
    }

    pub fn at_point(&self, p: &Point2d) -> &T {
        self.at(p.x(), p.y())
    }

    pub fn at_point_mut(&mut self, p: &Point2d) -> &mut T {
        self.at_mut(p.x(), p.y())
    }

    pub fn domain(&self) -> Box2d {
        Box2d::new(self.width, self.height)
    }
}

impl<T: Default + Clone> Image2d<T> {
    pub fn new(width: usize, height: usize) -> Image2d<T> {
        let vec = vec![<T as Default>::default(); width * height];
        return Image2d {
            data: vec,
            width: width,
            height: height,
        };
    }

    pub fn new_from_domain(domain: &Box2d) -> Image2d<T> {
        Image2d::new(domain.width(), domain.height())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Image2d, Point2d};

    #[test]
    fn test_image_creation() {
        let img = Image2d::<u8>::new(10, 15);
        assert_eq!(img.width(), 10);
        assert_eq!(img.height(), 15);
    }

    #[test]
    fn test_read_write() {
        let mut img = Image2d::<u8>::new(5, 8);
        let width = img.width();
        for y in 0..img.height() {
            for x in 0..img.width() {
                *img.at_mut(x, y) = (y * width + x) as u8;
            }
        }
        for y in 0..img.height() {
            for x in 0..img.width() {
                let val = (y * width + x) as u8;
                let p = Point2d::new(x, y);
                assert_eq!(*img.at(x, y), val);
                assert_eq!(*img.at_point(&p), val);
            }
        }
    }

    #[test]
    fn test_domain() {
        let img = Image2d::<u8>::new(3, 7);
        let domain = img.domain();
        assert_eq!(domain.width(), 3);
        assert_eq!(domain.height(), 7);

        let img2 = Image2d::<u8>::new_from_domain(&domain);
        assert_eq!(img2.width(), 3);
        assert_eq!(img2.height(), 7);
    }
}
