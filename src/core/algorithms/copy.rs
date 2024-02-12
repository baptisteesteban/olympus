use crate::traits::{Image, MutableImage, SizedDomain};

pub fn copy<I, O>(src: &I, dst: &mut O) -> Result<(), String>
where
    I: Image,
    I::Domain: SizedDomain,
    I::Value: Copy + Into<O::Value>,
    O: MutableImage<Domain = I::Domain>,
{
    let dom = src.domain();
    if dom != dst.domain() {
        return Err(String::from("Domain does not have the same size"));
    }
    for p in dom {
        *dst.at_point_mut(&p) = <I::Value as Into<O::Value>>::into(*src.at_point(&p));
    }
    Ok(())
}
