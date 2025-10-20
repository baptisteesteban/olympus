use std::ops::Sub;

use crate::BoundedValueSet;

/// A `Normalizer` is an object that takes a set of values (represented by an
/// iterator) and save their infimum and their supremum. It is used to obtain a
/// normalized value between 0 and 1.
pub struct Normalizer<V: Ord + BoundedValueSet + Sub<Output = V> + Into<f64> + Clone> {
    min: V,
    max: V,
}

impl<V: Ord + BoundedValueSet + Sub<Output = V> + Into<f64> + Clone> Normalizer<V> {
    /// Creates a new `Normalizer` object from a set of values.
    pub fn new(values: impl Iterator<Item = V>) -> Normalizer<V> {
        let mut min = <V as BoundedValueSet>::sup();
        let mut max = <V as BoundedValueSet>::inf();

        for v in values {
            min = std::cmp::min(min, v.clone());
            max = std::cmp::max(max, v.clone());
        }

        Normalizer { min, max }
    }

    pub fn min(&self) -> &V {
        &self.min
    }

    pub fn max(&self) -> &V {
        &self.max
    }

    /// Project the value `a` between 0 and 1 from a set of values.
    pub fn apply(&self, a: &V) -> f64 {
        (a.clone() - self.min.clone()).into() / (self.max.clone() - self.min.clone()).into()
    }
}
