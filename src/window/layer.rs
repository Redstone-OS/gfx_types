//! # Layer Type
//!
//! Tipos de camadas no compositor.

/// Tipo de camada no compositor.
///
/// Define a ordem de desenho e comportamento de diferentes tipos de superfícies.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum LayerType {
    /// Fundo/wallpaper (abaixo de tudo).
    #[default]
    Background = 0,
    /// Janelas normais de aplicações.
    Normal = 1,
    /// Janelas always-on-top.
    Top = 2,
    /// Painéis (taskbar, dock).
    Panel = 3,
    /// Overlay (notificações, menus popup).
    Overlay = 4,
    /// Lock screen.
    Lock = 5,
    /// Cursor do mouse (acima de tudo).
    Cursor = 6,
}

impl LayerType {
    /// Converte de u32.
    #[inline]
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Background),
            1 => Some(Self::Normal),
            2 => Some(Self::Top),
            3 => Some(Self::Panel),
            4 => Some(Self::Overlay),
            5 => Some(Self::Lock),
            6 => Some(Self::Cursor),
            _ => None,
        }
    }

    /// Converte para u32.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        *self as u32
    }

    /// Nome da camada.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Background => "Background",
            Self::Normal => "Normal",
            Self::Top => "Top",
            Self::Panel => "Panel",
            Self::Overlay => "Overlay",
            Self::Lock => "Lock",
            Self::Cursor => "Cursor",
        }
    }

    /// Verifica se janelas nesta camada recebem input normalmente.
    #[inline]
    pub const fn receives_input(&self) -> bool {
        matches!(self, Self::Normal | Self::Top | Self::Panel | Self::Overlay)
    }

    /// Verifica se esta camada deve bloquear camadas abaixo.
    #[inline]
    pub const fn blocks_below(&self) -> bool {
        matches!(self, Self::Lock)
    }
}
