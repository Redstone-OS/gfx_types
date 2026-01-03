//! # Cursor Types
//!
//! Tipos de cursor do sistema.

use crate::geometry::Point;

// =============================================================================
// CURSOR TYPE
// =============================================================================

/// Tipo de cursor do sistema.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum CursorType {
    /// Cursor padrão (seta).
    #[default]
    Default = 0,
    /// Ponteiro (mão, para links).
    Pointer = 1,
    /// Texto (I-beam).
    Text = 2,
    /// Aguarde (loading).
    Wait = 3,
    /// Progresso (loading com interatividade).
    Progress = 4,
    /// Crosshair (precisão).
    Crosshair = 5,
    /// Move (arrastar).
    Move = 6,
    /// Não permitido.
    NotAllowed = 7,
    /// Redimensionar Norte.
    ResizeN = 10,
    /// Redimensionar Nordeste.
    ResizeNE = 11,
    /// Redimensionar Leste.
    ResizeE = 12,
    /// Redimensionar Sudeste.
    ResizeSE = 13,
    /// Redimensionar Sul.
    ResizeS = 14,
    /// Redimensionar Sudoeste.
    ResizeSW = 15,
    /// Redimensionar Oeste.
    ResizeW = 16,
    /// Redimensionar Noroeste.
    ResizeNW = 17,
    /// Redimensionar Norte-Sul.
    ResizeNS = 18,
    /// Redimensionar Leste-Oeste.
    ResizeEW = 19,
    /// Redimensionar Nordeste-Sudoeste.
    ResizeNESW = 20,
    /// Redimensionar Noroeste-Sudeste.
    ResizeNWSE = 21,
    /// Agarrar (grab).
    Grab = 30,
    /// Agarrando (grabbing).
    Grabbing = 31,
    /// Zoom in.
    ZoomIn = 32,
    /// Zoom out.
    ZoomOut = 33,
    /// Ajuda.
    Help = 40,
    /// Contexto menu.
    ContextMenu = 41,
    /// Célula.
    Cell = 42,
    /// Copiar.
    Copy = 43,
    /// Alias.
    Alias = 44,
    /// Nenhum cursor.
    None = 255,
}

impl CursorType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Pointer),
            2 => Some(Self::Text),
            3 => Some(Self::Wait),
            4 => Some(Self::Progress),
            5 => Some(Self::Crosshair),
            6 => Some(Self::Move),
            7 => Some(Self::NotAllowed),
            10 => Some(Self::ResizeN),
            11 => Some(Self::ResizeNE),
            12 => Some(Self::ResizeE),
            13 => Some(Self::ResizeSE),
            14 => Some(Self::ResizeS),
            15 => Some(Self::ResizeSW),
            16 => Some(Self::ResizeW),
            17 => Some(Self::ResizeNW),
            18 => Some(Self::ResizeNS),
            19 => Some(Self::ResizeEW),
            20 => Some(Self::ResizeNESW),
            21 => Some(Self::ResizeNWSE),
            30 => Some(Self::Grab),
            31 => Some(Self::Grabbing),
            32 => Some(Self::ZoomIn),
            33 => Some(Self::ZoomOut),
            40 => Some(Self::Help),
            41 => Some(Self::ContextMenu),
            42 => Some(Self::Cell),
            43 => Some(Self::Copy),
            44 => Some(Self::Alias),
            255 => Some(Self::None),
            _ => None,
        }
    }

    /// Nome do cursor.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Pointer => "pointer",
            Self::Text => "text",
            Self::Wait => "wait",
            Self::Progress => "progress",
            Self::Crosshair => "crosshair",
            Self::Move => "move",
            Self::NotAllowed => "not-allowed",
            Self::ResizeN => "n-resize",
            Self::ResizeNE => "ne-resize",
            Self::ResizeE => "e-resize",
            Self::ResizeSE => "se-resize",
            Self::ResizeS => "s-resize",
            Self::ResizeSW => "sw-resize",
            Self::ResizeW => "w-resize",
            Self::ResizeNW => "nw-resize",
            Self::ResizeNS => "ns-resize",
            Self::ResizeEW => "ew-resize",
            Self::ResizeNESW => "nesw-resize",
            Self::ResizeNWSE => "nwse-resize",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
            Self::ZoomIn => "zoom-in",
            Self::ZoomOut => "zoom-out",
            Self::Help => "help",
            Self::ContextMenu => "context-menu",
            Self::Cell => "cell",
            Self::Copy => "copy",
            Self::Alias => "alias",
            Self::None => "none",
        }
    }

    /// Verifica se é um cursor de redimensionamento.
    #[inline]
    pub const fn is_resize(&self) -> bool {
        (*self as u8) >= 10 && (*self as u8) <= 21
    }

    /// Hotspot padrão para este tipo de cursor.
    #[inline]
    pub const fn default_hotspot(&self) -> CursorHotspot {
        match self {
            Self::Default | Self::Pointer | Self::Help | Self::ContextMenu => {
                CursorHotspot::new(0, 0)
            }
            Self::Text => CursorHotspot::new(8, 8),
            Self::Crosshair | Self::Move | Self::Cell => CursorHotspot::new(8, 8),
            Self::Wait | Self::Progress => CursorHotspot::new(8, 8),
            _ => CursorHotspot::new(8, 8),
        }
    }
}

// =============================================================================
// CURSOR HOTSPOT
// =============================================================================

/// Ponto ativo do cursor (pixel "quente").
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CursorHotspot {
    /// Offset X do hotspot.
    pub x: i32,
    /// Offset Y do hotspot.
    pub y: i32,
}

impl CursorHotspot {
    /// Cria novo hotspot.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Hotspot na origem.
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Converte para Point.
    #[inline]
    pub const fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl From<Point> for CursorHotspot {
    #[inline]
    fn from(p: Point) -> Self {
        Self::new(p.x, p.y)
    }
}

impl From<CursorHotspot> for Point {
    #[inline]
    fn from(h: CursorHotspot) -> Self {
        h.to_point()
    }
}
