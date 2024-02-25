use crate::{
    traits::{Domain, Image, ImageFromDomain, MutableImage},
    Image2d,
};

pub struct Tree<I, V>
where
    I: Image<Value = i32>,
{
    nodemap: I,
    parent: Vec<i32>,
    value: Vec<V>,
}

impl<I, V> Tree<I, V>
where
    I: Image<Value = i32>,
{
    pub fn build(nodemap: I, parent: Vec<i32>, value: Vec<V>) -> Tree<I, V> {
        Tree {
            nodemap: nodemap,
            parent: parent,
            value: value,
        }
    }

    pub fn build_with_values<U>(t: Tree<I, V>, v: Vec<U>) -> Tree<I, U> {
        Tree {
            nodemap: t.nodemap,
            parent: t.parent,
            value: v,
        }
    }

    pub fn parent(&self, n: i32) -> &i32 {
        self.parent.get(n as usize).unwrap()
    }

    pub fn value(&self, n: i32) -> &V {
        self.value.get(n as usize).unwrap()
    }

    pub fn node_at_point(&self, p: <<I as Image>::Domain as Domain>::Point) -> i32 {
        *self.nodemap.at_point(&p)
    }

    pub fn num_nodes(&self) -> usize {
        self.parent.len()
    }

    pub fn values(&self) -> &Vec<V> {
        &self.value
    }
}

impl<V> Tree<Image2d<i32>, V>
where
    V: Default + Copy,
{
    pub fn reconstruct(&self) -> Image2d<V> {
        let mut res = Image2d::new_from_domain(&self.nodemap.domain());

        for p in res.domain() {
            *res.at_point_mut(&p) = *self.value.get(*self.nodemap.at_point(&p) as usize).unwrap();
        }

        res
    }
}
