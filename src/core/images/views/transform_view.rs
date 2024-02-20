use std::{marker::PhantomData, ops::Deref};

use crate::traits::Image;

pub struct Transformed<'a, T> {
    v: T,
    _v: PhantomData<&'a ()>,
}

impl<'a, T> Deref for Transformed<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<'a, T> Transformed<'a, T> {
    fn new(v: T) -> Transformed<'a, T> {
        Transformed {
            v: v,
            _v: PhantomData,
        }
    }
}

pub struct TransformView<'a, I, F, O>
where
    I: Image,
    F: Fn(I::ReturnType<'a>) -> O,
{
    img: &'a I,
    f: F,
}

impl<'a, I, F, O> TransformView<'a, I, F, O>
where
    I: Image,
    F: Fn(I::ReturnType<'a>) -> O,
{
    pub fn new(img: &'a I, f: F) -> TransformView<'a, I, F, O> {
        TransformView { img: img, f: f }
    }
}

impl<'a, I, F, O> Image for TransformView<'a, I, F, O>
where
    I: Image,
    F: Fn(I::ReturnType<'a>) -> O,
{
    type Domain = I::Domain;
    type Value = O;
    type ReturnType<'b> = Transformed<'b, O>
    where
        Self: 'b;

    fn domain(&self) -> Self::Domain {
        self.img.domain()
    }

    fn at_point<'c>(
        &'c self,
        p: &<Self::Domain as crate::traits::Domain>::Point,
    ) -> Self::ReturnType<'c> {
        Transformed::new((self.f)(self.img.at_point(&p)))
    }
}
