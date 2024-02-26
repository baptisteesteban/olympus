use crate::{
    morpho::{CompressedUnionFind, UnionFind},
    Box2d, Image2d, Point2d,
};

#[test]
fn test_compressed_union_find() {
    let p1 = Point2d::new(0, 0);
    let p2 = Point2d::new(1, 0);
    let p3 = Point2d::new(1, 1);
    let mut uf = CompressedUnionFind::<Image2d<Point2d>>::new(Box2d::new_from_dimension(3, 3));
    uf.make_set(&p1);
    uf.make_set(&p2);
    uf.make_set(&p3);
    uf.union(&p1, &p2);
    uf.union(&p1, &p3);
    assert_eq!(uf.find(&p1), uf.find(&p2));
    assert_eq!(uf.find(&p1), uf.find(&p3));
}
