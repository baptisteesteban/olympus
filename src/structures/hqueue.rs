use std::collections::VecDeque;

use crate::Point2d;

pub struct HierarchicalQueue {
    queues: Vec<VecDeque<Point2d>>,
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
        self.queues.get_mut(v as usize).unwrap().push_back(p);
        self.size += 1;
        if self.cur > (v as usize) {
            self.cur = v as usize;
        }
    }

    pub fn pop(&mut self) -> Result<(u8, Point2d), &str> {
        if self.empty() {
            return Err("Empty queue");
        }

        // Get the point
        let p = self.queues.get_mut(self.cur).unwrap().pop_front().unwrap();
        let v = self.cur as u8;
        self.size -= 1;
        self.update_current();

        Ok((v, p))
    }

    fn update_current(&mut self) {
        if self.queues.get(self.cur).unwrap().len() == 0 {
            if self.size > 0 {
                while self.queues.get(self.cur).unwrap().len() == 0 {
                    self.cur += 1;
                }
            } else {
                self.cur = 256;
            }
        }
    }

    pub fn pop_nearest(&mut self, k: u8) -> Result<(u8, Point2d), &str> {
        if self.empty() {
            return Err("Empty queue");
        }
        let v = self.find_nearest(k);
        let p = self
            .queues
            .get_mut(v as usize)
            .unwrap()
            .pop_front()
            .unwrap();
        self.size -= 1;
        self.update_current();

        Ok((v, p))
    }

    fn find_nearest(&self, k: u8) -> u8 {
        if self.queues.get(k as usize).unwrap().len() > 0 {
            k
        } else {
            let mut d: i16 = 1;
            let mut res = 0;
            while k as i16 + d < 256 || k as i16 - d >= 0 {
                if k as i16 - d >= 0 && self.queues.get((k as i16 - d) as usize).unwrap().len() > 0
                {
                    res = k as i16 - d;
                    break;
                }
                if k as i16 + d < 256 && self.queues.get((k as i16 + d) as usize).unwrap().len() > 0
                {
                    res = k as i16 + d;
                    break;
                }
                d += 1;
            }
            res as u8
        }
    }
}
