use olympus::{
    morpho::{maxtree, ComponentTree},
    Image2d, C4,
};

use crate::{distance_transform, immersion};

/// Compute the Morphological Tree of Shapes as defined by the algorithm in
/// **The Tree of Shapes as a distance transform: building the ToS on
/// High-Dynamic Range images** from Edwin Carlinet and Baptiste Esteban.
pub fn tos(img: &Image2d<u8>) -> ComponentTree<u8> {
    let immersed = immersion(img);
    let (dt, flatten) = distance_transform(&immersed);
    let mt = maxtree(&dt, &C4);
    let mut value_vec = vec![0u8; mt.num_nodes()];
    for p in *mt.nodemap.domain() {
        value_vec[mt.nodemap[p]] = flatten[p];
    }
    ComponentTree::new(mt.parents, value_vec, mt.nodemap)
}

pub fn emersion<V>(t: ComponentTree<V>) -> ComponentTree<V> {
    let mut new_nodemap = unsafe {
        Image2d::<usize>::new_uninitialized(
            (t.nodemap.width() + 1) / 2,
            (t.nodemap.height() + 1) / 2,
        )
        .unwrap()
    };
    for p in *new_nodemap.domain() {
        new_nodemap[p] = t.nodemap[p * 2];
    }
    ComponentTree::new(t.parents, t.values, new_nodemap)
}
