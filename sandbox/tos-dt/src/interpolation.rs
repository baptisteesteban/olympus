use std::marker::PhantomData;

use olympus::Image2d;

use crate::MedianAccumulator;

// Helper for interpolation operators

trait InterpolationOperator<V> {
    fn from_2(a: V, b: V) -> V;
    fn from_4(a: V, b: V, c: V, d: V) -> V;
}

struct MaxInterpolation<V> {
    _a: PhantomData<V>,
}

impl<V> InterpolationOperator<V> for MaxInterpolation<V>
where
    V: Ord,
{
    fn from_2(a: V, b: V) -> V {
        std::cmp::max(a, b)
    }

    fn from_4(a: V, b: V, c: V, d: V) -> V {
        std::cmp::max(std::cmp::max(a, b), std::cmp::max(c, d))
    }
}

struct MinInterpolation<V> {
    _a: PhantomData<V>,
}

impl<V> InterpolationOperator<V> for MinInterpolation<V>
where
    V: Ord,
{
    fn from_2(a: V, b: V) -> V {
        std::cmp::min(a, b)
    }

    fn from_4(a: V, b: V, c: V, d: V) -> V {
        std::cmp::min(std::cmp::max(a, b), std::cmp::min(c, d))
    }
}

struct MedianInterpolation {}

impl InterpolationOperator<u8> for MedianInterpolation {
    fn from_2(a: u8, b: u8) -> u8 {
        let mut acc = MedianAccumulator::default();
        acc.take(a);
        acc.take(b);
        acc.result()
    }

    fn from_4(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let mut acc = MedianAccumulator::default();
        acc.take(a);
        acc.take(b);
        acc.take(c);
        acc.take(d);
        acc.result()
    }
}

fn interpolation<F, V>(img: &Image2d<V>) -> Image2d<V>
where
    F: InterpolationOperator<V>,
    V: Default + Clone + Copy,
{
    let mut res = Image2d::new(2 * img.width() - 1, 2 * img.height() - 1).unwrap();

    for p in *res.domain() {
        if p.x % 2 == 0 && p.y % 2 == 0 {
            res[p] = img[p / 2];
        } else if p.x % 2 == 0 && p.y % 2 == 1 {
            res[p] = F::from_2(img[(p.x / 2, p.y / 2)], img[(p.x / 2, p.y / 2 + 1)])
        } else if p.x % 2 == 1 && p.y % 2 == 0 {
            *res.at_mut(&p).unwrap() =
                F::from_2(img[(p.x / 2, p.y / 2)], img[(p.x / 2 + 1, p.y / 2)])
        }
    }

    for p in *res.domain() {
        if p.x % 2 == 1 && p.y % 2 == 1 {
            *res.at_mut(&p).unwrap() = F::from_4(
                res[(p.x - 1, p.y)],
                res[(p.x + 1, p.y)],
                res[(p.x, p.y - 1)],
                res[(p.x, p.y + 1)],
            )
        }
    }

    res
}

pub fn interpolation_max<V>(img: &Image2d<V>) -> Image2d<V>
where
    V: Default + Clone + Copy + Ord,
{
    interpolation::<MaxInterpolation<V>, V>(img)
}

pub fn interpolation_min<V>(img: &Image2d<V>) -> Image2d<V>
where
    V: Default + Clone + Copy + Ord,
{
    interpolation::<MinInterpolation<V>, V>(img)
}

pub fn interpolation_median(img: &Image2d<u8>) -> Image2d<u8> {
    interpolation::<MedianInterpolation, u8>(img)
}

#[cfg(test)]
mod tests {
    use olympus::Image2d;

    use crate::{interpolation_max, interpolation_median, interpolation_min};

    #[test]
    fn test_max_interpolation() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![0, 1, 2, 3]).unwrap();
        let ref_img = Image2d::<u8>::from_vec(3, 3, vec![0, 1, 1, 2, 3, 3, 2, 3, 3]).unwrap();
        let interpolated = interpolation_max(&img);
        assert_eq!(interpolated, ref_img);
    }

    #[test]
    fn test_min_interpolation() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![0, 1, 2, 3]).unwrap();
        let ref_img = Image2d::<u8>::from_vec(3, 3, vec![0, 0, 1, 0, 0, 1, 2, 2, 3]).unwrap();
        let interpolated = interpolation_min(&img);
        assert_eq!(interpolated, ref_img);
    }

    #[test]
    fn test_median_interpolation() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![0, 0, 6, 8]).unwrap();
        let ref_img = Image2d::<u8>::from_vec(3, 3, vec![0, 0, 0, 3, 3, 4, 6, 7, 8]).unwrap();
        let interpolated = interpolation_median(&img);
        assert_eq!(interpolated, ref_img);
    }
}
