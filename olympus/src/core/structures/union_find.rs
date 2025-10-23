use crate::{Domain, Image, Image2d, Point2d};

// TODO: Maybe using the Index and MutableIndex trait would be better than this. To investigate !!!

/// A trait that model a container accepted to be a Union-Find tree container.
pub trait UnionFindContainer {
    /// The type of the index for the Union-Find tree container. The type of the
    /// index *MUST* be the type of the container element.
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

/// Implementation of the Tarjan Union-Find with path compression.
pub struct UnionFind<Cont: UnionFindContainer> {
    /// Compressed parent array
    zpar: Cont,
}

impl<Cont: UnionFindContainer> UnionFind<Cont> {
    /// Returns a new Union-Find data structure
    pub fn new(zpar: Cont) -> UnionFind<Cont> {
        UnionFind { zpar }
    }

    /// Create a new set `n` in the Union-Find data structure.
    #[inline]
    pub fn make_set(&mut self, n: &Cont::Index) {
        debug_assert!(self.zpar.has(n));
        *self.zpar.at_mut(n) = *n;
    }

    /// Perform the union of two sets represented by their roots `a` and `b`.
    #[inline]
    pub fn union(&mut self, a: &Cont::Index, b: &Cont::Index) {
        debug_assert!(self.zpar.has(a) && self.zpar.has(b));
        *self.zpar.at_mut(b) = *a;
    }

    /// Find the root representing the set of a set `n`.
    pub fn find(&mut self, n: &Cont::Index) -> Cont::Index {
        debug_assert!(self.zpar.has(n));
        let mut r = *n;
        while *self.zpar.at(&r) != r {
            r = *self.zpar.at(&r);
        }

        let mut a = *n;
        while *self.zpar.at(&a) != a {
            let tmp = *self.zpar.at(&a);
            *self.zpar.at_mut(&a) = r;
            a = tmp;
        }
        r
    }
}
