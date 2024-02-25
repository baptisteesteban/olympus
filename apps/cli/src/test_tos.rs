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
    let t = tos(&img);
    let area = t.accumulate_on_points(CountAccumulator::new());
    let filtered_tree = t.direct_filter(|n| *area.get(n as usize).unwrap() > 300);
    imsave(&filtered_tree.reconstruct(), "/tmp/ord2.png").unwrap();
}
