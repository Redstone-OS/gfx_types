//! # Window Module
//!
//! Tipos relacionados a janelas e superfícies.

mod effects;
mod flags;
mod layer;
mod state;
mod surface;

pub use effects::{BlurParams, BlurType, OpacityParams, ShadowParams, WindowEffects};
pub use flags::WindowFlags;
pub use layer::LayerType;
pub use state::{ResizeEdge, WindowState, WindowType};
pub use surface::{BufferMode, SurfaceCommit, SurfaceConfig, SurfaceId, SurfaceType};
