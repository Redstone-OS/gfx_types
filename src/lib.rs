//! # GFX Types - Core Graphics Types
//!
//! Tipos compartilhados entre kernel e userspace para operações gráficas.
//! Este crate define a ABI estável para o subsistema de vídeo.
//!
//! ## Uso
//!
//! ```rust
//! use gfx_types::{PixelFormat, Rect, BufferDescriptor};
//! ```

#![no_std]

// ============================================================================
// PIXEL FORMAT
// ============================================================================

/// Formato de pixel suportado pelo sistema gráfico.
///
/// Define como os bytes de cor são organizados na memória.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
}

impl PixelFormat {
    /// Retorna o número de bytes por pixel para este formato.
    #[inline]
    pub const fn bytes_per_pixel(&self) -> u32 {
        match self {
            Self::ARGB8888 | Self::XRGB8888 | Self::BGRA8888 => 4,
            Self::RGB565 => 2,
        }
    }

    /// Retorna o número de bits por pixel.
    #[inline]
    pub const fn bits_per_pixel(&self) -> u32 {
        self.bytes_per_pixel() * 8
    }
}

// ============================================================================
// GEOMETRY PRIMITIVES
// ============================================================================

/// Ponto 2D com coordenadas signed (pode ser negativo para offscreen).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Cria novo ponto.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Ponto na origem (0, 0).
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Adiciona offset ao ponto.
    #[inline]
    pub const fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

/// Tamanho 2D (largura x altura).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Cria novo tamanho.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Tamanho zero.
    pub const ZERO: Self = Self {
        width: 0,
        height: 0,
    };

    /// Retorna área total em pixels.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Verifica se o tamanho é vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// Retângulo definido por posição e tamanho.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Cria novo retângulo.
    #[inline]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Retângulo vazio na origem.
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    /// Cria retângulo a partir de tamanho (posição = origem).
    #[inline]
    pub const fn from_size(size: Size) -> Self {
        Self {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }

    /// Retorna o canto superior esquerdo.
    #[inline]
    pub const fn origin(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Retorna o tamanho.
    #[inline]
    pub const fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Coordenada X da borda direita (exclusivo).
    #[inline]
    pub const fn right(&self) -> i32 {
        self.x.saturating_add(self.width as i32)
    }

    /// Coordenada Y da borda inferior (exclusivo).
    #[inline]
    pub const fn bottom(&self) -> i32 {
        self.y.saturating_add(self.height as i32)
    }

    /// Verifica se o retângulo é vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Verifica se contém um ponto.
    #[inline]
    pub fn contains_point(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.right() && p.y >= self.y && p.y < self.bottom()
    }

    /// Verifica se intersecta outro retângulo.
    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Calcula a interseção de dois retângulos.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = self.right().min(other.right());
        let y2 = self.bottom().min(other.bottom());

        if x1 < x2 && y1 < y2 {
            Some(Rect::new(x1, y1, (x2 - x1) as u32, (y2 - y1) as u32))
        } else {
            None
        }
    }

    /// Calcula a união (bounding box) de dois retângulos.
    pub fn union(&self, other: &Rect) -> Rect {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }

        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = self.right().max(other.right());
        let y2 = self.bottom().max(other.bottom());

        Rect::new(x1, y1, (x2 - x1) as u32, (y2 - y1) as u32)
    }

    /// Move o retângulo por um offset.
    #[inline]
    pub const fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            width: self.width,
            height: self.height,
        }
    }
}

// ============================================================================
// COLOR
// ============================================================================

