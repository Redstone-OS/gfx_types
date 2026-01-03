//! # Window Effects
//!
//! Efeitos visuais para janelas.

use crate::color::Color;

/// Parâmetros de sombra.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShadowParams {
    /// Offset X da sombra.
    pub offset_x: f32,
    /// Offset Y da sombra.
    pub offset_y: f32,
    /// Raio de blur.
    pub blur_radius: f32,
    /// Spread (expansão).
    pub spread: f32,
    /// Cor da sombra.
    pub color: Color,
}

impl ShadowParams {
    /// Cria parâmetros de sombra.
    #[inline]
    pub const fn new(offset_x: f32, offset_y: f32, blur_radius: f32, color: Color) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread: 0.0,
            color,
        }
    }

    /// Sombra padrão (drop shadow).
    pub const DEFAULT: Self = Self {
        offset_x: 0.0,
        offset_y: 4.0,
        blur_radius: 8.0,
        spread: 0.0,
        color: Color(0x40000000),
    };

    /// Sem sombra.
    pub const NONE: Self = Self {
        offset_x: 0.0,
        offset_y: 0.0,
        blur_radius: 0.0,
        spread: 0.0,
        color: Color::TRANSPARENT,
    };

    /// Com spread.
    #[inline]
    pub const fn with_spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    /// Verifica se a sombra é visível.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.blur_radius > 0.0 || self.spread > 0.0 || self.offset_x != 0.0 || self.offset_y != 0.0
    }
}

/// Parâmetros de blur.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlurParams {
    /// Raio do blur.
    pub radius: f32,
    /// Tipo de blur.
    pub blur_type: BlurType,
}

impl BlurParams {
    /// Cria parâmetros de blur.
    #[inline]
    pub const fn new(radius: f32, blur_type: BlurType) -> Self {
        Self { radius, blur_type }
    }

    /// Blur gaussiano com raio.
    #[inline]
    pub const fn gaussian(radius: f32) -> Self {
        Self {
            radius,
            blur_type: BlurType::Gaussian,
        }
    }

    /// Blur box com raio.
    #[inline]
    pub const fn box_blur(radius: f32) -> Self {
        Self {
            radius,
            blur_type: BlurType::Box,
        }
    }

    /// Sem blur.
    pub const NONE: Self = Self {
        radius: 0.0,
        blur_type: BlurType::Box,
    };

    /// Verifica se o blur é visível.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.radius > 0.0
    }
}

/// Tipo de blur.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum BlurType {
    /// Box blur (rápido).
    #[default]
    Box = 0,
    /// Gaussian blur (suave).
    Gaussian = 1,
    /// Motion blur.
    Motion = 2,
    /// Radial blur.
    Radial = 3,
}

impl BlurType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Box),
            1 => Some(Self::Gaussian),
            2 => Some(Self::Motion),
            3 => Some(Self::Radial),
            _ => None,
        }
    }

    /// Nome do tipo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Box => "Box",
            Self::Gaussian => "Gaussian",
            Self::Motion => "Motion",
            Self::Radial => "Radial",
        }
    }
}

/// Parâmetros de opacidade.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OpacityParams {
    /// Valor de opacidade (0.0 = transparente, 1.0 = opaco).
    pub value: f32,
}

impl OpacityParams {
    /// Cria parâmetros de opacidade.
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// Opaco.
    pub const OPAQUE: Self = Self { value: 1.0 };

    /// Transparente.
    pub const TRANSPARENT: Self = Self { value: 0.0 };

    /// Converte para valor alpha 0-255.
    #[inline]
    pub fn to_alpha(&self) -> u8 {
        (self.value.clamp(0.0, 1.0) * 255.0) as u8
    }
}

/// Efeitos combinados de uma janela.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowEffects {
    /// Sombra.
    pub shadow: ShadowParams,
    /// Blur de fundo (backdrop blur).
    pub backdrop_blur: BlurParams,
    /// Opacidade.
    pub opacity: OpacityParams,
    /// Corner radius.
    pub corner_radius: f32,
}

impl WindowEffects {
    /// Sem efeitos.
    pub const NONE: Self = Self {
        shadow: ShadowParams::NONE,
        backdrop_blur: BlurParams::NONE,
        opacity: OpacityParams::OPAQUE,
        corner_radius: 0.0,
    };

    /// Efeitos padrão.
    pub const DEFAULT: Self = Self {
        shadow: ShadowParams::DEFAULT,
        backdrop_blur: BlurParams::NONE,
        opacity: OpacityParams::OPAQUE,
        corner_radius: 8.0,
    };

    /// Com sombra.
    #[inline]
    pub const fn with_shadow(mut self, shadow: ShadowParams) -> Self {
        self.shadow = shadow;
        self
    }

    /// Com blur de fundo.
    #[inline]
    pub const fn with_backdrop_blur(mut self, blur: BlurParams) -> Self {
        self.backdrop_blur = blur;
        self
    }

    /// Com opacidade.
    #[inline]
    pub const fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = OpacityParams::new(opacity);
        self
    }

    /// Com corner radius.
    #[inline]
    pub const fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}
