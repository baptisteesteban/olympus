use crate::Point2d;

#[test]
fn test_creation_and_attributes() {
    let p1 = Point2d::new(10, 3);
    assert_eq!(p1.x(), 10);
    assert_eq!(p1.y(), 3);

    let p2 = Point2d::default();
    assert_eq!(p2.x(), 0);
    assert_eq!(p2.y(), 0);
}

#[test]
fn test_display() {
    let p = Point2d::new(10, 78);
    let formatted = String::from(format!("{}", p).as_str());
    assert_eq!(formatted, "(10, 78)");
}
