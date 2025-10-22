
use crate::{HQueue, Point2d};

#[test]
fn test_hqueue() {
    const REF_POINTS: [Point2d; 4] = [
        Point2d::new(0, 3),
        Point2d::new(0, 0),
        Point2d::new(1, 0),
        Point2d::new(0, 2),
    ];
    const REF_VALUES: [u8; 4] = [14, 10, 6, 3];

    let mut q = HQueue::default();
    q.push(Point2d::new(0, 0), 10);
    q.push(Point2d::new(1, 0), 6);
    q.push(Point2d::new(0, 2), 3);
    q.push(Point2d::new(0, 3), 14);

    let mut i = 0;
    while !q.is_empty() {
        let (p, v) = q.pop();
        assert_eq!(p, REF_POINTS[i]);
        assert_eq!(v, REF_VALUES[i]);
        i += 1;
    }
}
