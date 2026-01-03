//! # Buffer Usage
//!
//! Flags de uso e capacidades de buffers.

// =============================================================================
// BUFFER USAGE
// =============================================================================

/// Hints de uso do buffer para otimização.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum BufferUsage {
    /// Uso geral, sem otimização específica.
    #[default]
    Default = 0,
    /// Buffer estático, raramente modificado.
    Static = 1,
    /// Buffer dinâmico, modificado frequentemente.
    Dynamic = 2,
    /// Buffer de streaming, modificado todo frame.
    Streaming = 3,
    /// Buffer de leitura apenas.
    ReadOnly = 4,
    /// Buffer de escrita apenas.
    WriteOnly = 5,
}

impl BufferUsage {
    /// Converte de u32.
    #[inline]
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Static),
            2 => Some(Self::Dynamic),
            3 => Some(Self::Streaming),
            4 => Some(Self::ReadOnly),
            5 => Some(Self::WriteOnly),
            _ => None,
        }
    }

    /// Nome do uso.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Static => "Static",
            Self::Dynamic => "Dynamic",
            Self::Streaming => "Streaming",
            Self::ReadOnly => "ReadOnly",
            Self::WriteOnly => "WriteOnly",
        }
    }
}

// =============================================================================
// BUFFER CAPABILITIES
// =============================================================================

/// Capacidades/flags de um buffer.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufferCapabilities(pub u32);

impl BufferCapabilities {
    /// Nenhuma capacidade especial.
    pub const NONE: Self = Self(0);

    /// Buffer pode ser mapeado para CPU.
    pub const CPU_ACCESSIBLE: Self = Self(1 << 0);

    /// Buffer pode ser usado pela GPU.
    pub const GPU_ACCESSIBLE: Self = Self(1 << 1);

    /// Buffer pode ser usado para DMA.
    pub const DMA_CAPABLE: Self = Self(1 << 2);

    /// Buffer é contíguo em memória física.
    pub const CONTIGUOUS: Self = Self(1 << 3);

    /// Buffer está em memória de vídeo (VRAM).
    pub const VIDEO_MEMORY: Self = Self(1 << 4);

    /// Buffer é compartilhável entre processos.
    pub const SHAREABLE: Self = Self(1 << 5);

    /// Buffer pode ser redimensionado.
    pub const RESIZABLE: Self = Self(1 << 6);

    /// Buffer suporta leitura.
    pub const READABLE: Self = Self(1 << 7);

    /// Buffer suporta escrita.
    pub const WRITABLE: Self = Self(1 << 8);

    /// Verifica se tem uma flag.
    #[inline]
    pub const fn has(&self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    /// Combina com outra flag.
    #[inline]
    pub const fn with(&self, flag: Self) -> Self {
        Self(self.0 | flag.0)
    }

    /// Remove uma flag.
    #[inline]
    pub const fn without(&self, flag: Self) -> Self {
        Self(self.0 & !flag.0)
    }

    /// Valor raw.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
}

impl core::ops::BitOr for BufferCapabilities {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for BufferCapabilities {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitOrAssign for BufferCapabilities {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
