use crate::Point2d;

/// Implementation of a Hierarchical Queue priority queue.
pub struct HQueue {
    queues: [Vec<Point2d>; 256],
    cur: usize,
    size: usize,
}

impl Default for HQueue {
    fn default() -> Self {
        HQueue {
            queues: std::array::from_fn(|_| Default::default()),
            cur: 0,
            size: 0,
        }
    }
}

impl HQueue {
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, p: Point2d, v: u8) {
        let cv = v as usize;
        if cv > self.cur {
            self.cur = cv;
        }
        self.queues[cv].push(p);
        self.size += 1;
    }

    pub fn pop(&mut self) -> (Point2d, u8) {
        debug_assert!(!self.is_empty());
        if self.queues[self.cur].is_empty() {
            self.update_cur()
        }
        let v = self.cur as u8;
        let p = self.queues[self.cur].pop().unwrap();
        self.size -= 1;
        (p, v)
    }

    pub fn top(&mut self) -> (Point2d, u8) {
        debug_assert!(!self.is_empty());
        if self.queues[self.cur].is_empty() {
            self.update_cur()
        }
        let v = self.cur as u8;
        let p = self.queues[self.cur].last().unwrap();
        (*p, v)
    }

    fn update_cur(&mut self) {
        while self.queues[self.cur].is_empty() {
            self.cur -= 1;
        }
    }
}
