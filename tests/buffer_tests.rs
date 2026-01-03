//! # Testes de Buffer
//!
//! Testes para os tipos de buffer.

use gfx_types::buffer::*;
use gfx_types::color::PixelFormat;

// =============================================================================
// BUFFER DESCRIPTOR TESTS
// =============================================================================

#[test]
fn test_buffer_descriptor_new() {
    let desc = BufferDescriptor::new(800, 600, PixelFormat::ARGB8888);
    assert_eq!(desc.width, 800);
    assert_eq!(desc.height, 600);
    assert_eq!(desc.format, PixelFormat::ARGB8888);
    assert_eq!(desc.stride, 800 * 4); // auto-calculated
}

#[test]
fn test_buffer_descriptor_size() {
    let desc = BufferDescriptor::new(800, 600, PixelFormat::ARGB8888);
    assert_eq!(desc.size_bytes(), 800 * 600 * 4);
}

#[test]
fn test_buffer_descriptor_pixel_offset() {
    let desc = BufferDescriptor::new(100, 100, PixelFormat::ARGB8888);

    // First pixel
    assert_eq!(desc.pixel_offset(0, 0), 0);

    // Second pixel
    assert_eq!(desc.pixel_offset(1, 0), 4);

    // First pixel of second row
    assert_eq!(desc.pixel_offset(0, 1), 400); // 100 * 4
}

#[test]
fn test_buffer_descriptor_row_offset() {
    let desc = BufferDescriptor::new(100, 100, PixelFormat::ARGB8888);
    assert_eq!(desc.row_offset(0), 0);
    assert_eq!(desc.row_offset(1), 400);
    assert_eq!(desc.row_offset(10), 4000);
}

// =============================================================================
// BUFFER HANDLE TESTS
// =============================================================================

#[test]
fn test_buffer_handle_invalid() {
    let handle = BufferHandle::INVALID;
    assert!(!handle.is_valid());
}

#[test]
fn test_buffer_handle_new() {
    let handle = BufferHandle::new(42, 1);
    assert!(handle.is_valid());
    assert_eq!(handle.id(), 42);
    assert_eq!(handle.generation(), 1);
}

// =============================================================================
// BUFFER REGION TESTS
// =============================================================================

#[test]
fn test_buffer_region_new() {
    let region = BufferRegion::new(10, 20, 100, 50);
    assert_eq!(region.x, 10);
    assert_eq!(region.y, 20);
    assert_eq!(region.width, 100);
    assert_eq!(region.height, 50);
}

#[test]
fn test_buffer_region_area() {
    let region = BufferRegion::new(0, 0, 100, 50);
    assert_eq!(region.area(), 5000);
}

#[test]
fn test_buffer_region_contains() {
    let region = BufferRegion::new(10, 10, 100, 100);
    assert!(region.contains(50, 50));
    assert!(!region.contains(5, 5));
}
