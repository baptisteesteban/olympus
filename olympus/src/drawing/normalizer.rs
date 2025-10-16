use std::ops::Sub;

use crate::BoundedValueSet;

pub struct Normalizer<V: Ord + BoundedValueSet + Sub<Output = V> + Into<f64> + Clone> {
    min: V,
    max: V,
}

impl<V: Ord + BoundedValueSet + Sub<Output = V> + Into<f64> + Clone> Normalizer<V> {
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

    pub fn apply(&self, a: &V) -> f64 {
        (a.clone() - self.min.clone()).into() / (self.max.clone() - self.min.clone()).into()
    }
}
