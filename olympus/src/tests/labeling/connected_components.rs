use crate::{labeling::connected_components, Image2d, C4, C8};

#[test]
fn test_connected_components_image2d_c4() {
    let img = Image2d::<u8>::from_vec(4, 3, vec![7, 3, 1, 1, 4, 1, 7, 2, 1, 6, 3, 4]).unwrap();
    let ref_img =
        Image2d::<u16>::from_vec(4, 3, vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11]).unwrap();
    let res = connected_components(&img, &C4);
    assert_eq!(ref_img, res);
}

#[test]
fn test_connected_components_image2d_c8() {
    let img = Image2d::<u8>::from_vec(4, 3, vec![7, 3, 1, 1, 4, 1, 7, 2, 1, 6, 3, 4]).unwrap();
    let ref_img = Image2d::<u16>::from_vec(4, 3, vec![1, 2, 3, 3, 4, 3, 5, 6, 3, 7, 8, 9]).unwrap();
    let res = connected_components(&img, &C8);
    assert_eq!(ref_img, res);
}
