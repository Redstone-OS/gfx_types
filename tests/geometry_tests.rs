//! # Testes de Geometria
//!
//! Testes para os tipos geométricos.

use gfx_types::geometry::*;

// =============================================================================
// POINT TESTS
// =============================================================================

#[test]
fn test_point_new() {
    let p = Point::new(10, 20);
    assert_eq!(p.x, 10);
    assert_eq!(p.y, 20);
}

#[test]
fn test_point_zero() {
    let p = Point::ZERO;
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn test_point_offset() {
    let p = Point::new(10, 20);
    let p2 = p.offset(5, -10);
    assert_eq!(p2.x, 15);
    assert_eq!(p2.y, 10);
}

#[test]
fn test_point_add() {
    let p1 = Point::new(10, 20);
    let p2 = Point::new(5, 10);
    let p3 = p1 + p2;
    assert_eq!(p3.x, 15);
    assert_eq!(p3.y, 30);
}

#[test]
fn test_point_sub() {
    let p1 = Point::new(10, 20);
    let p2 = Point::new(5, 10);
    let p3 = p1 - p2;
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 10);
}

#[test]
fn test_point_distance_squared() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    assert_eq!(p1.distance_squared(&p2), 25);
}

#[test]
fn test_point_midpoint() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(10, 20);
    let mid = p1.midpoint(&p2);
    assert_eq!(mid.x, 5);
    assert_eq!(mid.y, 10);
}

// =============================================================================
// SIZE TESTS
// =============================================================================

#[test]
fn test_size_new() {
    let s = Size::new(100, 50);
    assert_eq!(s.width, 100);
    assert_eq!(s.height, 50);
}

#[test]
fn test_size_area() {
    let s = Size::new(100, 50);
    assert_eq!(s.area(), 5000);
}

#[test]
fn test_size_is_empty() {
    assert!(Size::ZERO.is_empty());
    assert!(Size::new(0, 100).is_empty());
    assert!(!Size::new(10, 10).is_empty());
}

// =============================================================================
// RECT TESTS
// =============================================================================

#[test]
fn test_rect_new() {
    let r = Rect::new(10, 20, 100, 50);
    assert_eq!(r.x, 10);
    assert_eq!(r.y, 20);
    assert_eq!(r.width, 100);
    assert_eq!(r.height, 50);
}

#[test]
fn test_rect_from_points() {
    let r = Rect::from_points(Point::new(10, 20), Point::new(110, 70));
    assert_eq!(r.x, 10);
    assert_eq!(r.y, 20);
    assert_eq!(r.width, 100);
    assert_eq!(r.height, 50);
}

#[test]
fn test_rect_edges() {
    let r = Rect::new(10, 20, 100, 50);
    assert_eq!(r.left(), 10);
    assert_eq!(r.top(), 20);
    assert_eq!(r.right(), 110);
    assert_eq!(r.bottom(), 70);
}

#[test]
fn test_rect_contains_point() {
    let r = Rect::new(0, 0, 100, 100);
    assert!(r.contains_point(Point::new(50, 50)));
    assert!(r.contains_point(Point::new(0, 0)));
    assert!(!r.contains_point(Point::new(100, 100))); // exclusive
    assert!(!r.contains_point(Point::new(-1, 50)));
}

#[test]
fn test_rect_intersection() {
    let r1 = Rect::new(0, 0, 100, 100);
    let r2 = Rect::new(50, 50, 100, 100);

    let inter = r1.intersection(&r2).unwrap();
    assert_eq!(inter.x, 50);
    assert_eq!(inter.y, 50);
    assert_eq!(inter.width, 50);
    assert_eq!(inter.height, 50);
}

#[test]
fn test_rect_no_intersection() {
    let r1 = Rect::new(0, 0, 50, 50);
    let r2 = Rect::new(100, 100, 50, 50);

    assert!(r1.intersection(&r2).is_none());
}

#[test]
fn test_rect_union() {
    let r1 = Rect::new(0, 0, 50, 50);
    let r2 = Rect::new(100, 100, 50, 50);

    let union = r1.union(&r2);
    assert_eq!(union.x, 0);
    assert_eq!(union.y, 0);
    assert_eq!(union.width, 150);
    assert_eq!(union.height, 150);
}

// =============================================================================
// TRANSFORM TESTS
// =============================================================================

#[test]
fn test_transform_identity() {
    let t = Transform2D::identity();
    let p = PointF::new(10.0, 20.0);
    let transformed = t.transform_point(p);

    assert!((transformed.x - 10.0).abs() < 0.0001);
    assert!((transformed.y - 20.0).abs() < 0.0001);
}

#[test]
fn test_transform_translate() {
    let t = Transform2D::translate(5.0, 10.0);
    let p = PointF::new(10.0, 20.0);
    let transformed = t.transform_point(p);

    assert!((transformed.x - 15.0).abs() < 0.0001);
    assert!((transformed.y - 30.0).abs() < 0.0001);
}

#[test]
fn test_transform_scale() {
    let t = Transform2D::scale(2.0);
    let p = PointF::new(10.0, 20.0);
    let transformed = t.transform_point(p);

    assert!((transformed.x - 20.0).abs() < 0.0001);
    assert!((transformed.y - 40.0).abs() < 0.0001);
}

// =============================================================================
// INSETS TESTS
// =============================================================================

#[test]
fn test_insets_uniform() {
    let i = Insets::uniform(10);
    assert_eq!(i.top, 10);
    assert_eq!(i.right, 10);
    assert_eq!(i.bottom, 10);
    assert_eq!(i.left, 10);
}

#[test]
fn test_insets_symmetric() {
    let i = Insets::symmetric(10, 20);
    assert_eq!(i.top, 10);
    assert_eq!(i.bottom, 10);
    assert_eq!(i.left, 20);
    assert_eq!(i.right, 20);
}
