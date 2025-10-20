use std::{
    fs::File,
    io::{Result, Write},
};

use crate::drawing::svg::SVGElement;

/// No documentation will be written for SVG as it will be completely rewitten
/// using [rust_tizk](https://docs.rs/rust_tikz/latest/rust_tikz/).
pub struct SVGDrawer {
    width: i32,
    height: i32,
    elements: Vec<Box<dyn SVGElement>>,
}

impl SVGDrawer {
    pub fn new(width: i32, height: i32) -> SVGDrawer {
        SVGDrawer {
            width,
            height,
            elements: Default::default(),
        }
    }

    pub fn add_element(&mut self, e: Box<dyn SVGElement>) {
        self.elements.push(e);
    }

    pub fn generate(&self) -> String {
        let mut res = format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">\n",
            self.width, self.height
        );
        for e in &self.elements {
            res.push_str(format!("\t{}\n", e.to_svg_string_node()).as_str());
        }
        res.push_str("</svg>");
        res
    }
}

pub fn save_svg(svg: &SVGDrawer, filename: &str) -> Result<()> {
    let mut file = File::create_new(filename).unwrap();
    let svg_str = svg.generate();
    file.write_all(svg_str.as_bytes()).unwrap();
    Ok(())
}
