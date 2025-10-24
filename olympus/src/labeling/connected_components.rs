use crate::{Domain, Image, Image2d, ImageMut, Point2d, UnionFind, Window};

pub fn connected_components<V, W>(img: &Image2d<V>, nbh: &W) -> Image2d<u16>
where
    V: Eq,
    W: Window<Point = Point2d>,
{
    // Constants
    const UNSEEN: u16 = u16::MAX;

    // Data structures
    let mut res = img.imchvalue_with_value::<u16>(UNSEEN);
    let mut uf = UnionFind::new(unsafe { img.imchvalue_uninitialized::<Point2d>() });

    // Connected components
    for p in *img.domain() {
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
    for p in *img.domain() {
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
