use olympus::{
    drawing::label2rgb,
    graph::AdjacencyList,
    morpho::{gradient, watershed_partition},
    GraphBuildFromNumberOfNodes, Image, MutableEdgeGraph, NodeDomain, NodeImage, NodeToNode,
    Point2d, Rgb8, SizedDomain, Window,
};

trait NodeImageDot {
    fn dot_string(&self, pos: Option<&[Point2d]>) -> String;
}

impl NodeImageDot for NodeImage<u8> {
    fn dot_string(&self, pos: Option<&[Point2d]>) -> String {
        let nbh = NodeToNode::new(self.domain());
        let mut visited = vec![false; self.domain().size()];

        let mut res = String::new();
        res.push_str("graph G {{");
        for p in self.domain().clone() {
            let pos = if let Some(pn) = pos {
                format!(" pos=\"{}.0, {}.0\"", pn[p as usize].x, pn[p as usize].y)
            } else {
                String::new()
            };
            res.push_str(
                format!(
                    "\t{} [label=\"\" fillcolor=\"#{}\", shape=\"circle\" style=\"filled\"{}]\n",
                    p,
                    Rgb8::new(self[p], self[p], self[p]).hex(),
                    pos
                )
                .as_str(),
            );
            for n in nbh.apply(&p) {
                if !visited[n as usize] {
                    res.push_str(format!("\t{} -- {}\n", p, n).as_str());
                }
            }
            visited[p as usize] = true;
        }
        res.push_str("}}");
        res
    }
}

impl NodeImageDot for NodeImage<Rgb8> {
    fn dot_string(&self, pos: Option<&[Point2d]>) -> String {
        let mut res = String::new();
        let nbh = NodeToNode::new(self.domain());
        let mut visited = vec![false; self.domain().size()];

        res.push_str("graph G {{");
        for p in self.domain().clone() {
            let pos = if let Some(pn) = pos {
                format!(" pos=\"{}.0, {}.0\"", pn[p as usize].x, pn[p as usize].y)
            } else {
                String::new()
            };
            res.push_str(
                format!(
                    "\t{} [label=\"\" fillcolor=\"#{}\", shape=\"circle\" style=\"filled\"{}]\n",
                    p,
                    self[p].hex(),
                    pos
                )
                .as_str(),
            );
            for n in nbh.apply(&p) {
                if !visited[n as usize] {
                    res.push_str(format!("\t{} -- {}\n", p, n).as_str());
                }
            }
            visited[p as usize] = true;
        }
        res.push_str("}}");
        res
    }
}

#[allow(dead_code)]
fn print_dot<I: NodeImageDot>(img: &I, pos: Option<&[Point2d]>) {
    println!("{}", img.dot_string(pos));
}

#[allow(dead_code)]
fn save_dot<I: NodeImageDot>(img: &I, out: &str, pos: Option<&[Point2d]>) {
    if let Err(e) = std::fs::write(out, img.dot_string(pos)) {
        eprintln!("Failed to write DOT file '{}': {}", out, e);
    }
}

fn main() {
    // Colors of the graph node image
    let colors: Vec<u8> = vec![
        180, 178, 177, 70, 185, 190, 178, 178, 131, 169, 164, 178, 134, 157, 33, 23, 22, 142, 25,
    ];

    let pos = [
        Point2d::new(37, 229),
        Point2d::new(138, 235),
        Point2d::new(204, 225),
        Point2d::new(104, 209),
        Point2d::new(93, 182),
        Point2d::new(22, 171),
        Point2d::new(144, 157),
        Point2d::new(209, 165),
        Point2d::new(78, 130),
        Point2d::new(31, 116),
        Point2d::new(210, 102),
        Point2d::new(147, 97),
        Point2d::new(111, 70),
        Point2d::new(34, 70),
        Point2d::new(197, 56),
        Point2d::new(76, 31),
        Point2d::new(144, 22),
        Point2d::new(19, 25),
        Point2d::new(211, 20),
    ];

    // Domain of the graph node image
    let mut g = AdjacencyList::new_from_number_of_nodes(19);
    g.add_edge(0, 1).unwrap();
    g.add_edge(0, 3).unwrap();
    g.add_edge(0, 5).unwrap();
    g.add_edge(1, 2).unwrap();
    g.add_edge(1, 3).unwrap();
    g.add_edge(1, 6).unwrap();
    g.add_edge(2, 6).unwrap();
    g.add_edge(2, 7).unwrap();
    g.add_edge(3, 4).unwrap();
    g.add_edge(3, 6).unwrap();
    g.add_edge(3, 8).unwrap();
    g.add_edge(3, 5).unwrap();
    g.add_edge(4, 6).unwrap();
    g.add_edge(4, 8).unwrap();
    g.add_edge(5, 8).unwrap();
    g.add_edge(5, 9).unwrap();
    g.add_edge(6, 7).unwrap();
    g.add_edge(6, 10).unwrap();
    g.add_edge(6, 11).unwrap();
    g.add_edge(6, 8).unwrap();
    g.add_edge(7, 10).unwrap();
    g.add_edge(8, 9).unwrap();
    g.add_edge(8, 11).unwrap();
    g.add_edge(8, 12).unwrap();
    g.add_edge(9, 12).unwrap();
    g.add_edge(9, 13).unwrap();
    g.add_edge(10, 11).unwrap();
    g.add_edge(10, 14).unwrap();
    g.add_edge(11, 12).unwrap();
    g.add_edge(11, 14).unwrap();
    g.add_edge(11, 16).unwrap();
    g.add_edge(12, 13).unwrap();
    g.add_edge(12, 15).unwrap();
    g.add_edge(12, 16).unwrap();
    g.add_edge(13, 15).unwrap();
    g.add_edge(13, 17).unwrap();
    g.add_edge(14, 16).unwrap();
    g.add_edge(14, 18).unwrap();
    g.add_edge(15, 16).unwrap();
    g.add_edge(15, 17).unwrap();
    g.add_edge(16, 18).unwrap();
    let domain = NodeDomain::new(g);

    let img = NodeImage::<u8>::new(domain, colors).unwrap();
    let nbh = NodeToNode::new(img.domain());
    save_dot(&img, "node_graph.dot", Some(&pos));
    let grad = gradient(&img, &nbh);
    save_dot(&grad, "node_graph_grad.dot", Some(&pos));
    let ws = watershed_partition(&grad, &nbh);
    save_dot(&label2rgb(&ws), "node_graph_watershed.dot", Some(&pos));
}
