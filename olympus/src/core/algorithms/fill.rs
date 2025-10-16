use crate::Image2d;

pub fn fill<V>(img: &mut Image2d<V>, v: V)
where
    V: Copy,
{
    for p in *img.domain() {
        img[p] = v;
    }
}
