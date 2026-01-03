//! # Text Module
//!
//! Tipos básicos para tipografia.

mod font;
mod glyph;

pub use font::{FontStyle, FontWeight, TextAlign, TextBaseline};
pub use glyph::{GlyphId, GlyphMetrics};
