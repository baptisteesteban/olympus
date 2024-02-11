use std::{any::Any, ops::Deref};

use image::{ImageBuffer, Luma, Primitive};

use crate::{
    traits::{Image, MutableImage},
    Image2d,
};

pub(crate) trait ImagersReader {
    fn read(&mut self) -> Result<(), String>;
    fn get_img(&mut self) -> Box<dyn Any>;
}

pub(crate) struct LumaImagersReader<V: Primitive + Default + Clone, Cont: Deref<Target = [V]>> {
    rs_img: ImageBuffer<Luma<V>, Cont>,
    olp_img: Box<dyn Any>,
}

impl<V, Cont> LumaImagersReader<V, Cont>
where
    V: Primitive + Default + Copy + 'static,
    Cont: Deref<Target = [V]>,
{
    pub(crate) fn new(rs_img: ImageBuffer<Luma<V>, Cont>) -> LumaImagersReader<V, Cont> {
        LumaImagersReader {
            rs_img: rs_img,
            olp_img: Box::<Image2d<V>>::new(Image2d::<V>::default()),
        }
    }
}

impl<V, Cont> ImagersReader for LumaImagersReader<V, Cont>
where
    V: Primitive + Default + Copy + 'static,
    Cont: Deref<Target = [V]>,
{
    fn read(&mut self) -> Result<(), String> {
        let img = match self.olp_img.downcast_mut::<Image2d<V>>() {
            Some(v) => v,
            None => return Err(String::from("Invalid conversion (should not happen)")),
        };
        let (width, height) = self.rs_img.dimensions();
        img.resize(width as i32, height as i32);
        for p in img.domain() {
            *img.at_point_mut(&p) = self.rs_img.get_pixel(p.x() as u32, p.y() as u32).0[0];
        }
        Ok(())
    }

    fn get_img(&mut self) -> Box<dyn Any> {
        let val = std::mem::replace(
            &mut self.olp_img,
            Box::<Image2d<V>>::new(Default::default()),
        );
        val
    }
}
