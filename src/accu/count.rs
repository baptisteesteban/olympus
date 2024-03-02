use crate::Point2d;

use super::{Accumulator, UntakeAccumulator};

#[derive(Clone)]
pub struct CountAccumulator {
    value: usize,
}

impl Accumulator for CountAccumulator {
    type Input = Point2d;
    type Output = usize;

    fn new() -> Self {
        CountAccumulator { value: 0 }
    }

    fn init(&mut self) {
        self.value = 0;
    }

    fn take(&mut self, _v: Self::Input) {
        self.value += 1;
    }

    fn take_accu(&mut self, ac: &Self) {
        self.value += ac.value
    }

    fn result(&self) -> Self::Output {
        self.value
    }
}

impl UntakeAccumulator for CountAccumulator {
    fn untake(&mut self, _v: Self::Input) {
        self.value -= 1;
    }

    fn untake_accu(&mut self, ac: &Self) {
        self.value -= ac.value
    }
}

impl Default for CountAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
