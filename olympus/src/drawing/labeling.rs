use crate::{ImageMut, Rgb8, SizedDomain, BLACK};

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

pub fn label2rgb<I>(labels: &I) -> I::ChangeValue<Rgb8>
where
    I: ImageMut<Value = u16>,
    I::Domain: SizedDomain,
{
    let mut res = unsafe { labels.imchvalue_uninitialized() };
    for p in labels.domain().clone() {
        res[p] = if labels[p] == 0 {
            BLACK
        } else {
            COLORS[(labels[p] % 10) as usize]
        };
    }
    res
}
