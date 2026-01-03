//! # Size Types
//!
//! Dimensões 2D (largura x altura).

use core::ops::{Add, Mul, Sub};

// =============================================================================
// SIZE (Integer)
// =============================================================================

/// Tamanho 2D (largura x altura).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Cria novo tamanho.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Tamanho zero.
    pub const ZERO: Self = Self {
        width: 0,
        height: 0,
    };

    /// Retorna área total em pixels.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Verifica se o tamanho é vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Retorna o maior lado.
    #[inline]
    pub const fn max_side(&self) -> u32 {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }

    /// Retorna o menor lado.
    #[inline]
    pub const fn min_side(&self) -> u32 {
        if self.width < self.height {
            self.width
        } else {
            self.height
        }
    }

    /// Calcula aspect ratio (width / height).
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            0.0
        } else {
            self.width as f32 / self.height as f32
        }
    }

    /// Escala proporcionalmente para caber em outro tamanho.
    #[inline]
    pub fn fit_inside(&self, container: Size) -> Size {
        if self.is_empty() || container.is_empty() {
            return Size::ZERO;
        }

        let scale_x = container.width as f32 / self.width as f32;
        let scale_y = container.height as f32 / self.height as f32;
        let scale = if scale_x < scale_y { scale_x } else { scale_y };

        Size {
            width: (self.width as f32 * scale) as u32,
            height: (self.height as f32 * scale) as u32,
        }
    }

    /// Escala proporcionalmente para cobrir outro tamanho.
    #[inline]
    pub fn cover(&self, container: Size) -> Size {
        if self.is_empty() || container.is_empty() {
            return Size::ZERO;
        }

        let scale_x = container.width as f32 / self.width as f32;
        let scale_y = container.height as f32 / self.height as f32;
        let scale = if scale_x > scale_y { scale_x } else { scale_y };

        Size {
            width: (self.width as f32 * scale) as u32,
            height: (self.height as f32 * scale) as u32,
        }
    }

    /// Converte para SizeF.
    #[inline]
    pub const fn to_float(&self) -> SizeF {
        SizeF {
            width: self.width as f32,
            height: self.height as f32,
        }
    }

    /// Cria a partir de tupla.
    #[inline]
    pub const fn from_tuple(t: (u32, u32)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }

    /// Converte para tupla.
    #[inline]
    pub const fn to_tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Add for Size {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for Size {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            width: self.width.saturating_sub(rhs.width),
            height: self.height.saturating_sub(rhs.height),
        }
    }
}

impl Mul<u32> for Size {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u32) -> Self {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl From<(u32, u32)> for Size {
    #[inline]
    fn from(t: (u32, u32)) -> Self {
        Self::from_tuple(t)
    }
}

impl From<Size> for (u32, u32) {
    #[inline]
    fn from(s: Size) -> Self {
        s.to_tuple()
    }
}

// =============================================================================
// SIZEF (Floating Point)
// =============================================================================

/// Tamanho 2D com ponto flutuante.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SizeF {
    pub width: f32,
    pub height: f32,
}

impl SizeF {
    /// Cria novo tamanho.
    #[inline]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Tamanho zero.
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };

    /// Área.
    #[inline]
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Verifica se é vazio.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Aspect ratio.
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0.0 {
            0.0
        } else {
            self.width / self.height
        }
    }

    /// Arredonda para Size inteiro.
    #[inline]
    pub fn round(&self) -> Size {
        Size {
            width: rdsmath::roundf(self.width) as u32,
            height: rdsmath::roundf(self.height) as u32,
        }
    }

    /// Trunca para Size inteiro.
    #[inline]
    pub fn floor(&self) -> Size {
        Size {
            width: rdsmath::floorf(self.width) as u32,
            height: rdsmath::floorf(self.height) as u32,
        }
    }

    /// Arredonda para cima.
    #[inline]
    pub fn ceil(&self) -> Size {
        Size {
            width: rdsmath::ceilf(self.width) as u32,
            height: rdsmath::ceilf(self.height) as u32,
        }
    }

    /// Interpolação linear.
    #[inline]
    pub fn lerp(&self, other: &SizeF, t: f32) -> Self {
        Self {
            width: self.width + (other.width - self.width) * t,
            height: self.height + (other.height - self.height) * t,
        }
    }
}

impl Add for SizeF {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for SizeF {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl Mul<f32> for SizeF {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl From<Size> for SizeF {
    #[inline]
    fn from(s: Size) -> Self {
        s.to_float()
    }
}
