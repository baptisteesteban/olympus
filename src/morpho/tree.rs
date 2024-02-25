use crate::{
    accu::Accumulator,
    traits::{Domain, Image, ImageFromDomain, MutableImage, SizedDomain},
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

    pub fn node_at_point(&self, p: &<<I as Image>::Domain as Domain>::Point) -> i32 {
        *self.nodemap.at_point(p)
    }

    pub fn num_nodes(&self) -> usize {
        self.parent.len()
    }

    pub fn values(&self) -> &Vec<V> {
        &self.value
    }

    pub fn accumulate_on_points<A, T>(&self, _acc: A) -> Vec<T>
    where
        I::Domain: SizedDomain,
        A: Accumulator<Input = <I::Domain as Domain>::Point, Output = T> + Default + Clone,
    {
        let mut attr: Vec<A> = vec![Default::default(); self.num_nodes()];
        for p in self.nodemap.domain() {
            attr.get_mut(self.node_at_point(&p) as usize)
                .unwrap()
                .take(p);
        }
        for n in (self.num_nodes() - 1)..0 {
            let acc_cur = attr.get(n).unwrap().clone();
            let acc_parent = attr.get_mut(*self.parent.get(n).unwrap() as usize).unwrap();
            acc_parent.take_accu(&acc_cur)
        }
        attr.into_iter().map(|v| v.result()).collect()
    }
}

impl<I, V> Tree<I, V>
where
    I: Image<Value = i32> + MutableImage + ImageFromDomain,
    I::Domain: SizedDomain,
    V: Copy,
{
    pub fn direct_filter<F>(&self, predicate: F) -> Tree<I, V>
    where
        F: Fn(i32) -> bool,
    {
        let mut new_parent = Vec::<i32>::with_capacity(self.num_nodes());
        let mut new_value = Vec::<V>::with_capacity(self.num_nodes());
        let mut new_nodemap = I::new_from_domain(&self.nodemap.domain());

        let mut links_table: Vec<i32> = vec![Default::default(); self.num_nodes()];
        let mut count = 1;
        new_parent.push(0);
        new_value.push(*self.value(0));

        for n in 1..self.num_nodes() {
            if !predicate(n as i32) {
                *links_table.get_mut(n).unwrap() =
                    *links_table.get(*self.parent(n as i32) as usize).unwrap();
            } else {
                *links_table.get_mut(n as usize).unwrap() = count;
                count += 1;
                new_parent.push(*links_table.get(*self.parent(n as i32) as usize).unwrap());
                new_value.push(*self.value(n as i32));
            }
        }

        for p in self.nodemap.domain() {
            *new_nodemap.at_point_mut(&p) = *links_table
                .get(*self.nodemap.at_point(&p) as usize)
                .unwrap();
        }

        Tree::build(new_nodemap, new_parent, new_value)
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
