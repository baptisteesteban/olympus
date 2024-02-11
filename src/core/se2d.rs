use crate::Point2d;

pub struct StructuringElement2d {
    offsets: Vec<Point2d>,
}

impl StructuringElement2d {
    pub fn new(width: i32, height: i32, mask: Vec<bool>) -> Result<StructuringElement2d, String> {
        if width % 2 == 0 || height % 2 == 0 {
            return Err(String::from("Structuring element must have odd dimensions"));
        }

        let size = (width * height) as usize;
        if size != mask.len() {
            return Err(format!(
                "Input size different from mask size (w*h: {}, mask: {})",
                size,
                mask.len()
            ));
        }

        let mut offsets = Vec::<Point2d>::with_capacity(size);
        let center = Point2d::new(width / 2, height / 2);

        for i in 0..mask.len() {
            if let Some(e) = mask.get(i) {
                if *e {
                    let cast_i = i as i32;
                    let cur_x = (cast_i % width) as i32;
                    let cur_y = (cast_i / width) as i32;
                    offsets.push(Point2d::new(cur_x - center.x(), cur_y - center.y()))
                }
            }
        }
        Ok(StructuringElement2d { offsets: offsets })
    }

    pub fn apply(&self, p: &Point2d) -> Vec<Point2d> {
        (&self.offsets).into_iter().map(|pse| *pse + *p).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Point2d;

    use super::StructuringElement2d;

    #[test]
    fn test_simple_se() {
        #[allow(non_snake_case)]
        let REF: [Point2d; 9] = [
            Point2d::new(-1, -1),
            Point2d::new(0, -1),
            Point2d::new(1, -1),
            Point2d::new(-1, 0),
            Point2d::new(0, 0),
            Point2d::new(1, 0),
            Point2d::new(-1, 1),
            Point2d::new(0, 1),
            Point2d::new(1, 1),
        ];

        let se = StructuringElement2d::new(3, 3, vec![true; 9]).unwrap();
        let mut i = 0;
        for v in se.apply(&Point2d::new(0, 0)) {
            assert_eq!(v, REF[i]);
            i += 1;
        }
    }
}
