use std::iter::zip;

use crate::{morpho::maxtree, Image2d, C4};

#[test]
fn test_maxtree_c4() {
    const VALUE_REF: [u8; 6] = [4, 4, 3, 3, 2, 1];
    const PARENT_REF: [usize; 6] = [3, 2, 5, 4, 5, 5];
    const NODEMAP_REF: [usize; 10] = [2, 2, 5, 0, 4, 1, 5, 4, 3, 5];

    let img = Image2d::<u8>::from_vec(5, 2, vec![3, 3, 1, 4, 2, 4, 1, 2, 3, 1]).unwrap();
    let t = maxtree(&img, &C4);

    assert_eq!(t.root(), 5);

    assert_eq!(t.parents.len(), 6);
    for (v, v_ref) in zip(t.parents, PARENT_REF) {
        assert_eq!(v, v_ref);
    }

    assert_eq!(t.values.len(), 6);
    for (v, v_ref) in zip(t.values, VALUE_REF) {
        assert_eq!(v, v_ref);
    }

    assert_eq!(t.nodemap.width(), 5);
    assert_eq!(t.nodemap.height(), 2);
    for (v, v_ref) in zip(t.nodemap.values(), NODEMAP_REF) {
        assert_eq!(v, v_ref);
    }
}
