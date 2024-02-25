use crate::{morpho, Image2d, C4};

#[test]
fn test_maxtree_image2d_c4() {
    const N: usize = 6;
    const PARENT_REF: [i32; N] = [0, 0, 1, 0, 3, 2];
    const VALUE_REF: [u8; N] = [1, 2, 3, 3, 4, 4];

    let img = Image2d::new_from_vec(5, 2, Vec::<u8>::from([3, 3, 1, 4, 2, 4, 1, 2, 3, 1]));
    let t = morpho::maxtree(&img, C4::new());

    assert_eq!(t.num_nodes(), N);
    for i in 0..N {
        assert_eq!(*t.parent(i as i32), PARENT_REF[i]);
        assert_eq!(*t.value(i as i32), VALUE_REF[i]);
    }
}
