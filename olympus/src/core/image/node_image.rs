use std::ops::{Index, IndexMut};

use crate::{Domain, Image, ImageMut, NodeDomain, SizedDomain};

pub struct NodeImage<T> {
    domain: NodeDomain,
    values: Vec<T>,
}

impl<T> NodeImage<T> {
    pub fn new(domain: NodeDomain, values: Vec<T>) -> Result<NodeImage<T>, String> {
        if domain.size() != values.len() {
            return Err(format!(
                "The number of values does not match the number of nodes (Got {}, expected {})",
                values.len(),
                domain.size()
            ));
        }
        Ok(NodeImage { domain, values })
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
    fn at_mut(&mut self, p: &<Self::Domain as Domain>::Point) -> Option<&mut Self::Value> {
        if !self.domain.has(p) {
            None
        } else {
            Some(&mut self[*p])
        }
    }
}
