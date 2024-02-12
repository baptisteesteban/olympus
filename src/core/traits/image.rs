use super::Domain;

pub trait Image {
    type Domain: Domain;
    type Value;
    type Point;

    fn new_from_domain(domain: &Self::Domain) -> Self;

    fn domain(&self) -> Self::Domain;
    fn at_point(&self, p: &Self::Point) -> &Self::Value;
}

pub trait MutableImage: Image {
    fn at_point_mut(&mut self, p: &Self::Point) -> &mut Self::Value;
}
