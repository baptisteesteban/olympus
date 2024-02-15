use crate::{algorithms::clone, Image2d};

#[test]
fn test_clone_img2d() {
    let img = Image2d::<u8>::new_from_vec(
        5,
        5,
        Vec::<u8>::from([
            1, 5, 7, 0, 3, 5, 1, 9, 0, 3, 1, 5, 32, 98, 5, 3, 5, 9, 3, 5, 1, 4, 7, 9, 5,
        ]),
    );
    let out: Image2d<u8> = clone(&img);
    assert!(img == out);
}
