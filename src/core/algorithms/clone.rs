use crate::traits::{Domain, Image, MutableImage, SizedDomain};

use super::copy;

pub fn clone<I, O>(input: &I) -> O
where
    I: Image,
    I::Domain: Domain + SizedDomain,
    O: MutableImage<Domain = I::Domain>,
    I::Value: Copy + Into<O::Value>,
{
    let mut out = O::new_from_domain(&input.domain());
    copy(input, &mut out).unwrap();
    out
}
