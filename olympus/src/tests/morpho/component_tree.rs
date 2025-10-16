use std::iter::zip;

use crate::{morpho::maxtree, Image2d, C4};

#[test]
fn test_depth() {
    const DEPTH_REF: [i32; 6] = [3, 2, 1, 2, 1, 0];
    let img = Image2d::<u8>::from_vec(5, 2, vec![3, 3, 1, 4, 2, 4, 1, 2, 3, 1]).unwrap();
    let t = maxtree(&img, &C4);
    let depth = t.compute_depth();

    assert_eq!(depth.len(), 6);
    for (v, v_ref) in zip(depth, DEPTH_REF) {
        assert_eq!(v, v_ref);
    }
}

#[test]
fn test_area() {
    const AREA_REF: [usize; 6] = [1, 1, 3, 2, 4, 10];
    let img = Image2d::<u8>::from_vec(5, 2, vec![3, 3, 1, 4, 2, 4, 1, 2, 3, 1]).unwrap();
    let t = maxtree(&img, &C4);
    let area = t.compute_area();

    assert_eq!(area.len(), 6);
    for (v, v_ref) in zip(area, AREA_REF) {
        assert_eq!(v, v_ref);
    }
}
