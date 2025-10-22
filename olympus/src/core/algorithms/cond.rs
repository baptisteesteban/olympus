use crate::{Domain, ImageMut, SizedDomain};

/// Apply a predication `f` on the image `img` and returns an image whose pixels
/// are set to `true` if the predicate is valid, `false` otherwise.
pub fn cond<I, F>(img: &I, f: F) -> I::ChangeValue<bool>
where
    I: ImageMut,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
    F: Fn(&<I::Domain as Domain>::Point, &I::Value) -> bool,
{
    let mut res = img.imchvalue::<bool>();
    for p in img.domain().clone() {
        res[p] = f(&p, &img[p]);
    }
    res
}
