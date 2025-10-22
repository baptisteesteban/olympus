use crate::{ImageMut, SizedDomain};

/// Fill an image `img` with the value `v` set to all its pixels.
pub fn fill<I>(img: &mut I, v: I::Value)
where
    I: ImageMut,
    I::Domain: SizedDomain,
    I::Value: Copy,
{
    for p in img.domain().clone() {
        img[p] = v;
    }
}
