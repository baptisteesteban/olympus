use clap::Parser;
use olympus::{
    drawing::label2rgb,
    io::{imread, imsave},
    labeling::local_minima,
    morpho::{direct_filter, gradient, mintree, reconstruct, watershed_partition},
    Image2d, Mask2d, C8,
};

#[derive(Parser)]
struct Args {
    input_filename: String,
    output_filename: String,
}

fn main() {
    let args = Args::parse();

    let mut img = Image2d::<u8>::default();
    imread(args.input_filename.as_str(), &mut img).unwrap();
    let grad = gradient(&img, &Mask2d::cross(3, 3).unwrap());
    let mt = mintree(&grad, &C8);
    let area = mt.compute_area();
    let mtf = direct_filter(&mt, |n| area[n] >= 1000);
    let grad_rec = reconstruct(&mtf);
    let ws = watershed_partition(&grad_rec, &C8);
    imsave("grad_rec.png", &label2rgb(&local_minima(&grad_rec, &C8))).unwrap();
    imsave(args.output_filename.as_str(), &label2rgb(&ws)).unwrap();
}
