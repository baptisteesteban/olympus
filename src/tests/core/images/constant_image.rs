use crate::{
    traits::{Image, ImageFromDomain},
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
