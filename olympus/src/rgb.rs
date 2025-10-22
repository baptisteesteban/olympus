/// Implementation of the (R, G, B) triple for any type.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Rgb<T> {
        Rgb { r, g, b }
    }
}

impl<T> From<[T; 3]> for Rgb<T>
where
    T: Copy,
{
    fn from(value: [T; 3]) -> Self {
        Rgb {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl<T> Default for Rgb<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
        }
    }
}

/// Specialization of the (R, G, B) triple on 8 bits.
pub type Rgb8 = Rgb<u8>;

impl Rgb<u8> {
    /// Returns the hexadecimal representation of a `Rgb8`` value
    pub fn hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Litteral to represent the `red` color.
pub const RED: Rgb8 = Rgb8 { r: 255, g: 0, b: 0 };
/// Litteral to represent the `green` color.
pub const GREEN: Rgb8 = Rgb8 { r: 0, g: 255, b: 0 };
/// Litteral to represent the `blue` color.
pub const BLUE: Rgb8 = Rgb8 { r: 0, g: 0, b: 255 };
/// Litteral to represent the `black` color.
pub const BLACK: Rgb8 = Rgb8 { r: 0, g: 0, b: 0 };
/// Litteral to represent the `white` color.
pub const WHITE: Rgb8 = Rgb8 {
    r: 255,
    g: 255,
    b: 255,
};
