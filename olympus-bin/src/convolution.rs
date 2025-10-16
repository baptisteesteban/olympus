use clap::Parser;
use olympus::{
    convolution,
    io::{imread, imsave},
    Image2d,
};

#[derive(Parser)]
struct Args {
    input_filename: String,
    output_filename: String,
}

fn main() {
    let args = Args::parse();
    let mut img = Image2d::<u8>::default();
    imread(&args.input_filename, &mut img).unwrap();
    let out = convolution(&img);
    imsave(&args.output_filename, &out).unwrap();
}
