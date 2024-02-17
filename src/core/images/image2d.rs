use crate::traits::{ChangeValueImage, Domain, Image, ImageFromDomain, MutableImage};
use crate::Box2d;

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

    pub fn new_from_vec(width: i32, height: i32, data: Vec<T>) -> Image2d<T> {
        Image2d {
            data: data,
            width: width,
            height: height,
        }
    }
}

impl<T> Image for Image2d<T> {
    type Domain = Box2d;
    type Value = T;

    fn domain(&self) -> Self::Domain {
        Self::Domain::new(self.width, self.height)
    }

    fn at_point(&self, p: &<Self::Domain as Domain>::Point) -> &Self::Value {
        self.at(p.x(), p.y())
    }
}

impl<T> ImageFromDomain for Image2d<T>
where
    T: Default + Clone,
{
    fn new_from_domain(domain: &Self::Domain) -> Self {
        Image2d::new(domain.width(), domain.height())
    }

    fn new_from_domain_with_value(domain: &Self::Domain, v: Self::Value) -> Self {
        Image2d::new_from_vec(
            domain.width(),
            domain.height(),
            vec![v; (domain.width() * domain.height()) as usize],
        )
    }
}

impl<T> MutableImage for Image2d<T> {
    fn at_point_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> &mut Self::Value {
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

impl<T, V> ChangeValueImage<T> for Image2d<V>
where
    T: Default + Copy,
{
    type ValueChangedImage = Image2d<T>;

    fn change_value(&self) -> Self::ValueChangedImage {
        Image2d::<T>::new_from_domain(&self.domain())
    }
}

impl<T, V> PartialEq<Image2d<T>> for Image2d<V>
where
    V: PartialEq<T>,
{
    fn eq(&self, other: &Image2d<T>) -> bool {
        if self.domain() != other.domain() {
            return false;
        }
        for p in self.domain() {
            if *self.at_point(&p) != *other.at_point(&p) {
                return false;
            }
        }
        true
    }
}
