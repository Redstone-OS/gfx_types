//! # Glyph Types
//!
//! Tipos para representação de glyphs.

/// ID de um glyph em uma fonte.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct GlyphId(pub u32);

impl GlyphId {
    /// Cria novo ID.
    #[inline]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// ID inválido/não encontrado.
    pub const NOTDEF: Self = Self(0);

    /// Verifica se é válido.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl From<u32> for GlyphId {
    #[inline]
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<GlyphId> for u32 {
    #[inline]
    fn from(id: GlyphId) -> Self {
        id.0
    }
}

/// Métricas de um glyph.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphMetrics {
    /// Largura do glyph em pixels.
    pub width: f32,
    /// Altura do glyph em pixels.
    pub height: f32,
    /// Bearing horizontal (offset da origem para o início do bitmap).
    pub bearing_x: f32,
    /// Bearing vertical.
    pub bearing_y: f32,
    /// Advance horizontal (distância para o próximo glyph).
    pub advance_x: f32,
    /// Advance vertical (geralmente 0 para texto horizontal).
    pub advance_y: f32,
}

impl GlyphMetrics {
    /// Cria novas métricas.
    #[inline]
    pub const fn new(width: f32, height: f32, advance_x: f32) -> Self {
        Self {
            width,
            height,
            bearing_x: 0.0,
            bearing_y: 0.0,
            advance_x,
            advance_y: 0.0,
        }
    }

    /// Métricas zeradas.
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
        bearing_x: 0.0,
        bearing_y: 0.0,
        advance_x: 0.0,
        advance_y: 0.0,
    };

    /// Métricas para uma fonte fixa (monospace).
    #[inline]
    pub const fn monospace(cell_width: f32, cell_height: f32) -> Self {
        Self {
            width: cell_width,
            height: cell_height,
            bearing_x: 0.0,
            bearing_y: cell_height,
            advance_x: cell_width,
            advance_y: 0.0,
        }
    }
}

/// Posição de um glyph em uma string renderizada.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphPosition {
    /// ID do glyph.
    pub id: GlyphId,
    /// Posição X.
    pub x: f32,
    /// Posição Y.
    pub y: f32,
    /// Offset X dentro do cluster (para ligatures).
    pub x_offset: f32,
    /// Offset Y.
    pub y_offset: f32,
}

impl GlyphPosition {
    /// Cria nova posição.
    #[inline]
    pub const fn new(id: GlyphId, x: f32, y: f32) -> Self {
        Self {
            id,
            x,
            y,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

/// Informações de cluster (agrupamento de caracteres).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClusterInfo {
    /// Índice do primeiro caractere no texto original.
    pub text_start: u32,
    /// Número de caracteres no cluster.
    pub text_len: u32,
    /// Índice do primeiro glyph.
    pub glyph_start: u32,
    /// Número de glyphs no cluster.
    pub glyph_len: u32,
}

impl ClusterInfo {
    /// Cria novo cluster simples (1:1).
    #[inline]
    pub const fn simple(text_index: u32, glyph_index: u32) -> Self {
        Self {
            text_start: text_index,
            text_len: 1,
            glyph_start: glyph_index,
            glyph_len: 1,
        }
    }
}
