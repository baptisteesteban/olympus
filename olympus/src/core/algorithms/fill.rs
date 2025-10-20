use crate::{Image, Image2d};

/// Fill an image `img` with the value `v` set to all its pixels.
pub fn fill<V>(img: &mut Image2d<V>, v: V)
where
    V: Copy,
{
    for p in *img.domain() {
        img[p] = v;
    }
}
