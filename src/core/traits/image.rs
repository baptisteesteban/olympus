use super::Domain;

pub trait Image {
    type Domain: Domain;
    type Value;

    fn domain(&self) -> Self::Domain;
    fn at_point(&self, p: &<Self::Domain as Domain>::Point) -> &Self::Value;
}

pub trait MutableImage: Image {
    fn at_point_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> &mut Self::Value;
}

pub trait ImageFromDomain: Image {
    fn new_from_domain(domain: &Self::Domain) -> Self;
    fn new_from_domain_with_value(domain: &Self::Domain, v: Self::Value) -> Self;
}
