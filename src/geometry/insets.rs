//! # Insets
//!
//! Margens e padding (top, right, bottom, left).

use core::ops::{Add, Sub};

/// Margens em todas as direções.
///
/// Usado para padding, margens e bordas.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Insets {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl Insets {
    /// Cria insets com valores individuais.
    #[inline]
    pub const fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Insets zero.
    pub const ZERO: Self = Self {
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    };

    /// Insets uniformes (mesmo valor em todas as direções).
    #[inline]
    pub const fn uniform(value: i32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Insets simétricos (vertical e horizontal).
    #[inline]
    pub const fn symmetric(vertical: i32, horizontal: i32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Insets apenas no topo.
    #[inline]
    pub const fn only_top(top: i32) -> Self {
        Self {
            top,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }

    /// Insets apenas na direita.
    #[inline]
    pub const fn only_right(right: i32) -> Self {
        Self {
            top: 0,
            right,
            bottom: 0,
            left: 0,
        }
    }

    /// Insets apenas embaixo.
    #[inline]
    pub const fn only_bottom(bottom: i32) -> Self {
        Self {
            top: 0,
            right: 0,
            bottom,
            left: 0,
        }
    }

    /// Insets apenas na esquerda.
    #[inline]
    pub const fn only_left(left: i32) -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left,
        }
    }

    /// Soma horizontal (left + right).
    #[inline]
    pub const fn horizontal(&self) -> i32 {
        self.left + self.right
    }

    /// Soma vertical (top + bottom).
    #[inline]
    pub const fn vertical(&self) -> i32 {
        self.top + self.bottom
    }

    /// Verifica se todos os valores são zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.top == 0 && self.right == 0 && self.bottom == 0 && self.left == 0
    }

    /// Verifica se todos os valores são não-negativos.
    #[inline]
    pub const fn is_non_negative(&self) -> bool {
        self.top >= 0 && self.right >= 0 && self.bottom >= 0 && self.left >= 0
    }

    /// Retorna insets com valores absolutos.
    #[inline]
    pub const fn abs(&self) -> Self {
        Self {
            top: if self.top < 0 { -self.top } else { self.top },
            right: if self.right < 0 {
                -self.right
            } else {
                self.right
            },
            bottom: if self.bottom < 0 {
                -self.bottom
            } else {
                self.bottom
            },
            left: if self.left < 0 { -self.left } else { self.left },
        }
    }

    /// Clamp de todos os valores para um mínimo.
    #[inline]
    pub const fn max(&self, min: i32) -> Self {
        Self {
            top: if self.top > min { self.top } else { min },
            right: if self.right > min { self.right } else { min },
            bottom: if self.bottom > min { self.bottom } else { min },
            left: if self.left > min { self.left } else { min },
        }
    }
}

impl Add for Insets {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
        }
    }
}

impl Sub for Insets {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
            left: self.left - rhs.left,
        }
    }
}

/// Alias para Insets.
pub type EdgeInsets = Insets;

/// Alias para Insets (padding).
pub type Padding = Insets;

/// Alias para Insets (margin).
pub type Margin = Insets;
