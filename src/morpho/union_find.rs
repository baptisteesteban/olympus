use crate::{
    traits::{Domain, Image, ImageFromDomain, MutableImage, UndefinedPoint},
    Box2d,
};

pub trait UnionFind {
    type I: Image;

    fn make_set(&mut self, v: &<<Self::I as Image>::Domain as Domain>::Point);
    fn union(
        &mut self,
        u: &<<Self::I as Image>::Domain as Domain>::Point,
        v: &<<Self::I as Image>::Domain as Domain>::Point,
    );
    fn find(
        &mut self,
        v: &<<Self::I as Image>::Domain as Domain>::Point,
    ) -> <<Self::I as Image>::Domain as Domain>::Point;
}

pub struct CompressedUnionFind<I>
where
    I: Image<Domain = Box2d, Value = <Box2d as Domain>::Point> + ImageFromDomain,
{
    parent: I,
}

impl<I> CompressedUnionFind<I>
where
    I: Image<Domain = Box2d, Value = <Box2d as Domain>::Point> + MutableImage + ImageFromDomain,
{
    pub fn new(domain: I::Domain) -> CompressedUnionFind<I> {
        CompressedUnionFind {
            parent: I::new_from_domain_with_value(
                &domain,
                <<Box2d as Domain>::Point as UndefinedPoint>::UNDEF,
            ),
        }
    }
}

impl<I> UnionFind for CompressedUnionFind<I>
where
    I: Image<Domain = Box2d, Value = <Box2d as Domain>::Point> + MutableImage + ImageFromDomain,
{
    type I = I;

    fn make_set(&mut self, v: &<<Self::I as Image>::Domain as Domain>::Point) {
        *self.parent.at_point_mut(&v) = *v;
    }

    fn union(
        &mut self,
        u: &<<Self::I as Image>::Domain as Domain>::Point,
        v: &<<Self::I as Image>::Domain as Domain>::Point,
    ) {
        *self.parent.at_point_mut(v) = *u;
    }

    fn find(
        &mut self,
        v: &<<Self::I as Image>::Domain as Domain>::Point,
    ) -> <<Self::I as Image>::Domain as Domain>::Point {
        let mut r = *v;
        let mut q = *v;
        while *self.parent.at_point(&r) != r {
            r = *self.parent.at_point_mut(&r);
        }
        while *self.parent.at_point(&q) != q {
            let tmp = q;
            q = *self.parent.at_point(&q);
            *self.parent.at_point_mut(&tmp) = r;
        }
        r
    }
}
