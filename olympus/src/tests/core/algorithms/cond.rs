use crate::{cond, Image2d};

#[test]
fn test_cond_image2d() {
    let ref_img = Image2d::<bool>::from_vec(2, 2, vec![true, false, false, true]).unwrap();
    let img = Image2d::<u8>::from_vec(2, 2, vec![3, 5, 2, 3]).unwrap();
    let res = cond(&img, |_, v| *v == 3);
    assert_eq!(res, ref_img);
}
