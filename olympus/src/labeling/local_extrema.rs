use std::cmp::Ordering;

use crate::{Domain, Image, Image2d, ImageMut, Point2d, UnionFind, Window};

fn local_extrema<V, W, O>(img: &Image2d<V>, nbh: &W, comp: O) -> Image2d<u16>
where
    V: Ord,
    W: Window<Point = Point2d>,
    O: Fn(&V, &V) -> Ordering,
{
    // Constants
    const UNSEEN: u16 = u16::MAX;

    // Data initialization
    let mut res = img.imchvalue_with_value(UNSEEN);
    let mut uf = UnionFind::new(unsafe {
        Image2d::<Point2d>::new_uninitialized(img.width(), img.height()).unwrap()
    });

    for p in *img.domain() {
        uf.make_set(&p);
        let mut rp = p;

        let mut maybe_an_extrema = true;
        for n in nbh.apply(&p) {
            if !img.domain().has(&n) || res[n] == UNSEEN {
                continue;
            }

            match comp(&img[p], &img[n]) {
                Ordering::Less => {
                    maybe_an_extrema = false;
                }
                Ordering::Greater => {
                    let rn = uf.find(&n);
                    res[rn] = 0;
                }
                Ordering::Equal => {
                    let rn = uf.find(&n);
                    if rp != rn {
                        let (r_min, r_max) = if rp < rn { (rp, rn) } else { (rn, rp) };
                        uf.union(&r_min, &r_max);
                        maybe_an_extrema = maybe_an_extrema && (res[rn] > 0);
                        rp = r_min;
                    }
                }
            }
        }
        (res[rp], res[p]) = if maybe_an_extrema { (1, 1) } else { (0, 0) };
        uf.union(&rp, &p);
    }

    let mut nlabel = 0;
    for p in *img.domain() {
        if res[p] > 0 {
            let r = uf.find(&p);
            if r == p {
                nlabel += 1;
                res[p] = nlabel;
            } else {
                res[p] = res[r];
            }
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
