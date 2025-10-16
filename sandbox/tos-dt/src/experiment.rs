use clap::Parser;
use olympus::{
    io::{imread, imsave},
    morpho::{direct_filter, reconstruct},
    Image2d,
};
use tos_dt::{add_median_border, interpolation_median, tos};

/// A program performing a grain filter on a Tree of Shapes built on an image
/// created with a max interpolation and with a new border.
#[derive(Parser)]
struct Args {
    /// The filename to the input image
    input_filename: String,
    /// The area threshold
    lambda: usize,
    /// The filename to the output image
    output_filename: String,
}

fn main() {
    let args = Args::parse();
    let mut img = Image2d::<u8>::default();
    imread(args.input_filename.as_str(), &mut img).unwrap();
    let bordered = add_median_border(&img);
    let interpolated = interpolation_median(&bordered);
    let t = tos(&interpolated);
    let area = t.compute_area();
    let tp = direct_filter(&t, |n| area[n] >= args.lambda);
    let rec = reconstruct(&tp);
    imsave(args.output_filename.as_str(), &rec).unwrap();
}
