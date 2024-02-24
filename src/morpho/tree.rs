use crate::traits::{Domain, Image};

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

    pub fn parent(&self, n: i32) -> &i32 {
        self.parent.get(n as usize).unwrap()
    }

    pub fn value(&self, n: i32) -> &V {
        self.value.get(n as usize).unwrap()
    }

    pub fn node_at_point(&self, p: <<I as Image>::Domain as Domain>::Point) -> i32 {
        *self.nodemap.at_point(&p)
    }
}
