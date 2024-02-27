use num::Bounded;
use olympus::{
    accu::{Accumulator, SupAccumulator},
    traits::{Image, ImageFromDomain, MutableImage, Window},
    Box2d, Image2d, Point2d, C4,
};

pub fn max_interpolation<T>(img: &Image2d<T>) -> Image2d<T>
where
    T: Default + Copy + Ord + Bounded,
{
    let domain = img.domain();
    let new_domain = Box2d::new_from_dimension(domain.width() * 2 - 1, domain.height() * 2 - 1);
    let mut res = Image2d::new_from_domain(&new_domain);

    // Copy original pixels
    for y in (0..res.height()).step_by(2) {
        for x in (0..res.width()).step_by(2) {
            *res.at_mut(x, y) = *img.at(x / 2, y / 2);
        }
    }

    // Interpolate on horizontal pixels
    for y in (0..res.height()).step_by(2) {
        for x in (1..res.width()).step_by(2) {
            *res.at_mut(x, y) = std::cmp::max(*res.at(x - 1, y), *res.at(x + 1, y));
        }
    }

    // Interpolate on vertical pixels
    for y in (1..res.height()).step_by(2) {
        for x in (0..res.width()).step_by(2) {
            *res.at_mut(x, y) = std::cmp::max(*res.at(x, y - 1), *res.at(x, y + 1))
        }
    }

    // Interpolate on cross pixels
    let nbh = C4::new();
    let mut sup = SupAccumulator::new();
    for y in (1..res.height()).step_by(2) {
        for x in (1..res.width()).step_by(2) {
            sup.init();
            let p = Point2d::new(x, y);
            for q in nbh.apply(&p) {
                sup.take(*res.at_point(&q));
            }
            *res.at_point_mut(&p) = sup.result();
        }
    }

    res
}
