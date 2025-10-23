//use clap::Parser;
use olympus::{
    cond,
    io::{imread, imsave},
    labeling::local_maxima,
    morpho::gradient,
    where_image, Image2d, Mask2d, C4,
};
/*
#[derive(Parser)]
struct Args {
    input_filename: String,
    output_filename: String,
}*/

fn main() {
    //let args = Args::parse();

    let mut img = Image2d::<u8>::default();
    imread("/home/baptou/Pictures/amazigh.pgm", &mut img).unwrap();
    let grad = gradient(&img, &Mask2d::cross(3, 3).unwrap());
    imsave("grad.png", &grad).unwrap();
    let minima = local_maxima(&img, &C4);
    let c = cond(&minima, |_, v| *v > 0);
    let w = where_image(&c, 255u8, 0u8);
    imsave("minima.png", &w).unwrap();
}
