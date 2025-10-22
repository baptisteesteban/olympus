use std::fmt::Display;

use crate::Image2d;

/// Print a 2D image in a terminal.
pub fn imprint2d<V>(img: &Image2d<V>)
where
    V: Display,
{
    for y in 0..img.height() {
        for x in 0..img.width() - 1 {
            print!("{} ", img[(x, y)]);
        }
        println!("{}", img[(img.width() - 1, y)]);
    }
}
