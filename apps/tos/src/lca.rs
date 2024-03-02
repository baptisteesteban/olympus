use olympus::{morpho::Tree, Image2d};

pub struct Lca<'a, V> {
    t: &'a Tree<Image2d<i32>, V>,
    depth: Vec<i32>,
}

impl<'a, T> Lca<'a, T> {
    pub fn new(t: &'a Tree<Image2d<i32>, T>) -> Lca<'a, T> {
        Lca {
            t: t,
            depth: t.depth(),
        }
    }

    pub fn find(&self, mut a: i32, mut b: i32) -> i32 {
        while self.depth.get(a as usize) > self.depth.get(b as usize) {
            a = *self.t.parent(a);
        }
        while self.depth.get(b as usize) > self.depth.get(a as usize) {
            b = *self.t.parent(b);
        }
        while a != b {
            a = *self.t.parent(a);
            b = *self.t.parent(b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use olympus::{morpho::Tree, Image2d};

    use crate::Lca;

    #[test]
    fn test_lca() {
        let t = Tree::build(
            Image2d::default(),
            Vec::<i32>::from([0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 5, 5, 7, 7]),
            Vec::<u8>::default(),
        );
        let lca = Lca::new(&t);

        assert_eq!(lca.find(8, 9), 3);
        assert_eq!(lca.find(8, 8), 8);
        assert_eq!(lca.find(8, 3), 3);
        assert_eq!(lca.find(10, 6), 0);
    }
}
