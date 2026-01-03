//! # Exemplo: Trabalhando com Cores
//!
//! Demonstra o uso do sistema de cores.

use gfx_types::color::*;

fn main() {
    println!("=== GFX Types - Exemplo de Cores ===\n");

    // Criando cores
    let red = Color::rgb(255, 0, 0);
    let green = Color::rgb(0, 255, 0);
    let blue = Color::rgb(0, 0, 255);

    println!(
        "Red: R={} G={} B={} A={}",
        red.red(),
        red.green(),
        red.blue(),
        red.alpha()
    );
    println!(
        "Green: R={} G={} B={} A={}",
        green.red(),
        green.green(),
        green.blue(),
        green.alpha()
    );
    println!(
        "Blue: R={} G={} B={} A={}",
        blue.red(),
        blue.green(),
        blue.blue(),
        blue.alpha()
    );
    println!();

    // Cores com alpha
    let semi_transparent = Color::argb(128, 255, 0, 0);
    println!("Semi-transparent red: {:08X}", semi_transparent.0);
    println!();

    // Constantes
    println!("WHITE: {:08X}", Color::WHITE.0);
    println!("BLACK: {:08X}", Color::BLACK.0);
    println!("TRANSPARENT: {:08X}", Color::TRANSPARENT.0);
    println!();

    // Operações
    let inverted = red.invert();
    println!(
        "Red inverted: R={} G={} B={}",
        inverted.red(),
        inverted.green(),
        inverted.blue()
    );

    let grayscale = Color::rgb(100, 150, 200).to_grayscale();
    println!("Grayscale: R={}", grayscale.red());
    println!();

    // Interpolação
    println!("Gradient Red -> Blue:");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let c = red.lerp(&blue, t);
        println!(
            "  t={:.1}: R={:3} G={:3} B={:3}",
            t,
            c.red(),
            c.green(),
            c.blue()
        );
    }
    println!();

    // Formatos de pixel
    println!("Pixel Formats:");
    println!(
        "  ARGB8888: {} bpp, has_alpha={}",
        PixelFormat::ARGB8888.bytes_per_pixel() * 8,
        PixelFormat::ARGB8888.has_alpha()
    );
    println!(
        "  RGB565:   {} bpp, has_alpha={}",
        PixelFormat::RGB565.bytes_per_pixel() * 8,
        PixelFormat::RGB565.has_alpha()
    );
    println!(
        "  Gray8:    {} bpp, has_alpha={}",
        PixelFormat::Gray8.bytes_per_pixel() * 8,
        PixelFormat::Gray8.has_alpha()
    );
    println!();

    // Blend modes
    println!("Blend Modes:");
    println!(
        "  Normal:     Porter-Duff={}",
        BlendMode::Normal.is_porter_duff()
    );
    println!(
        "  SourceOver: Porter-Duff={}",
        BlendMode::SourceOver.is_porter_duff()
    );
    println!(
        "  Multiply:   Porter-Duff={}",
        BlendMode::Multiply.is_porter_duff()
    );
    println!();

    // Paletas
    println!("Catppuccin Mocha Palette:");
    for i in 0..5 {
        if let Some(c) = CATPPUCCIN_MOCHA.get(i) {
            println!("  Color {}: {:08X}", i, c.0);
        }
    }
}
