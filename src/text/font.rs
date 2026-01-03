//! # Font Types
//!
//! Propriedades de fontes e texto.

/// Peso da fonte.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum FontWeight {
    /// Thin (100).
    Thin = 100,
    /// Extra Light (200).
    ExtraLight = 200,
    /// Light (300).
    Light = 300,
    /// Normal/Regular (400).
    #[default]
    Normal = 400,
    /// Medium (500).
    Medium = 500,
    /// Semi Bold (600).
    SemiBold = 600,
    /// Bold (700).
    Bold = 700,
    /// Extra Bold (800).
    ExtraBold = 800,
    /// Black (900).
    Black = 900,
}

impl FontWeight {
    /// Converte de u16.
    #[inline]
    pub fn from_u16(value: u16) -> Self {
        match value {
            0..=150 => Self::Thin,
            151..=250 => Self::ExtraLight,
            251..=350 => Self::Light,
            351..=450 => Self::Normal,
            451..=550 => Self::Medium,
            551..=650 => Self::SemiBold,
            651..=750 => Self::Bold,
            751..=850 => Self::ExtraBold,
            _ => Self::Black,
        }
    }

    /// Valor numérico.
    #[inline]
    pub const fn value(&self) -> u16 {
        *self as u16
    }

    /// Nome do peso.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Thin => "Thin",
            Self::ExtraLight => "ExtraLight",
            Self::Light => "Light",
            Self::Normal => "Normal",
            Self::Medium => "Medium",
            Self::SemiBold => "SemiBold",
            Self::Bold => "Bold",
            Self::ExtraBold => "ExtraBold",
            Self::Black => "Black",
        }
    }

    /// Verifica se é bold.
    #[inline]
    pub const fn is_bold(&self) -> bool {
        (*self as u16) >= 600
    }
}

/// Estilo da fonte.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum FontStyle {
    /// Normal.
    #[default]
    Normal = 0,
    /// Itálico.
    Italic = 1,
    /// Oblíquo.
    Oblique = 2,
}

impl FontStyle {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::Italic),
            2 => Some(Self::Oblique),
            _ => None,
        }
    }

    /// Nome do estilo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Italic => "Italic",
            Self::Oblique => "Oblique",
        }
    }

    /// Verifica se é inclinado.
    #[inline]
    pub const fn is_slanted(&self) -> bool {
        !matches!(self, Self::Normal)
    }
}

/// Alinhamento de texto.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum TextAlign {
    /// Alinhado à esquerda.
    #[default]
    Left = 0,
    /// Centralizado.
    Center = 1,
    /// Alinhado à direita.
    Right = 2,
    /// Justificado.
    Justify = 3,
}

impl TextAlign {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Left),
            1 => Some(Self::Center),
            2 => Some(Self::Right),
            3 => Some(Self::Justify),
            _ => None,
        }
    }

    /// Nome do alinhamento.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Center => "Center",
            Self::Right => "Right",
            Self::Justify => "Justify",
        }
    }
}

/// Baseline de texto.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum TextBaseline {
    /// Topo do texto.
    Top = 0,
    /// Meio.
    Middle = 1,
    /// Baseline alfabética (padrão).
    #[default]
    Alphabetic = 2,
    /// Fundo do texto.
    Bottom = 3,
    /// Hanging (para scripts como Devanagari).
    Hanging = 4,
    /// Ideográfico.
    Ideographic = 5,
}

impl TextBaseline {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Top),
            1 => Some(Self::Middle),
            2 => Some(Self::Alphabetic),
            3 => Some(Self::Bottom),
            4 => Some(Self::Hanging),
            5 => Some(Self::Ideographic),
            _ => None,
        }
    }

    /// Nome da baseline.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Top => "Top",
            Self::Middle => "Middle",
            Self::Alphabetic => "Alphabetic",
            Self::Bottom => "Bottom",
            Self::Hanging => "Hanging",
            Self::Ideographic => "Ideographic",
        }
    }
}

/// Decoração de texto.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextDecoration(pub u8);

impl TextDecoration {
    /// Sem decoração.
    pub const NONE: Self = Self(0);
    /// Sublinhado.
    pub const UNDERLINE: Self = Self(1 << 0);
    /// Linha sobre o texto.
    pub const OVERLINE: Self = Self(1 << 1);
    /// Tachado.
    pub const LINE_THROUGH: Self = Self(1 << 2);

    /// Verifica se tem decoração.
    #[inline]
    pub const fn has(&self, deco: Self) -> bool {
        (self.0 & deco.0) != 0
    }

    /// Combina decorações.
    #[inline]
    pub const fn with(&self, deco: Self) -> Self {
        Self(self.0 | deco.0)
    }
}
