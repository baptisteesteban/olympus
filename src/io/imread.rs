use std::any::Any;

use crate::{copy, Image2d};
use image::DynamicImage;

use super::internals::{ImagersReader, LumaImagersReader};

pub fn imread_dynamic(filename: &str) -> Result<Box<dyn Any>, String> {
    let attempt_read = image::open(filename);
    let dyn_img = match attempt_read {
        Ok(img) => img,
        Err(e) => return Err(e.to_string()),
    };
    let mut reader: Box<dyn ImagersReader>;
    match dyn_img {
        DynamicImage::ImageLuma8(v) => {
            reader =
                Box::<LumaImagersReader<u8, Vec<u8>>>::new(LumaImagersReader::<u8, Vec<u8>>::new(v))
        }
        DynamicImage::ImageLuma16(v) => {
            reader = Box::<LumaImagersReader<u16, Vec<u16>>>::new(
                LumaImagersReader::<u16, Vec<u16>>::new(v),
            )
        }
        _ => {
            return Err(String::from("Invalid image input type"));
        }
    }
    match reader.read() {
        Err(s) => Err(s),
        Ok(()) => Ok(reader.get_img()),
    }
}

pub fn imread<V: Copy + Default + 'static>(
    img: &mut Image2d<V>,
    filename: &str,
) -> Result<(), String> {
    let v = match imread_dynamic(filename) {
        Err(e) => {
            return Err(e);
        }
        Ok(img) => img,
    };
    let res = match v.downcast_ref::<Image2d<V>>() {
        None => {
            return Err(String::from("Invalid input image type"));
        }
        Some(e) => e,
    };
    img.resize(res.width(), res.height());
    copy(res, img).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{io::imread, traits::Image, Image2d};

    #[test]
    fn test_read_pgm_u8() {
        const REFVAL: [u8; 8] = [4, 3, 9, 67, 43, 125, 253, 37];

        let mut img = Image2d::<u8>::default();
        if let Err(_) = imread(&mut img, "imgs/test.pgm") {
            assert!(false);
        }

        assert_eq!(img.width(), 4);
        assert_eq!(img.height(), 2);

        for p in img.domain() {
            assert_eq!(
                *img.at_point(&p),
                REFVAL[(p.y() * img.width() + p.x()) as usize]
            );
        }
    }

    #[test]
    fn test_read_tiff_u16() {
        const REFVAL: [u16; 15] = [
            0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800,
        ];
        let mut img: Image2d<u16> = Default::default();
        imread(&mut img, "imgs/test_u16.tiff").unwrap();
        assert_eq!(img.width(), 5);
        assert_eq!(img.height(), 3);

        for p in img.domain() {
            assert_eq!(
                *img.at_point(&p),
                REFVAL[(p.y() * img.width() + p.x()) as usize]
            );
        }
    }
}
