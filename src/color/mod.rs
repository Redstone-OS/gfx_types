//! # Color Module
//!
//! Sistema de cores e formatos de pixel.

mod blend;
mod color;
mod format;
mod palette;
mod space;

pub use blend::{AlphaMode, BlendMode};
pub use color::{Color, ColorF};
pub use format::PixelFormat;
pub use palette::{Palette, CATPPUCCIN_LATTE, CATPPUCCIN_MOCHA, DRACULA, NORD, REDSTONE_DEFAULT};
pub use space::{apply_gamma, linear_to_srgb, remove_gamma, srgb_to_linear, ColorSpace};
