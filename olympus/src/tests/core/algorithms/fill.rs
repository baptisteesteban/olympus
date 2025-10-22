use crate::{
    fill, graph::AdjacencyList, GraphBuildFromNumberOfNodes, Image, Image2d, NodeDomain, NodeImage,
};

#[test]
fn test_fill_image2d() {
    let mut img = unsafe { Image2d::<u8>::new_uninitialized(3, 1).unwrap() };
    fill(&mut img, 3);
    for p in *img.domain() {
        assert_eq!(img[p], 3);
    }
}

#[test]
fn test_fill_node_image() {
    let g = AdjacencyList::new_from_number_of_nodes(3);
    let mut img = NodeImage::new(NodeDomain::new(g), vec![0, 0, 0]).unwrap();
    fill(&mut img, 5);
    for p in img.domain().clone() {
        assert_eq!(img[p], 5);
    }
}
