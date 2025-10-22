use std::cmp::Ordering;

use crate::{fill, Domain, Image, Image2d, ImageMut, Point2d, UnionFind, Window};

fn local_extrema<V, W, O>(img: &Image2d<V>, nbh: &W, comp: O) -> Image2d<u16>
where
    V: Ord,
    W: Window<Point = Point2d>,
    O: Fn(&V, &V) -> Ordering,
{
    // Resulting labelisation
    let mut res = img.imchvalue::<u16>();
    fill(&mut res, 0);

    let mut uf = UnionFind::new(img.imchvalue::<Point2d>());
    for p in *img.domain() {
        uf.make_set(&p);
    }

    for p in *img.domain() {
        let mut is_an_extrema = true;
        for n in nbh.apply(&p) {
            if !img.domain().has(&n) {
                continue;
            }

            match comp(&img[p], &img[n]) {
                Ordering::Greater => is_an_extrema = false,
                Ordering::Less => {
                    let r = uf.find(&n);
                    res[r] = 0;
                }
                _ => {
                    let r1 = uf.find(&p);
                    let r2 = uf.find(&n);
                    uf.union(&r1, &r2);
                }
            }
        }
        if is_an_extrema {
            let r = uf.find(&p);
            res[r] = 1;
        }
    }

    let mut num_label = 1;
    for p in *img.domain() {
        let r = uf.find(&p);
        if res[r] > 0 {
            if p == r {
                res[r] = num_label;
                num_label += 1;
            }
            res[p] = res[r];
        }
    }

    res
}

pub fn local_minima<V, W>(img: &Image2d<V>, nbh: &W) -> Image2d<u16>
where
    V: Ord,
    W: Window<Point = Point2d>,
{
    local_extrema(img, nbh, |v1, v2| v1.cmp(v2))
}

pub fn local_maxima<V, W>(img: &Image2d<V>, nbh: &W) -> Image2d<u16>
where
    V: Ord,
    W: Window<Point = Point2d>,
{
    local_extrema(img, nbh, |v1, v2| v2.cmp(v1))
}
