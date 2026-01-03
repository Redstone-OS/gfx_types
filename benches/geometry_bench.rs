//! # Benchmarks de Geometria
//!
//! Benchmarks para operações geométricas.
//!
//! Para rodar: cargo bench (requer nightly e criterion)

#![feature(test)]

extern crate test;

use gfx_types::geometry::*;
use test::Bencher;

// =============================================================================
// POINT BENCHMARKS
// =============================================================================

#[bench]
fn bench_point_add(b: &mut Bencher) {
    let p1 = Point::new(100, 200);
    let p2 = Point::new(50, 75);

    b.iter(|| test::black_box(p1 + p2));
}

#[bench]
fn bench_point_distance_squared(b: &mut Bencher) {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(100, 100);

    b.iter(|| test::black_box(p1.distance_squared(&p2)));
}

#[bench]
fn bench_pointf_distance(b: &mut Bencher) {
    let p1 = PointF::new(0.0, 0.0);
    let p2 = PointF::new(100.0, 100.0);

    b.iter(|| test::black_box(p1.distance(&p2)));
}

#[bench]
fn bench_pointf_normalize(b: &mut Bencher) {
    let p = PointF::new(3.0, 4.0);

    b.iter(|| test::black_box(p.normalize()));
}

// =============================================================================
// RECT BENCHMARKS
// =============================================================================

#[bench]
fn bench_rect_contains_point(b: &mut Bencher) {
    let r = Rect::new(0, 0, 1000, 1000);
    let p = Point::new(500, 500);

    b.iter(|| test::black_box(r.contains_point(p)));
}

#[bench]
fn bench_rect_intersection(b: &mut Bencher) {
    let r1 = Rect::new(0, 0, 100, 100);
    let r2 = Rect::new(50, 50, 100, 100);

    b.iter(|| test::black_box(r1.intersection(&r2)));
}

#[bench]
fn bench_rect_union(b: &mut Bencher) {
    let r1 = Rect::new(0, 0, 100, 100);
    let r2 = Rect::new(200, 200, 100, 100);

    b.iter(|| test::black_box(r1.union(&r2)));
}

// =============================================================================
// TRANSFORM BENCHMARKS
// =============================================================================

#[bench]
fn bench_transform_point(b: &mut Bencher) {
    let t = Transform2D::translate(10.0, 20.0)
        .then_scale(2.0, 2.0)
        .then_rotate(0.5);
    let p = PointF::new(50.0, 50.0);

    b.iter(|| test::black_box(t.transform_point(p)));
}

#[bench]
fn bench_transform_compose(b: &mut Bencher) {
    let t1 = Transform2D::translate(10.0, 20.0);
    let t2 = Transform2D::scale(2.0);

    b.iter(|| test::black_box(t1.then(&t2)));
}

#[bench]
fn bench_transform_inverse(b: &mut Bencher) {
    let t = Transform2D::translate(10.0, 20.0).then_scale(2.0, 2.0);

    b.iter(|| test::black_box(t.inverse()));
}

#[bench]
fn bench_transform_rotate(b: &mut Bencher) {
    b.iter(|| test::black_box(Transform2D::rotate(0.5)));
}
