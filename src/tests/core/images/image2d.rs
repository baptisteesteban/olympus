use crate::{
    traits::{Image, ImageFromDomain},
    Image2d, Point2d,
};

#[test]
fn test_image_creation() {
    let img = Image2d::<u8>::new(10, 15);
    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 15);
}

#[test]
fn test_read_write() {
    let mut img = Image2d::<u8>::new(5, 8);
    let width = img.width();
    for y in 0..img.height() {
        for x in 0..img.width() {
            *img.at_mut(x, y) = (y * width + x) as u8;
        }
    }
    for y in 0..img.height() {
        for x in 0..img.width() {
            let val = (y * width + x) as u8;
            let p = Point2d::new(x, y);
            assert_eq!(*img.at(x, y), val);
            assert_eq!(*img.at_point(&p), val);
        }
    }
}

#[test]
fn test_domain() {
    let img = Image2d::<u8>::new(3, 7);
    let domain = img.domain();
    assert_eq!(domain.width(), 3);
    assert_eq!(domain.height(), 7);

    let img2 = Image2d::<u8>::new_from_domain(&domain);
    assert_eq!(img2.width(), 3);
    assert_eq!(img2.height(), 7);
}

#[test]
fn test_resize() {
    const REFVAL: [u8; 6] = [0, 0, 0, 0, 10, 10];
    let mut img = Image2d::<u8>::new(2, 2);
    img.resize_with(3, 2, 10);
    assert_eq!(img.width(), 3);
    assert_eq!(img.height(), 2);
    for p in img.domain() {
        assert_eq!(
            *img.at_point(&p),
            REFVAL[(p.y() * img.width() + p.x()) as usize]
        );
    }
}

#[test]
fn test_eq() {
    const REFVAL: [u8; 6] = [0, 0, 0, 0, 10, 10];
    let img1 = Image2d::<u8>::new_from_vec(3, 2, Vec::<u8>::from(REFVAL));
    let img2 = Image2d::<u8>::new_from_vec(3, 2, Vec::<u8>::from(REFVAL));
    let img3 = Image2d::<u8>::new_from_vec(2, 3, Vec::<u8>::from(REFVAL));

    assert!(img1 == img2);
    assert!(img1 != img3);
}
