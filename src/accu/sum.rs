use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use super::{Accumulator, UntakeAccumulator};

pub struct SumAccumulator<V, O> {
    v: O,
    _v: PhantomData<V>,
}

impl<V, O> Accumulator for SumAccumulator<V, O>
where
    V: Into<O>,
    O: Add<O, Output = O> + Default + Copy,
{
    type Input = V;
    type Output = O;

    fn new() -> Self {
        Self {
            v: O::default(),
            _v: PhantomData,
        }
    }

    fn init(&mut self) {
        self.v = O::default();
    }

    fn take(&mut self, v: Self::Input) {
        self.v = self.v + v.into();
    }

    fn take_accu(&mut self, ac: &Self) {
        self.v = self.v + ac.v
    }

    fn result(&self) -> Self::Output {
        self.v
    }
}

impl<V, O> UntakeAccumulator for SumAccumulator<V, O>
where
    V: Into<O>,
    O: Add<O, Output = O> + Sub<O, Output = O> + Copy + Default,
{
    fn untake(&mut self, v: Self::Input) {
        self.v = self.v - v.into();
    }

    fn untake_accu(&mut self, ac: &Self) {
        self.v = self.v - ac.v
    }
}

impl<V, O> Default for SumAccumulator<V, O>
where
    V: Into<O>,
    O: Add<O, Output = O> + Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
