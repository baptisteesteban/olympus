mod structural_operation;
mod tree;
mod union_find;

pub use structural_operation::{closing, dilation, erosion, opening};
pub use tree::Tree;
pub use union_find::*;
