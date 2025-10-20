use crate::Image2d;

/// Returns an image whose value at a given pixel `p` is set to `yes` if the
/// value of `img` is `true` at pixel `p`, otherwise the value is set to `no`.
///
/// # Panics
///
/// The function panics if the construction of the image fails.
pub fn where_image<V>(img: &Image2d<bool>, yes: V, no: V) -> Image2d<V>
where
    V: Clone + Default,
{
    let mut res = unsafe { Image2d::new_uninitialized(img.width(), img.height()).unwrap() };

    for p in *img.domain() {
        res[p] = if img[p] { yes.clone() } else { no.clone() };
    }

    res
}
