use clap::{Parser, ValueEnum};
use olympus::{
    io::{imread, imsave},
    morpho::{closing, dilation, erosion, external_gradient, gradient, internal_gradient, opening},
    Image2d, Mask2d,
};

#[derive(Clone, ValueEnum)]
enum MorphologicalOperation {
    Erosion,
    Dilation,
    Opening,
    Closing,
    Gradient,
    InternalGradient,
    ExternalGradient,
}

#[derive(Clone, ValueEnum)]
enum SEType {
    Cross,
    Rect,
}

#[derive(Parser)]
struct Args {
    input_filename: String,
    operation: MorphologicalOperation,
    se_type: SEType,
    se_width: i32,
    se_height: i32,
    output_filename: String,
}

fn main() {
    let args = Args::parse();
    let mut img = Image2d::<u8>::default();
    imread(&args.input_filename, &mut img).unwrap();
    let mask = match args.se_type {
        SEType::Cross => Mask2d::cross(args.se_width, args.se_height),
        SEType::Rect => Mask2d::rect(args.se_width, args.se_height),
    }
    .unwrap();
    let out = match args.operation {
        MorphologicalOperation::Erosion => erosion(&img, &mask),
        MorphologicalOperation::Dilation => dilation(&img, &mask),
        MorphologicalOperation::Opening => opening(&img, &mask),
        MorphologicalOperation::Closing => closing(&img, &mask),
        MorphologicalOperation::Gradient => gradient(&img, &mask),
        MorphologicalOperation::InternalGradient => internal_gradient(&img, &mask),
        MorphologicalOperation::ExternalGradient => external_gradient(&img, &mask),
    };
    imsave(&args.output_filename, &out).unwrap();
}
