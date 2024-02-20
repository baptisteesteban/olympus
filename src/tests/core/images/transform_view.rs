use crate::{traits::Image, Image2d, TransformView};

#[test]
fn test_transform_view_image2d() {
    let ref_img = Image2d::new_from_vec(3, 3, Vec::<u8>::from([2, 4, 6, 8, 10, 12, 14, 16, 18]));

    let img = Image2d::new_from_vec(3, 3, Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
    let tested = TransformView::new(&img, |v| 2 * v);

    assert!(tested.domain() == ref_img.domain());

    for p in tested.domain() {
        assert_eq!(*tested.at_point(&p), *ref_img.at_point(&p));
    }
}
