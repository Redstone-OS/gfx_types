//! # Clipping
//!
//! Tipos para clipping de renderização.

use crate::geometry::Rect;

// =============================================================================
// CLIP RECT
// =============================================================================

/// Retângulo de clipping.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClipRect {
    /// Retângulo de clip.
    pub rect: Rect,
    /// Está ativo?
    pub enabled: bool,
}

impl ClipRect {
    /// Cria novo clip rect.
    #[inline]
    pub const fn new(rect: Rect) -> Self {
        Self {
            rect,
            enabled: true,
        }
    }

    /// Clip desativado.
    pub const NONE: Self = Self {
        rect: Rect::ZERO,
        enabled: false,
    };

    /// Verifica se um ponto está dentro do clip.
    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        if !self.enabled {
            return true;
        }
        self.rect.contains_point(crate::geometry::Point::new(x, y))
    }

    /// Intersecta com outro clip.
    #[inline]
    pub fn intersect(&self, other: &ClipRect) -> ClipRect {
        if !self.enabled {
            return *other;
        }
        if !other.enabled {
            return *self;
        }

        match self.rect.intersection(&other.rect) {
            Some(r) => ClipRect::new(r),
            None => ClipRect {
                rect: Rect::ZERO,
                enabled: true, // Clip ativo mas vazio = nada desenha
            },
        }
    }

    /// Verifica se está vazio (nada será desenhado).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.enabled && self.rect.is_empty()
    }
}

impl From<Rect> for ClipRect {
    #[inline]
    fn from(rect: Rect) -> Self {
        Self::new(rect)
    }
}

// =============================================================================
// CLIP OP
// =============================================================================

/// Operação de clipping.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum ClipOp {
    /// Substitui o clip atual.
    #[default]
    Replace = 0,
    /// Intersecta com clip atual.
    Intersect = 1,
    /// União com clip atual (raro).
    Union = 2,
    /// Subtrai do clip atual.
    Subtract = 3,
}

impl ClipOp {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Replace),
            1 => Some(Self::Intersect),
            2 => Some(Self::Union),
            3 => Some(Self::Subtract),
            _ => None,
        }
    }
}
