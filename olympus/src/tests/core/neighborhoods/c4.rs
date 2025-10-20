use crate::{Point2d, Window, C4};

#[test]
fn test_c4() {
    let p = Point2d::new(10, 14);
    const REF: [Point2d; 4] = [
        Point2d::new(9, 14),
        Point2d::new(10, 13),
        Point2d::new(11, 14),
        Point2d::new(10, 15),
    ];
    for (n, n_ref) in std::iter::zip(C4.apply(&p), REF) {
        assert_eq!(n, n_ref);
    }
}
