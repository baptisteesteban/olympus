use crate::{Box2d, Domain, Point2d, ShapeDomain, SizedDomain};

#[test]
fn test_point2d() {
    let p = Point2d::new(3, 7);
    assert_eq!(p.x, 3);
    assert_eq!(p.y, 7);

    let p2 = Point2d::default();
    assert_eq!(p2.x, 0);
    assert_eq!(p2.y, 0);
}

#[test]
fn test_box2d() {
    let d = Box2d::new(4, 8);
    assert_eq!(d.width(), 4);
    assert_eq!(d.height(), 8);

    let d2 = Box2d::default();
    assert_eq!(d2.width(), 0);
    assert_eq!(d2.height(), 0);

    let p = Point2d::default();
    assert!(d.has(&p));
    assert!(!d2.has(&p));
}

#[test]
fn test_box2d_iterator() {
    let d = Box2d::new(3, 2);
    let ref_points = [
        Point2d::new(0, 0),
        Point2d::new(1, 0),
        Point2d::new(2, 0),
        Point2d::new(0, 1),
        Point2d::new(1, 1),
        Point2d::new(2, 1),
    ];

    for (i, p) in d.into_iter().enumerate() {
        assert_eq!(p, ref_points[i]);
    }
}

#[test]
fn test_domain_traits() {
    let domain = Box2d::new(3, 7);
    assert_eq!(domain.shape(0).unwrap(), 3);
    assert_eq!(domain.shape(1).unwrap(), 7);
    assert!(domain.shape(2).is_none());
    assert_eq!(domain.size(), 21);
    assert!(domain.has(&Point2d::new(0, 0)));
    assert!(!domain.has(&Point2d::new(2, 7)));
}
