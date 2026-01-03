//! # Window Surface
//!
//! Superfícies e buffers de janela.

use crate::buffer::BufferHandle;
use crate::geometry::Size;

/// ID de superfície.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct SurfaceId(pub u32);

impl SurfaceId {
    /// Cria novo ID.
    #[inline]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// ID inválido.
    pub const INVALID: Self = Self(0);

    /// Verifica se é válido.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Tipo de superfície.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum SurfaceType {
    /// Superfície principal (toplevel).
    #[default]
    Toplevel = 0,
    /// Popup.
    Popup = 1,
    /// Subsuperfície.
    Subsurface = 2,
    /// Drag and drop.
    Dnd = 3,
}

impl SurfaceType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Toplevel),
            1 => Some(Self::Popup),
            2 => Some(Self::Subsurface),
            3 => Some(Self::Dnd),
            _ => None,
        }
    }

    /// Nome do tipo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Toplevel => "Toplevel",
            Self::Popup => "Popup",
            Self::Subsurface => "Subsurface",
            Self::Dnd => "Dnd",
        }
    }
}

/// Modo de buffer (single, double, triple buffering).
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum BufferMode {
    /// Single buffer (sem v-sync, pode ter tearing).
    Single = 1,
    /// Double buffer (v-sync, sem tearing).
    #[default]
    Double = 2,
    /// Triple buffer (v-sync, menor input lag).
    Triple = 3,
}

impl BufferMode {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Single),
            2 => Some(Self::Double),
            3 => Some(Self::Triple),
            _ => None,
        }
    }

    /// Número de buffers.
    #[inline]
    pub const fn buffer_count(&self) -> usize {
        *self as usize
    }
}

/// Configuração de superfície.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SurfaceConfig {
    /// Tamanho.
    pub size: Size,
    /// Tipo.
    pub surface_type: SurfaceType,
    /// Modo de buffer.
    pub buffer_mode: BufferMode,
    /// Superfície pai (para popup/subsurface).
    pub parent: SurfaceId,
}

impl SurfaceConfig {
    /// Cria nova configuração.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            size: Size::new(width, height),
            surface_type: SurfaceType::Toplevel,
            buffer_mode: BufferMode::Double,
            parent: SurfaceId::INVALID,
        }
    }

    /// Com tipo.
    #[inline]
    pub const fn with_type(mut self, stype: SurfaceType) -> Self {
        self.surface_type = stype;
        self
    }

    /// Com pai.
    #[inline]
    pub const fn with_parent(mut self, parent: SurfaceId) -> Self {
        self.parent = parent;
        self
    }

    /// Com modo de buffer.
    #[inline]
    pub const fn with_buffer_mode(mut self, mode: BufferMode) -> Self {
        self.buffer_mode = mode;
        self
    }
}

/// Estado de commit de superfície.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SurfaceCommit {
    /// Buffer a apresentar.
    pub buffer: BufferHandle,
    /// Escala (1 = normal, 2 = HiDPI).
    pub scale: u32,
    /// Offset X do buffer.
    pub offset_x: i32,
    /// Offset Y do buffer.
    pub offset_y: i32,
}

impl SurfaceCommit {
    /// Cria novo commit.
    #[inline]
    pub const fn new(buffer: BufferHandle) -> Self {
        Self {
            buffer,
            scale: 1,
            offset_x: 0,
            offset_y: 0,
        }
    }

    /// Com escala.
    #[inline]
    pub const fn with_scale(mut self, scale: u32) -> Self {
        self.scale = scale;
        self
    }

    /// Com offset.
    #[inline]
    pub const fn with_offset(mut self, x: i32, y: i32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }
}
