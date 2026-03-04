# FastImage Architecture: Trait Composition Pattern

## Overview

The FastImage library uses Rust's trait composition to create an inheritance-like structure. This pattern allows clean separation of concerns while keeping a single unified API.

## Architecture Analogy

Think of it like this:
- **ImageConversion** trait = "Having Legs" (base capability)
- **Drawer** trait = "Human uses Legs" (drawing operations)
- **Tone** trait = "Animal uses Legs" (color/tone adjustments)
- **FastImage** struct = "Both Human and Animal" (implements all capabilities)

## Module Structure

```
src/utils/
├── image.rs      - Base FastImage struct + ImageConversion trait
├── drawer.rs     - Drawer trait (depends on ImageConversion)
├── tone.rs       - Tone trait (depends on ImageConversion)
└── mod.rs        - Re-exports for convenient importing
```

## Core Components

### 1. FastImage Struct (image.rs)

```rust
pub struct FastImage {
  pub w: usize,      // Image width
  pub h: usize,      // Image height
  pub data: Vec<u8>, // BGR pixel data
}
```

### 2. ImageConversion Trait (image.rs)

Base trait that all other traits depend on:

```rust
pub trait ImageConversion {
  fn from_mat(mat: &Mat) -> Self where Self: Sized;
  fn to_mat(&self) -> Mat;
  fn get_data(&mut self) -> &mut Vec<u8>;
  fn get_dimensions(&self) -> (usize, usize);
}
```

**Purpose**: Provides conversion between OpenCV Mat and FastImage, plus access to internal data.

### 3. Drawer Trait (drawer.rs)

```rust
pub trait Drawer: ImageConversion {
  fn put_pixel_bgr(&mut self, x: i32, y: i32, b: u8, g: u8, r: u8, a: f32);
  fn draw_point(&mut self, x: i32, y: i32, b: u8, g: u8, r: u8);
  fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, b: u8, g: u8, r: u8);
  fn draw_line_aa(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: i32, b: u8, g: u8, r: u8);
  // ... more methods
}
```

**Purpose**: Drawing operations (circles, lines, anti-aliasing). Uses `ImageConversion` methods to access pixel data.

### 4. Tone Trait (tone.rs)

```rust
pub trait Tone: ImageConversion {
  fn adjust_exposure(&mut self, exposure: f32);
  fn adjust_exposure_gamma(&mut self, exposure: f32);
  fn adjust_exposure_smooth(&mut self, exposure: f32, highlights_protect: f32);
  // ... more tone operations
}
```

**Purpose**: Tone mapping and color adjustment operations. Uses `ImageConversion` methods to access pixel data.

## How It Works

### Trait Dependencies

The traits form a hierarchy:
```
ImageConversion (base)
    ├── Drawer: ImageConversion
    └── Tone: ImageConversion
```

This means:
- To implement `Drawer`, you **must** implement `ImageConversion`
- To implement `Tone`, you **must** implement `ImageConversion`
- `FastImage` implements all three

### Implementation

```rust
// FastImage implements ImageConversion
impl ImageConversion for FastImage {
  fn from_mat(mat: &Mat) -> Self { /* ... */ }
  fn to_mat(&self) -> Mat { /* ... */ }
  fn get_data(&mut self) -> &mut Vec<u8> { &mut self.data }
  fn get_dimensions(&self) -> (usize, usize) { (self.w, self.h) }
}

// FastImage implements Drawer (empty because all methods have default implementations)
impl Drawer for FastImage {}

// FastImage implements Tone (empty because all methods have default implementations)
impl Tone for FastImage {}
```

## Usage Examples

### Basic Usage

```rust
use computer_graphic_and_vision::utils::{FastImage, ImageConversion, Drawer, Tone};
use opencv::prelude::*;

// Create FastImage from OpenCV Mat
let mut img = FastImage::from_mat(&mat);

// Drawing operations (from Drawer trait)
img.draw_circle(100, 100, 50, 255, 0, 0);  // Red circle
img.draw_line_aa(0.0, 0.0, 100.0, 100.0, 2, 0, 255, 0);  // Green line

// Tone operations (from Tone trait)
img.adjust_exposure(1.0);  // Brighten by 1 stop
img.adjust_exposure_gamma(0.5);  // Gentle brightness with gamma correction

// Convert back to OpenCV Mat
let result_mat = img.to_mat();
```

### Why This Pattern?

**Advantages:**
1. **Separation of Concerns**: Drawing code separated from tone adjustment code
2. **Clean API**: All capabilities available on one struct (`FastImage`)
3. **Extensibility**: Easy to add new trait categories (e.g., `Filter`, `Transform`)
4. **Rust Idioms**: Uses Rust's trait system naturally (no inheritance gymnastics)
5. **Code Organization**: Each trait in its own file, easy to navigate

