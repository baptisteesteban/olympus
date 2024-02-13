use crate::{io::imread, tests::get_image, Image2d};

#[test]
fn test_read_pgm_u8() {
    let ref_img =
        Image2d::<u8>::new_from_vec(4, 2, Vec::<u8>::from([4, 3, 9, 67, 43, 125, 253, 37]));

    let mut img = Image2d::<u8>::default();
    imread(&mut img, get_image("test.pgm").as_str()).unwrap();

    assert!(img == ref_img);
}

#[test]
fn test_read_tiff_u16() {
    let ref_img = Image2d::<u16>::new_from_vec(
        5,
        3,
        Vec::<u16>::from([
            0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800,
        ]),
    );
    let mut img: Image2d<u16> = Default::default();
    imread(&mut img, get_image("test_u16.tiff").as_str()).unwrap();

    assert!(img == ref_img);
}
