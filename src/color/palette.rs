//! # Color Palettes
//!
//! Paletas de cores predefinidas.

use super::Color;

/// Paleta de cores nomeadas.
#[derive(Clone, Debug)]
pub struct Palette {
    /// Nome da paleta.
    pub name: &'static str,
    /// Cores da paleta.
    pub colors: &'static [Color],
}

impl Palette {
    /// Cria nova paleta.
    #[inline]
    pub const fn new(name: &'static str, colors: &'static [Color]) -> Self {
        Self { name, colors }
    }

    /// Número de cores na paleta.
    #[inline]
    pub const fn len(&self) -> usize {
        self.colors.len()
    }

    /// Verifica se está vazia.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Obtém cor por índice.
    #[inline]
    pub fn get(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }
}

// =============================================================================
// PREDEFINED PALETTES
// =============================================================================

/// Catppuccin Mocha (dark theme).
pub const CATPPUCCIN_MOCHA: Palette = Palette {
    name: "Catppuccin Mocha",
    colors: &[
        Color(0xFF1E1E2E), // Base
        Color(0xFF181825), // Mantle
        Color(0xFF11111B), // Crust
        Color(0xFFCDD6F4), // Text
        Color(0xFFBAC2DE), // Subtext1
        Color(0xFFA6ADC8), // Subtext0
        Color(0xFF9399B2), // Overlay2
        Color(0xFF7F849C), // Overlay1
        Color(0xFF6C7086), // Overlay0
        Color(0xFF585B70), // Surface2
        Color(0xFF45475A), // Surface1
        Color(0xFF313244), // Surface0
        Color(0xFFF5E0DC), // Rosewater
        Color(0xFFF2CDCD), // Flamingo
        Color(0xFFF5C2E7), // Pink
        Color(0xFFCBA6F7), // Mauve
        Color(0xFFFF0000), // Red
        Color(0xFFFAB387), // Peach
        Color(0xFFF9E2AF), // Yellow
        Color(0xFFA6E3A1), // Green
        Color(0xFF94E2D5), // Teal
        Color(0xFF89DCEB), // Sky
        Color(0xFF74C7EC), // Sapphire
        Color(0xFF89B4FA), // Blue
        Color(0xFFB4BEFE), // Lavender
    ],
};

/// Catppuccin Latte (light theme).
pub const CATPPUCCIN_LATTE: Palette = Palette {
    name: "Catppuccin Latte",
    colors: &[
        Color(0xFFEFF1F5), // Base
        Color(0xFFE6E9EF), // Mantle
        Color(0xFFDCE0E8), // Crust
        Color(0xFF4C4F69), // Text
        Color(0xFF5C5F77), // Subtext1
        Color(0xFF6C6F85), // Subtext0
        Color(0xFF7C7F93), // Overlay2
        Color(0xFF8C8FA1), // Overlay1
        Color(0xFF9CA0B0), // Overlay0
        Color(0xFFACB0BE), // Surface2
        Color(0xFFBCC0CC), // Surface1
        Color(0xFFCCD0DA), // Surface0
        Color(0xFFDC8A78), // Rosewater
        Color(0xFFDD7878), // Flamingo
        Color(0xFFEA76CB), // Pink
        Color(0xFF8839EF), // Mauve
        Color(0xFFD20F39), // Red
        Color(0xFFFE640B), // Peach
        Color(0xFFDF8E1D), // Yellow
        Color(0xFF40A02B), // Green
        Color(0xFF179299), // Teal
        Color(0xFF04A5E5), // Sky
        Color(0xFF209FB5), // Sapphire
        Color(0xFF1E66F5), // Blue
        Color(0xFF7287FD), // Lavender
    ],
};

/// Dracula theme.
pub const DRACULA: Palette = Palette {
    name: "Dracula",
    colors: &[
        Color(0xFF282A36), // Background
        Color(0xFF44475A), // Current Line
        Color(0xFFF8F8F2), // Foreground
        Color(0xFF6272A4), // Comment
        Color(0xFF8BE9FD), // Cyan
        Color(0xFF50FA7B), // Green
        Color(0xFFFFB86C), // Orange
        Color(0xFFFF79C6), // Pink
        Color(0xFFBD93F9), // Purple
        Color(0xFFFF5555), // Red
        Color(0xFFF1FA8C), // Yellow
    ],
};

/// Nord theme.
pub const NORD: Palette = Palette {
    name: "Nord",
    colors: &[
        Color(0xFF2E3440), // Polar Night 0
        Color(0xFF3B4252), // Polar Night 1
        Color(0xFF434C5E), // Polar Night 2
        Color(0xFF4C566A), // Polar Night 3
        Color(0xFFD8DEE9), // Snow Storm 0
        Color(0xFFE5E9F0), // Snow Storm 1
        Color(0xFFECEFF4), // Snow Storm 2
        Color(0xFF8FBCBB), // Frost 0
        Color(0xFF88C0D0), // Frost 1
        Color(0xFF81A1C1), // Frost 2
        Color(0xFF5E81AC), // Frost 3
        Color(0xFFBF616A), // Aurora Red
        Color(0xFFD08770), // Aurora Orange
        Color(0xFFEBCB8B), // Aurora Yellow
        Color(0xFFA3BE8C), // Aurora Green
        Color(0xFFB48EAD), // Aurora Purple
    ],
};

/// RedstoneOS default theme.
pub const REDSTONE_DEFAULT: Palette = Palette {
    name: "RedstoneOS",
    colors: &[
        Color(0xFF1E1E2E), // Background
        Color(0xFF2D2D2D), // Surface
        Color(0xFF45475A), // Surface Light
        Color(0xFFCDD6F4), // Text
        Color(0xFFA6ADC8), // Text Muted
        Color(0xFFEE6A50), // Primary (Redstone Orange)
        Color(0xFF89B4FA), // Accent (Blue)
        Color(0xFFA6E3A1), // Success (Green)
        Color(0xFFF9E2AF), // Warning (Yellow)
        Color(0xFFF38BA8), // Error (Red)
    ],
};
