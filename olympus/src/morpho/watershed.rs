use crate::{labeling::local_minima, Domain, HQueue, Image, Image2d, Point2d, Window};

pub fn watershed_partition_from_markers<W>(
    img: &Image2d<u8>,
    markers: &Image2d<u16>,
    nbh: &W,
) -> Image2d<u16>
where
    W: Window<Point = Point2d>,
{
    const UNSEEN: u16 = u16::MAX;

    // Initialization
    let mut res = unsafe { Image2d::<u16>::new_uninitialized(img.width(), img.height()).unwrap() };
    for p in *markers.domain() {
        res[p] = if markers[p] > 0 { markers[p] } else { UNSEEN };
    }
    let mut q = HQueue::default();

    // Add markers border
    for p in *res.domain() {
        if res[p] != UNSEEN {
            for n in nbh.apply(&p) {
                if res.domain().has(&n) && res[n] == UNSEEN {
                    q.push(p, img[p]);
                    break;
                }
            }
        }
    }

    while !q.is_empty() {
        let (p, _) = q.pop();
        let lbl = res[p];
        for n in nbh.apply(&p) {
            if res.domain().has(&n) && res[n] == UNSEEN {
                res[n] = lbl;
                q.push(n, img[n]);
            }
        }
    }

    res
}

pub fn watershed_partition<W>(img: &Image2d<u8>, nbh: &W) -> Image2d<u16>
where
    W: Window<Point = Point2d>,
{
    let markers = local_minima(img, nbh);
    watershed_partition_from_markers(img, &markers, nbh)
}
