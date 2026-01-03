//! # Polygon Types
//!
//! Polígonos e paths.

#[cfg(feature = "alloc")]
extern crate alloc;

use super::PointF;

/// Número máximo de pontos em um polígono sem alocação.
pub const MAX_STATIC_POINTS: usize = 16;

/// Polígono com pontos estáticos (sem alocação).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StaticPolygon {
    /// Pontos do polígono.
    points: [PointF; MAX_STATIC_POINTS],
    /// Número de pontos válidos.
    count: usize,
    /// Polígono fechado?
    closed: bool,
}

impl Default for StaticPolygon {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticPolygon {
    /// Cria polígono vazio.
    #[inline]
    pub const fn new() -> Self {
        Self {
            points: [PointF::ZERO; MAX_STATIC_POINTS],
            count: 0,
            closed: true,
        }
    }

    /// Número de pontos.
    #[inline]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Verifica se está vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Verifica se está fechado.
    #[inline]
    pub const fn is_closed(&self) -> bool {
        self.closed
    }

    /// Define se está fechado.
    #[inline]
    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    /// Adiciona um ponto.
    #[inline]
    pub fn push(&mut self, point: PointF) -> bool {
        if self.count >= MAX_STATIC_POINTS {
            return false;
        }
        self.points[self.count] = point;
        self.count += 1;
        true
    }

    /// Obtém um ponto por índice.
    #[inline]
    pub fn get(&self, index: usize) -> Option<PointF> {
        if index < self.count {
            Some(self.points[index])
        } else {
            None
        }
    }

    /// Iterador sobre os pontos.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PointF> {
        self.points[..self.count].iter()
    }

    /// Limpa o polígono.
    #[inline]
    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Cria triângulo.
    #[inline]
    pub fn triangle(p1: PointF, p2: PointF, p3: PointF) -> Self {
        let mut poly = Self::new();
        poly.push(p1);
        poly.push(p2);
        poly.push(p3);
        poly
    }

    /// Cria quadrilátero.
    #[inline]
    pub fn quad(p1: PointF, p2: PointF, p3: PointF, p4: PointF) -> Self {
        let mut poly = Self::new();
        poly.push(p1);
        poly.push(p2);
        poly.push(p3);
        poly.push(p4);
        poly
    }
}

/// Tipo de segmento de path.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment {
    /// Move para um ponto.
    MoveTo = 0,
    /// Linha até um ponto.
    LineTo = 1,
    /// Curva quadrática (1 ponto de controle).
    QuadTo = 2,
    /// Curva cúbica (2 pontos de controle).
    CubicTo = 3,
    /// Fecha o path.
    Close = 4,
}

impl PathSegment {
    /// Número de pontos consumidos por este segmento.
    #[inline]
    pub const fn point_count(&self) -> usize {
        match self {
            Self::MoveTo | Self::LineTo => 1,
            Self::QuadTo => 2,
            Self::CubicTo => 3,
            Self::Close => 0,
        }
    }
}

/// Winding rule para fill de paths.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum FillRule {
    /// Non-zero winding.
    #[default]
    NonZero = 0,
    /// Even-odd.
    EvenOdd = 1,
}
