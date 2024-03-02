use olympus::{traits::Image, Image2d};

use crate::{is_0_face, is_1h_face, is_1v_face};

pub fn kprint<V>(k: &Image2d<V>) {
    for p in k.domain() {
        if is_0_face(&p) {
            print!("+");
        } else if is_1h_face(&p) {
            print!("|");
        } else if is_1v_face(&p) {
            print!("-");
        } else {
            print!(" ");
        }
        if p.x() == k.width() - 1 {
            print!("\n");
        }
    }
}
