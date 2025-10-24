use olympus::{
    drawing::label2rgb, graph::AdjacencyList, morpho::watershed_partition, Image, MutableEdgeGraph,
    MutableNodeGraph, NodeDomain, NodeImage, NodeToNode, Rgb8, SizedDomain, Window,
};

trait NodeImageDot {
    fn print_dot(&self);
}

impl NodeImageDot for NodeImage<u8> {
    fn print_dot(&self) {
        let nbh = NodeToNode::new(self.domain());
        let mut visited = vec![false; self.domain().size()];

        println!("graph G {{");
        for p in self.domain().clone() {
            println!(
                "\t{} [label=\"\" fillcolor=\"#{}\", shape=\"circle\" style=\"filled\"]",
                p,
                Rgb8::new(self[p], self[p], self[p]).hex()
            );
            for n in nbh.apply(&p) {
                if !visited[n as usize] {
                    println!("\t{} -- {}", p, n);
                }
            }
            visited[p as usize] = true;
        }
        println!("}}");
    }
}

impl NodeImageDot for NodeImage<Rgb8> {
    fn print_dot(&self) {
        let nbh = NodeToNode::new(self.domain());
        let mut visited = vec![false; self.domain().size()];

        println!("graph G {{");
        for p in self.domain().clone() {
            println!(
                "\t{} [label=\"\" fillcolor=\"#{}\", shape=\"circle\" style=\"filled\"]",
                p,
                self[p].hex()
            );
            for n in nbh.apply(&p) {
                if !visited[n as usize] {
                    println!("\t{} -- {}", p, n);
                }
            }
            visited[p as usize] = true;
        }
        println!("}}");
    }
}

fn print_dot<I: NodeImageDot>(img: &I) {
    img.print_dot()
}

fn main() {
    let mut g = AdjacencyList::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge(n1, n2).unwrap();
    g.add_edge(n2, n3).unwrap();

    let img = NodeImage::<u8>::new(NodeDomain::new(g), vec![0, 5, 0]).unwrap();
    //print_dot(&img);
    let ws = watershed_partition(&img, &NodeToNode::new(img.domain()));
    print_dot(&label2rgb(&ws));
}
