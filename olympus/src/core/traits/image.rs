use std::ops::{Index, IndexMut};

use crate::{Domain, Value};

/// This trait aims to represent an image as a function from a Domain (point set) to a value set.
pub trait Image: Index<<Self::Domain as Domain>::Point, Output = Self::Value> {
    /// Domain (point set) of an image.
    type Domain: Domain;
    /// Value set of an image.
    type Value: Value;

    /// Accessor to the domain of an image.
    fn domain(&self) -> &Self::Domain;
    /// Safe accessor to a value of an image.
    fn at(&self, p: &<Self::Domain as Domain>::Point) -> Option<&Self::Value>;
}

/// This trait aims to represent an image but whose values are mutable.
pub trait ImageMut:
    Image + IndexMut<<Self::Domain as Domain>::Point, Output = Self::Value>
{
    // Safe mutable accessor to a value of an image.
    fn at_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> Option<&mut Self::Value>;
}
