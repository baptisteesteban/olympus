use crate::Image2d;

pub fn imsave(img: &mut Image2d<u8>, filename: &str) -> Result<(), &'static str> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_save_pgm_u8() {
        assert!(false);
    }
}