use crate::{Image2d, Point2d};

/// Apply a predication `f` on the image `img` and returns an image whose pixels
/// are set to `true` if the predicate is valid, `false` otherwise.
///
/// # Panics
///
/// This function panics if the output image construction fails.
pub fn cond<V, F>(img: &Image2d<V>, f: F) -> Image2d<bool>
where
    F: Fn(&Point2d, &V) -> bool,
{
    let mut res = unsafe { Image2d::<bool>::new_uninitialized(img.width(), img.height()).unwrap() };
    for p in *img.domain() {
        res[p] = f(&p, &img[p]);
    }
    res
}
