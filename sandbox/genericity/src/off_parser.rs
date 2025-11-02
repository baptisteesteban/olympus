use genericity::{parse_off, write_off};

fn main() {
    let off_content = "OFF\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n";
    let (_, mesh) = parse_off(off_content).unwrap();
    let off_written = write_off(&mesh);
    println!("Same: {}", off_content == off_written);
}
