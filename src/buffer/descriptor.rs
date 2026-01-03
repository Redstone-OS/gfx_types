//! # Buffer Descriptor
//!
//! Descritor de buffers de pixels.

use crate::color::PixelFormat;
use crate::geometry::{Rect, Size};

/// Descritor de um buffer de pixels.
///
/// Define as dimensões, formato e layout de um buffer gráfico.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BufferDescriptor {
    /// Largura em pixels.
    pub width: u32,
    /// Altura em pixels.
    pub height: u32,
    /// Bytes por linha (pode ser > width * bpp para alinhamento).
    pub stride: u32,
    /// Formato dos pixels.
    pub format: PixelFormat,
}

impl BufferDescriptor {
    /// Cria novo descritor.
    #[inline]
    pub const fn new(width: u32, height: u32, format: PixelFormat) -> Self {
        let stride = width * format.bytes_per_pixel();
        Self {
            width,
            height,
            stride,
            format,
        }
    }

    /// Cria descritor com stride customizado.
    #[inline]
    pub const fn with_stride(width: u32, height: u32, stride: u32, format: PixelFormat) -> Self {
        Self {
            width,
            height,
            stride,
            format,
        }
    }

    /// Cria a partir de Size.
    #[inline]
    pub const fn from_size(size: Size, format: PixelFormat) -> Self {
        Self::new(size.width, size.height, format)
    }

    /// Tamanho do buffer.
    #[inline]
    pub const fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Retângulo do buffer (origem = 0,0).
    #[inline]
    pub const fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    /// Tamanho total do buffer em bytes.
    #[inline]
    pub const fn size_bytes(&self) -> usize {
        (self.stride as usize) * (self.height as usize)
    }

    /// Número total de pixels.
    #[inline]
    pub const fn pixel_count(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    /// Calcula offset em bytes para um pixel.
    #[inline]
    pub const fn pixel_offset(&self, x: u32, y: u32) -> usize {
        (y as usize * self.stride as usize) + (x as usize * self.format.bytes_per_pixel() as usize)
    }

    /// Calcula offset em bytes para uma linha.
    #[inline]
    pub const fn row_offset(&self, y: u32) -> usize {
        y as usize * self.stride as usize
    }

    /// Verifica se coordenadas estão dentro do buffer.
    #[inline]
    pub const fn contains(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// Verifica se o buffer é vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Bytes efetivos por linha (sem padding).
    #[inline]
    pub const fn bytes_per_row(&self) -> u32 {
        self.width * self.format.bytes_per_pixel()
    }

    /// Padding por linha em bytes.
    #[inline]
    pub const fn row_padding(&self) -> u32 {
        self.stride - self.bytes_per_row()
    }

    /// Cria descritor para uma sub-região.
    #[inline]
    pub fn sub_region(&self, rect: Rect) -> Option<(Self, usize)> {
        // Verificar bounds
        if rect.x < 0 || rect.y < 0 {
            return None;
        }
        let x = rect.x as u32;
        let y = rect.y as u32;
        if x + rect.width > self.width || y + rect.height > self.height {
            return None;
        }

        let offset = self.pixel_offset(x, y);
        let desc = Self {
            width: rect.width,
            height: rect.height,
            stride: self.stride,
            format: self.format,
        };

        Some((desc, offset))
    }
}
