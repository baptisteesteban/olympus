use crate::{Point2d, StructuringElement2d};

#[test]
fn test_simple_se() {
    #[allow(non_snake_case)]
    let REF: [Point2d; 9] = [
        Point2d::new(-1, -1),
        Point2d::new(0, -1),
        Point2d::new(1, -1),
        Point2d::new(-1, 0),
        Point2d::new(0, 0),
        Point2d::new(1, 0),
        Point2d::new(-1, 1),
        Point2d::new(0, 1),
        Point2d::new(1, 1),
    ];

    let se = StructuringElement2d::new(3, 3, vec![true; 9]).unwrap();
    let mut i = 0;
    for v in se.apply(&Point2d::new(0, 0)) {
        assert_eq!(v, REF[i]);
        i += 1;
    }
}
