use crate::{
    traits::{ChangeValueImage, Image, ImageFromDomain},
    Box2d, ConstantImage,
};

#[test]
fn test_constant_image_creation_and_access() {
    let img = ConstantImage::<u8, Box2d>::new_from_domain_with_value(&Box2d::new(3, 6), 27);

    let domain = img.domain();
    assert_eq!(domain.width(), 3);
    assert_eq!(domain.height(), 6);

    for p in img.domain() {
        assert_eq!(*img.at_point(&p), 27);
    }
}

#[test]
fn test_constant_image_change_value() {
    let ref_img: ConstantImage<i64, Box2d> =
        ConstantImage::new_from_domain_with_value(&Box2d::new(2, 2), 7);
    let img = ConstantImage::new_from_domain_with_value(&Box2d::new(2, 2), 7);
    let img_changed_value =
        <ConstantImage<i32, Box2d> as ChangeValueImage<i64>>::change_value(&img);
    assert!(img_changed_value == ref_img);
}