**Trade-offs:**
- Must import traits to use their methods
- Slightly more complex than single impl block
- Trait bounds needed when writing generic code

## Adding New Features

### To add a new drawing method:

Edit [drawer.rs](src/utils/drawer.rs):
```rust
pub trait Drawer: ImageConversion {
  // ... existing methods ...
  
  fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, b: u8, g: u8, r: u8) {
    let data = self.get_data();
    let (width, height) = self.get_dimensions();
    // ... implementation using get_data() and get_dimensions() ...
  }
}
```

No changes needed to `FastImage` - it automatically gets the new method!

### To add a new tone operation:

Edit [tone.rs](src/utils/tone.rs):
```rust
pub trait Tone: ImageConversion {
  // ... existing methods ...
  
  fn adjust_contrast(&mut self, contrast: f32) {
    let data = self.get_data();
    // ... implementation ...
  }
}
```

### To add a new trait category (e.g., Filters):

1. Create [filter.rs](src/utils/filter.rs):
```rust
use super::image::ImageConversion;

pub trait Filter: ImageConversion {
  fn gaussian_blur(&mut self, radius: f32) {
    let data = self.get_data();
    let (w, h) = self.get_dimensions();
    // ... implementation ...
  }
}

impl Filter for FastImage {}
```

2. Update [mod.rs](src/utils/mod.rs):
```rust
pub mod filter;
pub use filter::Filter;
```

3. Use it:
```rust
use computer_graphic_and_vision::utils::{FastImage, Filter};
let mut img = FastImage::from_mat(&mat);
img.gaussian_blur(2.0);
```

## Mathematical Foundations

Each operation is documented with its mathematical formula:

### Drawing (Drawer trait)
- **Alpha Blending**: `C_result = C_fg × α + C_bg × (1 - α)`
- **Circle Rasterization**: `x² + y² ≤ r²`
- **Wu's Line Algorithm**: Fractional coordinates with distance-based alpha

### Tone Mapping (Tone trait)
- **Exposure**: `V_new = V_old × 2^E`
- **Gamma Correction**: `V_linear = (V_sRGB / 255)^2.2`
- **Highlight Protection**: Soft compression near white point

See [POINT_OPERATIONS_FORMULAS.md](POINT_OPERATIONS_FORMULAS.md) for complete mathematical documentation.

## Performance Considerations

- **In-place Operations**: All operations modify data in-place (no copying)
- **Direct Memory Access**: Uses `get_data()` to access raw pixel buffer
- **No Virtual Dispatch**: Trait methods are statically dispatched (zero overhead)
- **Iterator Patterns**: Step-by operations on flat BGR array

## Comparison to Traditional OOP

### Traditional Class Hierarchy (not possible in Rust):
```
Base
├── Base.from_mat()
├── Base.to_mat()
└── Base.data

Drawer extends Base
├── Drawer.draw_circle()
└── Drawer.draw_line()

Tone extends Base
├── Tone.adjust_exposure()
└── Tone.adjust_contrast()

FastImage extends Drawer, Tone  // Multiple inheritance - not in Rust!
```

### Our Trait Composition (Rust way):
```
ImageConversion trait (interface)
├── from_mat()
├── to_mat()
├── get_data()
└── get_dimensions()

Drawer trait: ImageConversion (requires ImageConversion)
├── draw_circle()
└── draw_line()

Tone trait: ImageConversion (requires ImageConversion)
├── adjust_exposure()
└── adjust_contrast()

FastImage struct implements all traits
├── impl ImageConversion for FastImage
├── impl Drawer for FastImage
└── impl Tone for FastImage
```

**Result**: Same API, same functionality, but using Rust idioms!

## References

- [MATH_EXPLAINED_v2.md](MATH_EXPLAINED_v2.md) - Mathematical theory behind operations
- [POINT_OPERATIONS_FORMULAS.md](POINT_OPERATIONS_FORMULAS.md) - Complete formula reference
- [REFERENCES.md](REFERENCES.md) - Academic citations for algorithms
- [COMPUTER_GRAPHICS_FEATURES.md](COMPUTER_GRAPHICS_FEATURES.md) - Feature roadmap

## Testing

```rust
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_trait_composition() {
    let mat = Mat::default();
    let mut img = FastImage::from_mat(&mat);
    
    // All traits work together
    img.draw_circle(50, 50, 20, 255, 0, 0);
    img.adjust_exposure(1.0);
    
    let result = img.to_mat();
    assert!(!result.empty());
  }
}
```

## Summary

This architecture provides:
✅ Clean separation of concerns  
✅ Single unified API  
✅ Easy extensibility  
✅ Zero runtime overhead  
✅ Rust-idiomatic design  
✅ Mathematical rigor  

The trait composition pattern gives us the benefits of inheritance without its drawbacks!
