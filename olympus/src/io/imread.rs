use image::ImageReader;

use super::ImageReaderDispatch;
use crate::{Image2d, Rgb8};

impl ImageReaderDispatch for Image2d<u8> {
    fn read(&mut self, filename: &str) -> Result<(), String> {
        let in_img = ImageReader::open(filename).unwrap().decode();
        if let Err(e) = in_img {
            return Err(e.to_string());
        }
        let in_buffer = in_img.as_ref().unwrap().as_luma8();
        if in_buffer.is_none() {
            return Err(String::from("Input image is not u8 image"));
        }
        unsafe {
            self.resize(
                in_img.as_ref().unwrap().width() as i32,
                in_img.as_ref().unwrap().height() as i32,
            );
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                self[(x, y)] = in_buffer.unwrap().get_pixel(x as u32, y as u32).0[0];
            }
        }
        Ok(())
    }
}

impl ImageReaderDispatch for Image2d<Rgb8> {
    fn read(&mut self, filename: &str) -> Result<(), String> {
        let in_img = ImageReader::open(filename).unwrap().decode();
        if let Err(e) = in_img {
            return Err(e.to_string());
        }
        let in_buffer = in_img.as_ref().unwrap().as_rgb8();
        if in_buffer.is_none() {
            return Err(String::from("Input image is ot rgb8 image"));
        }
        unsafe {
            self.resize(
                in_img.as_ref().unwrap().width() as i32,
                in_img.as_ref().unwrap().height() as i32,
            );
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                self[(x, y)].r = in_buffer.as_ref().unwrap().get_pixel(x as u32, y as u32).0[0];
                self[(x, y)].g = in_buffer.as_ref().unwrap().get_pixel(x as u32, y as u32).0[1];
                self[(x, y)].b = in_buffer.as_ref().unwrap().get_pixel(x as u32, y as u32).0[2];
            }
        }
        Ok(())
    }
}

/// Read an image from `filename` and store its data in `img`.
pub fn imread<I: ImageReaderDispatch>(filename: &str, img: &mut I) -> Result<(), String> {
    img.read(filename)
}
