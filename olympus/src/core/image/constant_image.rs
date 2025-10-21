use std::ops::Index;

use crate::{Domain, Image, Value};

pub struct ConstantImage<D: Domain, V: Value> {
    domain: D,
    value: V,
}

impl<D, V> ConstantImage<D, V>
where
    D: Domain,
    V: Value,
{
    pub fn new(domain: D, value: V) -> ConstantImage<D, V> {
        ConstantImage { domain, value }
    }
}

impl<D, V> Index<D::Point> for ConstantImage<D, V>
where
    D: Domain,
    V: Value,
{
    type Output = V;

    fn index(&self, index: D::Point) -> &Self::Output {
        debug_assert!(self.domain.has(&index));
        &self.value
    }
}

impl<D, V> Image for ConstantImage<D, V>
where
    D: Domain,
    V: Value,
{
    type Domain = D;
    type Value = V;

    fn domain(&self) -> &Self::Domain {
        &self.domain
    }

    fn at(&self, p: &<Self::Domain as Domain>::Point) -> Option<&Self::Value> {
        if self.domain.has(p) {
            Some(&self.value)
        } else {
            None
        }
    }
}
