use crate::Mesh;

pub fn write_off(mesh: &Mesh) -> String {
    let mut res = String::from("OFF\n");
    res.push_str(format!("{} {} 0\n", mesh.num_vertices(), mesh.num_triangles()).as_str());

    for p in mesh.vertices() {
        res.push_str(format!("{} {} {}\n", p.x, p.y, p.z).as_str());
    }

    for t in mesh.triangles() {
        res.push_str(format!("3 {} {} {}\n", t.v1, t.v2, t.v3).as_str());
    }

    res
}
