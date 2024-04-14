use olympus::{
    accu::{Accumulator, CountAccumulator},
    io::{imread, imsave},
    morpho::tos,
    Image2d, Point2d,
};
use olympus_tos::{add_median_border, max_interpolation};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(name = "input", help = "The input image path")]
    input_filename: String,
    #[arg(name = "minimum area", help = "The minimum size of a shape")]
    min_area: usize,
    #[arg(name = "output", help = "The output image path")]
    output_filename: String,
}

fn main() {
    let args = Args::parse();
    let mut img = Image2d::<u8>::default();
    imread(&mut img, &args.input_filename).unwrap();
    let bordered = add_median_border(&img);
    let interpolated = max_interpolation(&bordered);
    let p0 = Point2d::new(0, 0);
    let t = tos(&interpolated, &p0);
    let area = t.accumulate_on_points(CountAccumulator::new());
    let filtered = t.direct_filter(|n| *area.get(n as usize).unwrap() >= args.min_area);
    let rec = filtered.reconstruct();
    imsave(&rec, &args.output_filename).unwrap();
}
