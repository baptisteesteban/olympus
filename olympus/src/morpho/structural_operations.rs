use std::{marker::PhantomData, ops::Sub};

use crate::{sliding_window, Domain, Image, ImageMut, SizedDomain, Window, WindowOperation};

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
pub fn erosion<I, W>(img: &I, se: &W) -> I
where
    I: ImageMut,
    I::Value: Copy + Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    sliding_window::<I, W, ErosionOperation<<I as Image>::Value>>(img, se)
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
pub fn dilation<I, W>(img: &I, se: &W) -> I
where
    I: ImageMut,
    I::Value: Copy + Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    sliding_window::<I, W, DilationOperation<<I as Image>::Value>>(img, se)
}

/// Perform a morphological opening on the image `img` using the structuring
/// element `se`.
pub fn opening<I, W>(img: &I, se: &W) -> I
where
    I: ImageMut,
    I::Value: Copy + Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    let inter = erosion(img, se);
    dilation(&inter, se)
}

/// Perform a morphological closing on the image `img` using the structuring
/// element `se`.
pub fn closing<I, W>(img: &I, se: &W) -> I
where
    I: ImageMut,
    I::Value: Copy + Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    let inter = dilation(img, se);
    erosion(&inter, se)
}

/// Perform a morphological gradient on the image `img` using the structuring
/// element `se`.
pub fn gradient<I, W, O>(img: &I, se: &W) -> O
where
    I: ImageMut + Sub<Output = O>,
    I::Value: Copy + Ord + Sub,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
    O: ImageMut<Domain = I::Domain, Value = <I::Value as Sub>::Output>,
{
    let dil = dilation(img, se);
    let ero = erosion(img, se);
    dil - ero
}

/*
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
}*/
