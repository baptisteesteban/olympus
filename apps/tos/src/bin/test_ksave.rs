use olympus::Image2d;
use olympus_tos::ksave;

fn main() {
    let img = Image2d::<u8>::new(3, 3);
    ksave(&img, "tmp.svg")
}
