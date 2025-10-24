use std::cmp::Ordering;

use crate::{Domain, ImageMut, SizedDomain, UnionFind, Window};

fn local_extrema<I, W, O>(img: &I, nbh: &W, comp: O) -> I::ChangeValue<u16>
where
    I: ImageMut,
    I::Value: Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Eq + Ord + Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
    O: Fn(&I::Value, &I::Value) -> Ordering,
{
    // Constants
    const UNSEEN: u16 = u16::MAX;

    // Data initialization
    let mut res = img.imchvalue_with_value(UNSEEN);
    let mut uf =
        UnionFind::new(unsafe { res.imchvalue_uninitialized::<<I::Domain as Domain>::Point>() });

    for p in img.domain().clone() {
        uf.make_set(&p);
        let mut rp = p;

        let mut maybe_an_extrema = true;
        for n in nbh.apply(&p) {
            if !img.domain().has(&n) || res[n] == UNSEEN {
                continue;
            }

            match comp(&img[p], &img[n]) {
                Ordering::Greater => {
                    maybe_an_extrema = false;
                }
                Ordering::Less => {
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
    for p in img.domain().clone() {
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

pub fn local_minima<I, W>(img: &I, nbh: &W) -> I::ChangeValue<u16>
where
    I: ImageMut,
    I::Value: Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Eq + Ord + Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    local_extrema(img, nbh, |v1, v2| v1.cmp(v2))
}

pub fn local_maxima<I, W>(img: &I, nbh: &W) -> I::ChangeValue<u16>
where
    I: ImageMut,
    I::Value: Ord,
    I::Domain: SizedDomain,
    <I::Domain as Domain>::Point: Eq + Ord + Copy,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    local_extrema(img, nbh, |v1, v2| v2.cmp(v1))
}
