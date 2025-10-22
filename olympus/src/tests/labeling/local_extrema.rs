use crate::{labeling::local_minima, Image2d, C4, C8};

#[test]
fn test_local_minima_image2d_c4() {
    let img = Image2d::<u8>::from_vec(4, 3, vec![7, 3, 1, 1, 4, 5, 7, 2, 1, 6, 3, 4]).unwrap();
    let ref_img = Image2d::<u16>::from_vec(4, 3, vec![0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 3, 0]).unwrap();
    let res = local_minima(&img, &C4);
    assert_eq!(ref_img, res);
}

#[test]
fn test_local_minima_image2d_c8() {
    let img = Image2d::<u8>::from_vec(4, 3, vec![7, 3, 1, 1, 4, 5, 7, 2, 1, 6, 3, 4]).unwrap();
    let ref_img = Image2d::<u16>::from_vec(4, 3, vec![0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 0, 0]).unwrap();
    let res = local_minima(&img, &C8);
    assert_eq!(ref_img, res);
}
