pub trait Domain {
    /// The associated point that compose the domain set.
    type Point;

    /// Check if a point `p` belongs to the domain.
    fn has(&self, p: &Self::Point) -> bool;
}

pub trait SizedDomain: Domain + IntoIterator<Item = Self::Point> {
    /// Return the size of the domain.
    fn size(&self) -> usize;
}

pub trait ShapeDomain: SizedDomain {
    /// Return the i^{th} dimension of the domain.
    fn shape(&self, i: usize) -> Option<usize>;
}
