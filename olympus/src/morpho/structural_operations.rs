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

pub fn erosion<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    sliding_window::<T, Mask2d, ErosionOperation<T>>(img, mask)
}

struct DilationOperation<T> {
    _v: PhantomData<T>,
}

impl<T: Ord + Copy> WindowOperation<T> for DilationOperation<T> {
    fn op(cur: &mut T, a: &T, b: &T) {
        *cur = std::cmp::max(*a, *b);
    }
}

pub fn dilation<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    sliding_window::<T, Mask2d, DilationOperation<T>>(img, mask)
}

pub fn opening<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    let inter = erosion(img, mask);
    dilation(&inter, mask)
}

pub fn closing<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
{
    let inter = dilation(img, mask);
    erosion(&inter, mask)
}

pub fn gradient<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let dil = dilation(img, mask);
    let ero = erosion(img, mask);
    dil - ero
}

pub fn internal_gradient<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let ero = erosion(img, mask);
    img.clone() - ero
}

pub fn external_gradient<T>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<<T as Sub>::Output>
where
    T: Default + Copy + Ord + Sub,
    <T as Sub>::Output: Default + Copy,
{
    let dil = dilation(img, mask);
    dil - img.clone()
}
