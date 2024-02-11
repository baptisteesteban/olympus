use crate::Image2d;
use std::fmt::Display;

pub fn imprint<T: Display>(img: &Image2d<T>) {
    for y in 0..img.height() {
        for x in 0..img.width() - 1 {
            print!("{} ", img.at(x, y))
        }
        print!("{}\n", img.at(img.width() - 1, y));
    }
}
