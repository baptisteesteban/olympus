use crate::{HistogramHQueue, Image2d, Point2d};

#[test]
fn test_histogram_hqueue() {
    let img = Image2d::from_vec(3, 3, vec![1, 2, 5, 1, 7, 3, 3, 1, 2]).unwrap();

    let mut q = HistogramHQueue::new(&img);
    q.push(Point2d::new(2, 0), 5);
    q.push(Point2d::new(2, 2), 2);
    let (p1, v1) = q.pop();
    assert_eq!(p1, Point2d::new(2, 2));
    assert_eq!(v1, 2);
    let (p2, v2) = q.pop();
    assert_eq!(p2, Point2d::new(2, 0));
    assert_eq!(v2, 5);
    assert!(q.is_empty());
}
