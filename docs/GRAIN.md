# Film Grain Implementation Guide

Comprehensive guide for implementing Fujifilm-style film grain with **Strong/Weak** and **Large/Small** parameters.

---

## Mathematical Formulas (Implemented)

### 1. Luminance Calculation (ITU-R BT.709)
$$L = \frac{0.2126 \cdot R + 0.7152 \cdot G + 0.0722 \cdot B}{255}$$

Where:
- $L$ = normalized luminance (0.0 to 1.0)
- $R, G, B$ = pixel color values (0 to 255)

### 2. Luminance Weight Function (Piecewise)

$$
W(L) = \begin{cases}
0.3 & \text{if } L < 0.05 \text{ (deep shadows)} \\
0.3 + \frac{(L - 0.05)}{0.15} \times 0.7 & \text{if } 0.05 \leq L \leq 0.20 \text{ (ramp up)} \\
1.0 & \text{if } 0.20 < L \leq 0.80 \text{ (midtones - max grain)} \\
1.0 - \frac{(L - 0.80)}{0.15} \times 0.8 & \text{if } 0.80 < L \leq 0.95 \text{ (ramp down)} \\
0.2 & \text{if } L > 0.95 \text{ (bright highlights)}
\end{cases}
$$

### 3. Gaussian Noise Generation
$$N(x, y) \sim \mathcal{N}(0, 1)$$

Where $\mathcal{N}(0, 1)$ is a normal distribution with mean = 0, standard deviation = 1.

### 4. Box Blur for Grain Clumping
$$N_{\text{blur}}(x, y) = \frac{1}{(2r + 1)^2} \sum_{dy=-r}^{r} \sum_{dx=-r}^{r} N(x + dx, y + dy)$$

Where:
- $r$ = blur radius (1 for small grain, 3 for large grain)
- $(2r + 1)^2$ = kernel size

### 5. Grain Strength Calculation
$$S(x, y) = I_{\text{base}} \times W(L(x, y)) \times N_{\text{blur}}(x, y) \times 255$$

Where:
- $I_{\text{base}}$ = base intensity (0.10 for weak, 0.25-0.30 for strong)
- $W(L)$ = luminance weight function
- $N_{\text{blur}}$ = blurred noise value
- 255 = scale factor to match 8-bit range

### 6. Final Pixel Value
$$C'_{\text{channel}} = \text{clamp}(C_{\text{channel}} + S(x, y), 0, 255)$$

Applied to each channel (R, G, B) independently.

### Complete Pipeline (Step-by-Step Example)

**Given:** Pixel with RGB = (128, 100, 80), Small + Strong grain

**Step 1:** Generate Gaussian noise
```
N(x, y) ~ N(0, 1)  →  e.g., N(x, y) = -0.523
```

**Step 2:** Apply box blur (radius = 1 for small grain)
```
N_blur(x, y) = average of 3×3 neighbors
             = (n₁ + n₂ + ... + n₉) / 9
             = e.g., -0.412
```

**Step 3:** Calculate luminance
```
L = (0.2126 × 128 + 0.7152 × 100 + 0.0722 × 80) / 255
  = (27.21 + 71.52 + 5.78) / 255
  = 104.51 / 255
  = 0.410
```

**Step 4:** Get luminance weight (falls in midtone range)
```
W(0.410) = 1.0  (since 0.20 < 0.410 ≤ 0.80)
```

**Step 5:** Calculate grain strength
```
S = 0.30 × 1.0 × (-0.412) × 255
  = -31.52
```

**Step 6:** Apply to each channel
```
R' = clamp(128 + (-31.52), 0, 255) = 96
G' = clamp(100 + (-31.52), 0, 255) = 68
B' = clamp(80 + (-31.52), 0, 255) = 48
```

**Result:** RGB = (96, 68, 48) — darker due to negative noise value

---

## Implementation Parameters (Used in Code)

### Preset Parameters

