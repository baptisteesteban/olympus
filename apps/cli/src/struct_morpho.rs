use clap::{Parser, ValueEnum};
use olympus::Image2d;

#[derive(ValueEnum, Clone)]
enum SEArg {
    DISK,
    CROSS,
    RECT,
}

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
    #[arg(name = "se", help = "The structuring element to use")]
    se: SEArg,
    #[arg(
        name = "width",
        help = "The width of the structuring element (or the radius in the case of the disk)"
    )]
    width: i32,
    #[arg(name = "height", help = "The height of the structuring element")]
    height: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let mut input = Image2d::<u8>::default();
    olympus::io::imread(&mut input, &args.input_filename).unwrap();

    let se = match args.se {
        SEArg::CROSS => olympus::cross2d(args.width, args.height.unwrap_or(1)).unwrap(),
        SEArg::RECT => olympus::rect2d(args.width, args.height.unwrap_or(1)).unwrap(),
        SEArg::DISK => olympus::disk2d(args.width).unwrap(),
    };

    let output = match args.operation {
        OPArg::EROSION => olympus::morpho::erosion(&input, &se),
        OPArg::DILATION => olympus::morpho::dilation(&input, &se),
        OPArg::OPENING => olympus::morpho::opening(&input, &se),
        OPArg::CLOSING => olympus::morpho::closing(&input, &se),
    };

    olympus::io::imsave(&output, &args.output_filename).unwrap();
}
