//! # Point Types
//!
//! Pontos 2D com coordenadas inteiras e de ponto flutuante.

use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use rdsmath::{ceilf, floorf, roundf, sqrtf};

// =============================================================================
// POINT (Integer)
// =============================================================================

/// Ponto 2D com coordenadas signed.
///
/// Pode representar posições offscreen (negativas).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Cria novo ponto.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Ponto na origem (0, 0).
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Adiciona offset ao ponto.
    #[inline]
    pub const fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// Calcula distância ao quadrado para outro ponto.
    #[inline]
    pub const fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        dx * dx + dy * dy
    }

    /// Calcula ponto médio entre dois pontos.
    #[inline]
    pub const fn midpoint(&self, other: &Point) -> Self {
        Self {
            x: (self.x + other.x) / 2,
            y: (self.y + other.y) / 2,
        }
    }

    /// Converte para PointF.
    #[inline]
    pub const fn to_float(&self) -> PointF {
        PointF {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Cria ponto a partir de tupla.
    #[inline]
    pub const fn from_tuple(t: (i32, i32)) -> Self {
        Self { x: t.0, y: t.1 }
    }

    /// Converte para tupla.
    #[inline]
    pub const fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Point {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from(t: (i32, i32)) -> Self {
        Self::from_tuple(t)
    }
}

impl From<Point> for (i32, i32) {
    #[inline]
    fn from(p: Point) -> Self {
        p.to_tuple()
    }
}

// =============================================================================
// POINTF (Floating Point)
// =============================================================================

/// Ponto 2D com coordenadas de ponto flutuante.
///
/// Usado para precisão subpixel em animações e transformações.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    /// Cria novo ponto.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Ponto na origem.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Adiciona offset.
    #[inline]
    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// Calcula distância ao quadrado.
    #[inline]
    pub fn distance_squared(&self, other: &PointF) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Calcula distância euclidiana.
    #[inline]
    pub fn distance(&self, other: &PointF) -> f32 {
        sqrtf(self.distance_squared(other))
    }

    /// Ponto médio.
    #[inline]
    pub fn midpoint(&self, other: &PointF) -> Self {
        Self {
            x: (self.x + other.x) * 0.5,
            y: (self.y + other.y) * 0.5,
        }
    }

    /// Interpolação linear entre dois pontos.
    #[inline]
    pub fn lerp(&self, other: &PointF, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Arredonda para Point inteiro.
    #[inline]
    pub fn round(&self) -> Point {
        Point {
            x: roundf(self.x) as i32,
            y: roundf(self.y) as i32,
        }
    }

    /// Trunca para Point inteiro.
    #[inline]
    pub fn floor(&self) -> Point {
        Point {
            x: floorf(self.x) as i32,
            y: floorf(self.y) as i32,
        }
    }

    /// Arredonda para cima para Point inteiro.
    #[inline]
    pub fn ceil(&self) -> Point {
        Point {
            x: ceilf(self.x) as i32,
            y: ceilf(self.y) as i32,
        }
    }

    /// Normaliza o vetor (comprimento = 1).
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = sqrtf(self.x * self.x + self.y * self.y);
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    /// Produto escalar.
    #[inline]
    pub fn dot(&self, other: &PointF) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for PointF {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for PointF {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for PointF {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for PointF {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<Point> for PointF {
    #[inline]
    fn from(p: Point) -> Self {
        p.to_float()
    }
}
