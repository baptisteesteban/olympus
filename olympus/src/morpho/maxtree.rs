use std::cmp::Ordering;

use crate::morpho::{direct_filter, ComponentTree};
use crate::{Domain, Image2d, Point2d, UnionFind, Window};

fn sort_points<V, O>(img: &Image2d<V>, ord: O) -> Vec<Point2d>
where
    V: Ord + Clone,
    O: Fn(&V, &V) -> Ordering,
{
    let mut sorted_tuples =
        std::iter::zip(*img.domain(), img.values()).collect::<Vec<(Point2d, &V)>>();
    sorted_tuples.sort_by(|(_, v1), (_, v2)| ord(v1, v2));
    sorted_tuples
        .iter()
        .map(|(p, _)| *p)
        .collect::<Vec<Point2d>>()
}

fn union_find<V, C>(sorted: &[Point2d], img: &Image2d<V>, nbh: &C) -> ComponentTree<V>
where
    V: Copy,
    C: Window,
{
    let mut deja_vu = Image2d::<bool>::new_with_value(img.width(), img.height(), false).unwrap();
    let mut nodemap =
        unsafe { Image2d::<usize>::new_uninitialized(img.width(), img.height()).unwrap() };
    let mut parent = Vec::<usize>::with_capacity((img.width() * img.height()) as usize);
    let mut value = Vec::<V>::with_capacity((img.width() * img.height()) as usize);

    let domain = img.domain();

    let mut uf = UnionFind::new(unsafe {
        Image2d::<Point2d>::new_uninitialized(img.width(), img.height()).unwrap()
    });
    for (lbl, p) in sorted.iter().enumerate() {
        uf.make_set(p);
        *deja_vu.at_mut(p).unwrap() = true;
        parent.push(lbl);
        value.push(*img.at(p).unwrap());
        *nodemap.at_mut(p).unwrap() = lbl;

        for n in nbh.apply(p) {
            if domain.has(&n) && *deja_vu.at(&n).unwrap() {
                let r = uf.find(&n);
                if *p != r {
                    uf.union(p, &r);
                    *parent.get_mut(*nodemap.at(&r).unwrap()).unwrap() = *nodemap.at(p).unwrap();
                }
            }
        }
    }

    ComponentTree::new(parent, value, nodemap)
}

fn canonicalize<V>(t: &ComponentTree<V>) -> ComponentTree<V>
where
    V: Eq + Copy + Default,
{
    direct_filter(t, |n| {
        *t.value(n).unwrap() != *t.value(*t.parent(n).unwrap()).unwrap()
    })
}

pub fn maxtree<V, C>(img: &Image2d<V>, nbh: &C) -> ComponentTree<V>
where
    V: Ord + Copy + Default,
    C: Window,
{
    let sorted = sort_points(img, |v1, v2| v2.cmp(v1));
    let t1 = union_find(&sorted, img, nbh);
    canonicalize(&t1)
}

pub fn mintree<V, C>(img: &Image2d<V>, nbh: &C) -> ComponentTree<V>
where
    V: Ord + Copy + Default,
    C: Window,
{
    let sorted = sort_points(img, |v1, v2| v1.cmp(v2));
    let t1 = union_find(&sorted, img, nbh);
    canonicalize(&t1)
}
