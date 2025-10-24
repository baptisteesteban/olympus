use clap::Parser;
use olympus::{
    drawing::label2rgb,
    io::{imread, imsave},
    morpho::{direct_filter, gradient, mintree, reconstruct, watershed_partition},
    Image2d, Mask2d, C4,
};

#[derive(Parser)]
struct Args {
    input_filename: String,
    lambda: usize,
    output_filename: String,
}

fn main() {
    let args = Args::parse();

    let mut img = Image2d::<u8>::default();
    imread(args.input_filename.as_str(), &mut img).unwrap();
    let grad = gradient(&img, &Mask2d::cross(3, 3).unwrap());
    let t = mintree(&grad, &C4);
    let area = t.compute_area();
    let tf = direct_filter(&t, |n| area[n] >= args.lambda);
    let grad_f = reconstruct(&tf);
    let ws = watershed_partition(&grad_f, &C4);
    let out = label2rgb(&ws);
    imsave(args.output_filename.as_str(), &out).unwrap();
}
