use crate::traits::{ChangeValueImage, Domain, Image, ImageFromDomain};

pub struct ConstantImage<V, D: Domain> {
    v: V,
    domain: D,
}

impl<V, D: Domain> Image for ConstantImage<V, D> {
    type Domain = D;
    type Value = V;

    fn domain(&self) -> Self::Domain {
        return self.domain.clone();
    }

    fn at_point(&self, _p: &<Self::Domain as Domain>::Point) -> &Self::Value {
        &self.v
    }
}

impl<V, D: Domain> ImageFromDomain for ConstantImage<V, D>
where
    V: Default + Copy,
{
    fn new_from_domain(domain: &Self::Domain) -> Self {
        ConstantImage {
            v: Default::default(),
            domain: domain.clone(),
        }
    }

    fn new_from_domain_with_value(domain: &Self::Domain, v: Self::Value) -> Self {
        ConstantImage {
            v: v,
            domain: domain.clone(),
        }
    }
}

impl<T, D, V> ChangeValueImage<T> for ConstantImage<V, D>
where
    D: Domain + Clone,
    V: Into<T> + Copy,
{
    type ValueChangedImage = ConstantImage<T, Self::Domain>;

    fn change_value(&self) -> Self::ValueChangedImage {
        ConstantImage {
            v: self.v.into(),
            domain: self.domain.clone(),
        }
    }
}

impl<T, D> PartialEq for ConstantImage<T, D>
where
    D: Domain,
    T: PartialEq,
{
    fn eq(&self, other: &ConstantImage<T, D>) -> bool {
        self.domain() == other.domain() && self.v == other.v
    }
}
