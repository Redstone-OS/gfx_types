# GFX Types v0.2.0

**Core Graphics Types for RedstoneOS**

Biblioteca de tipos gráficos fundamentais compartilhados entre kernel, compositor e aplicações do RedstoneOS.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

---

## 📋 Índice

- [Características](#características)
- [Instalação](#instalação)
- [Módulos](#módulos)
- [Uso](#uso)
- [Tipos Principais](#tipos-principais)
- [Compatibilidade](#compatibilidade)
- [Contribuição](#contribuição)
- [Licença](#licença)

---

## ✨ Características

- 🚀 **`#![no_std]`** - Compatível com ambientes bare-metal e kernel
- 🔒 **ABI Estável** - Todos os tipos core usam `#[repr(C)]` para interoperabilidade
- 📦 **Modular** - Organização clara em submódulos temáticos
- 🎨 **Completo** - Cores, geometria, buffers, displays, janelas e mais
- 🔧 **Zero dependências externas** - Apenas `rdsmath` (interna)
- ⚡ **Otimizado** - Tipos inline e operações eficientes

---

## 📦 Instalação

### Como dependência local (RedstoneOS)

```toml
[dependencies]
gfx_types = { path = "../lib/gfx_types" }
```

### Do crates.io (futuro)

```toml
[dependencies]
gfx_types = "0.2"
```

---

## 📁 Módulos

| Módulo | Descrição | Tipos Principais |
|--------|-----------|------------------|
| [`geometry`](src/geometry/) | Primitivas geométricas | `Point`, `Size`, `Rect`, `Transform2D`, `Circle`, `Line` |
| [`color`](src/color/) | Sistema de cores | `Color`, `ColorF`, `PixelFormat`, `BlendMode`, `Palette` |
| [`buffer`](src/buffer/) | Buffers de pixels | `BufferDescriptor`, `BufferHandle`, `BufferView` |
| [`display`](src/display/) | Informações de display | `DisplayInfo`, `DisplayMode`, `VsyncMode`, `ConnectorType` |
| [`window`](src/window/) | Janelas e superfícies | `WindowFlags`, `WindowState`, `LayerType`, `WindowEffects` |
| [`damage`](src/damage/) | Damage tracking | `DamageRegion`, `DamageHint` |
| [`input`](src/input/) | Input gráfico | `CursorType`, `TouchPoint`, `GestureType` |
| [`render`](src/render/) | Comandos de render | `RenderOp`, `ClipRect`, `BlitParams`, `PipelineState` |
| [`text`](src/text/) | Tipografia básica | `FontWeight`, `FontStyle`, `GlyphMetrics`, `TextAlign` |

---

## 🚀 Uso

### Prelude (tipos mais comuns)

```rust
use gfx_types::prelude::*;

// Tudo que você precisa para começar
let rect = Rect::new(10, 20, 100, 50);
let color = Color::rgb(255, 128, 0);
let point = Point::new(50, 30);
```

### Geometria

```rust
use gfx_types::geometry::*;

// Pontos
let p = Point::new(10, 20);
let pf = PointF::new(10.5, 20.5);
let distance = pf.distance(&PointF::ZERO);

// Retângulos
let r = Rect::new(0, 0, 100, 50);
assert!(r.contains_point(Point::new(50, 25)));

let intersection = r.intersection(&Rect::new(50, 0, 100, 50));

// Transformações 2D
let transform = Transform2D::translate(10.0, 20.0)
    .then_rotate(0.5)
    .then_scale(2.0, 2.0);
    
let point = transform.transform_point(PointF::new(5.0, 5.0));

// Círculos e Elipses
let circle = Circle::from_coords(100.0, 100.0, 50.0);
let on_edge = circle.point_at_angle(rdsmath::PI / 4.0);
```

### Cores

```rust
use gfx_types::color::*;

// Criar cores
let red = Color::rgb(255, 0, 0);
let transparent_blue = Color::argb(128, 0, 0, 255);
let from_hex = Color::from_hex(0xFF5733);

// Manipular
let darker = red.with_alpha(200);
let inverted = red.invert();
let gray = red.to_grayscale();

// Interpolação
let gradient = red.lerp(&Color::BLUE, 0.5);

// Blend modes
let blend = BlendMode::SourceOver;
let multiply = BlendMode::Multiply;

// Paletas predefinidas
let mocha_base = CATPPUCCIN_MOCHA.get(0);
```

### Buffers

```rust
use gfx_types::buffer::*;
use gfx_types::color::PixelFormat;

// Descritor de buffer
let desc = BufferDescriptor::new(800, 600, PixelFormat::ARGB8888);
println!("Size: {} bytes", desc.size_bytes());
println!("Pixel at (10, 20): offset {}", desc.pixel_offset(10, 20));

// Buffer views
let mut data = vec![0u8; desc.size_bytes()];
let mut view = BufferViewMut::new(&mut data, desc).unwrap();
view.clear();
```

### Janelas

```rust
use gfx_types::window::*;

// Flags combinadas
let flags = WindowFlags::BORDERLESS | WindowFlags::TRANSPARENT;
assert!(flags.has(WindowFlags::BORDERLESS));

// Estados
let state = WindowState::Maximized;
assert!(state.is_visible());

// Efeitos
let effects = WindowEffects::DEFAULT
    .with_corner_radius(12.0)
    .with_opacity(0.95);
```

---

## 📐 Tipos Principais

### Point / PointF

```rust
#[repr(C)]
pub struct Point { pub x: i32, pub y: i32 }
pub struct PointF { pub x: f32, pub y: f32 }
```

### Size / SizeF

```rust
#[repr(C)]
pub struct Size { pub width: u32, pub height: u32 }
pub struct SizeF { pub width: f32, pub height: f32 }
```

### Rect / RectF

```rust
#[repr(C)]  
pub struct Rect { pub x: i32, pub y: i32, pub width: u32, pub height: u32 }
```

### Color

```rust
#[repr(transparent)]
pub struct Color(pub u32); // ARGB format
```

### PixelFormat

```rust
pub enum PixelFormat {
    ARGB8888, XRGB8888, RGB565, BGRA8888, RGBA8888,
    RGB888, BGR888, Gray8, Gray16, Alpha8
}
```

### WindowFlags

```rust
pub struct WindowFlags(pub u32);
// BORDERLESS, ALWAYS_ON_TOP, TRANSPARENT, FULLSCREEN, 
// NO_RESIZE, HAS_SHADOW, MODAL, etc.
```

---

## 🔧 Features

| Feature | Descrição |
|---------|-----------|
| `default` | Sem features extras |
| `alloc` | Tipos que precisam de alocação |
| `std` | Suporte a standard library |

---

## 🔗 Compatibilidade

| Componente | Status |
|------------|--------|
| Kernel (Forge) | ✅ Totalmente compatível |
| Compositor (Firefly) | ✅ Totalmente compatível |
| SDK (Redpowder) | ✅ Totalmente compatível |
| Aplicações | ✅ Via Redpowder |

---

## 📁 Estrutura do Projeto

```
gfx_types/
├── src/
│   ├── lib.rs              # Re-exports e prelude
│   ├── geometry/           # Primitivas geométricas
│   │   ├── point.rs
│   │   ├── size.rs
│   │   ├── rect.rs
│   │   ├── line.rs
│   │   ├── circle.rs
│   │   ├── insets.rs
│   │   ├── polygon.rs
│   │   └── transform.rs
│   ├── color/              # Sistema de cores
│   │   ├── color.rs
│   │   ├── format.rs
│   │   ├── blend.rs
│   │   ├── palette.rs
│   │   └── space.rs
│   ├── buffer/             # Buffers de pixels
│   ├── display/            # Display/output
│   ├── window/             # Janelas
│   ├── damage/             # Damage tracking
│   ├── input/              # Cursor/touch
│   ├── render/             # Comandos de render
│   └── text/               # Tipografia
├── tests/                  # Testes de integração
├── examples/               # Exemplos de uso
├── benches/                # Benchmarks
├── Cargo.toml
└── README.md
```

---

## 🤝 Contribuição

Contribuições são bem-vindas! Por favor:

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

---

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

**RedstoneOS Team** - [github.com/redstone-os](https://github.com/redstone-os)