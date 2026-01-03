//! # Rect Types
//!
//! Retângulos definidos por posição e tamanho.

use super::{Point, PointF, Size, SizeF};

// =============================================================================
// RECT (Integer)
// =============================================================================

/// Retângulo definido por posição e tamanho.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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

    /// Cria retângulo a partir de ponto e tamanho.
    #[inline]
    pub const fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Cria a partir de dois pontos (canto superior esquerdo e inferior direito).
    #[inline]
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let x1 = p1.x.min(p2.x);
        let y1 = p1.y.min(p2.y);
        let x2 = p1.x.max(p2.x);
        let y2 = p1.y.max(p2.y);
        Self {
            x: x1,
            y: y1,
            width: (x2 - x1) as u32,
            height: (y2 - y1) as u32,
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

    /// Coordenada X da borda esquerda (alias para x).
    #[inline]
    pub const fn left(&self) -> i32 {
        self.x
    }

    /// Coordenada Y da borda superior (alias para y).
    #[inline]
    pub const fn top(&self) -> i32 {
        self.y
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

    /// Centro do retângulo.
    #[inline]
    pub const fn center(&self) -> Point {
        Point {
            x: self.x + (self.width as i32 / 2),
            y: self.y + (self.height as i32 / 2),
        }
    }

    /// Verifica se o retângulo é vazio.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Área do retângulo.
    #[inline]
    pub const fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Verifica se contém um ponto.
    #[inline]
    pub fn contains_point(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.right() && p.y >= self.y && p.y < self.bottom()
    }

    /// Verifica se contém outro retângulo.
    #[inline]
    pub fn contains_rect(&self, other: &Rect) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
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

    /// Expande o retângulo em todas as direções.
    #[inline]
    pub fn expand(&self, amount: i32) -> Self {
        Self {
            x: self.x - amount,
            y: self.y - amount,
            width: (self.width as i32 + amount * 2).max(0) as u32,
            height: (self.height as i32 + amount * 2).max(0) as u32,
        }
    }

    /// Encolhe o retângulo em todas as direções.
    #[inline]
    pub fn shrink(&self, amount: i32) -> Self {
        self.expand(-amount)
    }

    /// Divide horizontalmente em duas partes.
    #[inline]
    pub fn split_horizontal(&self, at: u32) -> (Rect, Rect) {
        let at = at.min(self.width);
        (
            Rect::new(self.x, self.y, at, self.height),
            Rect::new(
                self.x + at as i32,
                self.y,
                self.width.saturating_sub(at),
                self.height,
            ),
        )
    }

    /// Divide verticalmente em duas partes.
    #[inline]
    pub fn split_vertical(&self, at: u32) -> (Rect, Rect) {
        let at = at.min(self.height);
        (
            Rect::new(self.x, self.y, self.width, at),
            Rect::new(
                self.x,
                self.y + at as i32,
                self.width,
                self.height.saturating_sub(at),
            ),
        )
    }

    /// Converte para RectF.
    #[inline]
    pub const fn to_float(&self) -> RectF {
        RectF {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

// =============================================================================
// RECTF (Floating Point)
// =============================================================================

/// Retângulo com coordenadas de ponto flutuante.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectF {
    /// Cria novo retângulo.
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Retângulo zero.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    /// Cria a partir de tamanho.
    #[inline]
    pub const fn from_size(size: SizeF) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    /// Origem.
    #[inline]
    pub const fn origin(&self) -> PointF {
        PointF::new(self.x, self.y)
    }

    /// Tamanho.
    #[inline]
    pub const fn size(&self) -> SizeF {
        SizeF::new(self.width, self.height)
    }

    /// Borda direita.
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Borda inferior.
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Centro.
    #[inline]
    pub fn center(&self) -> PointF {
        PointF {
            x: self.x + self.width * 0.5,
            y: self.y + self.height * 0.5,
        }
    }

    /// Verifica se é vazio.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Contém ponto.
    #[inline]
    pub fn contains_point(&self, p: PointF) -> bool {
        p.x >= self.x && p.x < self.right() && p.y >= self.y && p.y < self.bottom()
    }

    /// Offset.
    #[inline]
    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            width: self.width,
            height: self.height,
        }
    }

    /// Interpolação linear.
    #[inline]
    pub fn lerp(&self, other: &RectF, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            width: self.width + (other.width - self.width) * t,
            height: self.height + (other.height - self.height) * t,
        }
    }

    /// Arredonda para Rect inteiro.
    #[inline]
    pub fn round(&self) -> Rect {
        Rect {
            x: rdsmath::roundf(self.x) as i32,
            y: rdsmath::roundf(self.y) as i32,
            width: rdsmath::roundf(self.width) as u32,
            height: rdsmath::roundf(self.height) as u32,
        }
    }
}

impl From<Rect> for RectF {
    #[inline]
    fn from(r: Rect) -> Self {
        r.to_float()
    }
}

// =============================================================================
// ROUNDED RECT
// =============================================================================

/// Retângulo com cantos arredondados.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoundedRect {
    /// Retângulo base.
    pub rect: RectF,
    /// Raio dos cantos (igual para todos).
    pub radius: f32,
}

impl RoundedRect {
    /// Cria novo retângulo arredondado.
    #[inline]
    pub const fn new(rect: RectF, radius: f32) -> Self {
        Self { rect, radius }
    }

    /// Cria a partir de coordenadas.
    #[inline]
    pub const fn from_coords(x: f32, y: f32, width: f32, height: f32, radius: f32) -> Self {
        Self {
            rect: RectF::new(x, y, width, height),
            radius,
        }
    }

    /// Raio máximo permitido (metade do menor lado).
    #[inline]
    pub fn max_radius(&self) -> f32 {
        let min_side = if self.rect.width < self.rect.height {
            self.rect.width
        } else {
            self.rect.height
        };
        min_side * 0.5
    }

    /// Clamp do raio para o máximo permitido.
    #[inline]
    pub fn clamped_radius(&self) -> f32 {
        let max = self.max_radius();
        if self.radius > max {
            max
        } else {
            self.radius
        }
    }

    /// Retorna o retângulo interno (sem os cantos).
    #[inline]
    pub fn inner_rect(&self) -> RectF {
        let r = self.clamped_radius();
        RectF {
            x: self.rect.x + r,
            y: self.rect.y + r,
            width: self.rect.width - r * 2.0,
            height: self.rect.height - r * 2.0,
        }
    }
}
