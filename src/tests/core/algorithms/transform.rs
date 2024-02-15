use crate::{algorithms::transform, traits::Image, Image2d};

#[test]
fn test_transform_img2d() {
    const VAL: [u8; 9] = [1, 4, 9, 3, 5, 7, 9, 2, 3];
    let img = Image2d::<u8>::new_from_vec(3, 3, Vec::<u8>::from(VAL));
    let out: Image2d<u16> = transform(&img, |v| 2 * *v as u16);

    for p in out.domain() {
        assert_eq!(
            *out.at_point(&p),
            2 * VAL[(p.y() * img.width() + p.x()) as usize] as u16
        )
    }
}
