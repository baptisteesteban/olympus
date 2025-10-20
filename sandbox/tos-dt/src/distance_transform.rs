use olympus::{fill, BoundedValueSet, Image2d, Point2d, Window, C4};

use crate::{DistanceHQueue, Range};

#[inline]
fn proj(v: u8, a: u8, b: u8) -> u8 {
    if v < a {
        a
    } else if v > b {
        b
    } else {
        v
    }
}

/// Implementation of the **level lines distance transform** as defined in **The
/// Tree of Shapes as a distance transform: building the ToS on High-Dynamic
/// Range images** from Edwin Carlinet and Baptiste Esteban. This distance
/// transform is only implemented for `u8` images but it is planned to be
/// extended to other data types.
pub fn distance_transform(img: &Image2d<Range<u8>>) -> (Image2d<u32>, Image2d<u8>) {
    let mut q = DistanceHQueue::default();
    let mut dt = unsafe { Image2d::<u32>::new_uninitialized(img.width(), img.height()).unwrap() };
    let mut f = unsafe { Image2d::<u8>::new_uninitialized(img.width(), img.height()).unwrap() };
    let unvisited = u32::sup();
    fill(&mut dt, unvisited);

    let domain = img.domain();

    // Initialization
    let pinf = Point2d::new(0, 0);
    q.push(pinf, 0);
    *dt.at_mut(&pinf).unwrap() = 0;
    *f.at_mut(&pinf).unwrap() = img.at(&pinf).unwrap().l;

    // Propagation
    while !q.empty() {
        let (p, dist) = q.pop().unwrap();
        let current = *f.at(&p).unwrap();
        for n in C4.apply(&p) {
            if !domain.has(&n) {
                continue;
            }
            if *dt.at(&n).unwrap() != unvisited {
                continue;
            }
            let r = img.at(&n).unwrap();
            let projected = proj(current, r.l, r.h);
            let diff = projected.abs_diff(current);
            *dt.at_mut(&n).unwrap() = (dist + diff as usize) as u32;
            *f.at_mut(&n).unwrap() = projected;
            q.push(n, diff);
        }
    }

    (dt, f)
}
