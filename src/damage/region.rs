//! # Damage Region
//!
//! Regiões danificadas para composição.

use crate::geometry::Rect;

// =============================================================================
// DAMAGE REGION
// =============================================================================

/// Região danificada (área que precisa ser recomposta).
///
/// Usado para damage tracking e otimização de composição.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DamageRegion {
    /// Retângulo da região.
    pub rect: Rect,
}

impl DamageRegion {
    /// Cria nova região de dano.
    #[inline]
    pub const fn new(rect: Rect) -> Self {
        Self { rect }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
        }
    }

    /// Região vazia.
    pub const EMPTY: Self = Self { rect: Rect::ZERO };

    /// Verifica se está vazia.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.rect.is_empty()
    }

    /// Área da região.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.rect.area()
    }

    /// Une com outra região (bounding box).
    #[inline]
    pub fn union(&self, other: &DamageRegion) -> DamageRegion {
        DamageRegion {
            rect: self.rect.union(&other.rect),
        }
    }

    /// Interseção com outra região.
    #[inline]
    pub fn intersection(&self, other: &DamageRegion) -> Option<DamageRegion> {
        self.rect
            .intersection(&other.rect)
            .map(|r| DamageRegion { rect: r })
    }

    /// Verifica se intersecta outra região.
    #[inline]
    pub fn intersects(&self, other: &DamageRegion) -> bool {
        self.rect.intersects(&other.rect)
    }

    /// Offset por delta.
    #[inline]
    pub const fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            rect: self.rect.offset(dx, dy),
        }
    }

    /// Expande a região.
    #[inline]
    pub fn expand(&self, amount: i32) -> Self {
        Self {
            rect: self.rect.expand(amount),
        }
    }
}

impl From<Rect> for DamageRegion {
    #[inline]
    fn from(rect: Rect) -> Self {
        Self::new(rect)
    }
}

impl From<DamageRegion> for Rect {
    #[inline]
    fn from(region: DamageRegion) -> Self {
        region.rect
    }
}

// =============================================================================
// DAMAGE HINT
// =============================================================================

/// Hint sobre o tipo de dano.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum DamageHint {
    /// Dano desconhecido (assume tela inteira).
    #[default]
    Full = 0,
    /// Sem dano (nada mudou).
    None = 1,
    /// Dano parcial (lista de regiões).
    Partial = 2,
    /// Scroll (região movida).
    Scroll = 3,
}

impl DamageHint {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Full),
            1 => Some(Self::None),
            2 => Some(Self::Partial),
            3 => Some(Self::Scroll),
            _ => None,
        }
    }

    /// Nome do hint.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Full => "Full",
            Self::None => "None",
            Self::Partial => "Partial",
            Self::Scroll => "Scroll",
        }
    }

    /// Verifica se precisa recompor algo.
    #[inline]
    pub const fn needs_compose(&self) -> bool {
        !matches!(self, Self::None)
    }
}
