//! # Testes de Cores
//!
//! Testes para os tipos de cores.

use gfx_types::color::*;

// =============================================================================
// COLOR TESTS
// =============================================================================

#[test]
fn test_color_rgb() {
    let c = Color::rgb(255, 128, 64);
    assert_eq!(c.red(), 255);
    assert_eq!(c.green(), 128);
    assert_eq!(c.blue(), 64);
    assert_eq!(c.alpha(), 255); // opaque by default
}

#[test]
fn test_color_argb() {
    let c = Color::argb(128, 255, 128, 64);
    assert_eq!(c.alpha(), 128);
    assert_eq!(c.red(), 255);
    assert_eq!(c.green(), 128);
    assert_eq!(c.blue(), 64);
}

#[test]
fn test_color_from_hex() {
    let c = Color::from_hex(0xFF8040);
    assert_eq!(c.red(), 255);
    assert_eq!(c.green(), 128);
    assert_eq!(c.blue(), 64);
}

#[test]
fn test_color_with_alpha() {
    let c = Color::RED.with_alpha(128);
    assert_eq!(c.alpha(), 128);
    assert_eq!(c.red(), 255);
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::WHITE.0, 0xFFFFFFFF);
    assert_eq!(Color::BLACK.0, 0xFF000000);
    assert_eq!(Color::RED.red(), 255);
    assert_eq!(Color::GREEN.green(), 255);
    assert_eq!(Color::BLUE.blue(), 255);
    assert_eq!(Color::TRANSPARENT.alpha(), 0);
}

#[test]
fn test_color_invert() {
    let c = Color::rgb(100, 150, 200);
    let inv = c.invert();
    assert_eq!(inv.red(), 155);
    assert_eq!(inv.green(), 105);
    assert_eq!(inv.blue(), 55);
}

#[test]
fn test_color_grayscale() {
    let c = Color::rgb(100, 100, 100);
    let gray = c.to_grayscale();
    // Should be approximately the same
    assert!(gray.red() > 90 && gray.red() < 110);
}

// =============================================================================
// COLORF TESTS
// =============================================================================

#[test]
fn test_colorf_new() {
    let c = ColorF::new(1.0, 0.5, 0.25, 0.75);
    assert!((c.r - 1.0).abs() < 0.0001);
    assert!((c.g - 0.5).abs() < 0.0001);
    assert!((c.b - 0.25).abs() < 0.0001);
    assert!((c.a - 0.75).abs() < 0.0001);
}

#[test]
fn test_colorf_saturate() {
    let c = ColorF::new(1.5, -0.5, 0.5, 1.0);
    let sat = c.saturate();
    assert!((sat.r - 1.0).abs() < 0.0001);
    assert!((sat.g - 0.0).abs() < 0.0001);
    assert!((sat.b - 0.5).abs() < 0.0001);
}

#[test]
fn test_colorf_lerp() {
    let c1 = ColorF::new(0.0, 0.0, 0.0, 1.0);
    let c2 = ColorF::new(1.0, 1.0, 1.0, 1.0);
    let mid = c1.lerp(&c2, 0.5);

    assert!((mid.r - 0.5).abs() < 0.0001);
    assert!((mid.g - 0.5).abs() < 0.0001);
    assert!((mid.b - 0.5).abs() < 0.0001);
}

#[test]
fn test_colorf_to_color() {
    let cf = ColorF::new(1.0, 0.5, 0.0, 1.0);
    let c = cf.to_color();
    assert_eq!(c.red(), 255);
    assert_eq!(c.green(), 128); // ~0.5 * 255
    assert_eq!(c.blue(), 0);
}

// =============================================================================
// PIXEL FORMAT TESTS
// =============================================================================

#[test]
fn test_pixel_format_bytes() {
    assert_eq!(PixelFormat::ARGB8888.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::RGB565.bytes_per_pixel(), 2);
    assert_eq!(PixelFormat::RGB888.bytes_per_pixel(), 3);
    assert_eq!(PixelFormat::Gray8.bytes_per_pixel(), 1);
}

#[test]
fn test_pixel_format_has_alpha() {
    assert!(PixelFormat::ARGB8888.has_alpha());
    assert!(PixelFormat::BGRA8888.has_alpha());
    assert!(!PixelFormat::XRGB8888.has_alpha());
    assert!(!PixelFormat::RGB565.has_alpha());
}

#[test]
fn test_pixel_format_buffer_size() {
    let size = PixelFormat::ARGB8888.buffer_size(800, 600);
    assert_eq!(size, 800 * 600 * 4);
}

// =============================================================================
// BLEND MODE TESTS
// =============================================================================

#[test]
fn test_blend_mode_names() {
    assert_eq!(BlendMode::Normal.name(), "Normal");
    assert_eq!(BlendMode::SourceOver.name(), "SourceOver");
    assert_eq!(BlendMode::Multiply.name(), "Multiply");
}

#[test]
fn test_blend_mode_porter_duff() {
    assert!(BlendMode::SourceOver.is_porter_duff());
    assert!(BlendMode::DestOver.is_porter_duff());
    assert!(!BlendMode::Multiply.is_porter_duff());
}