| Preset | Base Intensity ($I_{\text{base}}$) | Blur Radius ($r$) | Use Case |
|--------|-----------------------------------|-------------------|----------|
| **Small + Weak** | 0.10 | 1 pixel | Provia, Velvia (subtle) |
| **Small + Strong** | 0.30 | 1 pixel | Pushed film (pronounced) |
| **Large + Weak** | 0.08 | 3 pixels | Classic Chrome (chunky) |
| **Large + Strong** | 0.25 | 3 pixels | High ISO film (heavy) |

### Blur Kernel Size
- **Small grain:** $3 \times 3$ kernel (radius = 1)
- **Large grain:** $7 \times 7$ kernel (radius = 3)

### Luminance Weight Breakpoints
- $L < 0.05$ → Weight = 0.3 (30% grain in deep shadows)
- $0.05 \leq L \leq 0.20$ → Linear ramp from 0.3 to 1.0
- $0.20 < L \leq 0.80$ → Weight = 1.0 (100% grain in midtones)
- $0.80 < L \leq 0.95$ → Linear ramp from 1.0 to 0.2
- $L > 0.95$ → Weight = 0.2 (20% grain in bright highlights)

---

## Code-to-Math Mapping

This section maps the actual Rust implementation code to the mathematical formulas above.

### Formula 1: Luminance Calculation
**Math:** $L = \frac{0.2126 \cdot R + 0.7152 \cdot G + 0.0722 \cdot B}{255}$

