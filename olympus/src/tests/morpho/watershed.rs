//use crate::{morpho::watershed_partition, Image2d, C4};
/*
#[test]
fn watershed_partition_c4() {
    let img = Image2d::<u8>::from_vec(
        5,
        5,
        vec![
            0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 3, 3, 3, 2, 3, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0,
        ],
    )
    .unwrap();
    let ref_img = Image2d::<u16>::from_vec(
        5,
        5,
        vec![
            1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 4, 2, 3, 3, 3, 4, 4, 3, 3, 3, 4, 4,
        ],
    )
    .unwrap();
    let res = watershed_partition(&img, &C4);
    assert_eq!(res, ref_img);
}
*/
