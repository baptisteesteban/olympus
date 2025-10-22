use crate::{fill, Domain, Image, Image2d, ImageMut, Point2d, UnionFind, Window};

pub fn local_minima<V, W>(img: &Image2d<V>, nbh: &W) -> Image2d<u8>
where
    V: Ord,
    W: Window<Point = Point2d>,
{
    // Resulting labelisation
    let mut res = img.imchvalue::<u8>();
    fill(&mut res, 0);

    let mut uf = UnionFind::new(img.imchvalue::<Point2d>());
    for p in *img.domain() {
        uf.make_set(&p);
    }

    for p in *img.domain() {
        let mut is_a_minimum = true;
        for n in nbh.apply(&p) {
            if !img.domain().has(&n) {
                continue;
            }

            if img[p] > img[n] {
                is_a_minimum = false;
            } else if img[n] > img[p] {
                let r = uf.find(&n);
                res[r] = 0;
            } else {
                let r1 = uf.find(&p);
                let r2 = uf.find(&n);
                uf.union(&r1, &r2);
            }
        }
        if is_a_minimum {
            let r = uf.find(&p);
            res[r] = 255;
        }
    }

    for p in *img.domain() {
        let r = uf.find(&p);
        if res[r] == 255 {
            res[p] = 255;
        }
    }

    res
}
