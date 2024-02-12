use crate::traits::{Domain, Image, MutableImage, SizedDomain};

pub fn transform<I, F, O, R>(input: &I, f: F) -> O
where
    I: Image,
    I::Domain: Domain<Point = I::Point> + SizedDomain,
    O: MutableImage<Domain = I::Domain, Point = I::Point>,
    F: Fn(&I::Value) -> R,
    R: Into<O::Value>,
{
    let mut out = O::new_from_domain(&input.domain());

    for p in input.domain() {
        *out.at_point_mut(&p) = <R as Into<O::Value>>::into(f(input.at_point(&p)));
    }

    out
}
