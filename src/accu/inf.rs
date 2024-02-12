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

    fn take_accu(&mut self, ac: Self) {
        self.v = min(self.v, ac.v);
    }

    fn result(&self) -> Self::Output {
        self.v
    }
}

#[cfg(test)]
mod tests {
    use crate::accu::Accumulator;

    use super::InfAccumulator;

    #[test]
    fn test_accu_inf() {
        let mut acc = InfAccumulator::<u8>::new();
        assert_eq!(acc.result(), u8::MAX);
        acc.take(10);
        acc.take(3);
        acc.take(100);
        acc.take(1);
        assert_eq!(acc.result(), 1);

        let mut acc2 = InfAccumulator::<u8>::new();
        acc2.take(28);
        acc2.take(104);
        assert_eq!(acc2.result(), 28);
        acc2.take_accu(acc);
        assert_eq!(acc2.result(), 1);
        acc2.init();
        assert_eq!(acc2.result(), u8::MAX);
    }
}