/// Cor ARGB de 32 bits.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    // Cores básicas
    pub const TRANSPARENT: Self = Self(0x00000000);
    pub const BLACK: Self = Self(0xFF000000);
    pub const WHITE: Self = Self(0xFFFFFFFF);
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const YELLOW: Self = Self(0xFFFFFF00);
    pub const CYAN: Self = Self(0xFF00FFFF);
    pub const MAGENTA: Self = Self(0xFFFF00FF);

    // Cores do tema RedstoneOS
    pub const REDSTONE_ORANGE: Self = Self(0xFFEE6A50);
    pub const DARK_GRAY: Self = Self(0xFF333333);
    pub const LIGHT_GRAY: Self = Self(0xFFAAAAAA);

    /// Cria cor a partir de componentes RGB (alpha = 255).
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::argb(255, r, g, b)
    }

    /// Cria cor a partir de componentes ARGB.
    #[inline]
    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Extrai componente alpha.
    #[inline]
    pub const fn alpha(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Extrai componente vermelho.
    #[inline]
    pub const fn red(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Extrai componente verde.
    #[inline]
    pub const fn green(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Extrai componente azul.
    #[inline]
    pub const fn blue(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Retorna se a cor é transparente.
    #[inline]
    pub const fn is_transparent(&self) -> bool {
        self.alpha() == 0
    }

    /// Retorna se a cor é totalmente opaca.
    #[inline]
    pub const fn is_opaque(&self) -> bool {
        self.alpha() == 255
    }

    /// Valor bruto como u32.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

// ============================================================================
// BUFFER DESCRIPTOR
// ============================================================================

/// Descritor de um buffer de pixels.
///
/// Define as dimensões, formato e layout de um buffer gráfico.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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
    pub const fn with_stride(width: u32, height: u32, stride: u32, format: PixelFormat) -> Self {
        Self {
            width,
            height,
            stride,
            format,
        }
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
}

// ============================================================================
// BUFFER HANDLE
// ============================================================================

/// Handle opaco para um buffer gerenciado pelo kernel.
///
/// Este handle é usado para referenciar buffers de display sem expor
/// detalhes de implementação.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BufferHandle(pub u64);

impl BufferHandle {
    /// Handle inválido/nulo.
    pub const INVALID: Self = Self(0);

    /// Verifica se o handle é válido.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }

    /// Retorna o valor bruto.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

// ============================================================================
// DISPLAY INFO
// ============================================================================

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
}

// ============================================================================
// DAMAGE REGION
// ============================================================================

/// Região danificada (área que precisa ser recomposta).
///
/// Usado para damage tracking e otimização de composição.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DamageRegion {
    pub rect: Rect,
}

impl DamageRegion {
    /// Cria nova região de dano.
    #[inline]
    pub const fn new(rect: Rect) -> Self {
        Self { rect }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
        }
    }
}

// ============================================================================
// WINDOW FLAGS
// ============================================================================

/// Flags para criação e comportamento de janelas.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowFlags(pub u32);

impl WindowFlags {
    /// Nenhuma flag especial.
    pub const NONE: Self = Self(0);
    /// Janela sem decorações (borderless).
    pub const BORDERLESS: Self = Self(1 << 0);
    /// Janela sempre no topo.
    pub const ALWAYS_ON_TOP: Self = Self(1 << 1);
    /// Janela transparente (suporta alpha).
    pub const TRANSPARENT: Self = Self(1 << 2);
    /// Janela fullscreen.
    pub const FULLSCREEN: Self = Self(1 << 3);
    /// Janela não redimensionável.
    pub const NO_RESIZE: Self = Self(1 << 4);

    /// Verifica se uma flag está ativa.
    #[inline]
    pub const fn has(&self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    /// Combina flags.
    #[inline]
    pub const fn with(&self, flag: Self) -> Self {
        Self(self.0 | flag.0)
    }
}

// ============================================================================
// LAYER TYPE
// ============================================================================

/// Tipo de camada no compositor.
///
/// Define a ordem de desenho e comportamento de diferentes
/// tipos de superfícies.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LayerType {
    /// Fundo/wallpaper (abaixo de tudo).
    #[default]
    Background = 0,
    /// Janelas normais de aplicações.
    Normal = 1,
    /// Painéis (taskbar, dock).
    Panel = 2,
    /// Overlay (notificações, menus popup).
    Overlay = 3,
    /// Cursor do mouse (acima de tudo).
    Cursor = 4,
}
