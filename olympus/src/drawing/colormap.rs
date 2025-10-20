use std::ops::Sub;

use crate::{drawing::Normalizer, BoundedValueSet, Image2d, Rgb8};

/// Apply a color map to an image.
///
/// # Notes
///
/// The reader may be interested by some explaination about correct color maps
/// [here](https://www.kennethmoreland.com/color-advice/)
pub fn apply_colormap<V, F>(img: &Image2d<V>, colormap: F) -> Image2d<Rgb8>
where
    V: Clone + Into<f64> + Ord + BoundedValueSet + Sub<Output = V>,
    F: Fn(f64) -> Rgb8,
{
    let normalize = Normalizer::new(img.values());
    let mut res: Image2d<Rgb8> = Image2d::new(img.width(), img.height()).unwrap();
    for p in *img.domain() {
        res[p] = colormap(normalize.apply(&img[p]));
    }
    res
}
