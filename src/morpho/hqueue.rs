use crate::Point2d;

pub struct HierarchicalQueue {
    queues: Vec<Vec<Point2d>>,
    cur: usize,
    size: usize,
}

impl HierarchicalQueue {
    pub fn new() -> HierarchicalQueue {
        HierarchicalQueue {
            queues: vec![Default::default(); 256],
            cur: 256,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn cur(&self) -> usize {
        self.cur
    }

    pub fn push(&mut self, v: u8, p: Point2d) {
        self.queues.get_mut(v as usize).unwrap().push(p);
        self.size += 1;
        if self.cur > (v as usize) {
            self.cur = v as usize;
        }
    }

    pub fn pop(&mut self) -> Result<(u8, Point2d), &str> {
        if self.empty() {
            return Err("Empty queue");
        }
        let p = self.queues.get_mut(self.cur).unwrap().remove(0);
        let v = self.cur as u8;
        self.size -= 1;
        if self.queues.get(self.cur).unwrap().len() == 0 {
            if self.size > 0 {
                while self.queues.get(self.cur).unwrap().len() == 0 {
                    self.cur += 1;
                }
            } else {
                self.cur = 256;
            }
        }
        Ok((v, p))
    }
}
