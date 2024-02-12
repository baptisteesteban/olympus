use crate::{
    traits::{Domain, Image, MutableImage},
    Box2d,
};

pub struct Image2d<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T: Default + Clone> Default for Image2d<T> {
    fn default() -> Self {
        Image2d::new(0, 0)
    }
}

impl<T> Image2d<T> {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn at(&self, x: i32, y: i32) -> &T {
        self.data
            .get((y * self.width + x) as usize)
            .expect("Invalid index")
    }

    pub fn at_mut(&mut self, x: i32, y: i32) -> &mut T {
        self.data
            .get_mut((y * self.width + x) as usize)
            .expect("Invalid index")
    }
}

impl<T> Image for Image2d<T>
where
    T: Default + Clone,
{
    type Domain = Box2d;
    type Value = T;
    type Point = <Self::Domain as Domain>::Point;

    fn new_from_domain(domain: &Self::Domain) -> Self {
        Image2d::new(domain.width(), domain.height())
    }

    fn domain(&self) -> Self::Domain {
        Self::Domain::new(self.width, self.height)
    }

    fn at_point(&self, p: &Self::Point) -> &Self::Value {
        self.at(p.x(), p.y())
    }
}

impl<T> MutableImage for Image2d<T>
where
    T: Default + Clone,
{
    fn at_point_mut(&mut self, p: &Self::Point) -> &mut Self::Value {
        self.at_mut(p.x(), p.y())
    }
}

impl<T: Default + Clone> Image2d<T> {
    pub fn new(width: i32, height: i32) -> Image2d<T> {
        let vec = vec![<T as Default>::default(); (width * height) as usize];
        return Image2d {
            data: vec,
            width: width,
            height: height,
        };
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.resize_with(width, height, <T as Default>::default());
    }

    pub fn resize_with(&mut self, width: i32, height: i32, v: T) {
        self.width = width;
        self.height = height;
        self.data.resize((width * height) as usize, v);
    }
}

#[cfg(test)]
mod tests {
    use crate::{traits::Image, Image2d, Point2d};

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

    #[test]
    fn test_resize() {
        const REFVAL: [u8; 6] = [0, 0, 0, 0, 10, 10];
        let mut img = Image2d::<u8>::new(2, 2);
        img.resize_with(3, 2, 10);
        assert_eq!(img.width(), 3);
        assert_eq!(img.height(), 2);
        for p in img.domain() {
            assert_eq!(
                *img.at_point(&p),
                REFVAL[(p.y() * img.width() + p.x()) as usize]
            );
        }
    }
}
