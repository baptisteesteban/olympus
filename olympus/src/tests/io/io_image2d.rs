use crate::io::{imread, imsave};
use crate::{Image2d, Rgb8};

#[test]
fn test_read_write_image2d_u8() {
    const PATH: &str = "test_img_io.png";

    let mut img = unsafe { Image2d::new_uninitialized(3, 5).unwrap() };
    assert_eq!(img.width(), 3);
    assert_eq!(img.height(), 5);

    // Generate iota
    for y in 0..img.height() {
        for x in 0..img.width() {
            img[(x, y)] = (y * 3 + x) as u8;
        }
    }

    let mut img2 = Image2d::<u8>::default();
    imsave(PATH, &img).unwrap();
    imread(PATH, &mut img2).unwrap();
    assert_eq!(img2.width(), img.width());
    assert_eq!(img2.height(), img.height());

    // Check value
    for y in 0..img2.height() {
        for x in 0..img2.width() {
            assert_eq!(*img2.get(x, y).unwrap(), *img.get(x, y).unwrap());
        }
    }
    std::fs::remove_file(PATH).unwrap();
}

#[test]
fn test_read_write_image2d_rgb8() {
    const PATH: &str = "test_img_rgb_io.png";

    let mut img = unsafe { Image2d::<Rgb8>::new_uninitialized(3, 5).unwrap() };
    assert_eq!(img.width(), 3);
    assert_eq!(img.height(), 5);

    // Generate iota
    for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(v) = img.get_mut(x, y) {
                v.r = (3 * (y * 3 + x)) as u8;
                v.g = (3 * (y * 3 + x) + 1) as u8;
                v.b = (3 * (y * 3 + x) + 2) as u8;
            }
        }
    }

    let mut img2 = Image2d::<Rgb8>::default();
    imsave(PATH, &img).unwrap();
    imread(PATH, &mut img2).unwrap();
    assert_eq!(img2.width(), img.width());
    assert_eq!(img2.height(), img.height());

    // Check value
    for y in 0..img2.height() {
        for x in 0..img2.width() {
            assert_eq!(img2.get(x, y).unwrap().r, img.get(x, y).unwrap().r);
            assert_eq!(img2.get(x, y).unwrap().g, img.get(x, y).unwrap().g);
            assert_eq!(img2.get(x, y).unwrap().b, img.get(x, y).unwrap().b);
        }
    }
    std::fs::remove_file(PATH).unwrap();
}
