use crate::accu::{Accumulator, SumAccumulator, UntakeAccumulator};

#[test]
fn test_sum_accu() {
    let mut acc1 = SumAccumulator::<u8, u16>::new();
    acc1.take(7);
    acc1.take(12);
    assert_eq!(acc1.result(), 19);

    let mut acc2 = SumAccumulator::<u8, u16>::new();
    acc2.take(5);
    acc1.take_accu(&acc2);
    assert_eq!(acc1.result(), 24);

    let mut acc3 = SumAccumulator::<u8, u16>::new();
    acc3.take(12);
    acc1.untake_accu(&acc3);
    assert_eq!(acc1.result(), 12);

    acc3.init();
    assert_eq!(acc3.result(), 0);
}
