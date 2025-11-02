use genericity::parse_off;

fn main() {
    let (remaining, (n_vertices, n_faces, n_edges)) =
        parse_off("OFF\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n").unwrap();
    println!(
        "Vertices: {} Faces: {} Edges: {}",
        n_vertices, n_faces, n_edges
    );
    println!("Remaining:\n\"\"\"\n{}\"\"\"", remaining);
    println!("Remaining size: {}", remaining.len());
}
