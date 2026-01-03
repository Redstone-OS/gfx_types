//! # Color Types
//!
//! Representação de cores ARGB.

// =============================================================================
// COLOR (32-bit ARGB)
// =============================================================================

/// Cor ARGB de 32 bits.
///
/// Layout: `0xAARRGGBB` (Alpha, Red, Green, Blue)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Color(pub u32);

impl Color {
    // =========================================================================
    // BASIC COLORS
    // =========================================================================

    pub const TRANSPARENT: Self = Self(0x00000000);
    pub const BLACK: Self = Self(0xFF000000);
    pub const WHITE: Self = Self(0xFFFFFFFF);
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const YELLOW: Self = Self(0xFFFFFF00);
    pub const CYAN: Self = Self(0xFF00FFFF);
    pub const MAGENTA: Self = Self(0xFFFF00FF);
    pub const ORANGE: Self = Self(0xFFFF8000);
    pub const PURPLE: Self = Self(0xFF800080);
    pub const PINK: Self = Self(0xFFFF69B4);
    pub const BROWN: Self = Self(0xFF8B4513);
    pub const GRAY: Self = Self(0xFF808080);
    pub const DARK_GRAY: Self = Self(0xFF404040);
    pub const LIGHT_GRAY: Self = Self(0xFFC0C0C0);

    // =========================================================================
    // REDSTONE OS THEME
    // =========================================================================

    pub const REDSTONE_PRIMARY: Self = Self(0xFFEE6A50);
    pub const REDSTONE_SECONDARY: Self = Self(0xFF2D2D2D);
    pub const REDSTONE_ACCENT: Self = Self(0xFF89B4FA);
    pub const REDSTONE_SURFACE: Self = Self(0xFF1E1E2E);
    pub const REDSTONE_TEXT: Self = Self(0xFFCDD6F4);

    // =========================================================================
    // CONSTRUCTORS
    // =========================================================================

    /// Cria cor a partir de valor raw.
    #[inline]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

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

    /// Cria cor a partir de componentes RGBA.
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::argb(a, r, g, b)
    }

    /// Cria cor em grayscale.
    #[inline]
    pub const fn gray(level: u8) -> Self {
        Self::rgb(level, level, level)
    }

    /// Cria cor a partir de valor hex (sem alpha, assume 0xFF).
    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        Self(0xFF000000 | (hex & 0x00FFFFFF))
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

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

    /// Retorna tupla (r, g, b).
    #[inline]
    pub const fn to_rgb(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    /// Retorna tupla (a, r, g, b).
    #[inline]
    pub const fn to_argb(&self) -> (u8, u8, u8, u8) {
        (self.alpha(), self.red(), self.green(), self.blue())
    }

    /// Retorna tupla (r, g, b, a).
    #[inline]
    pub const fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red(), self.green(), self.blue(), self.alpha())
    }

    /// Valor bruto como u32.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    // =========================================================================
    // PREDICATES
    // =========================================================================

    /// Retorna se a cor é totalmente transparente.
    #[inline]
    pub const fn is_transparent(&self) -> bool {
        self.alpha() == 0
    }

    /// Retorna se a cor é totalmente opaca.
    #[inline]
    pub const fn is_opaque(&self) -> bool {
        self.alpha() == 255
    }

    // =========================================================================
    // MODIFIERS
    // =========================================================================

    /// Retorna cor com novo alpha.
    #[inline]
    pub const fn with_alpha(&self, a: u8) -> Self {
        Self((self.0 & 0x00FFFFFF) | ((a as u32) << 24))
    }

    /// Retorna cor com novo red.
    #[inline]
    pub const fn with_red(&self, r: u8) -> Self {
        Self((self.0 & 0xFF00FFFF) | ((r as u32) << 16))
    }

    /// Retorna cor com novo green.
    #[inline]
    pub const fn with_green(&self, g: u8) -> Self {
        Self((self.0 & 0xFFFF00FF) | ((g as u32) << 8))
    }

    /// Retorna cor com novo blue.
    #[inline]
    pub const fn with_blue(&self, b: u8) -> Self {
        Self((self.0 & 0xFFFFFF00) | (b as u32))
    }

    /// Multiplica alpha por um fator (0.0 - 1.0).
    #[inline]
    pub fn multiply_alpha(&self, factor: f32) -> Self {
        let a = (self.alpha() as f32 * factor).clamp(0.0, 255.0) as u8;
        self.with_alpha(a)
    }

    /// Inverte a cor (não inverte alpha).
    #[inline]
    pub const fn invert(&self) -> Self {
        Self::argb(
            self.alpha(),
            255 - self.red(),
            255 - self.green(),
            255 - self.blue(),
        )
    }

    /// Retorna luminância percebida (0-255).
    #[inline]
    pub fn luminance(&self) -> u8 {
        // Fórmula padrão: 0.299*R + 0.587*G + 0.114*B
        let r = self.red() as f32;
        let g = self.green() as f32;
        let b = self.blue() as f32;
        (0.299 * r + 0.587 * g + 0.114 * b) as u8
    }

    /// Converte para grayscale mantendo alpha.
    #[inline]
    pub fn to_grayscale(&self) -> Self {
        let lum = self.luminance();
        Self::argb(self.alpha(), lum, lum, lum)
    }

    /// Interpolação linear entre duas cores.
    #[inline]
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;

        let a = (self.alpha() as f32 * inv_t + other.alpha() as f32 * t) as u8;
        let r = (self.red() as f32 * inv_t + other.red() as f32 * t) as u8;
        let g = (self.green() as f32 * inv_t + other.green() as f32 * t) as u8;
        let b = (self.blue() as f32 * inv_t + other.blue() as f32 * t) as u8;

        Self::argb(a, r, g, b)
    }

    /// Converte para ColorF.
    #[inline]
    pub fn to_float(&self) -> ColorF {
        ColorF {
            r: self.red() as f32 / 255.0,
            g: self.green() as f32 / 255.0,
            b: self.blue() as f32 / 255.0,
            a: self.alpha() as f32 / 255.0,
        }
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(raw: u32) -> Self {
        Self(raw)
    }
}

