use crate::accu::{Accumulator, InfAccumulator};

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
