//! # Window Flags
//!
//! Flags para criação e comportamento de janelas.

/// Flags para criação e comportamento de janelas.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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

    /// Janela não minimizável.
    pub const NO_MINIMIZE: Self = Self(1 << 5);

    /// Janela não maximizável.
    pub const NO_MAXIMIZE: Self = Self(1 << 6);

    /// Janela não fechável.
    pub const NO_CLOSE: Self = Self(1 << 7);

    /// Janela de fundo (background/desktop).
    pub const BACKGROUND: Self = Self(1 << 8);

    /// Janela de overlay (notificações, popups).
    pub const OVERLAY: Self = Self(1 << 9);

    /// Janela modal (bloqueia parent).
    pub const MODAL: Self = Self(1 << 10);

    /// Janela de splash/loading.
    pub const SPLASH: Self = Self(1 << 11);

    /// Janela recebe input mesmo quando não está focada.
    pub const ALWAYS_RECEIVE_INPUT: Self = Self(1 << 12);

    /// Janela não aparece na taskbar.
    pub const SKIP_TASKBAR: Self = Self(1 << 13);

    /// Janela não pode ser focada.
    pub const NO_FOCUS: Self = Self(1 << 14);

    /// Janela com sombra.
    pub const HAS_SHADOW: Self = Self(1 << 15);

    // =========================================================================
    // METHODS
    // =========================================================================

    /// Cria flags a partir de valor raw.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

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

    /// Remove uma flag.
    #[inline]
    pub const fn without(&self, flag: Self) -> Self {
        Self(self.0 & !flag.0)
    }

    /// Toggle de uma flag.
    #[inline]
    pub const fn toggle(&self, flag: Self) -> Self {
        Self(self.0 ^ flag.0)
    }

    /// Valor raw.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Verifica se é uma janela de fundo.
    #[inline]
    pub const fn is_background(&self) -> bool {
        self.has(Self::BACKGROUND)
    }

    /// Verifica se é uma janela de overlay.
    #[inline]
    pub const fn is_overlay(&self) -> bool {
        self.has(Self::OVERLAY)
    }

    /// Verifica se deve ter decorações.
    #[inline]
    pub const fn has_decorations(&self) -> bool {
        !self.has(Self::BORDERLESS)
    }
}

impl core::ops::BitOr for WindowFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for WindowFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitOrAssign for WindowFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::Not for WindowFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}
