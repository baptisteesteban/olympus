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

#[cfg(test)]
mod tests {
    use crate::{morpho::dilation, Image2d, StructuringElement2d};

    #[test]
    fn test_dilation() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24,
            ]),
        );
        let ref_img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 21, 22,
                23, 24, 24,
            ]),
        );

        let se = StructuringElement2d::new(
            3,
            3,
            vec![false, true, false, true, true, true, false, true, false],
        )
        .unwrap();
        let out = dilation(&img, &se);

        assert!(out == ref_img);
    }
}
