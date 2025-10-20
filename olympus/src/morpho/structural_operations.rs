use std::{marker::PhantomData, ops::Sub};

use crate::{sliding_window, Image2d, Mask2d, WindowOperation};

struct ErosionOperation<T> {
    _v: PhantomData<T>,
}
impl<T: Ord + Copy> WindowOperation<T> for ErosionOperation<T> {
    fn op(cur: &mut T, a: &T, b: &T) {
        *cur = std::cmp::min(*a, *b);
    }
}

/// Perform a morphological erosion on the image `img` using the structuring
/// element `se`.
pub fn erosion<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    sliding_window::<T, Mask2d, ErosionOperation<T>>(img, se)
}

struct DilationOperation<T> {
    _v: PhantomData<T>,
}

impl<T: Ord + Copy> WindowOperation<T> for DilationOperation<T> {
    fn op(cur: &mut T, a: &T, b: &T) {
        *cur = std::cmp::max(*a, *b);
    }
}

/// Perform a morphological dilation on the image `img` using the structuring
/// element `se`.
pub fn dilation<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    sliding_window::<T, Mask2d, DilationOperation<T>>(img, se)
}

/// Perform a morphological opening on the image `img` using the structuring
/// element `se`.
pub fn opening<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    let inter = erosion(img, se);
    dilation(&inter, se)
}

/// Perform a morphological closing on the image `img` using the structuring
/// element `se`.
pub fn closing<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    let inter = dilation(img, se);
    erosion(&inter, se)
}

/// Perform a morphological gradient on the image `img` using the structuring
/// element `se`.
pub fn gradient<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let dil = dilation(img, se);
    let ero = erosion(img, se);
    dil - ero
}

/// Perform a morphological internal gradient on the image `img` using the
/// structuring element `se`.
pub fn internal_gradient<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let ero = erosion(img, se);
    img.clone() - ero
}

/// Perform a morphological external gradient on the image `img` using the
/// structuring element `se`.
pub fn external_gradient<T>(img: &Image2d<T>, se: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let dil = dilation(img, se);
    dil - img.clone()
}
