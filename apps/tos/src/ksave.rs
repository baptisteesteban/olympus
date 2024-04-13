use svg::node::element::path::Data;
use svg::node::element::Circle;
use svg::node::element::Path;
use svg::Document;

use olympus::traits::Image;
use olympus::Image2d;

use crate::{is_0_face, is_1v_face, is_2_face};

pub fn ksave<V>(k: &Image2d<V>, filename: &str) {
    let mut document = Document::new().set("viewBox", (0, 0, 2 * k.width(), 2 * k.height()));

    for p in k.domain() {
        if !is_0_face(&p) {
            let data = {
                if is_2_face(&p) {
                    Data::new()
                        .move_to((2 * p.x(), 2 * p.y()))
                        .line_by((2, 0))
                        .line_by((0, 2))
                        .line_by((-2, 0))
                        .close()
                } else if is_1v_face(&p) {
                    Data::new()
                        .move_to((2.0 * (p.x() as f32) + 0.5, 2.0 * (p.y() as f32)))
                        .line_by((1, 0))
                        .line_by((0, 2))
                        .line_by((-1, 0))
                        .close()
                } else {
                    Data::new()
                        .move_to((2.0 * (p.x() as f32), 2.0 * (p.y() as f32) + 0.5))
                        .line_by((2, 0))
                        .line_by((0, 1))
                        .line_by((-2, 0))
                        .close()
                }
            };

            let element = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set("d", data);

            document = document.add(element);
        } else {
            let element = Circle::new()
                .set("cx", 2 * p.x() + 1)
                .set("cy", 2 * p.y() + 1)
                .set("r", 0.5)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2);
            document = document.add(element);
        }
    }

    svg::save(filename, &document).unwrap();
}
