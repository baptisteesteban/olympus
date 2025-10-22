use crate::{
    graph::AdjacencyList,
    morpho::{closing, dilation, erosion, opening},
    Image, Image2d, Mask2d, MutableEdgeGraph, MutableNodeGraph, NodeDomain, NodeImage, NodeToNode,
};

#[test]
fn test_erosion2d() {
    let img = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 118, 95, 86, 90, 112, 31, 214, 140, 253, 177, 59, 131, 63, 100, 140, 91, 126,
            188, 42, 180, 253, 94, 53, 8, 222, 46, 217, 246, 152, 125, 101, 30, 102, 142,
        ],
    )
    .unwrap();

    let ref_res = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 95, 86, 86, 59, 31, 31, 95, 118, 91, 59, 59, 42, 31, 100, 91, 53, 8, 42, 42, 42,
            94, 53, 8, 8, 8, 42, 46, 152, 94, 53, 8, 30, 30, 102,
        ],
    )
    .unwrap();

    let se = Mask2d::cross(3, 3).unwrap();
    let res: Image2d<u8> = erosion(&img, &se);
    assert_eq!(res, ref_res);
}

#[test]
fn test_dilation2d() {
    let img = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 118, 95, 86, 90, 112, 31, 214, 140, 253, 177, 59, 131, 63, 100, 140, 91, 126,
            188, 42, 180, 253, 94, 53, 8, 222, 46, 217, 246, 152, 125, 101, 30, 102, 142,
        ],
    )
    .unwrap();

    let ref_res = Image2d::from_vec(
        7,
        5,
        vec![
            214u8, 140, 253, 177, 112, 131, 112, 214, 253, 253, 253, 188, 131, 180, 253, 140, 253,
            188, 222, 188, 217, 253, 253, 125, 222, 222, 222, 217, 253, 246, 152, 125, 222, 142,
            217,
        ],
    )
    .unwrap();

    let se = Mask2d::cross(3, 3).unwrap();
    let res = dilation(&img, &se);
    assert_eq!(res, ref_res);
}

#[test]
fn test_opening2d() {
    let img = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 118, 95, 86, 90, 112, 31, 214, 140, 253, 177, 59, 131, 63, 100, 140, 91, 126,
            188, 42, 180, 253, 94, 53, 8, 222, 46, 217, 246, 152, 125, 101, 30, 102, 142,
        ],
    )
    .unwrap();

    let ref_res = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 118, 95, 86, 86, 59, 31, 118, 118, 118, 91, 59, 59, 42, 100, 118, 91, 59, 59, 42,
            46, 152, 94, 53, 8, 42, 46, 102, 152, 152, 94, 53, 30, 102, 102,
        ],
    )
    .unwrap();

    let se = Mask2d::cross(3, 3).unwrap();
    let res = opening(&img, &se);
    assert_eq!(res, ref_res);
}

#[test]
fn test_closing2d() {
    let img = Image2d::from_vec(
        7,
        5,
        vec![
            95u8, 118, 95, 86, 90, 112, 31, 214, 140, 253, 177, 59, 131, 63, 100, 140, 91, 126,
            188, 42, 180, 253, 94, 53, 8, 222, 46, 217, 246, 152, 125, 101, 30, 102, 142,
        ],
    )
    .unwrap();

    let ref_res = Image2d::from_vec(
        7,
        5,
        vec![
            140u8, 140, 140, 112, 112, 112, 112, 214, 140, 253, 177, 112, 131, 112, 140, 140, 125,
            188, 188, 131, 180, 253, 125, 125, 125, 222, 142, 217, 246, 152, 125, 125, 125, 142,
            142,
        ],
    )
    .unwrap();

    let se = Mask2d::cross(3, 3).unwrap();
    let res = closing(&img, &se);
    assert_eq!(res, ref_res);
}

#[test]
fn test_erosion_node_graph_image() {
    let mut g = AdjacencyList::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    let n4 = g.add_node();

    g.add_edge(n1, n2).unwrap();
    g.add_edge(n1, n3).unwrap();
    g.add_edge(n1, n4).unwrap();

    let img = NodeImage::<u8>::new(NodeDomain::new(g.clone()), vec![10, 14, 7, 2]).unwrap();
    let ref_res = NodeImage::<u8>::new(NodeDomain::new(g), vec![2, 10, 7, 2]).unwrap();

    let nbh = NodeToNode::new(img.domain());
    let res = erosion(&img, &nbh);

    assert_eq!(res, ref_res);
}
