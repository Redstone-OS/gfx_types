//! # Color Space
//!
//! Espaços de cor para conversão e gerenciamento de cores.

/// Espaço de cor.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum ColorSpace {
    /// sRGB (espaço padrão, gamma-corrected).
    #[default]
    SRGB = 0,
    /// RGB linear (sem gamma).
    LinearRGB = 1,
    /// Display P3 (wide gamut).
    DisplayP3 = 2,
    /// Adobe RGB.
    AdobeRGB = 3,
    /// Rec. 709 (HDTV).
    Rec709 = 4,
    /// Rec. 2020 (UHDTV, HDR).
    Rec2020 = 5,
}

impl ColorSpace {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::SRGB),
            1 => Some(Self::LinearRGB),
            2 => Some(Self::DisplayP3),
            3 => Some(Self::AdobeRGB),
            4 => Some(Self::Rec709),
            5 => Some(Self::Rec2020),
            _ => None,
        }
    }

    /// Nome do espaço de cor.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::SRGB => "sRGB",
            Self::LinearRGB => "Linear RGB",
            Self::DisplayP3 => "Display P3",
            Self::AdobeRGB => "Adobe RGB",
            Self::Rec709 => "Rec. 709",
            Self::Rec2020 => "Rec. 2020",
        }
    }

    /// Verifica se é gamma-corrected.
    #[inline]
    pub const fn is_gamma_corrected(&self) -> bool {
        !matches!(self, Self::LinearRGB)
    }

    /// Verifica se é wide gamut.
    #[inline]
    pub const fn is_wide_gamut(&self) -> bool {
        matches!(self, Self::DisplayP3 | Self::AdobeRGB | Self::Rec2020)
    }

    /// Verifica se é HDR capaz.
    #[inline]
    pub const fn is_hdr_capable(&self) -> bool {
        matches!(self, Self::Rec2020)
    }

    /// Gamma padrão para o espaço de cor.
    #[inline]
    pub const fn gamma(&self) -> f32 {
        match self {
            Self::SRGB | Self::DisplayP3 => 2.2,
            Self::LinearRGB => 1.0,
            Self::AdobeRGB => 2.2,
            Self::Rec709 => 2.4,
            Self::Rec2020 => 2.4,
        }
    }
}

/// Converte valor sRGB [0,1] para linear.
#[inline]
pub fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        rdsmath::powf((value + 0.055) / 1.055, 2.4)
    }
}

/// Converte valor linear [0,1] para sRGB.
#[inline]
pub fn linear_to_srgb(value: f32) -> f32 {
    if value <= 0.0031308 {
        value * 12.92
    } else {
        1.055 * rdsmath::powf(value, 1.0 / 2.4) - 0.055
    }
}

/// Aplica gamma a um valor.
#[inline]
pub fn apply_gamma(value: f32, gamma: f32) -> f32 {
    rdsmath::powf(value.clamp(0.0, 1.0), gamma)
}

/// Remove gamma de um valor.
#[inline]
pub fn remove_gamma(value: f32, gamma: f32) -> f32 {
    rdsmath::powf(value.clamp(0.0, 1.0), 1.0 / gamma)
}
