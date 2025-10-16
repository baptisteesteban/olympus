use clap::Parser;
use olympus::{
    drawing::{apply_colormap, inferno},
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
    imread(args.input_filename.as_str(), &mut img).unwrap();
    let res = apply_colormap(&img, inferno);
    imsave(args.output_filename.as_str(), &res).unwrap();
}
