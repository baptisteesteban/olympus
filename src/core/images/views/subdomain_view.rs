use crate::traits::{Domain, Image};

pub struct SubdomainView<'a, I, D>
where
    I: Image,
    D: Domain<Point = <<I as Image>::Domain as Domain>::Point>,
{
    base: &'a I,
    domain: D,
}

impl<'a, I, D> SubdomainView<'a, I, D>
where
    I: Image,
    D: Domain<Point = <<I as Image>::Domain as Domain>::Point>,
{
    pub fn new(img: &'a I, domain: D) -> SubdomainView<'a, I, D> {
        SubdomainView {
            base: img,
            domain: domain,
        }
    }
}

impl<'a, I, D> Image for SubdomainView<'a, I, D>
where
    I: Image,
    D: Domain<Point = <<I as Image>::Domain as Domain>::Point>,
{
    type Domain = D;
    type Value = <I as Image>::Value;
    type ReturnType<'b> = <I as Image>::ReturnType<'b>
    where
        Self: 'b;

    fn domain(&self) -> Self::Domain {
        self.domain
    }

    fn at_point<'b>(&'b self, p: &<Self::Domain as Domain>::Point) -> Self::ReturnType<'b> {
        self.base.at_point(&p)
    }
}
