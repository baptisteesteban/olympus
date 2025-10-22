use crate::{Domain, ImageMut, SizedDomain, Window};

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
pub fn sliding_window<I, W, Op>(img: &I, mask: &W) -> I
where
    I: ImageMut,
    I::Value: Copy + Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
    Op: WindowOperation<I::Value>,
{
    let mut res = img.duplicate();

    for p in img.domain().clone() {
        res[p] = img[p];
        for n in mask.apply(&p) {
            let a = res[p];
            if img.domain().has(&n) {
                Op::op(&mut res[p], &a, &img[n]);
            }
        }
    }

    res
}
