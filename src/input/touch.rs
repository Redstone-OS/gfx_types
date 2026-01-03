//! # Touch Input
//!
//! Tipos para entrada por toque.

use crate::geometry::PointF;

/// ID único de um toque.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct TouchId(pub u32);

impl TouchId {
    /// Cria novo ID.
    #[inline]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// ID inválido.
    pub const INVALID: Self = Self(u32::MAX);

    /// Verifica se é válido.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != u32::MAX
    }
}

/// Fase de um toque.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum TouchPhase {
    /// Toque iniciado.
    #[default]
    Begin = 0,
    /// Toque movido.
    Move = 1,
    /// Toque terminado (levantou).
    End = 2,
    /// Toque cancelado.
    Cancel = 3,
}

impl TouchPhase {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Begin),
            1 => Some(Self::Move),
            2 => Some(Self::End),
            3 => Some(Self::Cancel),
            _ => None,
        }
    }

    /// Verifica se é um toque ativo.
    #[inline]
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Begin | Self::Move)
    }

    /// Verifica se é o fim do toque.
    #[inline]
    pub const fn is_end(&self) -> bool {
        matches!(self, Self::End | Self::Cancel)
    }
}

/// Ponto de toque.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TouchPoint {
    /// ID do toque.
    pub id: TouchId,
    /// Fase atual.
    pub phase: TouchPhase,
    /// Posição.
    pub position: PointF,
    /// Pressão (0.0 - 1.0, 0 se não suportado).
    pub pressure: f32,
    /// Raio do toque em pixels (0 se não suportado).
    pub radius: f32,
}

impl TouchPoint {
    /// Cria novo ponto de toque.
    #[inline]
    pub const fn new(id: TouchId, phase: TouchPhase, position: PointF) -> Self {
        Self {
            id,
            phase,
            position,
            pressure: 1.0,
            radius: 0.0,
        }
    }

    /// Com pressão.
    #[inline]
    pub const fn with_pressure(mut self, pressure: f32) -> Self {
        self.pressure = pressure;
        self
    }

    /// Com raio.
    #[inline]
    pub const fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

/// Tipo de gesto.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GestureType {
    /// Toque simples.
    Tap = 0,
    /// Toque duplo.
    DoubleTap = 1,
    /// Toque longo.
    LongPress = 2,
    /// Swipe.
    Swipe = 3,
    /// Pinch (zoom).
    Pinch = 4,
    /// Rotate.
    Rotate = 5,
    /// Pan (arrastar).
    Pan = 6,
}

impl GestureType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Tap),
            1 => Some(Self::DoubleTap),
            2 => Some(Self::LongPress),
            3 => Some(Self::Swipe),
            4 => Some(Self::Pinch),
            5 => Some(Self::Rotate),
            6 => Some(Self::Pan),
            _ => None,
        }
    }

    /// Nome do gesto.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Tap => "Tap",
            Self::DoubleTap => "Double Tap",
            Self::LongPress => "Long Press",
            Self::Swipe => "Swipe",
            Self::Pinch => "Pinch",
            Self::Rotate => "Rotate",
            Self::Pan => "Pan",
        }
    }

    /// Número mínimo de toques para este gesto.
    #[inline]
    pub const fn min_touches(&self) -> usize {
        match self {
            Self::Tap | Self::DoubleTap | Self::LongPress | Self::Swipe | Self::Pan => 1,
            Self::Pinch | Self::Rotate => 2,
        }
    }
}

/// Direção de swipe.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwipeDirection {
    /// Para cima.
    Up = 0,
    /// Para baixo.
    Down = 1,
    /// Para esquerda.
    Left = 2,
    /// Para direita.
    Right = 3,
}

impl SwipeDirection {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Up),
            1 => Some(Self::Down),
            2 => Some(Self::Left),
            3 => Some(Self::Right),
            _ => None,
        }
    }

    /// Direção oposta.
    #[inline]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Verifica se é horizontal.
    #[inline]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    /// Verifica se é vertical.
    #[inline]
    pub const fn is_vertical(&self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
}
