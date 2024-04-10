use olympus::{
    accu::{Accumulator, CountAccumulator},
    io::{imread, imsave},
    morpho::tos,
    Image2d, Point2d,
};
use olympus_tos::{add_median_border, max_interpolation};

fn main() {
    let mut img = Image2d::<u8>::default();
    imread(&mut img, "/home/baptou/Pictures/condat/gray/IM106.tif").unwrap();
    let bordered = add_median_border(&img);
    let interpolated = max_interpolation(&bordered);
    let p0 = Point2d::new(0, 0);
    let t = tos(&interpolated, &p0);
    let area = t.accumulate_on_points(CountAccumulator::new());
    let filtered = t.direct_filter(|n| *area.get(n as usize).unwrap() > 100);
    let rec = filtered.reconstruct();
    imsave(&rec, "out.png").unwrap();
}
