use crate::Image2d;

pub fn copy<V: Copy>(src: &Image2d<V>, dst: &mut Image2d<V>) -> Result<(), String> {
    let dom = src.domain();
    if dom != dst.domain() {
        return Err(String::from("Domain does not have the same size"));
    }
    for p in dom {
        *dst.at_point_mut(&p) = *src.at_point(&p);
    }
    Ok(())
}
