use num::Bounded;

use super::Accumulator;

use std::cmp::min;

pub struct InfAccumulator<T: Bounded + Ord + Copy> {
    v: T,
}

impl<T: Bounded + Ord + Copy> Accumulator for InfAccumulator<T> {
    type Input = T;
    type Output = T;

    fn new() -> InfAccumulator<T> {
        InfAccumulator { v: T::max_value() }
    }

    fn init(&mut self) {
        self.v = T::max_value();
    }

    fn take(&mut self, v: Self::Input) {
        self.v = min(self.v, v);
    }

    fn take_accu(&mut self, ac: &Self) {
        self.v = min(self.v, ac.v);
    }

    fn result(&self) -> Self::Output {
        self.v
    }
}

impl<T: Bounded + Ord + Copy> Default for InfAccumulator<T> {
    fn default() -> Self {
        Self::new()
    }
}
