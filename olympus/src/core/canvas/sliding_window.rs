use crate::{Image2d, Mask2d, Window};

/// This trait defines the operation to be applied in the sliding window algorithmic canvas.
pub trait WindowOperation<T: Ord + Copy> {
    /// The operation called in the sliding window algorithmic canvas.
    fn op(cur: &mut T, a: &T, b: &T);
}

/// This function is an algorithmic canvas in which a sliding window traverses
/// the whole image and applies an operation using the value in this window.
/// Here, the window is binary and only the values from the image are used.
///
/// # Panics
///
/// Panics if the output image construction fails.
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
