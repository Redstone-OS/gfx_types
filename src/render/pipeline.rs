//! # Render Pipeline
//!
//! Tipos para pipeline de renderização.

use crate::color::BlendMode;

/// Operação raster (ROP).
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum RasterOp {
    /// Copia source para destination.
    #[default]
    Copy = 0,
    /// Source AND destination.
    And = 1,
    /// Source OR destination.
    Or = 2,
    /// Source XOR destination.
    Xor = 3,
    /// NOT source.
    NotSrc = 4,
    /// NOT destination.
    NotDst = 5,
    /// Clear (preenche com 0).
    Clear = 6,
    /// Set (preenche com 1).
    Set = 7,
    /// NAND.
    Nand = 8,
    /// NOR.
    Nor = 9,
}

impl RasterOp {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Copy),
            1 => Some(Self::And),
            2 => Some(Self::Or),
            3 => Some(Self::Xor),
            4 => Some(Self::NotSrc),
            5 => Some(Self::NotDst),
            6 => Some(Self::Clear),
            7 => Some(Self::Set),
            8 => Some(Self::Nand),
            9 => Some(Self::Nor),
            _ => None,
        }
    }

    /// Nome da operação.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Copy => "Copy",
            Self::And => "And",
            Self::Or => "Or",
            Self::Xor => "Xor",
            Self::NotSrc => "Not Src",
            Self::NotDst => "Not Dst",
            Self::Clear => "Clear",
            Self::Set => "Set",
            Self::Nand => "Nand",
            Self::Nor => "Nor",
        }
    }
}

/// Estado do pipeline de renderização.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PipelineState {
    /// Modo de blend.
    pub blend_mode: BlendMode,
    /// Operação raster.
    pub raster_op: RasterOp,
    /// Alpha global (0-255).
    pub global_alpha: u8,
    /// Antialiasing ativo?
    pub antialias: bool,
    /// Dithering ativo?
    pub dither: bool,
}

impl PipelineState {
    /// Estado padrão.
    pub const DEFAULT: Self = Self {
        blend_mode: BlendMode::Normal,
        raster_op: RasterOp::Copy,
        global_alpha: 255,
        antialias: false,
        dither: false,
    };

    /// Cria novo estado.
    #[inline]
    pub const fn new() -> Self {
        Self::DEFAULT
    }

    /// Com modo de blend.
    #[inline]
    pub const fn with_blend(mut self, blend: BlendMode) -> Self {
        self.blend_mode = blend;
        self
    }

    /// Com alpha global.
    #[inline]
    pub const fn with_alpha(mut self, alpha: u8) -> Self {
        self.global_alpha = alpha;
        self
    }

    /// Com antialiasing.
    #[inline]
    pub const fn with_antialias(mut self, aa: bool) -> Self {
        self.antialias = aa;
        self
    }
}

/// Qualidade de interpolação para escala.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum InterpolationQuality {
    /// Nearest neighbor (mais rápido, pixelado).
    #[default]
    Nearest = 0,
    /// Bilinear.
    Bilinear = 1,
    /// Bicubic.
    Bicubic = 2,
    /// Lanczos (melhor qualidade, mais lento).
    Lanczos = 3,
}

impl InterpolationQuality {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Nearest),
            1 => Some(Self::Bilinear),
            2 => Some(Self::Bicubic),
            3 => Some(Self::Lanczos),
            _ => None,
        }
    }

    /// Nome.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Nearest => "Nearest",
            Self::Bilinear => "Bilinear",
            Self::Bicubic => "Bicubic",
            Self::Lanczos => "Lanczos",
        }
    }
}
