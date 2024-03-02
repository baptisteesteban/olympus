pub trait Accumulator: Default {
    type Input;
    type Output;

    fn new() -> Self;
    fn init(&mut self);
    fn take(&mut self, v: Self::Input);
    fn take_accu(&mut self, ac: &Self);
    fn result(&self) -> Self::Output;
}

pub trait UntakeAccumulator: Accumulator {
    fn untake(&mut self, v: Self::Input);
    fn untake_accu(&mut self, ac: &Self);
}
