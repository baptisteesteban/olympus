use num::Bounded;

use crate::{
    accu::{Accumulator, InfAccumulator, SupAccumulator},
    Image2d, StructuringElement2d,
};

mod internals {
    use crate::{
        accu::Accumulator,
        traits::{Domain, Image, MutableImage},
        Image2d, StructuringElement2d,
    };

    pub(crate) fn structural_operation<T, A>(
        img: &Image2d<T>,
        se: &StructuringElement2d,
        mut accu: A,
    ) -> Image2d<T>
    where
        T: Default + Copy,
        A: Accumulator<Input = T, Output = T>,
    {
        let mut out = Image2d::<T>::new(img.width(), img.height());

        let domain = img.domain();
        for p in img.domain() {
            accu.init();
            for q in se.apply(&p) {
                if domain.has(&q) {
                    accu.take(*img.at_point(&q));
                }
            }
            *out.at_point_mut(&p) = accu.result();
        }

        out
    }
}

pub fn erosion<T>(img: &Image2d<T>, se: &StructuringElement2d) -> Image2d<T>
where
    T: Default + Copy + Ord + Bounded,
{
    internals::structural_operation(&img, se, InfAccumulator::new())
}

pub fn dilation<T>(img: &Image2d<T>, se: &StructuringElement2d) -> Image2d<T>
where
    T: Default + Copy + Ord + Bounded,
{
    internals::structural_operation(&img, se, SupAccumulator::new())
}

pub fn opening<T>(img: &Image2d<T>, se: &StructuringElement2d) -> Image2d<T>
where
    T: Default + Copy + Ord + Bounded,
{
    dilation(&erosion(&img, &se), &se)
}

pub fn closing<T>(img: &Image2d<T>, se: &StructuringElement2d) -> Image2d<T>
where
    T: Default + Copy + Ord + Bounded,
{
    erosion(&dilation(&img, &se), &se)
}
