use crate::Image2d;

pub fn where_image<V>(img: &Image2d<bool>, yes: V, no: V) -> Image2d<V>
where
    V: Clone + Default,
{
    let mut res = Image2d::new(img.width(), img.height()).unwrap();

    for p in *img.domain() {
        res[p] = if img[p] { yes.clone() } else { no.clone() };
    }

    res
}
