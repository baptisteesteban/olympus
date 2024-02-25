use image::{EncodableLayout, ImageBuffer, Luma, PixelWithColorType, Primitive};

use crate::Image2d;

mod internals {
    use std::ops::DerefMut;

    use crate::{traits::Image, Image2d};
    use image::{ImageBuffer, Luma, Primitive};

    pub fn write_luma<V: Clone + Default + Primitive, Cont: DerefMut<Target = [V]>>(
        img: &Image2d<V>,
        buffer: &mut ImageBuffer<Luma<V>, Cont>,
    ) {
        for p in img.domain() {
            *buffer.get_pixel_mut(p.x() as u32, p.y() as u32) = Luma([*img.at_point(&p)]);
        }
    }
}

pub fn imsave<T>(img: &Image2d<T>, filename: &str) -> Result<(), String>
where
    T: Default + Primitive,
    [T]: EncodableLayout,
    Luma<T>: PixelWithColorType<Subpixel = T>,
{
    let mut rimg = ImageBuffer::<Luma<T>, Vec<T>>::new(img.width() as u32, img.height() as u32);
    internals::write_luma(&img, &mut rimg);
    if let Err(e) = rimg.save(filename) {
        return Err(format!("Unable to write the image {}: {}", filename, e));
    }
    Ok(())
}
