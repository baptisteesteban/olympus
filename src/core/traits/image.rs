use std::ops::Deref;

use super::Domain;

pub trait Image {
    type Domain: Domain;
    type Value;
    type ReturnType<'a>: Deref<Target = Self::Value>
    where
        Self: 'a;

    fn domain(&self) -> Self::Domain;
    fn at_point<'a>(&'a self, p: &<Self::Domain as Domain>::Point) -> Self::ReturnType<'a>;
}

pub trait MutableImage: Image {
    fn at_point_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> &mut Self::Value;
}

pub trait ImageFromDomain: Image {
    fn new_from_domain(domain: &Self::Domain) -> Self;
    fn new_from_domain_with_value(domain: &Self::Domain, v: Self::Value) -> Self;
}

pub trait ChangeValueImage<T>: Image {
    type ValueChangedImage: Image<Domain = Self::Domain, Value = T>;

    fn change_value(&self) -> Self::ValueChangedImage;
}
