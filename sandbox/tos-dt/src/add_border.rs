use olympus::{Image2d, Point2d};

use crate::MedianAccumulator;

/// Add an artificial border to the image `img` whose value is set to `v`.
pub fn add_border<V>(img: &Image2d<V>, v: V) -> Image2d<V>
where
    V: Copy + Default,
{
    let mut res =
        unsafe { Image2d::<V>::new_uninitialized(img.width() + 2, img.height() + 2).unwrap() };
    const DP: Point2d = Point2d::new(1, 1);

    for p in *img.domain() {
        res[p + DP] = img[p];
    }

    let (width, height) = (res.width(), res.height());

    for x in 0..width {
        res[(x, 0)] = v;
        res[(x, height - 1)] = v;
    }
    for y in 1..height - 1 {
        res[(0, y)] = v;
        res[(width - 1, y)] = v;
    }

    res
}

fn get_border_median_value(img: &Image2d<u8>) -> u8 {
    let (width, height) = (img.width(), img.height());
    let mut acc =
        MedianAccumulator::new_with_capacity(2 * width as usize + 2 * (height as usize - 2));

    for x in 0..width {
        acc.take(img[(x, 0)]);
        acc.take(img[(x, height - 1)]);
    }
    for y in 1..height - 1 {
        acc.take(img[(0, y)]);
        acc.take(img[(width - 1, y)]);
    }

    acc.result()
}

/// Add an artificial border to `img` whose value is set to the median value of
/// the border of `img`.
pub fn add_median_border(img: &Image2d<u8>) -> Image2d<u8> {
    let median = get_border_median_value(img);
    add_border(img, median)
}

#[cfg(test)]
mod tests {
    use olympus::Image2d;

    use crate::{add_border, add_median_border};

    #[test]
    fn test_add_border_constant_value() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![1, 2, 3, 4]).unwrap();
        let ref_img = Image2d::<u8>::from_vec(
            4,
            4,
            vec![10, 10, 10, 10, 10, 1, 2, 10, 10, 3, 4, 10, 10, 10, 10, 10],
        )
        .unwrap();
        let res = add_border(&img, 10);
        assert_eq!(res, ref_img);
    }

    #[test]
    fn test_add_median_border() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![1, 2, 3, 4]).unwrap();
        let ref_img =
            Image2d::<u8>::from_vec(4, 4, vec![2, 2, 2, 2, 2, 1, 2, 2, 2, 3, 4, 2, 2, 2, 2, 2])
                .unwrap();
        let res = add_median_border(&img);
        assert_eq!(res, ref_img);
    }
}
