use std::fmt::Display;

/// Simple implementation of a range defined by `[l - h]`.
#[derive(Debug, Clone)]
pub struct Range<V>
where
    V: Ord,
{
    pub l: V,
    pub h: V,
}

impl<V> Range<V>
where
    V: Ord + Copy,
{
    pub fn new(l: V, h: V) -> Result<Range<V>, String> {
        if l > h {
            return Err("Lower value must be greater than upper value".to_string());
        }
        Ok(Range { l, h })
    }

    pub fn new_single_value(v: V) -> Range<V> {
        Range { l: v, h: v }
    }
}

impl<V> Default for Range<V>
where
    V: Ord + Copy + Default,
{
    fn default() -> Self {
        Range::<V>::new_single_value(Default::default())
    }
}

impl<V> Display for Range<V>
where
    V: Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.l == self.h {
            f.write_str(format!("{{{}}}", self.l).as_str())
        } else {
            f.write_str(format!("[{} - {}]", self.l, self.h).as_str())
        }
    }
}

impl<V> PartialEq for Range<V>
where
    V: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.h == other.h
    }
}

#[cfg(test)]
mod tests {
    use crate::Range;

    #[test]
    fn test_range() {
        let r1 = Range::<u8>::new(5, 5).unwrap();
        let r2 = Range::<u8>::new_single_value(5);
        assert_eq!(r1, r2);

        let r3 = Range::<u8>::new(5, 3);
        assert!(r3.is_err());

        let r4 = Range::new(3, 5).unwrap();
        let fmt_single = format!("{}", r1);
        let fmt_double = format!("{}", r4);

        assert_eq!(fmt_single, "{5}");
        assert_eq!(fmt_double, "[3 - 5]");
    }
}
