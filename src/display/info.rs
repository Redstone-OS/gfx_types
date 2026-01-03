//! # Display Info
//!
//! Informações sobre displays/monitores.

use crate::buffer::BufferDescriptor;
use crate::color::PixelFormat;
use crate::geometry::Size;

// =============================================================================
// DISPLAY INFO
// =============================================================================

/// Informações sobre um display/monitor.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayInfo {
    /// ID único do display.
    pub id: u32,
    /// Largura em pixels.
    pub width: u32,
    /// Altura em pixels.
    pub height: u32,
    /// Taxa de atualização em milihertz (ex: 60000 = 60Hz).
    pub refresh_rate_mhz: u32,
    /// Formato de pixel do display.
    pub format: PixelFormat,
    /// Stride do framebuffer em bytes.
    pub stride: u32,
}

impl DisplayInfo {
    /// Cria novo DisplayInfo.
    #[inline]
    pub const fn new(
        id: u32,
        width: u32,
        height: u32,
        refresh_rate_mhz: u32,
        format: PixelFormat,
        stride: u32,
    ) -> Self {
        Self {
            id,
            width,
            height,
            refresh_rate_mhz,
            format,
            stride,
        }
    }

    /// Retorna o tamanho como struct Size.
    #[inline]
    pub const fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Retorna como BufferDescriptor.
    #[inline]
    pub const fn as_buffer_descriptor(&self) -> BufferDescriptor {
        BufferDescriptor {
            width: self.width,
            height: self.height,
            stride: self.stride,
            format: self.format,
        }
    }

    /// Taxa de atualização em Hz (inteiro).
    #[inline]
    pub const fn refresh_rate_hz(&self) -> u32 {
        self.refresh_rate_mhz / 1000
    }

    /// Taxa de atualização em Hz (float).
    #[inline]
    pub fn refresh_rate_hz_f(&self) -> f32 {
        self.refresh_rate_mhz as f32 / 1000.0
    }

    /// Área total em pixels.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Tamanho do framebuffer em bytes.
    #[inline]
    pub const fn framebuffer_size(&self) -> usize {
        (self.stride as usize) * (self.height as usize)
    }
}

// =============================================================================
// DISPLAY MODE
// =============================================================================

/// Modo de display (resolução + refresh rate).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisplayMode {
    /// Largura em pixels.
    pub width: u32,
    /// Altura em pixels.
    pub height: u32,
    /// Taxa de atualização em milihertz.
    pub refresh_rate_mhz: u32,
    /// Flags do modo.
    pub flags: u32,
}

impl DisplayMode {
    /// Flag: modo preferido pelo display.
    pub const FLAG_PREFERRED: u32 = 1 << 0;
    /// Flag: modo atual.
    pub const FLAG_CURRENT: u32 = 1 << 1;
    /// Flag: modo interlaced.
    pub const FLAG_INTERLACED: u32 = 1 << 2;

    /// Cria novo modo.
    #[inline]
    pub const fn new(width: u32, height: u32, refresh_rate_mhz: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate_mhz,
            flags: 0,
        }
    }

    /// Tamanho.
    #[inline]
    pub const fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Verifica se é o modo preferido.
    #[inline]
    pub const fn is_preferred(&self) -> bool {
        (self.flags & Self::FLAG_PREFERRED) != 0
    }

    /// Verifica se é o modo atual.
    #[inline]
    pub const fn is_current(&self) -> bool {
        (self.flags & Self::FLAG_CURRENT) != 0
    }

    /// Verifica se é interlaced.
    #[inline]
    pub const fn is_interlaced(&self) -> bool {
        (self.flags & Self::FLAG_INTERLACED) != 0
    }
}

// =============================================================================
// VSYNC MODE
// =============================================================================

/// Modo de sincronização vertical.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum VsyncMode {
    /// VSync desativado (sem limite de FPS).
    Off = 0,
    /// VSync ativado (sincroniza com refresh rate).
    #[default]
    On = 1,
    /// VSync adaptativo (ativa apenas se FPS >= refresh rate).
    Adaptive = 2,
    /// VSync com mailbox (triple buffering, sem tearing, sem input lag).
    Mailbox = 3,
}

impl VsyncMode {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Off),
            1 => Some(Self::On),
            2 => Some(Self::Adaptive),
            3 => Some(Self::Mailbox),
            _ => None,
        }
    }

    /// Nome do modo.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::On => "On",
            Self::Adaptive => "Adaptive",
            Self::Mailbox => "Mailbox",
        }
    }
}
