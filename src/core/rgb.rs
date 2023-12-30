use std::{fmt::Display, ops::Add};

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r: r, g: g, b: b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, rhs: Self) -> Self::Output {
        RGB {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RGB;

    #[test]
    fn test_creation() {
        let c = RGB::new(7, 25, 11);
        assert_eq!(c.r(), 7);
        assert_eq!(c.g(), 25);
        assert_eq!(c.b(), 11);
    }

    #[test]
    fn test_display() {
        let c = RGB::new(7, 25, 11);
        let formatted = String::from(format!("{}", c).as_str());
        assert_eq!(formatted, "(7, 25, 11)");
    }
}
