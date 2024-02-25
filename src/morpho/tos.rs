use crate::{algorithms::transform, Image2d, Point2d};

mod internal {
    use std::fmt::Display;

    use num::Bounded;

    use crate::{
        accu::{Accumulator, InfAccumulator, SupAccumulator},
        morpho::HierarchicalQueue,
        traits::{Domain, Image, ImageFromDomain, MutableImage, SizedDomain, Window},
        Box2d, Image2d, Point2d, C4,
    };

    #[derive(Clone, Copy, Default)]
    pub(crate) struct Range<T>
    where
        T: Ord + Display + Copy + Default,
    {
        min: T,
        max: T,
    }

    impl<T> Range<T>
    where
        T: Ord + Display + Copy + Default,
    {
        pub(crate) fn new(min: T, max: T) -> Range<T> {
            Range { min: min, max: max }
        }

        pub(crate) fn set(&mut self, min: T, max: T) -> Result<(), String> {
            if min > max {
                Err(format!("Invalid value ({} > {})", min, max))
            } else {
                self.min = min;
                self.max = max;
                Ok(())
            }
        }

        pub(crate) fn min(&self) -> T {
            self.min
        }

        pub(crate) fn max(&self) -> T {
            self.max
        }
    }

    impl<T> Display for Range<T>
    where
        T: Ord + Display + Copy + Default,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{} - {}]", self.min, self.max)
        }
    }

    pub(crate) fn max_interpolation<T>(img: &Image2d<T>) -> Image2d<T>
    where
        T: Default + Copy + Ord + Bounded,
    {
        let domain = img.domain();
        let new_domain = Box2d::new(domain.width() * 2 - 1, domain.height() * 2 - 1);
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

    pub(crate) fn immerse<T>(img: &Image2d<T>) -> Image2d<Range<T>>
    where
        T: Ord + Display + Copy + Default + Bounded,
    {
        let domain = img.domain();
        let new_domain = Box2d::new(domain.width() * 2 - 1, domain.height() * 2 - 1);
        let mut res = Image2d::<Range<T>>::new_from_domain(&new_domain);

        // Immerse original pixels (2F)
        for y in (0..res.height()).step_by(2) {
            for x in (0..res.width()).step_by(2) {
                let val = *img.at(x / 2, y / 2);
                *res.at_mut(x, y) = Range::new(val, val);
            }
        }

        // Immerse on horizontal pixels (1F)
        for y in (0..res.height()).step_by(2) {
            for x in (1..res.width()).step_by(2) {
                let min = std::cmp::min(res.at(x - 1, y).min(), res.at(x + 1, y).min());
                let max = std::cmp::max(res.at(x - 1, y).max(), res.at(x + 1, y).max());
                res.at_mut(x, y).set(min, max).unwrap();
            }
        }

        // Immerse on vertical pixels (1F)
        for y in (1..res.height()).step_by(2) {
            for x in (0..res.width()).step_by(2) {
                let min = std::cmp::min(res.at(x, y - 1).min(), res.at(x, y + 1).min());
                let max = std::cmp::max(res.at(x, y - 1).max(), res.at(x, y + 1).max());
                res.at_mut(x, y).set(min, max).unwrap()
            }
        }

        // Immerse on cross pixels (0F)
        let nbh = C4::new();
        let mut sup = SupAccumulator::new();
        let mut inf = InfAccumulator::new();
        for y in (1..res.height()).step_by(2) {
            for x in (1..res.width()).step_by(2) {
                sup.init();
                inf.init();
                let p = Point2d::new(x, y);
                for q in nbh.apply(&p) {
                    sup.take(res.at_point(&q).max());
                    inf.take(res.at_point(&q).min())
                }
                res.at_point_mut(&p)
                    .set(inf.result(), sup.result())
                    .unwrap();
            }
        }

        res
    }

    pub(crate) fn median_on_border<T>(img: &Image2d<T>) -> Image2d<T>
    where
        T: Copy + Default + Ord,
    {
        let domain = img.domain();
        let new_domain = Box2d::new(domain.width() + 2, domain.height() + 2);
        let mut res = Image2d::<T>::new_from_domain(&new_domain);

        for x in 0..img.width() {
            for y in 0..img.height() {
                *res.at_mut(x + 1, y + 1) = *img.at(x, y);
            }
        }

        let mut border_values =
            Vec::<T>::with_capacity((2 * (img.width() + img.height()) - 4) as usize);
        for x in 0..img.width() {
            border_values.push(*img.at(x, 0));
            border_values.push(*img.at(x, img.height() - 1));
        }
        for y in 1..(img.height() - 1) {
            border_values.push(*img.at(0, y));
            border_values.push(*img.at(img.width() - 1, y));
        }
        border_values.sort();

        let median = border_values.get(border_values.len() / 2).unwrap();
        for x in 0..res.width() {
            *res.at_mut(x, 0) = *median;
            *res.at_mut(x, res.height() - 1) = *median;
        }
        for y in 1..(res.height() - 1) {
            *res.at_mut(0, y) = *median;
            *res.at_mut(res.width() - 1, y) = *median;
        }

        res
    }

    pub(crate) fn compute_order_map(
        immersed: &Image2d<Range<u8>>,
        start_point: Point2d,
        start_value: u8,
    ) -> Image2d<i32> {
        let mut queue = HierarchicalQueue::new();
        let mut ord = Image2d::<i32>::new_from_domain_with_value(&immersed.domain(), -1);

        let mut lmd_old = start_value;
        queue.push(lmd_old, start_point);
        let mut d = 0;

        let nbh = C4::new();
        let domain = immersed.domain();

        while !queue.empty() {
            let (lmd, p) = queue.pop_nearest(lmd_old).unwrap();
            if queue.size() > domain.size() as usize {
                panic!("The impossible has happened");
            }
            if lmd_old != lmd {
                d += 1;
            }
            *ord.at_point_mut(&p) = d;
            for n in nbh.apply(&p) {
                if !domain.has(&n) {
                    continue;
                }
                if *ord.at_point(&n) < 0 {
                    let c = *immersed.at_point(&n);
                    if lmd < c.min() {
                        queue.push(c.min(), n);
                    } else if lmd > c.max() {
                        queue.push(c.max(), n)
                    } else {
                        queue.push(lmd, n);
                    }
                    *ord.at_point_mut(&n) = 0;
                }
            }
            lmd_old = lmd;
        }

        ord
    }
}

pub fn tos(img: &Image2d<u8>) -> Image2d<u16> {
    let bordered = internal::median_on_border(img);
    let interp = internal::max_interpolation(&bordered);
    let k = internal::immerse(&img);
    let ordered = internal::compute_order_map(&k, Point2d::new(0, 0), *interp.at(0, 0));
    let casted_ordered = transform(&ordered, |v| *v as u16);
    casted_ordered
}
