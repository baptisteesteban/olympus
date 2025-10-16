use olympus::Point2d;

pub struct DistanceHQueue {
    queues: [Vec<Point2d>; 256],
    dist: usize,
    size: usize,
}

impl Default for DistanceHQueue {
    fn default() -> Self {
        DistanceHQueue {
            queues: std::array::from_fn(|_| Default::default()),
            dist: 0,
            size: 0,
        }
    }
}

impl DistanceHQueue {
    pub fn push(&mut self, p: Point2d, d: u8) {
        self.queues[(self.dist + d as usize) % 256].push(p);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<(Point2d, usize)> {
        if self.empty() {
            return None;
        }
        self.advance();
        let p = self.queues[self.dist % 256].pop().unwrap();
        self.size -= 1;
        Some((p, self.dist))
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn dist(&self) -> usize {
        self.dist
    }

    fn advance(&mut self) {
        while self.queues[self.dist % 256].is_empty() {
            self.dist += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use olympus::Point2d;

    use crate::DistanceHQueue;

    #[test]
    fn test_distance_hqueue() {
        let mut q = DistanceHQueue::default();
        q.push(Point2d::new(0, 0), 0);
        q.push(Point2d::new(0, 1), 50);
        q.push(Point2d::new(0, 2), 220);

        const REF_POINTS: [Point2d; 3] =
            [Point2d::new(0, 0), Point2d::new(0, 1), Point2d::new(0, 2)];
        const REF_DIST: [usize; 3] = [0, 50, 220];
        for (ref_p, ref_d) in std::iter::zip(REF_POINTS, REF_DIST) {
            let (p, d) = q.pop().unwrap();
            assert_eq!(p, ref_p);
            assert_eq!(d, ref_d);
        }
        assert!(q.empty());
        assert_eq!(q.dist(), 220);
    }
}
