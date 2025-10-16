use crate::Image2d;

#[test]
fn test_image2d() {
    let mut img = Image2d::new(3, 5).unwrap();
    assert_eq!(img.width(), 3);
    assert_eq!(img.height(), 5);

    // Generate iota
    for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(v) = img.get_mut(x, y) {
                *v = (y * 3 + x) as u8;
            }
        }
    }

    // Check value
    for y in 0..img.height() {
        for x in 0..img.width() {
            assert_eq!(*img.get(x, y).unwrap(), (y * 3 + x) as u8);
        }
    }

    // Check domain
    let d = img.domain();
    assert_eq!(d.width(), img.width());
    assert_eq!(d.height(), img.height());
}

#[test]
fn test_image2d_equality() {
    let mut img1 = Image2d::new(2, 3).unwrap();
    let mut img2 = Image2d::new(2, 3).unwrap();

    for y in 0..img1.height() {
        for x in 0..img1.width() {
            if let Some(v) = img1.get_mut(x, y) {
                *v = (y * 2 + x) as u8;
            }
            if let Some(v) = img2.get_mut(x, y) {
                *v = (y * 2 + x) as u8;
            }
        }
    }

    assert_eq!(img1, img2);
    if let Some(v) = img1.get_mut(0, 0) {
        *v = 5;
    }
    assert_ne!(img1, img2);
}

#[test]
fn test_image2d_add() {
    let img1 = Image2d::<u8>::from_vec(3, 3, vec![34, 6, 66, 40, 72, 26, 11, 27, 101]).unwrap();
    let img2 = Image2d::<u8>::from_vec(3, 3, vec![55, 5, 18, 87, 10, 89, 42, 2, 64]).unwrap();
    let ref_res =
        Image2d::<u8>::from_vec(3, 3, vec![89, 11, 84, 127, 82, 115, 53, 29, 165]).unwrap();
    let res = img1 + img2;
    assert_eq!(res, ref_res)
}

#[test]
fn test_image2d_sub() {
    let img1 =
        Image2d::<u8>::from_vec(3, 3, vec![242, 230, 175, 194, 254, 189, 235, 229, 252]).unwrap();
    let img2 = Image2d::<u8>::from_vec(3, 3, vec![124, 46, 28, 27, 90, 58, 92, 98, 24]).unwrap();
    let ref_res =
        Image2d::<u8>::from_vec(3, 3, vec![118, 184, 147, 167, 164, 131, 143, 131, 228]).unwrap();
    let res = img1 - img2;
    assert_eq!(res, ref_res);
}
