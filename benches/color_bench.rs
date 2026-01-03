//! # Benchmarks de Cores
//!
//! Benchmarks para operações de cor.

#![feature(test)]

extern crate test;

use gfx_types::color::*;
use test::Bencher;

// =============================================================================
// COLOR BENCHMARKS
// =============================================================================

#[bench]
fn bench_color_rgb(b: &mut Bencher) {
    b.iter(|| test::black_box(Color::rgb(255, 128, 64)));
}

#[bench]
fn bench_color_extract_components(b: &mut Bencher) {
    let c = Color::rgb(255, 128, 64);

    b.iter(|| test::black_box((c.red(), c.green(), c.blue(), c.alpha())));
}

#[bench]
fn bench_color_lerp(b: &mut Bencher) {
    let c1 = Color::RED;
    let c2 = Color::BLUE;

    b.iter(|| test::black_box(c1.lerp(&c2, 0.5)));
}

#[bench]
fn bench_color_invert(b: &mut Bencher) {
    let c = Color::rgb(100, 150, 200);

    b.iter(|| test::black_box(c.invert()));
}

#[bench]
fn bench_color_grayscale(b: &mut Bencher) {
    let c = Color::rgb(100, 150, 200);

    b.iter(|| test::black_box(c.to_grayscale()));
}

// =============================================================================
// COLORF BENCHMARKS
// =============================================================================

#[bench]
fn bench_colorf_lerp(b: &mut Bencher) {
    let c1 = ColorF::new(0.0, 0.0, 0.0, 1.0);
    let c2 = ColorF::new(1.0, 1.0, 1.0, 1.0);

    b.iter(|| test::black_box(c1.lerp(&c2, 0.5)));
}

#[bench]
fn bench_colorf_to_color(b: &mut Bencher) {
    let cf = ColorF::new(0.5, 0.5, 0.5, 1.0);

    b.iter(|| test::black_box(cf.to_color()));
}

#[bench]
fn bench_colorf_saturate(b: &mut Bencher) {
    let cf = ColorF::new(1.5, -0.5, 0.5, 1.0);

    b.iter(|| test::black_box(cf.saturate()));
}

#[bench]
fn bench_color_from_colorf(b: &mut Bencher) {
    let cf = ColorF::new(0.8, 0.6, 0.4, 1.0);

    b.iter(|| test::black_box(Color::from(cf)));
}
