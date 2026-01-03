//! # Line Types
//!
//! Linhas e segmentos.

use super::{Point, PointF};

/// Segmento de linha entre dois pontos (inteiro).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Line {
    /// Ponto inicial.
    pub start: Point,
    /// Ponto final.
    pub end: Point,
}

impl Line {
    /// Cria nova linha.
    #[inline]
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }

    /// Comprimento ao quadrado.
    #[inline]
    pub const fn length_squared(&self) -> i64 {
        self.start.distance_squared(&self.end)
    }

    /// Comprimento.
    #[inline]
    pub fn length(&self) -> f32 {
        rdsmath::sqrtf(self.length_squared() as f32)
    }

    /// Delta X.
    #[inline]
    pub const fn dx(&self) -> i32 {
        self.end.x - self.start.x
    }

    /// Delta Y.
    #[inline]
    pub const fn dy(&self) -> i32 {
        self.end.y - self.start.y
    }

    /// Ponto médio.
    #[inline]
    pub const fn midpoint(&self) -> Point {
        self.start.midpoint(&self.end)
    }

    /// Verifica se é horizontal.
    #[inline]
    pub const fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// Verifica se é vertical.
    #[inline]
    pub const fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// Verifica se é um ponto (start == end).
    #[inline]
    pub const fn is_point(&self) -> bool {
        self.start.x == self.end.x && self.start.y == self.end.y
    }

    /// Converte para LineF.
    #[inline]
    pub const fn to_float(&self) -> LineF {
        LineF {
            start: self.start.to_float(),
            end: self.end.to_float(),
        }
    }

    /// Inverte a direção.
    #[inline]
    pub const fn reverse(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }
}

/// Segmento de linha entre dois pontos (float).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LineF {
    /// Ponto inicial.
    pub start: PointF,
    /// Ponto final.
    pub end: PointF,
}

impl LineF {
    /// Cria nova linha.
    #[inline]
    pub const fn new(start: PointF, end: PointF) -> Self {
        Self { start, end }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            start: PointF::new(x1, y1),
            end: PointF::new(x2, y2),
        }
    }

    /// Comprimento ao quadrado.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.start.distance_squared(&self.end)
    }

    /// Comprimento.
    #[inline]
    pub fn length(&self) -> f32 {
        self.start.distance(&self.end)
    }

    /// Delta X.
    #[inline]
    pub fn dx(&self) -> f32 {
        self.end.x - self.start.x
    }

    /// Delta Y.
    #[inline]
    pub fn dy(&self) -> f32 {
        self.end.y - self.start.y
    }

    /// Ponto médio.
    #[inline]
    pub fn midpoint(&self) -> PointF {
        self.start.midpoint(&self.end)
    }

    /// Ponto em t (0.0 = start, 1.0 = end).
    #[inline]
    pub fn point_at(&self, t: f32) -> PointF {
        self.start.lerp(&self.end, t)
    }

    /// Direção normalizada.
    #[inline]
    pub fn direction(&self) -> PointF {
        PointF::new(self.dx(), self.dy()).normalize()
    }

    /// Normal da linha (perpendicular).
    #[inline]
    pub fn normal(&self) -> PointF {
        let d = self.direction();
        PointF::new(-d.y, d.x)
    }

    /// Ângulo em radianos.
    #[inline]
    pub fn angle(&self) -> f32 {
        rdsmath::atan2f(self.dy(), self.dx())
    }

    /// Inverte a direção.
    #[inline]
    pub fn reverse(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }

    /// Arredonda para Line inteiro.
    #[inline]
    pub fn round(&self) -> Line {
        Line {
            start: self.start.round(),
            end: self.end.round(),
        }
    }
}

impl From<Line> for LineF {
    #[inline]
    fn from(l: Line) -> Self {
        l.to_float()
    }
}
