use crate::traits::{Domain, Image, ShapedDomain};

pub struct ValueExtended<'a, I>
where
    I: Image,
    I::Domain: ShapedDomain,
{
    img: &'a I,
    v: I::Value,
    domain: I::Domain,
}

impl<'a, I> ValueExtended<'a, I>
where
    I: Image,
    I::Domain: ShapedDomain,
{
    pub fn new(img: &'a I, v: I::Value) -> ValueExtended<'a, I> {
        ValueExtended {
            img: img,
            v: v,
            domain: (*img).domain(),
        }
    }
}

impl<'a, I> Image for ValueExtended<'a, I>
where
    I: Image,
    I::Value: Copy,
    I::ReturnType<'a>: From<I::Value>,
    I::Domain: ShapedDomain,
{
    type Domain = I::Domain;
    type Value = I::Value;
    type ReturnType<'b> = I::ReturnType<'a>
    where
        Self: 'b;

    fn domain(&self) -> Self::Domain {
        self.domain
    }

    fn at_point<'b>(&'b self, p: &<Self::Domain as Domain>::Point) -> Self::ReturnType<'b> {
        if self.domain.has(&p) {
            self.img.at_point(&p)
        } else {
            self.v.into()
        }
    }
}
