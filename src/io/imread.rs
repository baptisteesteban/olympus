use crate::Image2d;

use image::DynamicImage;

mod internals {
    use std::ops::Deref;

    use image::{ImageBuffer, Luma, Primitive, GenericImageView};
    use crate::Image2d;

    pub fn read_luma<V: Clone + Default + Primitive, Cont: Deref<Target = [V]>>(img: &mut Image2d<V>, buffer: ImageBuffer<Luma<V>, Cont>) {
        let (width, height) = buffer.dimensions();
        img.resize(width as usize, height as usize);
        for p in img.domain() {
            let v = buffer.get_pixel(p.x() as u32, p.y() as u32).0;
            *img.at_point_mut(&p) = v[0];
        }
    }
}

pub fn imread(img: &mut Image2d<u8>, filename: &str) -> Result<(), &'static str> {
    let rimg = image::open(filename).expect("Image reader error");
    match rimg {
        DynamicImage::ImageLuma8(buffer) => internals::read_luma(img, buffer),
        _ => { return Err("Only u8 images are handled"); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{io::imread, Image2d};

    #[test]
    fn test_read_pgm_u8() {
        const REFVAL : [u8; 8] = [
            4, 3, 9, 67,
            43, 125, 253, 37
        ];

        let mut img = Image2d::<u8>::default();
        if let Err(_) = imread(&mut img, "imgs/test.pgm") {
            assert!(false);
        }

        assert_eq!(img.width(), 4);
        assert_eq!(img.height(), 2);

        for p in img.domain() {
            assert_eq!(*img.at_point(&p), REFVAL[p.y() * img.width() + p.x()]);
        }
    }
}