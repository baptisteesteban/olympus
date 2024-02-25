use olympus::{
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
    //let img = Image2d::<u8>::new_from_vec(3, 3, Vec::<u8>::from([1, 1, 1, 1, 2, 1, 1, 1, 1]));
    let t = maxtree(&img, C4::new());
    imsave(&t.reconstruct(), "/tmp/ord2.png").unwrap();
}
