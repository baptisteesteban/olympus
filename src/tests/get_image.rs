use std::env;

// TODO: Use `Path`
pub(crate) fn get_image(img: &str) -> String {
    let mut path = env::var("CARGO_MANIFEST_DIR").unwrap();
    path.push_str("/imgs/");
    path.push_str(img);
    path
}
