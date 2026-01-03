//! # Exemplo: Geometria Básica
//!
//! Demonstra o uso dos tipos geométricos.

use gfx_types::prelude::*;

fn main() {
    println!("=== GFX Types - Exemplo de Geometria ===\n");

    // Pontos
    let p1 = Point::new(10, 20);
    let p2 = Point::new(30, 40);
    println!("Point p1: ({}, {})", p1.x, p1.y);
    println!("Point p2: ({}, {})", p2.x, p2.y);
    println!("Midpoint: {:?}", p1.midpoint(&p2));
    println!("Distance²: {}", p1.distance_squared(&p2));
    println!();

    // Retângulos
    let r1 = Rect::new(0, 0, 100, 100);
    let r2 = Rect::new(50, 50, 100, 100);
    println!("Rect r1: {:?}", r1);
    println!("Rect r2: {:?}", r2);
    println!(
        "r1 contains (25, 25): {}",
        r1.contains_point(Point::new(25, 25))
    );
    println!("r1 ∩ r2: {:?}", r1.intersection(&r2));
    println!("r1 ∪ r2: {:?}", r1.union(&r2));
    println!();

    // Transformações
    let transform = Transform2D::translate(10.0, 20.0).then_scale(2.0, 2.0);

    let original = PointF::new(5.0, 5.0);
    let transformed = transform.transform_point(original);
    println!("Original: {:?}", original);
    println!("Transformed: {:?}", transformed);
    println!();

    // Círculos
    let circle = Circle::from_coords(50.0, 50.0, 25.0);
    println!("Circle center: {:?}", circle.center);
    println!("Circle radius: {}", circle.radius);
    println!("Circle area: {:.2}", circle.area());
    println!(
        "Contains (50, 60): {}",
        circle.contains_point(PointF::new(50.0, 60.0))
    );
    println!();

    // Insets
    let padding = Insets::uniform(10);
    let margin = Insets::symmetric(5, 10);
    println!("Padding: {:?}", padding);
    println!("Margin: {:?}", margin);
}
