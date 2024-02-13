use num::Bounded;

use super::Accumulator;

use std::cmp::max;

pub struct SupAccumulator<T: Bounded + Ord + Copy> {
    v: T,
}

impl<T: Bounded + Ord + Copy> Accumulator for SupAccumulator<T> {
    type Input = T;
    type Output = T;

    fn new() -> SupAccumulator<T> {
        SupAccumulator { v: T::min_value() }
    }

    fn init(&mut self) {
        self.v = T::min_value();
    }

    fn take(&mut self, v: Self::Input) {
        self.v = max(self.v, v);
    }

    fn take_accu(&mut self, ac: Self) {
        self.v = max(self.v, ac.v);
    }

    fn result(&self) -> Self::Output {
        self.v
    }
}
