#![allow(clippy::missing_safety_doc)]

use std::{
    cmp::max,
    ops::{Add, Index, IndexMut, Sub},
};

use crate::{Box2d, Point2d};

/// Implementation of an image defined on a 2D regular grid whose values,
/// encoded by the type `T`, are stored in a contiguous buffer.
#[derive(Debug, Clone)]
pub struct Image2d<T> {
    /// The definition domain of the image. In this case, it is a 2D regular
    /// grid.
    domain: Box2d,
    /// The buffer storing the values of the image pixels.
    data: Vec<T>,
}

impl<T> Image2d<T> {
    /// Build a new image whose dimension are set to `width` and `height` and
    /// whose values given by the vector `vec`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the number of values in the vector
    /// `vec` does not match the dimensions given as arguments.
    pub fn from_vec(width: i32, height: i32, vec: Vec<T>) -> Result<Image2d<T>, String> {
        if (width * height) as usize != vec.len() {
            return Err(String::from(
                "Image and input vector should have the same number of elements",
            ));
        }
        Ok(Image2d {
            domain: Box2d::new(width, height),
            data: vec,
        })
    }

    /// Returns the width of the image.
    #[inline]
    pub fn width(&self) -> i32 {
        self.domain.width()
    }

    /// Returns the height of the image.
    #[inline]
    pub fn height(&self) -> i32 {
        self.domain.height()
    }

    #[inline]
    fn point_to_index(&self, x: i32, y: i32) -> i32 {
        y * self.domain.width() + x
    }

    /// Returns a reference to a value of the image located at point `(x, y)`.
    /// This function returns `None` if the image value is not contained in the
    /// vector.
    ///
    /// *TODO*: Check the coordinate instead of the vector index
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        let i = self.point_to_index(x, y);
        self.data.get(i as usize)
    }

    /// Returns a mutable reference to a value of the image located at point
    /// `(x, y)`. This function returns `None` if the image value is not
    /// contained in the vector.
    ///
    /// *TODO*: Check the coordinate instead of the vector index
    #[inline]
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        let i = self.point_to_index(x, y);
        self.data.get_mut(i as usize)
    }

    /// Returns a reference to a value of the image located at point `(x, y)`.
    ///
    /// # Safety
    ///
    /// This function is unsafe as it does not check the coordinate. Thus, if
    /// the coordinates are invalid, it leads to an undefined behavior.
    #[inline]
    pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &T {
        let i = self.point_to_index(x, y);
        self.data.get_unchecked(i as usize)
    }

    /// Returns a mutable reference to a value of the image located at point
    /// `(x, y)`.
    ///
    /// # Safety
    ///
    /// This function is unsafe as it does not check the coordinate. Thus, if
    /// the coordinates are invalid, it leads to an undefined behavior.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut T {
        let i = self.point_to_index(x, y);
        self.data.get_unchecked_mut(i as usize)
    }

    #[inline]
    pub fn at(&self, p: &Point2d) -> Option<&T> {
        self.get(p.x, p.y)
    }

    #[inline]
    pub fn at_mut(&mut self, p: &Point2d) -> Option<&mut T> {
        self.get_mut(p.x, p.y)
    }

    #[inline]
    pub unsafe fn at_unchecked(&self, p: &Point2d) -> &T {
        self.get_unchecked(p.x, p.y)
    }

    #[inline]
    pub unsafe fn at_unchecked_mut(&mut self, p: &Point2d) -> &mut T {
        self.get_unchecked_mut(p.x, p.y)
    }

    /// Returns the domain of the image.
    pub fn domain(&self) -> &Box2d {
        &self.domain
    }
}

impl<T: Clone> Image2d<T> {
    ///
    pub fn resize_with_value(&mut self, width: i32, height: i32, v: T) {
        self.domain = Box2d::new(max(width, 0), max(height, 0));
        self.data.resize((width * height) as usize, v);
    }

    /// This function returns an iterator over the image pixel values.
    pub fn values(&self) -> impl Iterator<Item = T> {
        self.data.clone().into_iter()
    }
}

impl<T: Default + Clone> Image2d<T> {
    /// Build a new image defined over a 2D regular grid of size `(width, height)`.
    ///
    /// # Errors
    ///
    /// This runction returns an error if the dimensions are incorrect.
    pub fn new(width: i32, height: i32) -> Result<Image2d<T>, String> {
        if width < 0 || height < 0 {
            return Err(String::from("Width and height must be superior to 0"));
        }
        Ok(Image2d {
            domain: Box2d::new(width, height),
            data: vec![T::default(); (width * height) as usize],
        })
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.resize_with_value(width, height, T::default());
    }
}

impl<V> Index<(i32, i32)> for Image2d<V> {
    type Output = V;

    #[inline]
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        debug_assert!(
            index.0 >= 0 && index.1 >= 0 && index.0 < self.width() && index.1 < self.height()
        );
        unsafe { self.get_unchecked(index.0, index.1) }
    }
}

impl<V> IndexMut<(i32, i32)> for Image2d<V> {
    #[inline]
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        debug_assert!(
            index.0 >= 0 && index.1 >= 0 && index.0 < self.width() && index.1 < self.height()
        );
        unsafe { self.get_unchecked_mut(index.0, index.1) }
    }
}

impl<V> Index<Point2d> for Image2d<V> {
    type Output = V;

    #[inline]
    fn index(&self, index: Point2d) -> &Self::Output {
        debug_assert!(self.domain().has(&index));
        unsafe { self.at_unchecked(&index) }
    }
}

impl<V> IndexMut<Point2d> for Image2d<V> {
    #[inline]
    fn index_mut(&mut self, index: Point2d) -> &mut Self::Output {
        debug_assert!(self.domain().has(&index));
        unsafe { self.at_unchecked_mut(&index) }
    }
}

impl<T: Default> Default for Image2d<T> {
    fn default() -> Self {
        Self {
            domain: Box2d::default(),
            data: Default::default(),
        }
    }
}

impl<T: PartialEq> PartialEq for Image2d<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.width() != other.width() || self.height() != other.height() {
            return false;
        }

        for p in self.domain {
            if self[p] != other[p] {
                return false;
            }
        }

        true
    }
}

impl<T> Add for Image2d<T>
where
    T: Add + Copy,
    <T as Add>::Output: Default + Clone,
{
    type Output = Image2d<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Image2d::<<T as Add>::Output>::new(self.width(), self.height()).unwrap();
        for p in self.domain {
            res[p] = self[p] + rhs[p];
        }
        res
    }
}

impl<T> Sub for Image2d<T>
where
    T: Sub + Copy,
    <T as Sub>::Output: Clone + Default,
{
    type Output = Image2d<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Image2d::<<T as Sub>::Output>::new(self.width(), self.height()).unwrap();
        for p in self.domain {
            res[p] = self[p] - rhs[p];
        }
        res
    }
}
