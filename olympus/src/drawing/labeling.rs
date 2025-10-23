use crate::{Image, Image2d, Rgb8, BLACK};

// From scikit-image `label2rgb`
const COLORS: [Rgb8; 10] = [
    Rgb8::new(255, 0, 0),
    Rgb8::new(0, 0, 255),
    Rgb8::new(255, 255, 0),
    Rgb8::new(255, 0, 255),
    Rgb8::new(0, 128, 0),
    Rgb8::new(74, 0, 130),
    Rgb8::new(255, 139, 0),
    Rgb8::new(0, 255, 255),
    Rgb8::new(255, 192, 202),
    Rgb8::new(154, 205, 49),
];

pub fn label2rgb(labels: &Image2d<u16>) -> Image2d<Rgb8> {
    let mut res =
        unsafe { Image2d::<Rgb8>::new_uninitialized(labels.width(), labels.height()).unwrap() };
    for p in *labels.domain() {
        res[p] = if labels[p] == 0 {
            BLACK
        } else {
            COLORS[(labels[p] % 10) as usize]
        };
    }
    res
}
