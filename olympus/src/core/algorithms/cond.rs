use crate::{Image2d, Point2d};

pub fn cond<V, F>(img: &Image2d<V>, f: F) -> Image2d<bool>
where
    F: Fn(&Point2d, &V) -> bool,
{
    let mut res = Image2d::<bool>::new(img.width(), img.height()).unwrap();
    for p in *img.domain() {
        res[p] = f(&p, &img[p]);
    }
    res
}
