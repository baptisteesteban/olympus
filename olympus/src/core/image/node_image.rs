use std::ops::{Add, Index, IndexMut, Sub};

use crate::{
    core::image::details::NDBuffer, fill, Domain, Image, ImageMut, NodeDomain, SizedDomain,
};

#[derive(Debug)]
pub struct NodeImage<T> {
    domain: NodeDomain,
    values: NDBuffer<T>,
}

impl<T> NodeImage<T> {
    /// Build a new `NodeImage` for a domain.
    ///
    /// # Safety
    ///
    /// The value array is uninitialized and may lead to undefined behavior if it is not initialized before
    pub unsafe fn new_uninitialized(domain: NodeDomain) -> NodeImage<T> {
        let size = domain.size();
        NodeImage {
            domain,
            values: NDBuffer::new_with_capacity(size),
        }
    }

    pub fn new(domain: NodeDomain, values: Vec<T>) -> Result<NodeImage<T>, String> {
        if domain.size() != values.len() {
            return Err(format!(
                "The number of values does not match the number of nodes (Got {}, expected {})",
                values.len(),
                domain.size()
            ));
        }
        let mut res = unsafe { NodeImage::<T>::new_uninitialized(domain) };
        for (i, v) in values.into_iter().enumerate() {
            res[i as i32] = v;
        }
        Ok(res)
    }
}

impl<T> Index<i32> for NodeImage<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        debug_assert!(self.domain.has(&index));
        unsafe { self.values.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<i32> for NodeImage<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        debug_assert!(self.domain.has(&index));
        unsafe { self.values.get_unchecked_mut(index as usize) }
    }
}

impl<T> Image for NodeImage<T> {
    type Domain = NodeDomain;
    type Value = T;

    fn domain(&self) -> &Self::Domain {
        &self.domain
    }

    fn at(&self, p: &<Self::Domain as crate::Domain>::Point) -> Option<&Self::Value> {
        if !self.domain.has(p) {
            None
        } else {
            Some(&self[*p])
        }
    }
}

impl<T> ImageMut for NodeImage<T> {
    type ChangeValue<V> = NodeImage<V>;

    fn at_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> Option<&mut Self::Value> {
        if !self.domain.has(p) {
            None
        } else {
            Some(&mut self[*p])
        }
    }

    unsafe fn imchvalue_uninitialized<V>(&self) -> Self::ChangeValue<V> {
        NodeImage::<V>::new_uninitialized(self.domain.clone())
    }

    fn imchvalue_with_value<V: Copy>(&self, v: V) -> Self::ChangeValue<V> {
        let mut res = unsafe { self.imchvalue_uninitialized::<V>() };
        fill(&mut res, v);
        res
    }

    fn imchvalue<V: Default + Copy>(&self) -> Self::ChangeValue<V> {
        self.imchvalue_with_value(V::default())
    }

    fn duplicate(&self) -> Self {
        unsafe { Self::new_uninitialized(self.domain.clone()) }
    }
}

impl<T: PartialEq> PartialEq for NodeImage<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.domain.size() != other.domain.size() {
            return false;
        }
        for p in self.domain.clone() {
            if self[p] != other[p] {
                return false;
            }
        }
        true
    }
}

impl<V> Add for NodeImage<V>
where
    V: Add<V, Output = V> + Copy,
{
    type Output = NodeImage<V>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.duplicate();
        for p in self.domain().clone() {
            res[p] = self[p] + rhs[p];
        }
        res
    }
}

impl<V> Sub for NodeImage<V>
where
    V: Sub<V, Output = V> + Copy,
{
    type Output = NodeImage<V>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self.duplicate();
        for p in self.domain().clone() {
            res[p] = self[p] - rhs[p];
        }
        res
    }
}
