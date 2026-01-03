//! # Pixel Format
//!
//! Formatos de pixel suportados pelo sistema gráfico.

/// Formato de pixel suportado pelo sistema gráfico.
///
/// Define como os bytes de cor são organizados na memória.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum PixelFormat {
    /// Alpha-Red-Green-Blue (32-bit, formato padrão)
    #[default]
    ARGB8888 = 0,
    /// Ignored-Red-Green-Blue (32-bit, sem alpha)
    XRGB8888 = 1,
    /// Red-Green-Blue (16-bit, economia de memória)
    RGB565 = 2,
    /// Blue-Green-Red-Alpha (32-bit, usado por alguns hardwares)
    BGRA8888 = 3,
    /// Red-Green-Blue-Alpha (32-bit)
    RGBA8888 = 4,
    /// Red-Green-Blue (24-bit, sem padding)
    RGB888 = 5,
    /// Blue-Green-Red (24-bit)
    BGR888 = 6,
    /// Grayscale 8-bit
    Gray8 = 7,
    /// Grayscale 16-bit
    Gray16 = 8,
    /// Alpha only 8-bit (masks)
    Alpha8 = 9,
}

impl PixelFormat {
    /// Retorna o número de bytes por pixel para este formato.
    #[inline]
    pub const fn bytes_per_pixel(&self) -> u32 {
        match self {
            Self::ARGB8888 | Self::XRGB8888 | Self::BGRA8888 | Self::RGBA8888 => 4,
            Self::RGB888 | Self::BGR888 => 3,
            Self::RGB565 | Self::Gray16 => 2,
            Self::Gray8 | Self::Alpha8 => 1,
        }
    }

    /// Retorna o número de bits por pixel.
    #[inline]
    pub const fn bits_per_pixel(&self) -> u32 {
        self.bytes_per_pixel() * 8
    }

    /// Verifica se o formato tem canal alpha.
    #[inline]
    pub const fn has_alpha(&self) -> bool {
        matches!(
            self,
            Self::ARGB8888 | Self::BGRA8888 | Self::RGBA8888 | Self::Alpha8
        )
    }

    /// Verifica se é um formato com alpha pre-multiplicado.
    #[inline]
    pub const fn is_premultiplied(&self) -> bool {
        // Para compatibilidade, assumimos que formatos com alpha não são premultiplied por padrão
        false
    }

    /// Verifica se é formato grayscale.
    #[inline]
    pub const fn is_grayscale(&self) -> bool {
        matches!(self, Self::Gray8 | Self::Gray16 | Self::Alpha8)
    }

    /// Calcula stride mínimo para uma largura.
    #[inline]
    pub const fn min_stride(&self, width: u32) -> u32 {
        width * self.bytes_per_pixel()
    }

    /// Calcula stride alinhado a N bytes.
    #[inline]
    pub const fn aligned_stride(&self, width: u32, alignment: u32) -> u32 {
        let min = self.min_stride(width);
        ((min + alignment - 1) / alignment) * alignment
    }

    /// Calcula tamanho de buffer para dimensões.
    #[inline]
    pub const fn buffer_size(&self, width: u32, height: u32) -> usize {
        (self.min_stride(width) as usize) * (height as usize)
    }

    /// Converte de valor u32.
    #[inline]
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::ARGB8888),
            1 => Some(Self::XRGB8888),
            2 => Some(Self::RGB565),
            3 => Some(Self::BGRA8888),
            4 => Some(Self::RGBA8888),
            5 => Some(Self::RGB888),
            6 => Some(Self::BGR888),
            7 => Some(Self::Gray8),
            8 => Some(Self::Gray16),
            9 => Some(Self::Alpha8),
            _ => None,
        }
    }

    /// Converte para valor u32.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        *self as u32
    }

    /// Nome do formato como string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ARGB8888 => "ARGB8888",
            Self::XRGB8888 => "XRGB8888",
            Self::RGB565 => "RGB565",
            Self::BGRA8888 => "BGRA8888",
            Self::RGBA8888 => "RGBA8888",
            Self::RGB888 => "RGB888",
            Self::BGR888 => "BGR888",
            Self::Gray8 => "Gray8",
            Self::Gray16 => "Gray16",
            Self::Alpha8 => "Alpha8",
        }
    }
}
