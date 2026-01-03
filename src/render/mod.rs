//! # Render Module
//!
//! Comandos e operações de renderização.

mod clip;
mod command;
mod pipeline;

pub use clip::{ClipOp, ClipRect};
pub use command::{BlitParams, FillParams, RenderOp};
pub use pipeline::{InterpolationQuality, PipelineState, RasterOp};
