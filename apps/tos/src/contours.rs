use std::fmt::Display;

use olympus::{
    accu::{Accumulator, UntakeAccumulator},
    morpho::Tree,
    traits::Image,
    Image2d, Point2d,
};

use crate::{is_1h_face, is_1v_face};

pub fn accumulate_on_contours_points<A, V>(t: &Tree<Image2d<i32>, u8>, _acc: A) -> Vec<V>
where
    A: Accumulator<Input = Point2d, Output = V> + UntakeAccumulator + Clone,
    V: Display,
{
    let mut contours_acc = vec![A::new(); t.num_nodes()];

    contours_acc.into_iter().map(|acc| acc.result()).collect()
}

#[cfg(test)]
mod tests {
    use olympus::{accu::Accumulator, Image2d, Point2d};

    use crate::accumulate_on_contours_points;

    #[test]
    fn test_contours_length() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::from([
                2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 1, 4, 4, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2,
            ]),
        );
        const REF: [usize; 4] = [0, 8, 10, 6];
        let t = olympus::morpho::tos(&img, &Point2d::new(0, 0));
        let attr = accumulate_on_contours_points(&t, olympus::accu::CountAccumulator::new());
        for i in 0..4 {
            assert_eq!(REF[i], *attr.get(i).unwrap());
        }
    }
}
