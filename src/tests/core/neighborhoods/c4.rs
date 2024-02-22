use crate::{traits::Window, Point2d, C4};

#[test]
fn test_c4() {
    let ref_values = Vec::<Point2d>::from([
        Point2d::new(9, 12),
        Point2d::new(10, 13),
        Point2d::new(11, 12),
        Point2d::new(10, 11),
    ]);
    let p = Point2d::new(10, 12);
    let c4 = C4::new();

    let nbh = c4.apply(&p);
    assert_eq!(nbh.len(), ref_values.len());
    for i in 0..nbh.len() {
        assert_eq!(*nbh.get(i).unwrap(), *ref_values.get(i).unwrap());
    }
}
