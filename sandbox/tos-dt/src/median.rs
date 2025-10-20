/// A simple accumulator to obtain the median value of a set of value. **This is
/// not optimized !!!**
#[derive(Default)]
pub struct MedianAccumulator {
    values: Vec<u8>,
}

impl MedianAccumulator {
    pub fn new_with_capacity(n: usize) -> MedianAccumulator {
        MedianAccumulator {
            values: Vec::<u8>::with_capacity(n),
        }
    }

    pub fn take(&mut self, v: u8) {
        self.values.push(v);
    }

    pub fn result(&mut self) -> u8 {
        self.values.sort();
        let length = self.values.len();
        let i = (length - 1) / 2;
        if length % 2 == 1 {
            *self.values.get(i).unwrap()
        } else {
            let a = *self.values.get(i).unwrap();
            let b = *self.values.get(i + 1).unwrap();
            ((a as u16 + b as u16) / 2) as u8
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MedianAccumulator;

    #[test]
    fn test_two_values() {
        let mut acc = MedianAccumulator::default();
        acc.take(2);
        acc.take(9);
        let res = acc.result();
        assert_eq!(res, 5);
    }

    #[test]
    fn test_three_values() {
        let mut acc = MedianAccumulator::default();
        acc.take(2);
        acc.take(9);
        acc.take(7);
        let res = acc.result();
        assert_eq!(res, 7);
    }
}
