pub trait Domain {
    type Point;

    fn has(&self, p: &Self::Point) -> bool;
}

pub trait SizedDomain: Domain + IntoIterator {
    fn size(&self) -> i32;
}

pub trait ShapedDomain: SizedDomain {
    const N: usize;

    fn shape(&self, i: usize) -> Result<i32, String>;
}
