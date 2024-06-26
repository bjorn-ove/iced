use crate::core::image;
use crate::core::svg;
use crate::core::{Color, Rectangle};

/// A raster or vector image.
#[derive(Debug, Clone)]
pub enum Image {
    /// A raster image.
    Raster {
        /// The handle of a raster image.
        handle: image::Handle,

        /// The filter method of a raster image.
        filter_method: image::FilterMethod,

        /// The bounds of the image.
        bounds: Rectangle,

        /// Border radius to apply
        border_radius: [f32; 4],
    },
    /// A vector image.
    Vector {
        /// The handle of a vector image.
        handle: svg::Handle,

        /// The [`Color`] filter
        color: Option<Color>,

        /// The bounds of the image.
        bounds: Rectangle,
    },
}
