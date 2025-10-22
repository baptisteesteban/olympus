use clap::Parser;
use olympus::{
    drawing::label2rgb,
    io::{imread, imsave},
    labeling::local_minima,
    Image2d, C4,
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
    let minima = local_minima(&img, &C4);
    imsave(args.output_filename.as_str(), &label2rgb(&minima)).unwrap();
}
