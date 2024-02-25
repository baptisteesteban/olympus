use crate::accu::{Accumulator, SupAccumulator};

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
    acc2.take_accu(&acc);
    assert_eq!(acc2.result(), 63);
    acc2.init();
    assert_eq!(acc2.result(), u8::MIN);
}
