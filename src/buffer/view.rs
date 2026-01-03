//! # Buffer View
//!
//! Views para acesso a buffers.

use crate::buffer::BufferDescriptor;
use crate::color::PixelFormat;

/// View imutável de um buffer de pixels.
#[derive(Clone, Copy, Debug)]
pub struct BufferView<'a> {
    /// Dados do buffer.
    data: &'a [u8],
    /// Descritor do buffer.
    desc: BufferDescriptor,
}

impl<'a> BufferView<'a> {
    /// Cria nova view.
    #[inline]
    pub fn new(data: &'a [u8], desc: BufferDescriptor) -> Option<Self> {
        if data.len() >= desc.size_bytes() {
            Some(Self { data, desc })
        } else {
            None
        }
    }

    /// Cria view sem verificação de tamanho.
    ///
    /// # Safety
    /// O slice deve ter pelo menos `desc.size_bytes()` bytes.
    #[inline]
    pub unsafe fn new_unchecked(data: &'a [u8], desc: BufferDescriptor) -> Self {
        Self { data, desc }
    }

    /// Descritor do buffer.
    #[inline]
    pub const fn descriptor(&self) -> &BufferDescriptor {
        &self.desc
    }

    /// Largura.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.desc.width
    }

    /// Altura.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.desc.height
    }

    /// Stride em bytes.
    #[inline]
    pub const fn stride(&self) -> u32 {
        self.desc.stride
    }

    /// Formato de pixel.
    #[inline]
    pub const fn format(&self) -> PixelFormat {
        self.desc.format
    }

    /// Dados raw.
    #[inline]
    pub fn data(&self) -> &[u8] {
        self.data
    }

    /// Obtém slice de uma linha.
    #[inline]
    pub fn row(&self, y: u32) -> Option<&[u8]> {
        if y >= self.desc.height {
            return None;
        }
        let start = self.desc.row_offset(y);
        let end = start + self.desc.bytes_per_row() as usize;
        Some(&self.data[start..end])
    }

    /// Obtém offset de um pixel.
    #[inline]
    pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.desc.width || y >= self.desc.height {
            return None;
        }
        Some(self.desc.pixel_offset(x, y))
    }
}

/// View mutável de um buffer de pixels.
#[derive(Debug)]
pub struct BufferViewMut<'a> {
    /// Dados do buffer.
    data: &'a mut [u8],
    /// Descritor do buffer.
    desc: BufferDescriptor,
}

impl<'a> BufferViewMut<'a> {
    /// Cria nova view mutável.
    #[inline]
    pub fn new(data: &'a mut [u8], desc: BufferDescriptor) -> Option<Self> {
        if data.len() >= desc.size_bytes() {
            Some(Self { data, desc })
        } else {
            None
        }
    }

    /// Cria view sem verificação de tamanho.
    ///
    /// # Safety
    /// O slice deve ter pelo menos `desc.size_bytes()` bytes.
    #[inline]
    pub unsafe fn new_unchecked(data: &'a mut [u8], desc: BufferDescriptor) -> Self {
        Self { data, desc }
    }

    /// Descritor do buffer.
    #[inline]
    pub const fn descriptor(&self) -> &BufferDescriptor {
        &self.desc
    }

    /// Largura.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.desc.width
    }

    /// Altura.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.desc.height
    }

    /// Stride em bytes.
    #[inline]
    pub const fn stride(&self) -> u32 {
        self.desc.stride
    }

    /// Formato de pixel.
    #[inline]
    pub const fn format(&self) -> PixelFormat {
        self.desc.format
    }

    /// Dados raw imutáveis.
    #[inline]
    pub fn data(&self) -> &[u8] {
        self.data
    }

    /// Dados raw mutáveis.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    /// Obtém slice mutável de uma linha.
    #[inline]
    pub fn row_mut(&mut self, y: u32) -> Option<&mut [u8]> {
        if y >= self.desc.height {
            return None;
        }
        let start = self.desc.row_offset(y);
        let end = start + self.desc.bytes_per_row() as usize;
        Some(&mut self.data[start..end])
    }

    /// Preenche o buffer com um valor.
    #[inline]
    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    /// Limpa o buffer (preenche com zero).
    #[inline]
    pub fn clear(&mut self) {
        self.fill(0);
    }
}
