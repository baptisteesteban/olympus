use crate::{traits::Window, Point2d, C8};

#[test]
fn test_c8() {
    let ref_values = Vec::<Point2d>::from([
        Point2d::new(9, 12),
        Point2d::new(9, 13),
        Point2d::new(10, 13),
        Point2d::new(11, 13),
        Point2d::new(11, 12),
        Point2d::new(11, 11),
        Point2d::new(10, 11),
        Point2d::new(9, 11),
    ]);
    let p = Point2d::new(10, 12);
    let c8 = C8::new();

    let nbh = c8.apply(&p);
    assert_eq!(nbh.len(), ref_values.len());
    for i in 0..nbh.len() {
        assert_eq!(*nbh.get(i).unwrap(), *ref_values.get(i).unwrap());
    }
}
