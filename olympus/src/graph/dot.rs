pub trait DotFormat {
    fn to_dot(&self) -> String;
}

pub fn dot_format<D: DotFormat>(g: &D) -> String {
    g.to_dot()
}
