//! # GFX Types - Core Graphics Types for RedstoneOS
//!
//! Esta biblioteca define os tipos gráficos fundamentais compartilhados entre:
//! - **Kernel (Forge)**: Driver de display, syscalls de framebuffer
//! - **Compositor (Firefly)**: Window management, composição
//! - **SDK (Redpowder)**: API de alto nível para aplicações
//!
//! ## Módulos
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | [`geometry`] | Primitivas geométricas (Point, Size, Rect, etc.) |
//! | [`color`] | Sistema de cores e formatos de pixel |
//! | [`buffer`] | Buffers de pixels e descritores |
//! | [`display`] | Informações de display e output |
//! | [`window`] | Flags e tipos de janela |
//! | [`damage`] | Damage tracking para composição |
//! | [`render`] | Comandos e operações de renderização |
//! | [`input`] | Tipos de cursor e input |
//! | [`text`] | Tipografia básica |
//!
//! ## Exemplo
//!
//! ```rust
//! use gfx_types::prelude::*;
//!
//! let rect = Rect::new(10, 20, 100, 50);
//! let color = Color::rgb(255, 128, 0);
//! let point = Point::new(50, 30);
//!
//! assert!(rect.contains_point(point));
//! ```

#![no_std]
#![allow(dead_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

// =============================================================================
// MODULES
// =============================================================================

pub mod buffer;
pub mod color;
pub mod damage;
pub mod display;
pub mod geometry;
pub mod input;
pub mod render;
pub mod text;
pub mod window;

// =============================================================================
// PRELUDE - Most common types
// =============================================================================

/// Prelude contendo os tipos mais usados.
pub mod prelude {
    // Geometry
    pub use crate::geometry::{
        Circle, Ellipse, Insets, Line, LineF, Point, PointF, Rect, RectF, RoundedRect, Size, SizeF,
        Transform2D,
    };

    // Color
    pub use crate::color::{AlphaMode, BlendMode, Color, ColorF, PixelFormat};

    // Buffer
    pub use crate::buffer::{BufferDescriptor, BufferHandle, BufferUsage};

    // Display
    pub use crate::display::DisplayInfo;

    // Window
    pub use crate::window::{LayerType, WindowFlags, WindowState, WindowType};

    // Damage
    pub use crate::damage::DamageRegion;

    // Text
    pub use crate::text::{FontStyle, FontWeight, TextAlign};
}

// =============================================================================
// RE-EXPORTS (Backward Compatibility)
// =============================================================================

// Geometry
pub use geometry::{
    Circle, Ellipse, Insets, Line, LineF, Point, PointF, Rect, RectF, RoundedRect, Size, SizeF,
    Transform2D,
};

// Color
pub use color::{AlphaMode, BlendMode, Color, ColorF, PixelFormat};

// Buffer
pub use buffer::{BufferDescriptor, BufferHandle, BufferRegion, BufferUsage};

// Display
pub use display::{DisplayInfo, DisplayMode, VsyncMode};

// Window
pub use window::{LayerType, WindowEffects, WindowFlags, WindowState, WindowType};

// Damage
pub use damage::DamageRegion;

// Input
pub use input::{CursorHotspot, CursorType};

// Text
pub use text::{FontStyle, FontWeight, GlyphId, GlyphMetrics, TextAlign, TextBaseline};
