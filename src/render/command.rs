//! # Render Commands
//!
//! Operações de renderização.

use crate::buffer::BufferHandle;
use crate::color::{BlendMode, Color};
use crate::geometry::Rect;

// =============================================================================
// RENDER OP
// =============================================================================

/// Operação de renderização.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderOp {
    /// Sem operação.
    Nop = 0,
    /// Limpa com uma cor.
    Clear = 1,
    /// Preenche retângulo.
    FillRect = 2,
    /// Desenha borda de retângulo.
    StrokeRect = 3,
    /// Desenha linha.
    DrawLine = 4,
    /// Blit de buffer.
    Blit = 5,
    /// Blit escalado.
    BlitScaled = 6,
    /// Define clip.
    SetClip = 7,
    /// Remove clip.
    ClearClip = 8,
    /// Salva estado.
    Save = 9,
    /// Restaura estado.
    Restore = 10,
}

impl RenderOp {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Nop),
            1 => Some(Self::Clear),
            2 => Some(Self::FillRect),
            3 => Some(Self::StrokeRect),
            4 => Some(Self::DrawLine),
            5 => Some(Self::Blit),
            6 => Some(Self::BlitScaled),
            7 => Some(Self::SetClip),
            8 => Some(Self::ClearClip),
            9 => Some(Self::Save),
            10 => Some(Self::Restore),
            _ => None,
        }
    }

    /// Nome da operação.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Nop => "Nop",
            Self::Clear => "Clear",
            Self::FillRect => "FillRect",
            Self::StrokeRect => "StrokeRect",
            Self::DrawLine => "DrawLine",
            Self::Blit => "Blit",
            Self::BlitScaled => "BlitScaled",
            Self::SetClip => "SetClip",
            Self::ClearClip => "ClearClip",
            Self::Save => "Save",
            Self::Restore => "Restore",
        }
    }
}

// =============================================================================
// FILL PARAMS
// =============================================================================

/// Parâmetros para operação de fill.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FillParams {
    /// Retângulo a preencher.
    pub rect: Rect,
    /// Cor de preenchimento.
    pub color: Color,
    /// Modo de blend.
    pub blend: BlendMode,
}

impl FillParams {
    /// Cria novos parâmetros.
    #[inline]
    pub const fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color,
            blend: BlendMode::Normal,
        }
    }

    /// Com modo de blend.
    #[inline]
    pub const fn with_blend(mut self, blend: BlendMode) -> Self {
        self.blend = blend;
        self
    }
}

// =============================================================================
// BLIT PARAMS
// =============================================================================

/// Parâmetros para operação de blit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BlitParams {
    /// Handle do buffer fonte.
    pub src: BufferHandle,
    /// Retângulo fonte (área a copiar).
    pub src_rect: Rect,
    /// Posição destino (X, Y).
    pub dst_x: i32,
    pub dst_y: i32,
    /// Modo de blend.
    pub blend: BlendMode,
    /// Alpha global (0-255).
    pub alpha: u8,
}

impl BlitParams {
    /// Cria novos parâmetros.
    #[inline]
    pub const fn new(src: BufferHandle, src_rect: Rect, dst_x: i32, dst_y: i32) -> Self {
        Self {
            src,
            src_rect,
            dst_x,
            dst_y,
            blend: BlendMode::SourceOver,
            alpha: 255,
        }
    }

    /// Com modo de blend.
    #[inline]
    pub const fn with_blend(mut self, blend: BlendMode) -> Self {
        self.blend = blend;
        self
    }

    /// Com alpha global.
    #[inline]
    pub const fn with_alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    /// Retângulo destino calculado.
    #[inline]
    pub const fn dst_rect(&self) -> Rect {
        Rect::new(
            self.dst_x,
            self.dst_y,
            self.src_rect.width,
            self.src_rect.height,
        )
    }
}
