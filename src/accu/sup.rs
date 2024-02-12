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

#[cfg(test)]
mod tests {
    use crate::accu::Accumulator;

    use super::SupAccumulator;

    #[test]
    fn test_accu_sup() {
        let mut acc = SupAccumulator::<u8>::new();
        assert_eq!(acc.result(), u8::MIN);
        acc.take(20);
        acc.take(63);
        acc.take(48);
        assert_eq!(acc.result(), 63);

        let mut acc2 = SupAccumulator::<u8>::new();
        acc2.take(13);
        acc2.take(46);
        acc2.take(32);
        assert_eq!(acc2.result(), 46);
        acc2.take_accu(acc);
        assert_eq!(acc2.result(), 63);
        acc2.init();
        assert_eq!(acc2.result(), u8::MIN);
    }
}
