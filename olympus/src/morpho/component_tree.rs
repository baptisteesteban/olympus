use crate::{Image2d, Point2d};

pub struct ComponentTree<V> {
    pub parents: Vec<usize>,
    pub values: Vec<V>,
    pub nodemap: Image2d<usize>,
}

impl<V> ComponentTree<V> {
    pub fn new(parents: Vec<usize>, values: Vec<V>, nodemap: Image2d<usize>) -> ComponentTree<V> {
        ComponentTree {
            parents,
            values,
            nodemap,
        }
    }

    pub fn parent(&self, n: usize) -> Option<&usize> {
        self.parents.get(n)
    }

    pub fn value(&self, n: usize) -> Option<&V> {
        self.values.get(n)
    }

    pub fn node_at(&self, p: &Point2d) -> Option<&usize> {
        self.nodemap.at(p)
    }

    pub fn root(&self) -> usize {
        self.parents.len() - 1
    }

    pub fn num_nodes(&self) -> usize {
        self.parents.len()
    }

    pub fn compute_depth(&self) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; self.num_nodes()];

        for n in (0..res.len() - 1).rev() {
            res[n] = 1 + res[self.parents[n]];
        }

        res
    }

    pub fn compute_area(&self) -> Vec<usize> {
        let mut res: Vec<usize> = vec![0; self.num_nodes()];

        for p in *self.nodemap.domain() {
            res[self.nodemap[p]] += 1;
        }

        for n in 0..self.num_nodes() - 1 {
            res[self.parents[n]] += res[n];
        }

        res
    }
}

pub fn direct_filter<V, F>(t: &ComponentTree<V>, predicate: F) -> ComponentTree<V>
where
    V: Copy + Default + Eq,
    F: Fn(usize) -> bool,
{
    let mut to_keep = vec![false; t.parents.len()];
    to_keep[t.parents.len() - 1] = true;
    let mut count: i32 = 1;
    for (n, v) in to_keep.iter_mut().enumerate().take(t.num_nodes() - 1) {
        if predicate(n) {
            count += 1;
            *v = true;
        }
    }

    let mut parent_vec: Vec<usize> = vec![0; count as usize];
    let mut value_vec = vec![Default::default(); count as usize];
    let mut nodemap = Image2d::<usize>::new(t.nodemap.width(), t.nodemap.height()).unwrap();
    count -= 1;

    let mut mapping: Vec<usize> = vec![0; t.parents.len()];
    mapping[t.parents.len() - 1] = count as usize;
    parent_vec[count as usize] = count as usize;
    value_vec[count as usize] = t.values[t.parents.len() - 1];
    count -= 1;

    for n in (0..t.parents.len() - 1).rev() {
        if to_keep[n] {
            mapping[n] = count as usize;
            parent_vec[count as usize] = mapping[t.parents[n]];
            value_vec[count as usize] = t.values[n];

            count -= 1;
        } else {
            mapping[n] = mapping[t.parents[n]];
        }
    }

    for p in *nodemap.domain() {
        nodemap[p] = mapping[t.nodemap[p]];
    }

    ComponentTree::<V>::new(parent_vec, value_vec, nodemap)
}

pub fn reconstruct_from_values<V, A>(t: &ComponentTree<V>, values: &[A]) -> Image2d<A>
where
    A: Default + Copy,
{
    let mut res = Image2d::<A>::new(t.nodemap.width(), t.nodemap.height()).unwrap();

    for p in *res.domain() {
        res[p] = values[t.nodemap[p]];
    }

    res
}

pub fn reconstruct<V>(t: &ComponentTree<V>) -> Image2d<V>
where
    V: Default + Copy,
{
    reconstruct_from_values(t, &t.values)
}
