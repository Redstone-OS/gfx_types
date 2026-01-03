//! # Circle Types
//!
//! Círculos e elipses.

use super::{PointF, RectF};

/// Círculo definido por centro e raio.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    /// Centro.
    pub center: PointF,
    /// Raio.
    pub radius: f32,
}

impl Circle {
    /// Cria novo círculo.
    #[inline]
    pub const fn new(center: PointF, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(cx: f32, cy: f32, radius: f32) -> Self {
        Self {
            center: PointF::new(cx, cy),
            radius,
        }
    }

    /// Círculo vazio na origem.
    pub const ZERO: Self = Self {
        center: PointF::ZERO,
        radius: 0.0,
    };

    /// Diâmetro.
    #[inline]
    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    /// Área.
    #[inline]
    pub fn area(&self) -> f32 {
        core::f32::consts::PI * self.radius * self.radius
    }

    /// Circunferência.
    #[inline]
    pub fn circumference(&self) -> f32 {
        2.0 * core::f32::consts::PI * self.radius
    }

    /// Bounding box.
    #[inline]
    pub fn bounds(&self) -> RectF {
        RectF::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.diameter(),
            self.diameter(),
        )
    }

    /// Verifica se contém um ponto.
    #[inline]
    pub fn contains_point(&self, p: PointF) -> bool {
        self.center.distance_squared(&p) <= self.radius * self.radius
    }

    /// Verifica se intersecta outro círculo.
    #[inline]
    pub fn intersects(&self, other: &Circle) -> bool {
        let dist = self.center.distance(&other.center);
        dist < self.radius + other.radius
    }

    /// Ponto na borda em um ângulo (radianos).
    #[inline]
    pub fn point_at_angle(&self, angle: f32) -> PointF {
        PointF::new(
            self.center.x + self.radius * rdsmath::cosf(angle),
            self.center.y + self.radius * rdsmath::sinf(angle),
        )
    }

    /// Verifica se é vazio.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.radius <= 0.0
    }

    /// Move o círculo.
    #[inline]
    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self {
            center: self.center.offset(dx, dy),
            radius: self.radius,
        }
    }

    /// Escala o círculo.
    #[inline]
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            center: self.center,
            radius: self.radius * factor,
        }
    }
}

/// Elipse definida por centro e raios.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ellipse {
    /// Centro.
    pub center: PointF,
    /// Raio X (horizontal).
    pub radius_x: f32,
    /// Raio Y (vertical).
    pub radius_y: f32,
}

impl Ellipse {
    /// Cria nova elipse.
    #[inline]
    pub const fn new(center: PointF, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center,
            radius_x,
            radius_y,
        }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        Self {
            center: PointF::new(cx, cy),
            radius_x: rx,
            radius_y: ry,
        }
    }

    /// Cria elipse inscrita em um retângulo.
    #[inline]
    pub fn inscribed(rect: RectF) -> Self {
        Self {
            center: rect.center(),
            radius_x: rect.width * 0.5,
            radius_y: rect.height * 0.5,
        }
    }

    /// Bounding box.
    #[inline]
    pub fn bounds(&self) -> RectF {
        RectF::new(
            self.center.x - self.radius_x,
            self.center.y - self.radius_y,
            self.radius_x * 2.0,
            self.radius_y * 2.0,
        )
    }

    /// Área.
    #[inline]
    pub fn area(&self) -> f32 {
        core::f32::consts::PI * self.radius_x * self.radius_y
    }

    /// Verifica se é um círculo (raios iguais).
    #[inline]
    pub fn is_circle(&self) -> bool {
        let diff = self.radius_x - self.radius_y;
        (if diff < 0.0 { -diff } else { diff }) < 0.0001
    }

    /// Converte para círculo (média dos raios).
    #[inline]
    pub fn to_circle(&self) -> Circle {
        Circle {
            center: self.center,
            radius: (self.radius_x + self.radius_y) * 0.5,
        }
    }

    /// Verifica se contém um ponto.
    #[inline]
    pub fn contains_point(&self, p: PointF) -> bool {
        if self.radius_x <= 0.0 || self.radius_y <= 0.0 {
            return false;
        }
        let dx = (p.x - self.center.x) / self.radius_x;
        let dy = (p.y - self.center.y) / self.radius_y;
        dx * dx + dy * dy <= 1.0
    }

    /// Verifica se é vazia.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.radius_x <= 0.0 || self.radius_y <= 0.0
    }

    /// Move a elipse.
    #[inline]
    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self {
            center: self.center.offset(dx, dy),
            radius_x: self.radius_x,
            radius_y: self.radius_y,
        }
    }
}
