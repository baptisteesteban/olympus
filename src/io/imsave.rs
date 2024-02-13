use image::{ImageBuffer, Luma};

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

pub fn imsave(img: &Image2d<u8>, filename: &str) -> Result<(), String> {
    let mut rimg = ImageBuffer::<Luma<u8>, Vec<u8>>::new(img.width() as u32, img.height() as u32);
    internals::write_luma(&img, &mut rimg);
    if let Err(e) = rimg.save(filename) {
        return Err(format!("Unable to write the image {}: {}", filename, e));
    }
    Ok(())
}
