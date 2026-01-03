//! # Blend Modes
//!
//! Modos de blending e composição de cores.

// =============================================================================
// BLEND MODE
// =============================================================================

/// Modo de blending para composição de cores.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum BlendMode {
    /// Substitui completamente (sem blending).
    #[default]
    Normal = 0,

    // =========================================================================
    // Porter-Duff Compositing
    // =========================================================================
    /// Source over destination (default alpha blending).
    SourceOver = 1,
    /// Source where destination exists.
    SourceIn = 2,
    /// Source where destination doesn't exist.
    SourceOut = 3,
    /// Source on top of destination.
    SourceAtop = 4,
    /// Destination over source.
    DestOver = 5,
    /// Destination where source exists.
    DestIn = 6,
    /// Destination where source doesn't exist.
    DestOut = 7,
    /// Destination on top of source.
    DestAtop = 8,
    /// XOR of source and destination.
    Xor = 9,
    /// Clear (fully transparent).
    Clear = 10,

    // =========================================================================
    // Photoshop-style Blend Modes
    // =========================================================================
    /// Multiplica cores (escurece).
    Multiply = 20,
    /// Screen (clareia).
    Screen = 21,
    /// Overlay (combina multiply e screen).
    Overlay = 22,
    /// Darken (mantém mais escuro).
    Darken = 23,
    /// Lighten (mantém mais claro).
    Lighten = 24,
    /// Color Dodge (clareia dramaticamente).
    ColorDodge = 25,
    /// Color Burn (escurece dramaticamente).
    ColorBurn = 26,
    /// Hard Light (como spotlight).
    HardLight = 27,
    /// Soft Light (suave).
    SoftLight = 28,
    /// Difference (diferença absoluta).
    Difference = 29,
    /// Exclusion (como difference, mais suave).
    Exclusion = 30,

    // =========================================================================
    // Additive
    // =========================================================================
    /// Adiciona cores (usado em efeitos de luz).
    Add = 40,
    /// Subtrai cores.
    Subtract = 41,
}

impl BlendMode {
    /// Verifica se é um modo Porter-Duff.
    #[inline]
    pub const fn is_porter_duff(&self) -> bool {
        (*self as u8) >= 1 && (*self as u8) <= 10
    }

    /// Verifica se é um modo photoshop-style.
    #[inline]
    pub const fn is_photoshop_style(&self) -> bool {
        (*self as u8) >= 20 && (*self as u8) <= 30
    }

    /// Verifica se precisa de alpha blending.
    #[inline]
    pub const fn needs_alpha(&self) -> bool {
        !matches!(self, Self::Normal | Self::Clear)
    }

    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::SourceOver),
            2 => Some(Self::SourceIn),
            3 => Some(Self::SourceOut),
            4 => Some(Self::SourceAtop),
            5 => Some(Self::DestOver),
            6 => Some(Self::DestIn),
            7 => Some(Self::DestOut),
            8 => Some(Self::DestAtop),
            9 => Some(Self::Xor),
            10 => Some(Self::Clear),
            20 => Some(Self::Multiply),
            21 => Some(Self::Screen),
            22 => Some(Self::Overlay),
            23 => Some(Self::Darken),
            24 => Some(Self::Lighten),
            25 => Some(Self::ColorDodge),
            26 => Some(Self::ColorBurn),
            27 => Some(Self::HardLight),
            28 => Some(Self::SoftLight),
            29 => Some(Self::Difference),
            30 => Some(Self::Exclusion),
            40 => Some(Self::Add),
            41 => Some(Self::Subtract),
            _ => None,
        }
    }

    /// Nome do modo de blending.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::SourceOver => "SourceOver",
            Self::SourceIn => "SourceIn",
            Self::SourceOut => "SourceOut",
            Self::SourceAtop => "SourceAtop",
            Self::DestOver => "DestOver",
            Self::DestIn => "DestIn",
            Self::DestOut => "DestOut",
            Self::DestAtop => "DestAtop",
            Self::Xor => "Xor",
            Self::Clear => "Clear",
            Self::Multiply => "Multiply",
            Self::Screen => "Screen",
            Self::Overlay => "Overlay",
            Self::Darken => "Darken",
            Self::Lighten => "Lighten",
            Self::ColorDodge => "ColorDodge",
            Self::ColorBurn => "ColorBurn",
            Self::HardLight => "HardLight",
            Self::SoftLight => "SoftLight",
            Self::Difference => "Difference",
            Self::Exclusion => "Exclusion",
            Self::Add => "Add",
            Self::Subtract => "Subtract",
        }
    }
}

// =============================================================================
// ALPHA MODE
// =============================================================================

/// Modo de tratamento do canal alpha.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum AlphaMode {
    /// Alpha é independente das cores (straight alpha).
    #[default]
    Straight = 0,
    /// Alpha é multiplicado nas cores (premultiplied alpha).
    Premultiplied = 1,
    /// Sem canal alpha (ignorado).
    Opaque = 2,
}

impl AlphaMode {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Straight),
            1 => Some(Self::Premultiplied),
            2 => Some(Self::Opaque),
            _ => None,
        }
    }

    /// Nome do modo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Straight => "Straight",
            Self::Premultiplied => "Premultiplied",
            Self::Opaque => "Opaque",
        }
    }
}