**Code:** [`src/utils/grain.rs:158`](../src/utils/grain.rs#L158)
```rust
let luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;
```

---

### Formula 2: Luminance Weight Function
**Math:** 
$$
W(L) = \begin{cases}
0.3 & \text{if } L < 0.05 \\
0.3 + \frac{(L - 0.05)}{0.15} \times 0.7 & \text{if } 0.05 \leq L \leq 0.20 \\
1.0 & \text{if } 0.20 < L \leq 0.80 \\
1.0 - \frac{(L - 0.80)}{0.15} \times 0.8 & \text{if } 0.80 < L \leq 0.95 \\
0.2 & \text{if } L > 0.95
\end{cases}
$$

**Code:** [`src/utils/grain.rs:58-71`](../src/utils/grain.rs#L58-L71)
```rust
fn luminance_weight(luminance: f32) -> f32 {
  match luminance {
    l if l < 0.05 => 0.3,  // Deep shadows
    l if l >= 0.05 && l <= 0.20 => {
      // Shadows to midtones: ramp up
      0.3 + (l - 0.05) / 0.15 * 0.7
    },
    l if l > 0.20 && l <= 0.80 => 1.0,  // Midtones: maximum grain
    l if l > 0.80 && l <= 0.95 => {
      // Highlights: ramp down
      1.0 - (l - 0.80) / 0.15 * 0.8
    },
    _ => 0.2,  // Bright highlights
  }
}
```

---

### Formula 3: Gaussian Noise Generation
**Math:** $N(x, y) \sim \mathcal{N}(0, 1)$

**Code:** [`src/utils/grain.rs:111-114`](../src/utils/grain.rs#L111-L114)
```rust
let mut rng = rand::thread_rng();
let normal = Normal::new(0.0, 1.0).unwrap();
let mut noise: Vec<f32> = (0..width * height)
  .map(|_| normal.sample(&mut rng))
  .collect();
```

---

### Formula 4: Box Blur for Grain Clumping
**Math:** $N_{\text{blur}}(x, y) = \frac{1}{(2r + 1)^2} \sum_{dy=-r}^{r} \sum_{dx=-r}^{r} N(x + dx, y + dy)$

**Code:** [`src/utils/grain.rs:117-134`](../src/utils/grain.rs#L117-L134)
```rust
if blur_radius > 0 {
  let mut blurred = vec![0.0f32; width * height];
  for y in 0..height {
    for x in 0..width {
      let mut sum = 0.0;
      let mut count = 0;
      
      // Box blur kernel
      for dy in -(blur_radius as i32)..=(blur_radius as i32) {
        for dx in -(blur_radius as i32)..=(blur_radius as i32) {
          let ny = (y as i32 + dy).max(0).min((height - 1) as i32) as usize;
          let nx = (x as i32 + dx).max(0).min((width - 1) as i32) as usize;
          sum += noise[ny * width + nx];  // ← Numerator summation
          count += 1;                      // ← Denominator (2r+1)²
        }
      }
      blurred[y * width + x] = sum / count as f32;  // ← Division
    }
  }
  noise = blurred;
}
```

**Note:** `count` equals $(2r + 1)^2$:
- For `r=1` (small): `count = (2×1+1)² = 3² = 9`
- For `r=3` (large): `count = (2×3+1)² = 7² = 49`

---

### Formula 5: Grain Strength Calculation
**Math:** $S(x, y) = I_{\text{base}} \times W(L(x, y)) \times N_{\text{blur}}(x, y) \times 255$

**Code:** [`src/utils/grain.rs:165`](../src/utils/grain.rs#L165)
```rust
let grain_strength = base_intensity * weight * noise_val * 255.0;
//                   └─── I_base ──┘  └─ W(L) ─┘  └─ N_blur ─┘  └─ 255 ─┘
```

**Where:**
- `base_intensity` = $I_{\text{base}}$ (0.10 for weak, 0.30 for strong)
- `weight` = $W(L)$ from luminance_weight function
- `noise_val` = $N_{\text{blur}}(x, y)$ from blurred noise
- `255.0` = scale factor

---

### Formula 6: Final Pixel Value
**Math:** $C'_{\text{channel}} = \text{clamp}(C_{\text{channel}} + S(x, y), 0, 255)$

**Code:** [`src/utils/grain.rs:168-170`](../src/utils/grain.rs#L168-L170)
```rust
self.base.data[idx] = ((b + grain_strength).max(0.0).min(255.0)) as u8;
//                     └─── C_channel ──┘ └─ S(x,y) ─┘ └── clamp ──────┘

self.base.data[idx + 1] = ((g + grain_strength).max(0.0).min(255.0)) as u8;
self.base.data[idx + 2] = ((r + grain_strength).max(0.0).min(255.0)) as u8;
```

**Where:**
- `b`, `g`, `r` = $C_{\text{channel}}$ (original pixel values)
- `grain_strength` = $S(x, y)$ (calculated grain)
- `.max(0.0).min(255.0)` = $\text{clamp}(value, 0, 255)$

---

### Complete Pipeline in Code Order

```rust
// Line 103: Define parameters (I_base and r)
let (base_intensity, blur_radius) = match (intensity, size) {
  (GrainIntensity::Weak, GrainSize::Small) => (0.10, 1),
  // ... other cases
};

// Lines 111-114: Generate Gaussian noise N(0,1)
let normal = Normal::new(0.0, 1.0).unwrap();
let mut noise: Vec<f32> = (0..width * height)
  .map(|_| normal.sample(&mut rng))
  .collect();

// Lines 117-134: Apply box blur → N_blur
for dy in -(blur_radius as i32)..=(blur_radius as i32) {
  for dx in -(blur_radius as i32)..=(blur_radius as i32) {
    sum += noise[ny * width + nx];
    count += 1;
  }
}
blurred[y * width + x] = sum / count as f32;

// Line 158: Calculate luminance L
let luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;

// Line 161: Get weight W(L)
let weight = Self::luminance_weight(luminance);

// Line 165: Calculate grain strength S
let grain_strength = base_intensity * weight * noise_val * 255.0;

// Lines 168-170: Apply to channels C' = clamp(C + S)
self.base.data[idx] = ((b + grain_strength).max(0.0).min(255.0)) as u8;
self.base.data[idx + 1] = ((g + grain_strength).max(0.0).min(255.0)) as u8;
self.base.data[idx + 2] = ((r + grain_strength).max(0.0).min(255.0)) as u8;
```

---

### Parameter Values in Code

**Line 103-107:** Preset parameter selection
```rust
let (base_intensity, blur_radius) = match (intensity, size) {
  (GrainIntensity::Weak, GrainSize::Small) => (0.10, 1),    // I_base=0.10, r=1
  (GrainIntensity::Weak, GrainSize::Large) => (0.08, 3),    // I_base=0.08, r=3
  (GrainIntensity::Strong, GrainSize::Small) => (0.30, 1),  // I_base=0.30, r=1
  (GrainIntensity::Strong, GrainSize::Large) => (0.25, 3),  // I_base=0.25, r=3
};
```

**Correspondence:**
- `base_intensity` → $I_{\text{base}}$ in Formula 5
- `blur_radius` → $r$ in Formula 4

---

### Execution Flow with Line Numbers

```
┌─────────────────────────────────────────────────────────────┐
│ GRAIN APPLICATION PIPELINE                                  │
└─────────────────────────────────────────────────────────────┘

Step 1: Setup Parameters (Lines 98-107)
  ├─ Get image dimensions: width, height
  └─ Select base_intensity (I_base) and blur_radius (r)
     └─ Formula: Lookup table based on intensity/size enum
     
Step 2: Generate Noise (Lines 111-114) 
  └─ Formula 3: N(x,y) ~ 𝒩(0,1)
     └─ Code: normal.sample(&mut rng)

Step 3: Blur Noise (Lines 117-134)
  └─ Formula 4: N_blur = (1/(2r+1)²) × Σ N(neighbors)
     └─ Code: sum / count as f32

Step 4: For Each Pixel (Lines 138-171)
  ├─ 4a. Load RGB values (Lines 148-150)
  │    └─ Code: let r = self.base.data[idx + 2] as f32
  │
  ├─ 4b. Calculate Luminance (Line 158)
  │    └─ Formula 1: L = (0.2126×R + 0.7152×G + 0.0722×B)/255
  │       └─ Code: (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
  │
  ├─ 4c. Get Luminance Weight (Line 161)
  │    └─ Formula 2: W(L) = piecewise function
  │       └─ Code: Self::luminance_weight(luminance)
  │
  ├─ 4d. Calculate Grain Strength (Line 165)
  │    └─ Formula 5: S = I_base × W(L) × N_blur × 255
  │       └─ Code: base_intensity * weight * noise_val * 255.0
  │
  └─ 4e. Apply to Channels (Lines 168-170)
       └─ Formula 6: C' = clamp(C + S, 0, 255)
          └─ Code: ((b + grain_strength).max(0.0).min(255.0)) as u8
```

---

## Overview

Fujifilm cameras provide four grain parameters:
- **Intensity**: Weak / Strong (blend strength)
- **Size**: Small / Large (grain particle size)
- **Characteristic**: Luminance-dependent application (more in midtones/shadows, less in highlights)
- **Authenticity**: Mimics analog film grain patterns, not uniform digital noise

---

## Solution Tiers

### 1. Good Enough: Gaussian Noise + Luminance Mask

**Speed**: ⚡⚡⚡ Fastest  
**Quality**: ★★★☆☆ Decent for most use cases  
**Complexity**: Simple

#### Algorithm:
```
1. Generate Gaussian noise (-1.0 to 1.0 range)
2. Apply Gaussian blur based on size parameter
3. Calculate luminance mask from original image
4. Blend grain with image using luminance-weighted intensity
```

#### Implementation Details:

**Size Control (Gaussian Blur):**
- Small grain: σ = 0.5 - 1.0
- Large grain: σ = 2.0 - 4.0

**Intensity Control (Blend Factor):**
- Weak: 0.05 - 0.15 (5-15% opacity)
- Strong: 0.20 - 0.40 (20-40% opacity)

**Luminance Curve (Fujifilm-like):**
```rust
fn luminance_weight(luminance: f32) -> f32 {
    // luminance: 0.0 (black) to 1.0 (white)
    match luminance {
        l if l < 0.05 => 0.3,           // Deep shadows: minimal grain
        l if l >= 0.05 && l <= 0.20 => {
            // Shadows to midtones: ramp up
            0.3 + (l - 0.05) / 0.15 * 0.7
        },
        l if l > 0.20 && l <= 0.80 => 1.0,  // Midtones: maximum grain
        l if l > 0.80 && l <= 0.95 => {
            // Highlights: ramp down
            1.0 - (l - 0.80) / 0.15 * 0.8
        },
        _ => 0.2,                       // Bright highlights: minimal grain
    }
}
```

#### Pseudocode:
```rust
for each pixel (x, y):
    // 1. Generate and blur noise
    noise = gaussian_random(-1.0, 1.0)
    blurred_noise = gaussian_blur(noise, sigma)
    
    // 2. Calculate luminance
    rgb = image[x, y]
    luminance = 0.299*R + 0.587*G + 0.114*B
    
    // 3. Apply luminance-weighted grain
    weight = luminance_weight(luminance / 255.0)
    grain_strength = base_intensity * weight
    
    // 4. Blend
    for each channel (R, G, B):
        new_value = rgb[channel] + blurred_noise * grain_strength * 255
        image[x, y][channel] = clamp(new_value, 0, 255)
```

#### Pros:
- Fast: Single-pass generation
- Simple to implement
- Temporal stability (good for video)

#### Cons:
- Grain pattern is too uniform
- Lacks organic film texture
- No chromatic grain variation

---

### 2. Better: Perlin/Simplex Noise

**Speed**: ⚡⚡☆ Moderate  
**Quality**: ★★★★☆ Natural-looking  
**Complexity**: Medium

#### Algorithm:
```
1. Generate Perlin/Simplex noise at multiple frequencies
2. Combine octaves for natural clumping
3. Apply luminance mask
4. Add chromatic variation (different noise per channel)
5. Blend with intensity control
```

#### Implementation Details:

**Multi-Octave Noise:**
```rust
fn perlin_grain(x, y, size_param) -> f32 {
    let base_freq = match size_param {
        Small => 0.1,
        Large => 0.03,
    };
    
    // Combine 2-3 octaves
    let octave1 = perlin(x * base_freq, y * base_freq) * 1.0;
    let octave2 = perlin(x * base_freq * 2.0, y * base_freq * 2.0) * 0.5;
    let octave3 = perlin(x * base_freq * 4.0, y * base_freq * 4.0) * 0.25;
    
    (octave1 + octave2 + octave3) / 1.75
}
```

**Chromatic Grain (Per-Channel Variation):**
```rust
// Use different noise seeds for each channel
noise_r = perlin_grain(x, y, seed=42)
noise_g = perlin_grain(x, y, seed=123)
noise_b = perlin_grain(x, y, seed=789)

// Slight correlation (80% shared, 20% unique)
grain_r = 0.8 * noise_shared + 0.2 * noise_r
grain_g = 0.8 * noise_shared + 0.2 * noise_g
grain_b = 0.8 * noise_shared + 0.2 * noise_b
```

#### Pseudocode:
```rust
// Pre-generate noise maps
shared_noise = perlin_noise_map(width, height, seed_shared)
red_noise = perlin_noise_map(width, height, seed_r)
green_noise = perlin_noise_map(width, height, seed_g)
blue_noise = perlin_noise_map(width, height, seed_b)

for each pixel (x, y):
    // Get base image
    rgb = image[x, y]
    luminance = calculate_luminance(rgb)
    weight = luminance_weight(luminance)
    
    // Apply chromatic grain
    grain_r = 0.8 * shared_noise[x,y] + 0.2 * red_noise[x,y]
    grain_g = 0.8 * shared_noise[x,y] + 0.2 * green_noise[x,y]
    grain_b = 0.8 * shared_noise[x,y] + 0.2 * blue_noise[x,y]
    
    // Blend with luminance weighting
    strength = base_intensity * weight
    image[x, y].r += grain_r * strength * 255
    image[x, y].g += grain_g * strength * 255
    image[x, y].b += grain_b * strength * 255
```

#### Pros:
- Natural, organic grain patterns
- Better clumping than Gaussian
- Chromatic grain variation
- Smooth gradient-like quality

#### Cons:
- Slower than Gaussian noise
- Requires noise library (noise-rs in Rust)
- Harder to achieve exact "film look"

---

### 3. Professional: Real Film Grain Textures

**Speed**: ⚡⚡⚡ Fast (after preprocessing)  
**Quality**: ★★★★★ Authentic film look  
**Complexity**: High (requires assets)

#### Algorithm:
```
1. Acquire real film grain scans (high-resolution)
2. Pre-process grain textures (extract, normalize)
3. Create grain atlases for different sizes
4. Tile/sample grain texture across image
5. Apply luminance-dependent blending
6. Add chromatic variations from film grain channels
```

#### Implementation Details:

**Grain Texture Acquisition:**

Sources:
- Scan actual film (Kodak, Fuji stocks)
- Purchase from texture libraries:
  - Film Grain Central
  - CineGrain
  - FXPhd grain plates
- Extract from film photography databases

**Preprocessing:**
```rust
// 1. Scan or acquire high-res film grain (e.g., 4K scan of Fuji 400H)
// 2. Extract grain layer
fn extract_grain(film_scan, neutral_gray_scan) -> GrainTexture {
    // Subtract neutral gray to isolate grain
    grain = film_scan - neutral_gray_scan
    
    // Normalize to -1.0 to 1.0 range
    grain_normalized = (grain - 128) / 128.0
    
    // Create seamless tileable texture (optional)
    grain_tileable = make_seamless(grain_normalized)
    
    return grain_tileable
}
```

**Grain Atlas Organization:**
```
grain_assets/
├── fuji_provia/
│   ├── small_weak.png
│   ├── small_strong.png
│   ├── large_weak.png
│   └── large_strong.png
├── fuji_velvia/
│   ├── small_weak.png
│   └── ...
└── fuji_acros/
    ├── small_weak.png
    └── ...
```

**Application (Tiling & Blending):**
```rust
fn apply_film_grain(
    image: &mut Image,
    grain_texture: &GrainTexture,
    intensity: GrainIntensity,
    size: GrainSize
) {
    // Select appropriate grain texture
    let grain = load_grain_texture(film_type, intensity, size);
    
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        // Tile grain texture across image
        let grain_x = x % grain.width();
        let grain_y = y % grain.height();
        let grain_value = grain.get_pixel(grain_x, grain_y);
        
        // Calculate luminance weight
        let luminance = calculate_luminance(pixel);
        let weight = luminance_weight(luminance);
        
        // Apply per-channel grain (chromatic)
        pixel.r = clamp(pixel.r + grain_value.r * weight);
        pixel.g = clamp(pixel.g + grain_value.g * weight);
        pixel.b = clamp(pixel.b + grain_value.b * weight);
    }
}
```

**Advanced: Random Offset (Avoid Tiling Patterns):**
```rust
// Add random offset per frame/image to prevent visible tiling
let offset_x = random(0, grain.width());
let offset_y = random(0, grain.height());

let grain_x = (x + offset_x) % grain.width();
let grain_y = (y + offset_y) % grain.height();
```

#### Pros:
- Authentic film look (actual film grain)
- Fast runtime (texture lookup)
- Perfect for specific film stock emulation
- Professional film industry standard

#### Cons:
- Requires asset acquisition/creation
- Storage overhead (texture files)
- Licensing concerns for commercial use
- Less procedural flexibility

---

## Recommended Parameters Table

### Fujifilm-Style Settings

| Parameter | Small/Weak | Small/Strong | Large/Weak | Large/Strong |
|-----------|------------|--------------|------------|--------------|
| **Blur σ** | 0.5-0.8 | 0.5-0.8 | 2.5-3.5 | 2.5-3.5 |
| **Intensity** | 0.08-0.12 | 0.25-0.35 | 0.05-0.10 | 0.20-0.30 |
| **Midtone Weight** | 1.0 | 1.0 | 1.0 | 1.0 |
| **Shadow Weight** | 0.3-0.5 | 0.3-0.5 | 0.3-0.5 | 0.3-0.5 |
| **Highlight Weight** | 0.1-0.2 | 0.1-0.2 | 0.1-0.2 | 0.1-0.2 |

### Film Stock Equivalents

**Fujifilm Simulations:**
- **Classic Chrome**: Large grain, moderate intensity
- **Acros (B&W)**: Fine grain, high definition
- **Provia**: Minimal grain (weak/small)
- **Velvia**: Fine grain, vivid (weak/small)
- **Pro Neg Hi/Std**: Medium grain (small/weak to medium/weak)

---

## Implementation Recommendations

### For Your Rust Project:

**Quick Start (Good Enough):**
```bash
# Add dependencies to Cargo.toml
rand = "0.8"
imageproc = "0.23"
```

**Better Quality:**
```bash
# Add Perlin noise
noise = "0.8"
```

**Professional:**
```bash
# No additional dependencies, use pre-made grain textures
# Just load PNG/EXR grain plates
```

### Crate Choice Matrix:

| Need | Crate | Purpose |
|------|-------|---------|
| Gaussian noise | `rand` | RNG for noise generation |
| Gaussian blur | `imageproc::filter::gaussian_blur_f32` | Blur grain |
| Perlin noise | `noise` crate (Perlin, Simplex) | Natural grain patterns |
| Image I/O | `image` crate | Load grain textures |

---

## Testing & Validation

### Visual Tests:
1. **Gray ramp test**: Apply grain to 0-255 gradient, verify luminance curve
2. **Flat gray test**: 50% gray image shows grain distribution
3. **Portrait test**: Check grain behavior in skin tones
4. **High contrast test**: Ensure grain doesn't blow highlights

### Metrics:
- **PSNR**: Should decrease by 1-3 dB with grain
- **Perceived quality**: Grain should be visible but not distracting
- **Temporal stability** (video): Grain shouldn't flicker between frames

---

## References

### Academic Papers:
1. **Gastal, Eduardo S. L., and Manuel M. Oliveira.** "Domain transform for edge-aware image and video processing." *ACM TOG (SIGGRAPH 2011)*, 30(4):69.
   - Edge-preserving filters for grain application

2. **Hasinoff, Samuel W., et al.** "Burst photography for high dynamic range and low-light imaging on mobile cameras." *ACM TOG (SIGGRAPH Asia 2016)*, 35(6):192.
   - Google's HDR+ and grain handling in computational photography

### Industry Resources:
3. **Fujifilm X-Series Manual** - Film Simulation documentation
   - Official grain parameter descriptions

4. **RED Camera Grain Documentation**
   - Professional cinema camera grain implementation
   - URL: https://www.red.com/red-101/grain-effects

5. **CineGrain** - Professional film grain library
   - URL: https://cinegrain.com/

### Open Source References:
6. **darktable** - `src/iop/grain.c`
   - Open-source implementation of film grain
   - URL: https://github.com/darktable-org/darktable

7. **RawTherapee** - Film grain module
   - URL: https://github.com/Beep6581/RawTherapee

---

## Next Steps

To implement in your project:
1. Start with **Solution 1 (Good Enough)** for fastest results
2. Add proper luminance weighting curve
3. Test with various images (portraits, landscapes, high contrast)
4. If needed, upgrade to **Solution 2 (Perlin)** for better quality
5. For professional film emulation, acquire grain textures for **Solution 3**

---

*Last updated: March 19, 2026*
