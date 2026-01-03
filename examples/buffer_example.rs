//! # Exemplo: Buffers de Pixels
//!
//! Demonstra o uso de buffers e descritores.

use gfx_types::buffer::*;
use gfx_types::color::PixelFormat;

fn main() {
    println!("=== GFX Types - Exemplo de Buffers ===\n");

    // Criar descritor
    let desc = BufferDescriptor::new(800, 600, PixelFormat::ARGB8888);

    println!("Buffer Descriptor:");
    println!("  Size: {}x{}", desc.width, desc.height);
    println!("  Format: {:?}", desc.format);
    println!("  Stride: {} bytes", desc.stride);
    println!(
        "  Total size: {} bytes ({:.2} MB)",
        desc.size_bytes(),
        desc.size_bytes() as f64 / 1024.0 / 1024.0
    );
    println!();

    // Calcular offsets
    println!("Pixel Offsets:");
    println!("  (0, 0): {} bytes", desc.pixel_offset(0, 0));
    println!("  (1, 0): {} bytes", desc.pixel_offset(1, 0));
    println!("  (0, 1): {} bytes", desc.pixel_offset(0, 1));
    println!("  (100, 100): {} bytes", desc.pixel_offset(100, 100));
    println!();

    // Row offsets
    println!("Row Offsets:");
    for row in [0, 1, 10, 100, 599] {
        println!("  Row {}: {} bytes", row, desc.row_offset(row));
    }
    println!();

    // Diferentes formatos
    println!("Buffer sizes for 1920x1080:");
    for format in [
        PixelFormat::ARGB8888,
        PixelFormat::RGB888,
        PixelFormat::RGB565,
        PixelFormat::Gray8,
    ] {
        let d = BufferDescriptor::new(1920, 1080, format);
        println!(
            "  {:?}: {:.2} MB",
            format,
            d.size_bytes() as f64 / 1024.0 / 1024.0
        );
    }
    println!();

    // Handles
    let handle = BufferHandle::new(42, 1);
    println!("Buffer Handle:");
    println!("  Raw: {:016X}", handle.raw());
    println!("  ID: {}", handle.id());
    println!("  Generation: {}", handle.generation());
    println!("  Valid: {}", handle.is_valid());
    println!();

    // Regiões
    let region = BufferRegion::new(100, 100, 200, 150);
    println!("Buffer Region:");
    println!("  Position: ({}, {})", region.x, region.y);
    println!("  Size: {}x{}", region.width, region.height);
    println!("  Area: {} pixels", region.area());
    println!("  Contains (150, 150): {}", region.contains(150, 150));
    println!("  Contains (50, 50): {}", region.contains(50, 50));
}
