use olympus::{
    accu::{Accumulator, CountAccumulator},
    io::{imread, imsave},
    morpho::{maxtree, tos},
    Image2d, C4,
};

fn main() {
    let mut img = Image2d::<u8>::default();
    imread(
        &mut img,
        "/home/baptou/Pictures/laurent_condat_database/gray/IM106.tif",
    )
    .unwrap();
    let t = maxtree(&img, C4::new());
    let area = t.accumulate_on_points(CountAccumulator::new());
    imsave(&t.reconstruct(), "/tmp/ord2.png").unwrap();
}
