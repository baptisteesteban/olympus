use crate::{
    labeling::local_minima, Domain, HistogramHQueue, Image, ImageMut, SizedDomain, Window,
};

pub fn watershed_partition_from_markers<I, W>(
    img: &I,
    markers: &I::ChangeValue<u16>,
    nbh: &W,
) -> I::ChangeValue<u16>
where
    I: ImageMut<Value = u8>,
    I::Domain: SizedDomain,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    const UNSEEN: u16 = u16::MAX;

    // Initialization
    let mut res = unsafe { img.imchvalue_uninitialized() };
    for p in markers.domain().clone() {
        res[p] = if markers[p] > 0 { markers[p] } else { UNSEEN };
    }
    let mut q = HistogramHQueue::new(img);

    // Add markers border
    for p in res.domain().clone() {
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

pub fn watershed_partition<I, W>(img: &I, nbh: &W) -> I::ChangeValue<u16>
where
    I: ImageMut<Value = u8>,
    I::Domain: SizedDomain,
    W: Window<Point = <I::Domain as Domain>::Point>,
{
    let markers = local_minima(img, nbh);
    watershed_partition_from_markers(img, &markers, nbh)
}
