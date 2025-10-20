use olympus::Image2d;

use crate::Range;

/// Immerse an image `img` in an interval set-valued map as defined
/// [here](https://hal.science/hal-00798620/document) and
/// [here](https://hal.science/hal-00798574v1/file/discreteContinuityISMM2013.pdf).
pub fn immersion<V>(img: &Image2d<V>) -> Image2d<Range<V>>
where
    V: Ord + Copy + Default,
{
    let mut res = unsafe {
        Image2d::<Range<V>>::new_uninitialized(2 * img.width() - 1, 2 * img.height() - 1).unwrap()
    };

    // 2-faces
    for p in *img.domain() {
        res[p * 2] = Range::<V>::new_single_value(img[p]);
    }

    // 1-faces horizontal
    for x in (0..res.width()).step_by(2) {
        for y in (1..res.height()).step_by(2) {
            res[(x, y)].l = std::cmp::min(res[(x, y - 1)].l, res[(x, y + 1)].l);
            res[(x, y)].h = std::cmp::max(res[(x, y - 1)].h, res[(x, y + 1)].h);
        }
    }

    // 1-faces vertical
    for x in (1..res.width()).step_by(2) {
        for y in (0..res.height()).step_by(2) {
            res[(x, y)].l = std::cmp::min(res[(x + 1, y)].l, res[(x - 1, y)].l);
            res[(x, y)].h = std::cmp::max(res[(x + 1, y)].h, res[(x - 1, y)].h);
        }
    }

    // 0-faces
    for x in (1..res.width()).step_by(2) {
        for y in (1..res.height()).step_by(2) {
            res[(x, y)].l = std::cmp::min(
                std::cmp::min(res[(x + 1, y)].l, res[(x - 1, y)].l),
                std::cmp::min(res[(x, y + 1)].l, res[(x, y - 1)].l),
            );
            res[(x, y)].h = std::cmp::max(
                std::cmp::max(res[(x + 1, y)].h, res[(x - 1, y)].h),
                std::cmp::max(res[(x, y + 1)].h, res[(x, y - 1)].h),
            );
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use olympus::Image2d;

    use crate::{immersion, Range};

    #[test]
    fn test_immersion() {
        let img = Image2d::<u8>::from_vec(2, 2, vec![1, 2, 3, 4]).unwrap();
        let ref_img = Image2d::<Range<u8>>::from_vec(
            3,
            3,
            vec![
                Range::new_single_value(1),
                Range::new(1, 2).unwrap(),
                Range::new_single_value(2),
                Range::new(1, 3).unwrap(),
                Range::new(1, 4).unwrap(),
                Range::new(2, 4).unwrap(),
                Range::new_single_value(3),
                Range::new(3, 4).unwrap(),
                Range::new_single_value(4),
            ],
        )
        .unwrap();
        let res = immersion(&img);
        assert_eq!(res, ref_img);
    }
}
