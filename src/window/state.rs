//! # Window State
//!
//! Estados e tipos de janela.

// =============================================================================
// WINDOW STATE
// =============================================================================

/// Estado atual de uma janela.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum WindowState {
    /// Estado normal.
    #[default]
    Normal = 0,
    /// Minimizada.
    Minimized = 1,
    /// Maximizada.
    Maximized = 2,
    /// Fullscreen.
    Fullscreen = 3,
    /// Tiled left (metade esquerda da tela).
    TiledLeft = 4,
    /// Tiled right (metade direita da tela).
    TiledRight = 5,
    /// Tiled top (metade superior).
    TiledTop = 6,
    /// Tiled bottom (metade inferior).
    TiledBottom = 7,
    /// Oculta.
    Hidden = 8,
}

impl WindowState {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::Minimized),
            2 => Some(Self::Maximized),
            3 => Some(Self::Fullscreen),
            4 => Some(Self::TiledLeft),
            5 => Some(Self::TiledRight),
            6 => Some(Self::TiledTop),
            7 => Some(Self::TiledBottom),
            8 => Some(Self::Hidden),
            _ => None,
        }
    }

    /// Nome do estado.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Minimized => "Minimized",
            Self::Maximized => "Maximized",
            Self::Fullscreen => "Fullscreen",
            Self::TiledLeft => "Tiled Left",
            Self::TiledRight => "Tiled Right",
            Self::TiledTop => "Tiled Top",
            Self::TiledBottom => "Tiled Bottom",
            Self::Hidden => "Hidden",
        }
    }

    /// Verifica se a janela é visível.
    #[inline]
    pub const fn is_visible(&self) -> bool {
        !matches!(self, Self::Minimized | Self::Hidden)
    }

    /// Verifica se é um estado de tile.
    #[inline]
    pub const fn is_tiled(&self) -> bool {
        matches!(
            self,
            Self::TiledLeft | Self::TiledRight | Self::TiledTop | Self::TiledBottom
        )
    }
}

// =============================================================================
// WINDOW TYPE
// =============================================================================

/// Tipo de janela.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum WindowType {
    /// Janela normal de aplicação.
    #[default]
    Normal = 0,
    /// Diálogo.
    Dialog = 1,
    /// Menu popup.
    Menu = 2,
    /// Tooltip.
    Tooltip = 3,
    /// Notificação.
    Notification = 4,
    /// Splash screen.
    Splash = 5,
    /// Desktop/background.
    Desktop = 6,
    /// Dock/taskbar.
    Dock = 7,
    /// Dropdown menu.
    Dropdown = 8,
    /// Popup (combo box, etc).
    Popup = 9,
    /// Drag and drop feedback.
    Dnd = 10,
}

impl WindowType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::Dialog),
            2 => Some(Self::Menu),
            3 => Some(Self::Tooltip),
            4 => Some(Self::Notification),
            5 => Some(Self::Splash),
            6 => Some(Self::Desktop),
            7 => Some(Self::Dock),
            8 => Some(Self::Dropdown),
            9 => Some(Self::Popup),
            10 => Some(Self::Dnd),
            _ => None,
        }
    }

    /// Nome do tipo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Dialog => "Dialog",
            Self::Menu => "Menu",
            Self::Tooltip => "Tooltip",
            Self::Notification => "Notification",
            Self::Splash => "Splash",
            Self::Desktop => "Desktop",
            Self::Dock => "Dock",
            Self::Dropdown => "Dropdown",
            Self::Popup => "Popup",
            Self::Dnd => "Dnd",
        }
    }

    /// Verifica se pode ser focado.
    #[inline]
    pub const fn is_focusable(&self) -> bool {
        matches!(self, Self::Normal | Self::Dialog)
    }

    /// Verifica se é transiente (popup, menu, etc).
    #[inline]
    pub const fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::Menu | Self::Tooltip | Self::Dropdown | Self::Popup | Self::Dnd
        )
    }
}

// =============================================================================
// RESIZE EDGE
// =============================================================================

/// Borda/canto para redimensionamento.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResizeEdge {
    /// Borda superior.
    Top = 1,
    /// Borda inferior.
    Bottom = 2,
    /// Borda esquerda.
    Left = 4,
    /// Borda direita.
    Right = 8,
    /// Canto superior esquerdo.
    TopLeft = 5, // Top | Left
    /// Canto superior direito.
    TopRight = 9, // Top | Right
    /// Canto inferior esquerdo.
    BottomLeft = 6, // Bottom | Left
    /// Canto inferior direito.
    BottomRight = 10, // Bottom | Right
}

impl ResizeEdge {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Top),
            2 => Some(Self::Bottom),
            4 => Some(Self::Left),
            8 => Some(Self::Right),
            5 => Some(Self::TopLeft),
            9 => Some(Self::TopRight),
            6 => Some(Self::BottomLeft),
            10 => Some(Self::BottomRight),
            _ => None,
        }
    }

    /// Verifica se inclui topo.
    #[inline]
    pub const fn has_top(&self) -> bool {
        (*self as u8) & 1 != 0
    }

    /// Verifica se inclui fundo.
    #[inline]
    pub const fn has_bottom(&self) -> bool {
        (*self as u8) & 2 != 0
    }

    /// Verifica se inclui esquerda.
    #[inline]
    pub const fn has_left(&self) -> bool {
        (*self as u8) & 4 != 0
    }

    /// Verifica se inclui direita.
    #[inline]
    pub const fn has_right(&self) -> bool {
        (*self as u8) & 8 != 0
    }
}
