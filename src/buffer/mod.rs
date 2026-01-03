//! # Buffer Module
//!
//! Buffers de pixels e descritores.

mod descriptor;
mod handle;
mod region;
mod usage;
mod view;

pub use descriptor::BufferDescriptor;
pub use handle::BufferHandle;
pub use region::BufferRegion;
pub use usage::{BufferCapabilities, BufferUsage};
pub use view::{BufferView, BufferViewMut};
