use crate::{Image2d, Point2d};

pub trait UnionFindContainer {
    type Index: Copy + PartialEq;

    fn at(&self, index: &Self::Index) -> &Self::Index;
    fn at_mut(&mut self, index: &Self::Index) -> &mut Self::Index;
    fn has(&self, index: &Self::Index) -> bool;
}

impl UnionFindContainer for Vec<usize> {
    type Index = usize;

    #[inline]
    fn at(&self, index: &Self::Index) -> &Self::Index {
        debug_assert!(self.has(index));
        &self[*index]
    }

    #[inline]
    fn at_mut(&mut self, index: &Self::Index) -> &mut Self::Index {
        debug_assert!(self.has(index));
        &mut self[*index]
    }

    #[inline]
    fn has(&self, index: &Self::Index) -> bool {
        *index < self.len()
    }
}

impl UnionFindContainer for Image2d<Point2d> {
    type Index = Point2d;

    #[inline]
    fn at(&self, index: &Self::Index) -> &Self::Index {
        debug_assert!(self.has(index));
        &self[*index]
    }

    #[inline]
    fn at_mut(&mut self, index: &Self::Index) -> &mut Self::Index {
        debug_assert!(self.has(index));
        &mut self[*index]
    }

    #[inline]
    fn has(&self, index: &Self::Index) -> bool {
        self.domain().has(index)
    }
}

pub struct UnionFind<Cont: UnionFindContainer> {
    zpar: Cont,
}

impl<Cont: UnionFindContainer> UnionFind<Cont> {
    pub fn new(zpar: Cont) -> UnionFind<Cont> {
        UnionFind { zpar }
    }

    #[inline]
    pub fn make_set(&mut self, n: &Cont::Index) {
        debug_assert!(self.zpar.has(n));
        *self.zpar.at_mut(n) = *n;
    }

    #[inline]
    pub fn union(&mut self, a: &Cont::Index, b: &Cont::Index) {
        debug_assert!(self.zpar.has(a) && self.zpar.has(b));
        *self.zpar.at_mut(b) = *a;
    }

    pub fn find(&mut self, n: &Cont::Index) -> Cont::Index {
        debug_assert!(self.zpar.has(n));
        let mut r = *n;
        while *self.zpar.at(&r) != r {
            r = *self.zpar.at(&r);
        }

        let mut a = *n;
        while *self.zpar.at(&r) != r {
            let tmp = *self.zpar.at(&a);
            *self.zpar.at_mut(&a) = r;
            a = tmp;
        }
        r
    }
}
