use super::{Accumulator, UntakeAccumulator};

#[derive(Clone)]
pub struct AccuNAccumulator<const N: usize, A>
where
    A: Accumulator,
{
    v: A,
}

impl<const N: usize, A> AccuNAccumulator<N, A>
where
    A: Accumulator,
{
    pub fn extract_acc(&self) -> &A {
        &self.v
    }
}

impl<const N: usize, A> Accumulator for AccuNAccumulator<N, A>
where
    A: Accumulator,
    A::Input: Copy,
{
    type Input = A::Input;
    type Output = A::Output;

    fn new() -> Self {
        AccuNAccumulator { v: A::new() }
    }

    fn init(&mut self) {
        self.v.init();
    }

    fn take(&mut self, v: Self::Input) {
        for _i in 0..N {
            self.v.take(v);
        }
    }

    fn take_accu(&mut self, ac: &Self) {
        for _i in 0..N {
            self.v.take_accu(&ac.v);
        }
    }

    fn result(&self) -> Self::Output {
        self.v.result()
    }
}

impl<const N: usize, A> UntakeAccumulator for AccuNAccumulator<N, A>
where
    A: Accumulator + UntakeAccumulator,
    A::Input: Copy,
{
    fn untake(&mut self, v: Self::Input) {
        for _i in 0..N {
            self.v.untake(v);
        }
    }

    fn untake_accu(&mut self, ac: &Self) {
        for _i in 0..N {
            self.v.untake_accu(&ac.v);
        }
    }
}

impl<const N: usize, A> Default for AccuNAccumulator<N, A>
where
    A: Accumulator,
    A::Input: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
