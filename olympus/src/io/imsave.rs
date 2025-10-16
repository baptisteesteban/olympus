use image::{GrayImage, Luma, Rgb, RgbImage};

use super::ImageWriterDispatch;
use crate::{Image2d, Rgb8};

impl ImageWriterDispatch for Image2d<u8> {
    fn save(&self, filename: &str) -> Result<(), String> {
        let mut out = GrayImage::new(self.width() as u32, self.height() as u32);
        for y in 0..self.height() {
            for x in 0..self.width() {
                *out.get_pixel_mut(x as u32, y as u32) = Luma::<u8>::from([self[(x, y)]]);
            }
        }
        let err = out.save(filename);
        if let Err(e) = err {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}

impl ImageWriterDispatch for Image2d<Rgb8> {
    fn save(&self, filename: &str) -> Result<(), String> {
        let mut out = RgbImage::new(self.width() as u32, self.height() as u32);
        for y in 0..self.height() {
            for x in 0..self.width() {
                *out.get_pixel_mut(x as u32, y as u32) =
                    Rgb::<u8>::from([self[(x, y)].r, self[(x, y)].g, self[(x, y)].b]);
            }
        }
        let err = out.save(filename);
        if let Err(e) = err {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}

pub fn imsave<I: ImageWriterDispatch>(filename: &str, img: &I) -> Result<(), String> {
    img.save(filename)
}
