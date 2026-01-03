//! # Input Module
//!
//! Tipos de cursor e input gráfico.

mod cursor;
mod touch;

pub use cursor::{CursorHotspot, CursorType};
pub use touch::{GestureType, SwipeDirection, TouchId, TouchPhase, TouchPoint};
