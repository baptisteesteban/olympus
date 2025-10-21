use crate::{Box2d, ConstantImage, Image, Point2d};

#[test]
fn test_constant_image2d() {
    let img = ConstantImage::new(Box2d::new(3, 5), 8);
    assert_eq!(img.at(&Point2d::new(3, 5)), None);
    assert_eq!(img.at(&Point2d::new(2, 4)), Some(&8));
    assert_eq!(img[Point2d::new(2, 4)], 8);
    assert_eq!(img.domain().width(), 3);
    assert_eq!(img.domain().height(), 5);
}
