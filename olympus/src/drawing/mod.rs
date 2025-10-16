mod colormap;
mod normalizer;
mod svg_engine;

pub mod svg;

pub use colormap::{apply_colormap, inferno};
pub use normalizer::Normalizer;
pub use svg_engine::{save_svg, SVGDrawer};
