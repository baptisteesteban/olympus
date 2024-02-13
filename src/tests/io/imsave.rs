use crate::{
    io::{imread, imsave},
    traits::{Image, MutableImage},
    Image2d,
};

#[test]
fn test_read_save_pgm_u8() {
    const WIDTH: i32 = 4;
    const HEIGHT: i32 = 2;
    const REFVAL: [u8; 8] = [4, 3, 9, 67, 43, 125, 253, 37];
    let mut in_img = Image2d::<u8>::new(WIDTH, HEIGHT);
    for p in in_img.domain() {
        *in_img.at_point_mut(&p) = REFVAL[(p.y() * in_img.width() + p.x()) as usize];
    }
    let path = "/tmp/test_img.pgm";
    imsave(&in_img, path).unwrap();
    let mut out_img = Image2d::<u8>::default();
    imread(&mut out_img, path).unwrap();

    assert_eq!(in_img.width(), out_img.width());
    assert_eq!(in_img.height(), out_img.height());
    for p in in_img.domain() {
        assert_eq!(*in_img.at_point(&p), *out_img.at_point(&p));
    }
    std::fs::remove_file(path).unwrap();
}