impl From<Color> for u32 {
    #[inline]
    fn from(c: Color) -> Self {
        c.0
    }
}

// =============================================================================
// COLORF (Floating Point)
// =============================================================================

/// Cor com componentes de ponto flutuante [0.0, 1.0].
///
/// Útil para cálculos de blending e gamma correction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorF {
    /// Cria nova cor.
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Cor preta opaca.
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Cor branca opaca.
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Cor transparente.
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    /// Cria cor RGB (alpha = 1.0).
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Verifica se é transparente.
    #[inline]
    pub fn is_transparent(&self) -> bool {
        self.a <= 0.0
    }

    /// Verifica se é opaco.
    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.a >= 1.0
    }

    /// Clamp de todos os componentes para [0.0, 1.0].
    #[inline]
    pub fn saturate(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Interpolação linear.
    #[inline]
    pub fn lerp(&self, other: &ColorF, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Converte para Color (8-bit).
    #[inline]
    pub fn to_color(&self) -> Color {
        let s = self.saturate();
        Color::argb(
            (s.a * 255.0) as u8,
            (s.r * 255.0) as u8,
            (s.g * 255.0) as u8,
            (s.b * 255.0) as u8,
        )
    }

    /// Pre-multiplied alpha.
    #[inline]
    pub fn premultiply(&self) -> Self {
        Self {
            r: self.r * self.a,
            g: self.g * self.a,
            b: self.b * self.a,
            a: self.a,
        }
    }

    /// Un-premultiply alpha.
    #[inline]
    pub fn unpremultiply(&self) -> Self {
        if self.a <= 0.0 {
            return Self::TRANSPARENT;
        }
        let inv_a = 1.0 / self.a;
        Self {
            r: self.r * inv_a,
            g: self.g * inv_a,
            b: self.b * inv_a,
            a: self.a,
        }
    }
}

impl From<Color> for ColorF {
    #[inline]
    fn from(c: Color) -> Self {
        c.to_float()
    }
}

impl From<ColorF> for Color {
    #[inline]
    fn from(c: ColorF) -> Self {
        c.to_color()
    }
}
