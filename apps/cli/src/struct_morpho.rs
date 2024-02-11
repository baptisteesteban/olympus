use clap::{Parser, ValueEnum};
use olympus::Image2d;

//enum SEArg {
//    DISK(i32),
//    CROSS(i32, i32),
//    RECT(i32, i32),
//}

#[derive(ValueEnum, Clone)]
enum OPArg {
    EROSION,
    DILATION,
    OPENING,
    CLOSING,
}

#[derive(Parser)]
struct Args {
    #[arg(name = "input", help = "The input image path")]
    input_filename: String,
    #[arg(name = "output", help = "The output image path")]
    output_filename: String,
    #[arg(name = "operation", help = "The operation to perform")]
    operation: OPArg,
    #[arg(name = "width")]
    width: i32,
    #[arg(name = "height")]
    height: i32,
}

fn main() {
    let args = Args::parse();
    let mut input = Image2d::<u8>::default();
    olympus::io::imread(&mut input, &args.input_filename).unwrap();

    let se = olympus::rect2d(args.width, args.height).unwrap();

    let output = match args.operation {
        OPArg::EROSION => olympus::morpho::erosion(&input, &se),
        OPArg::DILATION => olympus::morpho::dilation(&input, &se),
        OPArg::OPENING => olympus::morpho::opening(&input, &se),
        OPArg::CLOSING => olympus::morpho::closing(&input, &se),
    };

    olympus::io::imsave(&output, &args.output_filename).unwrap();
}
