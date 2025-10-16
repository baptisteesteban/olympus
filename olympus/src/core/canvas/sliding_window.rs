use crate::{Image2d, Mask2d, Window};

pub trait WindowOperation<T: Ord + Copy> {
    fn op(cur: &mut T, a: &T, b: &T);
}

pub fn sliding_window<T, M, Op>(img: &Image2d<T>, mask: &Mask2d) -> Image2d<T>
where
    T: Default + Copy + Ord,
    Op: WindowOperation<T>,
{
    let mut res = Image2d::new(img.width(), img.height()).unwrap();

    let domain = img.domain();

    for p in *domain {
        res[p] = img[p];
        for n in mask.apply(&p) {
            let a = res[p];
            if domain.has(&n) {
                Op::op(&mut res[p], &a, &img[n]);
            }
        }
    }

    res
}
