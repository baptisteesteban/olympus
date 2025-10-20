/// This traits is used to represent objects that are drawable using the [dot
/// language](https://graphviz.org/doc/info/lang.html).
///
/// *TODO*: Move to drawing
pub trait DotFormat {
    fn to_dot(&self) -> String;
}

pub fn dot_format<D: DotFormat>(g: &D) -> String {
    g.to_dot()
}
