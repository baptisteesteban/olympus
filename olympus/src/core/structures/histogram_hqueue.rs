use std::cmp::min;

use crate::{Image, Image2d, NDBuffer, Point2d, SizedDomain};

#[derive(Default)]
struct Meta {
    start: usize,
    end: usize,
    capacity: usize,
}

impl Meta {
    #[inline]
    fn size(&self) -> usize {
        self.end - self.start // End is outside
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub struct HistogramHQueue {
    data: NDBuffer<Point2d>,
    meta: [Meta; 256],
    size: usize,
    cur: usize,
}

impl HistogramHQueue {
    pub fn new(img: &Image2d<u8>) -> HistogramHQueue {
        // Compute histogram
        let mut h: [usize; 256] = std::array::from_fn(|_| 0);
        for p in *img.domain() {
            h[img[p] as usize] += 1;
        }

        // Compute metadata
        let mut meta: [Meta; 256] = std::array::from_fn(|_| Default::default());
        meta[0].start = 0;
        meta[0].end = 0;
        meta[0].capacity = h[0];
        for i in 1..256 {
            meta[i].start = meta[i - 1].start + h[i - 1];
            meta[i].end = meta[i].start;
            meta[i].capacity = h[i];
        }

        // Return hqueue
        HistogramHQueue {
            data: NDBuffer::new_with_capacity(img.domain().size()),
            meta,
            size: 0,
            cur: 256,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, p: Point2d, v: u8) {
        let vi = v as usize;
        debug_assert_ne!(self.meta[vi].size(), self.meta[vi].capacity);
        self.data[self.meta[vi].end] = p;
        self.meta[vi].end += 1;
        self.cur = min(self.cur, vi);
        self.size += 1;
    }

    pub fn pop(&mut self) -> (Point2d, u8) {
        debug_assert!(!self.is_empty());
        self.update_cur();
        let res_p = self.data[self.meta[self.cur].start];
        self.meta[self.cur].start += 1;
        self.size -= 1;
        (res_p, self.cur as u8)
    }

    fn update_cur(&mut self) {
        debug_assert!(!self.is_empty());
        while self.cur < 256 && self.meta[self.cur].is_empty() {
            self.cur += 1;
        }
    }
}
