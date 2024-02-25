use crate::{morpho::HierarchicalQueue, Point2d};

#[test]
fn test_hqueue() {
    let mut q = HierarchicalQueue::new();
    q.push(7, Point2d::new(0, 0));
    assert_eq!(q.cur(), 7);
    q.push(4, Point2d::new(1, 0));
    assert_eq!(q.cur(), 4);
    q.push(8, Point2d::new(2, 0));
    assert_eq!(q.cur(), 4);
    let (v1, p1) = q.pop().unwrap();
    assert_eq!(v1, 4);
    assert_eq!(p1, Point2d::new(1, 0));
    assert_eq!(q.cur(), 7);

    q.push(2, Point2d::new(0, 1));
    q.push(9, Point2d::new(1, 1));
    q.push(3, Point2d::new(2, 1));
    assert_eq!(q.cur(), 2);
    assert_eq!(q.size(), 5);

    let (v2, p2) = q.pop_nearest(4).unwrap();
    assert_eq!(v2, 3);
    assert_eq!(p2, Point2d::new(2, 1));

    for _ in 0..4 {
        let (_, _) = q.pop().unwrap();
    }
    assert!(q.empty());
}
