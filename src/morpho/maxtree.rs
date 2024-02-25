use crate::{
    traits::{Image, Window},
    Box2d, Image2d, Point2d,
};

use super::Tree;

mod internal {
    use crate::{
        morpho::{CompressedUnionFind, Tree, UnionFind},
        traits::{Domain, Image, ImageFromDomain, MutableImage, UndefinedPoint, Window},
        Box2d, Image2d, Point2d,
    };

    pub(crate) fn compute_tree<C>(
        sorted_points: &Vec<Point2d>,
        domain: Box2d,
        nbh: C,
    ) -> Image2d<Point2d>
    where
        C: Window<Domain = Box2d>,
    {
        let mut uf: CompressedUnionFind<Image2d<Point2d>> =
            CompressedUnionFind::new(domain.clone());
        let mut parent = Image2d::new_from_domain(&domain);
        for p in sorted_points {
            uf.make_set(p);
            *parent.at_point_mut(p) = *p;
            for n in nbh.apply(p) {
                if domain.has(&n) && *uf.parent().at_point(&n) != Point2d::UNDEF {
                    let r = uf.find(&n);
                    if r != *p {
                        uf.union(p, &r);
                        *parent.at_point_mut(&r) = *p;
                    }
                }
            }
        }

        parent
    }

    pub(crate) fn canonize<T>(
        sorted_points: &Vec<Point2d>,
        parent: &mut Image2d<Point2d>,
        img: &Image2d<T>,
    ) where
        T: PartialEq,
    {
        for p in sorted_points.iter().rev() {
            let q = *parent.at_point(&p);
            if *img.at_point(&q) == *img.at_point(parent.at_point(&q)) {
                *parent.at_point_mut(&p) = *parent.at_point(&q);
            }
        }
    }

    pub(crate) fn to_tree_representation<T>(
        sorted_points: &Vec<Point2d>,
        parent: &Image2d<Point2d>,
        img: &Image2d<T>,
    ) -> Tree<Image2d<i32>, T>
    where
        T: PartialEq + Copy,
    {
        let mut nodemap = Image2d::<i32>::new_from_domain_with_value(&img.domain(), -1);
        let mut parent_vec = Vec::<i32>::with_capacity(sorted_points.len());
        let mut value_vec = Vec::<T>::with_capacity(sorted_points.len());

        let mut n = 0;
        for p in sorted_points.iter().rev() {
            if *parent.at_point(p) == *p || *img.at_point(p) != *img.at_point(parent.at_point(p)) {
                *nodemap.at_point_mut(p) = n;
                n += 1;
                parent_vec.push(*nodemap.at_point(parent.at_point(p)));
                value_vec.push(*img.at_point(p));
            } else {
                *nodemap.at_point_mut(p) = *nodemap.at_point(parent.at_point(p));
            }
        }

        parent_vec.shrink_to_fit();
        value_vec.shrink_to_fit();
        Tree::build(nodemap, parent_vec, value_vec)
    }
}

pub fn maxtree<T, C>(img: &Image2d<T>, nbh: C) -> Tree<Image2d<i32>, T>
where
    T: PartialOrd + Copy,
    C: Window<Domain = Box2d>,
{
    let mut points: Vec<Point2d> = img.domain().into_iter().collect();
    points.sort_by(|p, q| img.at_point(q).partial_cmp(img.at_point(p)).unwrap());
    let mut parent = internal::compute_tree(&points, img.domain(), nbh);
    internal::canonize(&points, &mut parent, img);
    internal::to_tree_representation(&points, &parent, img)
}

pub fn mintree<T, C>(img: &Image2d<T>, nbh: C) -> Tree<Image2d<i32>, T>
where
    T: PartialOrd + Copy,
    C: Window<Domain = Box2d>,
{
    let mut points: Vec<Point2d> = img.domain().into_iter().collect();
    points.sort_by(|p, q| img.at_point(p).partial_cmp(img.at_point(q)).unwrap());
    let mut parent = internal::compute_tree(&points, img.domain(), nbh);
    internal::canonize(&points, &mut parent, img);
    internal::to_tree_representation(&points, &parent, img)
}
