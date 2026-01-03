//! # Geometry Module
//!
//! Primitivas geométricas para operações gráficas.

mod circle;
mod insets;
mod line;
mod point;
mod polygon;
mod rect;
mod size;
mod transform;

pub use circle::{Circle, Ellipse};
pub use insets::Insets;
pub use line::{Line, LineF};
pub use point::{Point, PointF};
pub use polygon::{FillRule, PathSegment, StaticPolygon, MAX_STATIC_POINTS};
pub use rect::{Rect, RectF, RoundedRect};
pub use size::{Size, SizeF};
pub use transform::Transform2D;
