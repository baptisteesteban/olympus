mod colormap;
mod colormaps;
mod normalizer;
mod svg_engine;

pub mod svg;

pub use colormap::apply_colormap;
pub use colormaps::*;
pub use normalizer::Normalizer;
pub use svg_engine::{save_svg, SVGDrawer};
