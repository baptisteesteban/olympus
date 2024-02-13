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
    use crate::{
        morpho::{closing, dilation, erosion, opening},
        Image2d, StructuringElement2d,
    };

    #[test]
    fn test_dilation() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 231, 119, 69, 2, 60, 57, 51, 24, 181, 24, 245, 159, 249, 40, 217, 197, 48, 236,
                79, 45, 105, 151, 144, 27,
            ]),
        );
        let ref_img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                231, 231, 231, 119, 181, 60, 245, 159, 249, 181, 245, 245, 249, 249, 249, 217, 245,
                236, 249, 236, 217, 197, 151, 236, 144,
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

    #[test]
    fn test_erosion() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 231, 119, 69, 2, 60, 57, 51, 24, 181, 24, 245, 159, 249, 40, 217, 197, 48, 236,
                79, 45, 105, 151, 144, 27,
            ]),
        );
        let ref_img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 57, 51, 2, 2, 24, 51, 24, 24, 2, 24, 24, 48, 24, 40, 24, 48, 48, 48, 27, 45,
                45, 48, 27, 27,
            ]),
        );

        let se = StructuringElement2d::new(
            3,
            3,
            vec![false, true, false, true, true, true, false, true, false],
        )
        .unwrap();
        let out = erosion(&img, &se);

        assert!(out == ref_img);
    }

    #[test]
    fn test_opening() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 231, 119, 69, 2, 60, 57, 51, 24, 181, 24, 245, 159, 249, 40, 217, 197, 48, 236,
                79, 45, 105, 151, 144, 27,
            ]),
        );
        let ref_img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 58, 57, 51, 2, 58, 57, 51, 24, 40, 24, 51, 48, 48, 40, 48, 48, 48, 48, 48, 45,
                48, 48, 48, 27,
            ]),
        );

        let se = StructuringElement2d::new(
            3,
            3,
            vec![false, true, false, true, true, true, false, true, false],
        )
        .unwrap();
        let out = opening(&img, &se);

        assert!(out == ref_img);
    }

    #[test]
    fn test_closing() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                58, 231, 119, 69, 2, 60, 57, 51, 24, 181, 24, 245, 159, 249, 40, 217, 197, 48, 236,
                79, 45, 105, 151, 144, 27,
            ]),
        );
        let ref_img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::<u8>::from([
                60, 231, 119, 119, 119, 60, 60, 159, 119, 181, 60, 245, 159, 249, 181, 217, 197,
                151, 236, 144, 197, 151, 151, 144, 144,
            ]),
        );

        let se = StructuringElement2d::new(
            3,
            3,
            vec![false, true, false, true, true, true, false, true, false],
        )
        .unwrap();
        let out = closing(&img, &se);

        assert!(out == ref_img);
    }
}
