use std::any::Any;

use crate::algorithms::copy;
use crate::Image2d;
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
