use crate::{Domain, ImageMut, SizedDomain, UnionFind, Window};

pub fn connected_components<I, W>(img: &I, nbh: &W) -> I::ChangeValue<u16>
where
    I: ImageMut,
    I::Value: Eq + Copy,
    I::Domain: SizedDomain,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    // Constants
    const UNSEEN: u16 = u16::MAX;

    // Data structures
    let mut res = img.imchvalue_with_value::<u16>(UNSEEN);
    let mut uf =
        UnionFind::new(unsafe { img.imchvalue_uninitialized::<<I::Domain as Domain>::Point>() });

    // Connected components
    for p in img.domain().clone() {
        uf.make_set(&p);
        res[p] = 1;
        for n in nbh.apply(&p) {
            if !img.domain().has(&n) || res[n] == UNSEEN {
                continue;
            }
            if img[p] == img[n] {
                let rp = uf.find(&p);
                let rn = uf.find(&n);
                if rp != rn {
                    let (r_min, r_max) = if rp < rn { (rp, rn) } else { (rn, rp) };
                    uf.union(&r_min, &r_max);
                }
            }
        }
    }

    // Labeling
    let mut nlabel = 0;
    for p in img.domain().clone() {
        let r = uf.find(&p);
        if r == p {
            nlabel += 1;
            res[p] = nlabel;
        } else {
            res[p] = res[r];
        }
    }

    res
}
