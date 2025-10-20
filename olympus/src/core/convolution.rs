use crate::{Image2d, Mask2d, Window};

/// Performs a Gaussian filter on a 2D image.
///
/// # Panics
///
/// This function panics if the image fails to build
pub fn convolution(img: &Image2d<u8>) -> Image2d<u8> {
    let mut res = Image2d::<u8>::new(img.width(), img.height()).unwrap();

    let domain = img.domain();

    let nbh = Mask2d::rect(3, 3).unwrap();
    let mask: [u8; 9] = [1, 2, 1, 2, 4, 2, 1, 2, 1];

    for p in *domain {
        let mut s: u16 = 0;
        for (cur, n) in nbh.apply(&p).enumerate() {
            s += if domain.has(&n) {
                mask[cur] as u16 * img[n] as u16
            } else {
                0u16
            };
        }
        res[p] = (s / 16) as u8;
    }

    res
}
