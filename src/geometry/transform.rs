//! # Transform2D
//!
//! Matriz de transformação 2D para operações afins.

use super::{Point, PointF, Rect, RectF};

/// Matriz de transformação 2D (3x2 para transformações afins).
///
/// Layout:
/// ```text
/// | a  b  tx |
/// | c  d  ty |
/// | 0  0  1  |
/// ```
///
/// Onde:
/// - `a`, `d`: Scale
/// - `b`, `c`: Skew/Rotation
/// - `tx`, `ty`: Translation
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2D {
    pub a: f32,  // Scale X
    pub b: f32,  // Skew Y
    pub c: f32,  // Skew X
    pub d: f32,  // Scale Y
    pub tx: f32, // Translate X
    pub ty: f32, // Translate Y
}

impl Default for Transform2D {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform2D {
    /// Cria nova transformação com todos os valores.
    #[inline]
    pub const fn new(a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /// Matriz identidade (sem transformação).
    #[inline]
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Cria transformação de translação.
    #[inline]
    pub const fn translate(tx: f32, ty: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx,
            ty,
        }
    }

    /// Cria transformação de escala uniforme.
    #[inline]
    pub const fn scale(s: f32) -> Self {
        Self::scale_xy(s, s)
    }

    /// Cria transformação de escala não-uniforme.
    #[inline]
    pub const fn scale_xy(sx: f32, sy: f32) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Cria transformação de rotação (ângulo em radianos).
    #[inline]
    pub fn rotate(angle: f32) -> Self {
        let cos = rdsmath::cosf(angle);
        let sin = rdsmath::sinf(angle);
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Cria transformação de rotação (ângulo em graus).
    #[inline]
    pub fn rotate_degrees(degrees: f32) -> Self {
        Self::rotate(degrees * core::f32::consts::PI / 180.0)
    }

    /// Cria transformação de skew.
    #[inline]
    pub fn skew(skew_x: f32, skew_y: f32) -> Self {
        Self {
            a: 1.0,
            b: rdsmath::tanf(skew_y),
            c: rdsmath::tanf(skew_x),
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Verifica se é a matriz identidade.
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.a == 1.0
            && self.b == 0.0
            && self.c == 0.0
            && self.d == 1.0
            && self.tx == 0.0
            && self.ty == 0.0
    }

    /// Verifica se contém apenas translação.
    #[inline]
    pub fn is_translation_only(&self) -> bool {
        self.a == 1.0 && self.b == 0.0 && self.c == 0.0 && self.d == 1.0
    }

    /// Verifica se contém apenas escala e translação.
    #[inline]
    pub fn is_scale_translation(&self) -> bool {
        self.b == 0.0 && self.c == 0.0
    }

    /// Concatena com outra transformação (this * other).
    #[inline]
    pub fn then(&self, other: &Transform2D) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.tx * other.a + self.ty * other.c + other.tx,
            ty: self.tx * other.b + self.ty * other.d + other.ty,
        }
    }

    /// Pré-concatena com outra transformação (other * this).
    #[inline]
    pub fn pre(&self, other: &Transform2D) -> Self {
        other.then(self)
    }

    /// Adiciona translação.
    #[inline]
    pub fn then_translate(&self, tx: f32, ty: f32) -> Self {
        Self {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            tx: self.tx + tx,
            ty: self.ty + ty,
        }
    }

    /// Adiciona escala.
    #[inline]
    pub fn then_scale(&self, sx: f32, sy: f32) -> Self {
        Self {
            a: self.a * sx,
            b: self.b * sy,
            c: self.c * sx,
            d: self.d * sy,
            tx: self.tx * sx,
            ty: self.ty * sy,
        }
    }

    /// Adiciona rotação.
    #[inline]
    pub fn then_rotate(&self, angle: f32) -> Self {
        self.then(&Self::rotate(angle))
    }

    /// Calcula o determinante.
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.a * self.d - self.b * self.c
    }

    /// Calcula a inversa (se possível).
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(Self {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.c * self.ty - self.d * self.tx) * inv_det,
            ty: (self.b * self.tx - self.a * self.ty) * inv_det,
        })
    }

    /// Transforma um ponto.
    #[inline]
    pub fn transform_point(&self, p: PointF) -> PointF {
        PointF {
            x: self.a * p.x + self.c * p.y + self.tx,
            y: self.b * p.x + self.d * p.y + self.ty,
        }
    }

    /// Transforma um ponto inteiro.
    #[inline]
    pub fn transform_point_i(&self, p: Point) -> Point {
        self.transform_point(p.to_float()).round()
    }

    /// Transforma um retângulo (retorna bounding box).
    #[inline]
    pub fn transform_rect(&self, r: RectF) -> RectF {
        if self.is_scale_translation() {
            // Caso otimizado: sem rotação/skew
            RectF {
                x: r.x * self.a + self.tx,
                y: r.y * self.d + self.ty,
                width: r.width * rdsmath::absf(self.a),
                height: r.height * rdsmath::absf(self.d),
            }
        } else {
            // Caso geral: transformar os 4 cantos e calcular bounding box
            let p1 = self.transform_point(PointF::new(r.x, r.y));
            let p2 = self.transform_point(PointF::new(r.right(), r.y));
            let p3 = self.transform_point(PointF::new(r.x, r.bottom()));
            let p4 = self.transform_point(PointF::new(r.right(), r.bottom()));

            let min_x = p1.x.min(p2.x).min(p3.x).min(p4.x);
            let max_x = p1.x.max(p2.x).max(p3.x).max(p4.x);
            let min_y = p1.y.min(p2.y).min(p3.y).min(p4.y);
            let max_y = p1.y.max(p2.y).max(p3.y).max(p4.y);

            RectF::new(min_x, min_y, max_x - min_x, max_y - min_y)
        }
    }

    /// Transforma um retângulo inteiro.
    #[inline]
    pub fn transform_rect_i(&self, r: Rect) -> Rect {
        self.transform_rect(r.to_float()).round()
    }
}

impl core::ops::Mul for Transform2D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.then(&rhs)
    }
}

impl core::ops::Mul<PointF> for Transform2D {
    type Output = PointF;
    #[inline]
    fn mul(self, rhs: PointF) -> PointF {
        self.transform_point(rhs)
    }
}
