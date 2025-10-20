use crate::{cond, where_image, Image2d, Rgb8, GREEN, RED};

#[test]
fn test_where_image() {
    let ref_img = Image2d::<Rgb8>::from_vec(2, 2, vec![GREEN, RED, RED, GREEN]).unwrap();
    let img = Image2d::<u8>::from_vec(2, 2, vec![3, 5, 2, 3]).unwrap();
    let res = where_image(&cond(&img, |_, v| *v == 3), GREEN, RED);
    assert_eq!(res, ref_img);
}
