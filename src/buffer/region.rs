//! # Buffer Region
//!
//! Sub-regiões de buffers.

use crate::geometry::Rect;

/// Sub-região de um buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BufferRegion {
    /// Offset X dentro do buffer.
    pub x: u32,
    /// Offset Y dentro do buffer.
    pub y: u32,
    /// Largura da região.
    pub width: u32,
    /// Altura da região.
    pub height: u32,
}

impl BufferRegion {
    /// Cria nova região.
    #[inline]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Região vazia.
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    /// Cria região do buffer inteiro.
    #[inline]
    pub const fn full(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// Verifica se está vazia.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Área em pixels.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Converte para Rect (signed).
    #[inline]
    pub const fn to_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }

    /// Cria de Rect (clampa valores negativos).
    #[inline]
    pub fn from_rect(rect: Rect) -> Self {
        Self {
            x: rect.x.max(0) as u32,
            y: rect.y.max(0) as u32,
            width: rect.width,
            height: rect.height,
        }
    }

    /// Verifica se contém coordenadas.
    #[inline]
    pub const fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

impl From<Rect> for BufferRegion {
    #[inline]
    fn from(r: Rect) -> Self {
        Self::from_rect(r)
    }
}

impl From<BufferRegion> for Rect {
    #[inline]
    fn from(r: BufferRegion) -> Self {
        r.to_rect()
    }
}
