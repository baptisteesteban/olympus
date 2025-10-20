use crate::{Domain, ImageMut, SizedDomain};

// TODO: Change `yes` and `no` by images

/// Returns an image whose value at a given pixel `p` is set to `yes` if the
/// value of `img` is `true` at pixel `p`, otherwise the value is set to `no`.
///
/// # Panics
///
/// The function panics if the construction of the image fails.
pub fn where_image<V, I>(img: &I, yes: V, no: V) -> I::ChangeValue<V>
where
    V: Clone,
    I: ImageMut<Value = bool>,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Copy,
{
    let mut res = img.imchvalue::<V>();

    for p in img.domain().clone() {
        res[p] = if img[p] { yes.clone() } else { no.clone() };
    }

    res
}
