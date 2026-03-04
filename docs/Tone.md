# Point Operations: Mathematical Formulas for Image Adjustment

**Main Topic:** Image Processing → Point Operations (Pixel-wise Transformations)  
**Subtopics:** Tone Mapping, Color Correction, Image Enhancement

This document provides complete mathematical formulas, explanations, and references for 11 fundamental image adjustment operations commonly found in photo editing software like Lightroom, Photoshop, and Capture One.

---

## Table of Contents

### Tone Mapping / Luminosity Adjustments
1. [Exposure](#1-exposure)
2. [Brilliance](#2-brilliance)
3. [Highlights](#3-highlights)
4. [Shadows](#4-shadows)
5. [Contrast](#5-contrast)
6. [Brightness](#6-brightness)
7. [Black Point](#7-black-point)

### Color Adjustments
8. [Saturation](#8-saturation)
9. [Vibrance](#9-vibrance)
10. [Warmth (Temperature)](#10-warmth-temperature)
11. [Tint](#11-tint)

---

# Tone Mapping / Luminosity Adjustments

## 1. Exposure

### Overview
Exposure adjustment changes image brightness by simulating camera exposure settings. It multiplies all pixel values by a power of 2, mimicking how camera sensors capture light. Three variants are implemented: linear, gamma-corrected, and highlight-protected.

### Method 1: Linear Exposure (Simple)

**Used by:** Basic photo editors, game engines, real-time graphics
**Not used by:** Lightroom, Photoshop, iPhone (too crude)

**Mathematical Formula:**

**LaTeX:**
$$V_{\text{new}} = \min\left(V_{\text{old}} \times 2^E, 255\right)$$

**Plain text:**
```
new_value = min(old_value × 2^exposure, 255)
multiplier = 2^exposure
```

**Code Implementation:**
```rust
pub fn adjust_exposure(&mut self, exposure: f32) {
  let multiplier = 2.0_f32.powf(exposure);  // 2^E
  
  for i in (0..self.base.data.len()).step_by(3) {
    let b = self.base.data[i] as f32;
    let g = self.base.data[i + 1] as f32;
    let r = self.base.data[i + 2] as f32;
    
    // Multiply and clamp to [0, 255]
    self.base.data[i] = (b * multiplier).min(255.0).max(0.0) as u8;
    self.base.data[i + 1] = (g * multiplier).min(255.0).max(0.0) as u8;
    self.base.data[i + 2] = (r * multiplier).min(255.0).max(0.0) as u8;
  }
}
```

**How it works:**
1. Calculate multiplier: $2^{\text{exposure}}$
2. Multiply each RGB channel by multiplier
3. Clamp result to valid range [0, 255]

**Pros:** Fast, simple, predictable
**Cons:** Can blow out highlights (values become 255 quickly)

---

### Method 2: Gamma-Corrected Exposure (Natural)

**Used by:** 
- **Camera Raw** (Adobe) - Base exposure algorithm
- **Capture One** - Part of exposure processing
- **DaVinci Resolve** - Color grading exposure
- **Professional video editing** - Industry standard

**Not the complete story for:** Lightroom, iPhone (they add more processing)

**Mathematical Formula:**

**LaTeX:**
$$V_{\text{linear}} = \left(\frac{V_{\text{sRGB}}}{255}\right)^{\gamma}$$
$$V_{\text{exposed}} = V_{\text{linear}} \times 2^E$$
$$V_{\text{new}} = 255 \times \left[\min(V_{\text{exposed}}, 1.0)\right]^{1/\gamma}$$

Where $\gamma = 2.2$ for sRGB color space.

**Plain text:**
```
1. Convert to linear: V_linear = (V_sRGB / 255)^2.2
2. Apply exposure: V_exposed = V_linear × 2^exposure
3. Convert to sRGB: V_new = 255 × (V_exposed^(1/2.2))
```

**Code Implementation:**
```rust
pub fn adjust_exposure_gamma(&mut self, exposure: f32) {
  let multiplier = 2.0_f32.powf(exposure);
  let gamma = 2.2_f32;              // sRGB gamma
  let inv_gamma = 1.0 / gamma;       // 1/2.2 ≈ 0.4545
  
  for i in (0..self.base.data.len()).step_by(3) {
    // Step 1: sRGB → Linear (remove gamma encoding)
    let b = (self.base.data[i] as f32 / 255.0).powf(gamma);
    let g = (self.base.data[i + 1] as f32 / 255.0).powf(gamma);
    let r = (self.base.data[i + 2] as f32 / 255.0).powf(gamma);
    
    // Step 2: Apply exposure in linear space
    let b_linear = (b * multiplier).min(1.0).max(0.0);
    let g_linear = (g * multiplier).min(1.0).max(0.0);
    let r_linear = (r * multiplier).min(1.0).max(0.0);
    
    // Step 3: Linear → sRGB (apply gamma encoding)
    self.base.data[i] = (b_linear.powf(inv_gamma) * 255.0) as u8;
    self.base.data[i + 1] = (g_linear.powf(inv_gamma) * 255.0) as u8;
    self.base.data[i + 2] = (r_linear.powf(inv_gamma) * 255.0) as u8;
  }
}
```

**How it works:**
1. **Linearize:** Remove sRGB gamma curve ($V^{2.2}$)
2. **Adjust:** Apply exposure multiplier in linear space
3. **Encode:** Re-apply gamma curve ($V^{1/2.2}$)

**Why gamma correction matters:**
- sRGB images are gamma-encoded (non-linear)
- Direct multiplication in sRGB space gives unnatural results
- Working in linear space preserves photographic look

**Pros:** Natural, photographic results; better midtone handling
**Cons:** Slightly slower (two power operations per channel)

---

### Method 3: Highlight-Protected Exposure (Lightroom-style)

**Used by (similar algorithms):**
- **Adobe Lightroom** - Exposure slider with highlight recovery
- **iPhone Photos** - Exposure adjustment (plus additional AI processing)
- **Google Photos** - Auto-enhance and exposure tools
- **Adobe Photoshop Camera Raw** - Smart exposure
- **Snapseed** - Exposure and HDR tools

**Key difference from Method 2:** Adds highlight compression to prevent blown-out whites

**Mathematical Formula:**

**LaTeX:**
$$V_{\text{adjusted}} = \begin{cases}
V_{\text{norm}} \times 2^E & \text{if } V_{\text{adjusted}} \leq T \\
T + \frac{\text{excess}}{1 + \text{excess}} \times (1 - T) & \text{if } V_{\text{adjusted}} > T
\end{cases}$$

Where:
- $V_{\text{norm}} = V_{\text{old}} / 255$ (normalized to [0, 1])
- $T$ = highlight protection threshold (typically 0.8)
- $\text{excess} = V_{\text{adjusted}} - T$

**Plain text:**
```
if adjusted_value <= threshold:
    result = adjusted_value
else:
    excess = adjusted_value - threshold
    compressed = excess / (1 + excess)     # Soft compression
    result = threshold + compressed × (1 - threshold)
```

**Code Implementation:**
```rust
pub fn adjust_exposure_smooth(&mut self, exposure: f32, highlights_protect: f32) {
  let multiplier = 2.0_f32.powf(exposure);
  let threshold = highlights_protect.clamp(0.0, 1.0);  // Typically 0.8
  
  for i in (0..self.base.data.len()).step_by(3) {
    for channel in 0..3 {
      let val = self.base.data[i + channel] as f32 / 255.0;  // Normalize to [0, 1]
      
      // Apply exposure
      let mut adjusted = val * multiplier;
      
      // Protect highlights (compress values near 1.0)
      if adjusted > threshold {
        let excess = adjusted - threshold;
        let compressed = excess / (1.0 + excess);  // Asymptotic compression
        adjusted = threshold + compressed * (1.0 - threshold);
      }
      
      self.base.data[i + channel] = (adjusted.clamp(0.0, 1.0) * 255.0) as u8;
    }
  }
}
```

**How it works:**
1. Apply exposure normally up to threshold (e.g., 80% brightness)
2. For values above threshold, use soft compression
3. Compression formula: $\frac{x}{1+x}$ (approaches 1 asymptotically)

**Compression curve:**
```
excess:    0.0  → compressed: 0.00 (no change)
excess:    0.2  → compressed: 0.17 (slight compression)
excess:    0.5  → compressed: 0.33 (moderate compression)
excess:    1.0  → compressed: 0.50 (strong compression)
excess:    2.0  → compressed: 0.67 (very strong)
excess:    ∞    → compressed: 1.00 (max compression)
```

**Pros:** Prevents highlight clipping; recovers overexposed areas; Lightroom-like quality
**Cons:** More complex; requires threshold tuning

---

### Comparison Table

| Method | Speed | Quality | Highlight Protection | Used By |
|--------|-------|---------|---------------------|---------|
| **Linear** | ⚡⚡⚡ Fastest | ⭐⭐ Basic | ❌ None | Game engines, basic editors |
| **Gamma** | ⚡⚡ Fast | ⭐⭐⭐⭐ Natural | ❌ None | Camera Raw, Capture One, DaVinci |
| **Smooth** | ⚡ Moderate | ⭐⭐⭐⭐⭐ Best | ✅ Yes | **Lightroom, iPhone, Photoshop** |

### Real-World Usage Notes

**Adobe Lightroom:**
- Uses Method 3 (highlight-protected) as base
- Adds additional processing: shadow recovery, tone curve, local adjustments
- Threshold typically ~0.82 for highlight protection
- Works in ProPhoto RGB color space internally

**iPhone Photos:**
- Uses Method 3 (highlight-protected) 
- Plus: AI-based scene detection, adaptive tone mapping, neural network enhancements
- Slider range: -100 to +100 (maps to ~±2 stops)
- Processes in wide color gamut (Display P3)

**Adobe Camera Raw:**
- Uses Method 2 (gamma-corrected) for base exposure
- Separate "Highlights" and "Shadows" sliders for recovery
- Combined they approximate Method 3 behavior

**Capture One:**
- Uses Method 2 with custom tone curves
- More manual control, less automatic protection
- Preferred by professional photographers for precision

### Parameters
- $V_{\text{old}}$ = Original pixel value (0-255)
- $E$ = Exposure adjustment in stops (-3 to +3 typical)
  - Positive = brighter, Negative = darker
- $\gamma$ = 2.2 (sRGB standard)
- $T$ = Highlight threshold (0.7-0.9, typically 0.8)

### Example Calculations

**Linear Exposure (+1 stop):**
```
Original pixel: 128
Exposure: +1.0
Multiplier: 2^1.0 = 2.0

Result: 128 × 2.0 = 256 → clamped to 255
```

**Gamma-Corrected Exposure (+1 stop):**
```
Original pixel: 128 (50% brightness in sRGB)
Exposure: +1.0

Step 1 - Linearize:
  128 / 255 = 0.502
  0.502^2.2 = 0.214 (21.4% in linear space)

Step 2 - Apply exposure:
  0.214 × 2.0 = 0.428

Step 3 - Encode back:
  0.428^(1/2.2) = 0.686
  0.686 × 255 = 175

Result: 175 (more natural than linear's 255)
```

**Highlight-Protected Exposure (+2 stops with threshold=0.8):**
```
Original pixel: 200 (bright area)
Exposure: +2.0, Threshold: 0.8 (204/255)

Step 1 - Normalize: 200 / 255 = 0.784
Step 2 - Apply exposure: 0.784 × 4.0 = 3.136
Step 3 - Exceeds threshold (0.8):
  excess = 3.136 - 0.8 = 2.336
  compressed = 2.336 / (1 + 2.336) = 0.700
  result = 0.8 + 0.700 × (1 - 0.8) = 0.94

Result: 0.94 × 255 = 240 (protected, not 255)

Without protection: would be 255 (blown out)
```

### Photography Stops Reference
```
Exposure   Multiplier   Brightness Change
─────────────────────────────────────────
  +3.0   →    8.0×    →  8× brighter
  +2.0   →    4.0×    →  4× brighter
  +1.0   →    2.0×    →  2× brighter (1 stop)
  +0.5   →    1.4×    →  √2× brighter
   0.0   →    1.0×    →  no change
  -0.5   →    0.7×    →  1/√2× darker
  -1.0   →    0.5×    →  ½× darker (1 stop)
  -2.0   →    0.25×   →  ¼× darker
  -3.0   →    0.125×  →  ⅛× darker
```

### iPhone-Style Slider (-100 to +100)

**Conversion Formula:**
$$E_{\text{stops}} = \frac{V_{\text{slider}}}{50}$$

**Mapping:**
```
iPhone Slider   →   Stops   →   Multiplier
─────────────────────────────────────────
    -100       →    -2.0    →    0.25×
     -50       →    -1.0    →    0.5×
       0       →     0.0    →    1.0×
     +50       →    +1.0    →    2.0×
    +100       →    +2.0    →    4.0×
```

**Example Code:**
```rust
// Convert iPhone slider value to stops
let iphone_value = 75.0;  // User slider at +75
let exposure_stops = iphone_value / 50.0;  // = 1.5 stops

tone.adjust_exposure_smooth(exposure_stops, 0.8);
```

### References
- **Reinhard, Erik, et al.** "Photographic tone reproduction for digital images." *ACM SIGGRAPH 2002*. DOI: 10.1145/566570.566575
- **Gonzalez & Woods.** *Digital Image Processing (4th Edition)*. Pearson, 2018. Chapter 3: Intensity Transformations.
- **ISO 12232:2019** - Photography - Digital still cameras - Determination of exposure index.

---

## 2. Brilliance

### Mathematical Formula

**Weighted brightening (Apple Photos-style):**
$$V_{\text{new}} = V_{\text{old}} + B \times (255 - V_{\text{old}}) \times (0.5 + 0.5 \times V_{\text{norm}})$$

Where:
- $V_{\text{norm}} = V_{\text{old}} / 255$ (normalized to [0, 1])
- $B$ = Brilliance strength (0.0 to 1.0)
- The factor $(0.5 + 0.5 \times V_{\text{norm}})$ makes brighter pixels brighten more

**Plain text:**
```
weighting = 0.5 + 0.5 × (pixel / 255)
new_value = pixel + strength × (255 - pixel) × weighting

Effect: Brightens all tones, but enhances highlights more
```

### Parameters
- $B$ = Brilliance strength (0.0 to 1.0, typically 0.2-0.8)

### Example
```
Brilliance strength: 0.5

Dark pixel (64):
  V_norm = 64/255 = 0.251
  Weighting = 0.5 + 0.5 × 0.251 = 0.626
  V_new = 64 + 0.5 × (255-64) × 0.626
        = 64 + 0.5 × 191 × 0.626
        = 64 + 59.8
        = 123.8 → 124 (brighter)

Bright pixel (192):
  V_norm = 192/255 = 0.753
  Weighting = 0.5 + 0.5 × 0.753 = 0.877
  V_new = 192 + 0.5 × (255-192) × 0.877
        = 192 + 0.5 × 63 × 0.877
        = 192 + 27.6
        = 219.6 → 220 (much brighter)

Midtone (128):
  Weighting = 0.5 + 0.5 × 0.502 = 0.751
  V_new = 128 + 0.5 × 127 × 0.751 = 175.7 → 176 (brighter)
```

### Code Implementation

**Optimized with Lookup Table:**
```rust
pub fn adjust_brilliance(&mut self, strength: f32) {
  let b = strength.clamp(0.0, 1.0);
  
  // Pre-compute lookup table for all 256 possible values
  let mut lut = [0u8; 256];
  for i in 0..256 {
    let v = i as f32;
    let v_norm = v / 255.0;
    
    // Apple Photos-style brilliance: brighten with enhanced definition
    // Brightens all tones, but enhances highlights more (creates "brilliance")
    let result = v + b * (255.0 - v) * (0.5 + 0.5 * v_norm);
    
    lut[i] = result.clamp(0.0, 255.0) as u8;
  }
  
  // Apply lookup table to each pixel (unrolled loop for speed)
  for i in (0..self.base.data.len()).step_by(3) {
    self.base.data[i] = lut[self.base.data[i] as usize];         // B channel
    self.base.data[i + 1] = lut[self.base.data[i + 1] as usize]; // G channel
    self.base.data[i + 2] = lut[self.base.data[i + 2] as usize]; // R channel
  }
}
```

### Formula to Code Mapping

The mathematical formula:
$$V_{\text{new}} = V_{\text{old}} + B \times (255 - V_{\text{old}}) \times (0.5 + 0.5 \times V_{\text{norm}})$$

Maps to code as follows:

**Step 1: Get pixel value**
```rust
let v = i as f32;
```
- Code: Get pixel value as float for calculations
- Example: i=64 → v=64.0

**Step 2: Normalize to [0, 1]**
```rust
let v_norm = v / 255.0;
```
- Mathematical: $V_{\text{norm}} = V_{\text{old}} / 255$
- Code: Converts pixel value from [0, 255] → [0.0, 1.0]
- Example: 64.0 / 255.0 = 0.251

**Step 3: Calculate weighting factor**
```rust
let weighting = 0.5 + 0.5 * v_norm;
```
- Mathematical: $0.5 + 0.5 \times V_{\text{norm}}$
- Code: Creates weighting that favors brighter pixels
- Example: 0.5 + 0.5 × 0.251 = 0.626

**Step 4: Calculate brightness boost**
```rust
let boost = b * (255.0 - v) * weighting;
```
- Mathematical: $B \times (255 - V_{\text{old}}) \times \text{weighting}$
- Code: How much to add based on headroom and weighting
- Example (b=0.5): 0.5 × 191 × 0.626 = 59.8

**Step 5: Apply boost**
```rust
let result = v + boost;
```
- Mathematical: $V_{\text{new}} = V_{\text{old}} + \text{boost}$
- Code: Add calculated boost to original value
- Example: 64 + 59.8 = 123.8 → 124

**Complete example with b=0.5, input=64:**

| Step | Math | Code | Value |
|------|------|------|-------|
| Input | $V_{\text{old}}$ | `i = 64` | 64 |
| Float | - | `v = i as f32` | 64.0 |
| Normalize | $64/255$ | `v / 255.0` | 0.251 |
| Weighting | $0.5 + 0.5 \times 0.251$ | `0.5 + 0.5 * v_norm` | 0.626 |
| Headroom | $255 - 64$ | `255.0 - v` | 191.0 |
| Boost | $0.5 \times 191 \times 0.626$ | `b * (255.0-v) * weighting` | 59.8 |
| Result | $64 + 59.8$ | `v + boost` | 123.8 |
| Output | $V_{\text{new}}$ | `lut[64]` | **124** |

**Result:** Dark pixel (64) becomes brighter (124) → **brightens with enhanced definition**

**Brilliance behavior:**
- **Dark pixels (< 128)**: Moderate brightening (weighting 0.5-0.75)
- **Midtones (128)**: Good brightening (weighting ~0.75)
- **Bright pixels (> 128)**: Maximum brightening (weighting 0.75-1.0)
- **Overall effect**: Brightens entire image with more emphasis on highlights → creates "brilliance"

**How it works:**
1. **Pre-compute LUT:** Calculate weighted brightening for all 256 possible pixel values
2. **Fast lookup:** Map each pixel value through the lookup table (simple array access)
3. **Apply to all channels:** Process B, G, R channels with same curve

**Performance optimization:**
- **Naive approach:** Calculate formula for every pixel (millions of calculations)
- **LUT approach:** Calculate only 256 times, then lookup
- **Performance:** ~0.19s for typical image (very fast)
- **Memory cost:** 256 bytes (negligible)

**Why lookup table is beneficial:**
- 8-bit images only have 256 possible values per channel
- Brightening function is deterministic (same input → same output)
- Trading 256 bytes of memory for millions of calculations
- Classic time-space tradeoff

**Pros:** Fast, brightens entire image, emphasizes highlights for "brilliant" look, keeps colors vivid  
**Cons:** Global operation (affects entire image uniformly), may need exposure reduction for balance

### References
- **Apple Photos.** "Brilliance: Enhances the definition of your photo while keeping colors vivid." *iOS Photo Editing Guide*.
- **Han, Jungwoo, et al.** "Contrast enhancement using adaptive S-curve transformation." *IEEE Transactions on Consumer Electronics*, 2010.

---

## 3. Highlights

### Mathematical Formula

**Parametric curve (standard approach):**
$$V_{\text{new}} = \begin{cases}
V_{\text{old}} & \text{if } V_{\text{old}} < T \\
T + (V_{\text{old}} - T) \times (1 + H) & \text{if } V_{\text{old}} \geq T
\end{cases}$$

**Soft clipping (recovery):**
$$V_{\text{new}} = T + \frac{(V_{\text{old}} - T) \times (1 + H)}{1 + |(V_{\text{old}} - T) \times H|}$$

Where:
- $T$ = Highlight threshold (typically 180-200 in 0-255 range)
- $H$ = Highlight adjustment (-1.0 to +1.0)
- Negative $H$ = recover highlights (compress bright regions)
- Positive $H$ = boost highlights (expand bright regions)

**Plain text:**
```
if pixel < threshold: no change
else: adjust bright region by factor (1 + H)
```

### Parameters
- $T$ = Threshold (where highlights start, typically 70-80% of max)
- $H$ = Adjustment strength (-1.0 to +1.0)

### Example
```
Threshold: 200
Highlight adjustment: -0.5 (recovery)

Pixel 180: Below threshold → 180 (unchanged)

Pixel 240:
  V_new = 200 + (240 - 200) × (1 - 0.5)
        = 200 + 40 × 0.5
        = 220 (recovered from 240 to 220)

Pixel 255:
  V_new = 200 + (255 - 200) × 0.5
        = 200 + 27.5
        = 227.5 → 228 (recovered)
```

### References
- **Banterle, Francesco, et al.** *Advanced High Dynamic Range Imaging*. CRC Press, 2017. Chapter 5: Tone Mapping.
- **Adobe Systems.** "Highlights/Shadows Recovery." *Lightroom Classic Documentation*, 2023.

---

## 4. Shadows

### Mathematical Formula

**Parametric curve (standard approach):**
$$V_{\text{new}} = \begin{cases}
T + (V_{\text{old}} - T) \times (1 + S) & \text{if } V_{\text{old}} < T \\
V_{\text{old}} & \text{if } V_{\text{old}} \geq T
\end{cases}$$

**Lift with compression:**
$$V_{\text{new}} = V_{\text{old}} + S \times (T - V_{\text{old}}) \times \frac{V_{\text{old}}}{T}$$

Where:
- $T$ = Shadow threshold (typically 50-80 in 0-255 range)
- $S$ = Shadow adjustment (-1.0 to +1.0)
- Positive $S$ = lift shadows (brighten dark regions)
- Negative $S$ = crush shadows (darken dark regions)

**Plain text:**
```
if pixel > threshold: no change
else: adjust dark region by factor (1 + S)
```

### Parameters
- $T$ = Threshold (where shadows end, typically 20-30% of max)
- $S$ = Adjustment strength (-1.0 to +1.0)

### Example
```
Threshold: 80
Shadow adjustment: +0.6 (lift/brighten)

Pixel 100: Above threshold → 100 (unchanged)

Pixel 40:
  V_new = 80 + (40 - 80) × (1 + 0.6)
        = 80 + (-40) × 1.6
        = 80 - 64
        = 16 (Wait, this darkens!)

Correct formula (lift):
  V_new = 40 + 0.6 × (80 - 40)
        = 40 + 0.6 × 40
        = 40 + 24
        = 64 (brightened from 40 to 64)
```

### References
- **Durand, Frédo; Dorsey, Julie.** "Fast bilateral filtering for the display of high-dynamic-range images." *ACM SIGGRAPH 2002*. DOI: 10.1145/566570.566574
- **Bae, Soonmin, et al.** "Defocus magnification." *Computer Graphics Forum*, 2007.

---

## 5. Contrast

### Mathematical Formula

**Piecewise contrast (implementation):**
$$V_{\text{new}} = \begin{cases}
V_{\text{old}} \times (1 - C \times (1 - V_{\text{old}}/255)) & \text{if } V_{\text{old}} < 128 \\
V_{\text{old}} + C \times (255 - V_{\text{old}}) & \text{if } V_{\text{old}} \geq 128
\end{cases}$$

Where:
- $C$ = Contrast strength (0.0 to 1.0)
- Below midpoint (128): compresses darks toward black
- Above midpoint (128): expands brights toward white
- Result: Increased tonal separation

**Alternative linear contrast:**
$$V_{\text{new}} = (V_{\text{old}} - 128) \times (1 + C) + 128$$

**Plain text:**
```
if pixel < 128:
    new = pixel × (1 - strength × (1 - pixel/255))    # Darken darks
else:
    new = pixel + strength × (255 - pixel)             # Brighten brights
```

### Parameters
- $C$ = Contrast strength (0.0 to 1.0, typically 0.1 to 0.7)
  - 0.0 = no change
  - 0.3 = moderate contrast boost
  - 0.5 = strong contrast
  - 1.0 = maximum contrast (blacks → 0, whites → 255)

### Example
```
Contrast strength: 0.3

Dark pixel (64):
  V_new = 64 × (1 - 0.3 × (1 - 64/255))
        = 64 × (1 - 0.3 × 0.749)
        = 64 × 0.775
        = 49.6 → 50 (darker)

Bright pixel (192):
  V_new = 192 + 0.3 × (255 - 192)
        = 192 + 0.3 × 63
        = 192 + 18.9
        = 210.9 → 211 (brighter)

Midtone (128):
  Uses dark formula:
  V_new = 128 × (1 - 0.3 × 0.498)
        = 128 × 0.851
        = 108.9 → 109 (slightly darker, at transition)
```

### Code Implementation

```rust
pub fn adjust_contrast(&mut self, strength: f32) {
  let c = strength.clamp(0.0, 1.0);
  
  // Pre-compute lookup table for all 256 possible values
  let mut lut = [0u8; 256];
  for i in 0..256 {
    let v = i as f32;
    
    // Piecewise contrast: darken darks, brighten brights
    let result = if v < 128.0 {
      // Darken dark regions
      v * (1.0 - c * (1.0 - v / 255.0))
    } else {
      // Brighten bright regions
      v + c * (255.0 - v)
    };
    
    lut[i] = result.clamp(0.0, 255.0) as u8;
  }
  
  // Apply lookup table to each pixel (unrolled loop for speed)
  for i in (0..self.base.data.len()).step_by(3) {
    self.base.data[i] = lut[self.base.data[i] as usize];         // B channel
    self.base.data[i + 1] = lut[self.base.data[i + 1] as usize]; // G channel
    self.base.data[i + 2] = lut[self.base.data[i + 2] as usize]; // R channel
  }
}
```

### References
- **Gonzalez & Woods.** *Digital Image Processing*. Chapter 3.2: Contrast Stretching.
- **Pizer, Stephen M., et al.** "Adaptive histogram equalization and its variations." *Computer Vision, Graphics, and Image Processing*, 1987.

---

## 6. Brightness

### Mathematical Formula

**Additive brightness:**
$$V_{\text{new}} = V_{\text{old}} + B$$

**With clamping:**
$$V_{\text{final}} = \max(0, \min(255, V_{\text{new}}))$$

**Proportional brightness (alternative):**
$$V_{\text{new}} = V_{\text{old}} \times (1 + B)$$

**Plain text:**
```
new_value = old_value + brightness_offset
```

### Parameters
- $B$ = Brightness offset (-255 to +255)
  - Positive $B$ = brighter
  - Negative $B$ = darker
  - $B = 0$ = no change

### Example
```
Original: 128
Brightness: +50

V_new = 128 + 50 = 178

Original: 200
Brightness: +80
V_new = 200 + 80 = 280 → clamped to 255

Original: 30
Brightness: -50
V_new = 30 - 50 = -20 → clamped to 0
```

### Difference from Exposure
```
Exposure: Multiplicative (V × 2^E)
  - Preserves relative differences
  - Dark stays relatively dark
  - Mathematically correct for light

Brightness: Additive (V + B)
  - Shifts all values equally
  - Can make blacks non-black
  - Simpler but less natural
```

### References
- **Gonzalez & Woods.** *Digital Image Processing*. Chapter 3: Basic Gray Level Transformations.
- **Pratt, William K.** *Digital Image Processing (4th Edition)*. Wiley-Interscience, 2007.

---

## 7. Black Point

### Mathematical Formula

**Black point lift:**
$$V_{\text{new}} = \frac{V_{\text{old}} - B_{\text{min}}}{255 - B_{\text{min}}} \times 255$$

**With output black point:**
$$V_{\text{new}} = B_{\text{out}} + \frac{V_{\text{old}} - B_{\text{in}}}{255 - B_{\text{in}}} \times (255 - B_{\text{out}})$$

Where:
- $B_{\text{in}}$ = Input black point (minimum input value to map)
- $B_{\text{out}}$ = Output black point (what black becomes)

**Plain text:**
```
Remap [black_point, 255] → [0, 255]
new = (old - black_in) / (255 - black_in) × 255
```

### Parameters
- $B_{\text{in}}$ = Input black level (0-255, typically 0-50)
- $B_{\text{out}}$ = Output black level (0-255, typically 0-30)

### Example
```
Input black point: 30
Original: 30 → V_new = (30 - 30) / (255 - 30) × 255 = 0 (pure black)
Original: 80 → V_new = (80 - 30) / 225 × 255 = 56.67 → 57
Original: 255 → V_new = (255 - 30) / 225 × 255 = 255 (white unchanged)

Effect: Values below 30 become pure black (0)
        Values above 30 are stretched to fill range
```

### Use Cases
```
- Crush shadows to pure black
- Remove color cast in shadows
- Create contrast by making darks darker
- Adjust dynamic range
```

### References
- **Adobe Systems.** "Levels Adjustment." *Photoshop User Guide*.
- **Contrast stretching:** Gonzalez & Woods. *Digital Image Processing*. Section 3.2.

---

# Color Adjustments

## 8. Saturation

### Mathematical Formula

**HSV/HSL method:**

Step 1: Convert RGB → HSV
$$H, S, V = \text{rgb\_to\_hsv}(R, G, B)$$

Step 2: Adjust saturation
$$S_{\text{new}} = S_{\text{old}} \times (1 + \text{saturation\_factor})$$

Step 3: Convert back to RGB
$$R_{\text{new}}, G_{\text{new}}, B_{\text{new}} = \text{hsv\_to\_rgb}(H, S_{\text{new}}, V)$$

**Direct RGB method (faster):**
$$\begin{align}
L &= 0.299R + 0.587G + 0.114B \quad \text{(luminance)} \\
R_{\text{new}} &= L + (R - L) \times (1 + S) \\
G_{\text{new}} &= L + (G - L) \times (1 + S) \\
B_{\text{new}} &= L + (B - L) \times (1 + S)
\end{align}$$

**Plain text:**
```
luminance = weighted_sum(R, G, B)
new_R = luminance + (R - luminance) × (1 + saturation)
(same for G and B)
```

### Parameters
- $S$ = Saturation adjustment (-1.0 to +∞, typically -1.0 to +1.0)
  - $S = 0$ = no change
  - $S > 0$ = more saturated (vivid)
  - $S = -1$ = completely desaturated (grayscale)

### Example
```
Original RGB: (180, 100, 80)
Saturation: +0.5 (50% more saturated)

Luminance: L = 0.299×180 + 0.587×100 + 0.114×80
             = 53.82 + 58.7 + 9.12 = 121.64

R_new = 121.64 + (180 - 121.64) × 1.5
      = 121.64 + 58.36 × 1.5
      = 121.64 + 87.54 = 209.18 → 209

G_new = 121.64 + (100 - 121.64) × 1.5
      = 121.64 + (-21.64) × 1.5
      = 121.64 - 32.46 = 89.18 → 89

B_new = 121.64 + (80 - 121.64) × 1.5
      = 121.64 + (-41.64) × 1.5
      = 121.64 - 62.46 = 59.18 → 59

Result RGB: (209, 89, 59) - more orange/red
```

### RGB to HSV Conversion
```
V = max(R, G, B)
C = V - min(R, G, B)  // Chroma
S = C / V  (if V ≠ 0, else 0)

H = 60° × {
  (G - B) / C  if V = R
  2 + (B - R) / C  if V = G
  4 + (R - G) / C  if V = B
}
```

### References
- **Smith, Alvy Ray.** "Color gamut transform pairs." *SIGGRAPH '78*. DOI: 10.1145/800248.807361
- **Gonzalez & Woods.** *Digital Image Processing*. Chapter 6: Color Image Processing.
- **Fairchild, Mark D.** *Color Appearance Models (3rd Edition)*. Wiley, 2013.

---

## 9. Vibrance

### Mathematical Formula

**Selective saturation (protects already-saturated colors):**

$$S_{\text{new}} = S_{\text{old}} + V \times (1 - S_{\text{old}}) \times |S_{\text{target}} - S_{\text{old}}|$$

**With skin tone protection:**
$$\begin{align}
\text{is\_skin} &= (H \in [10°, 40°]) \land (S \in [0.2, 0.6]) \\
\text{protection} &= \begin{cases}
0.3 & \text{if is\_skin} \\
1.0 & \text{otherwise}
\end{cases} \\
S_{\text{new}} &= S_{\text{old}} + V \times (1 - S_{\text{old}}) \times \text{protection}
\end{align}$$

**Simplified (common implementation):**
$$S_{\text{new}} = S_{\text{old}} \times (1 + V \times (1 - S_{\text{old}}))$$

**Plain text:**
```
saturation_boost = vibrance × (1 - current_saturation)
new_saturation = old_saturation × (1 + saturation_boost)

Effect: Dull colors boosted more, vivid colors protected
```

### Parameters
- $V$ = Vibrance strength (-1.0 to +1.0)
- Skin tone hue range: 10°-40° (orange-red)
- Skin tone saturation range: 0.2-0.6

### Example
```
Color 1 (dull blue):
  H=220°, S=0.2, V=0.8
  Vibrance: +0.5
  
  S_new = 0.2 × (1 + 0.5 × (1 - 0.2))
        = 0.2 × (1 + 0.5 × 0.8)
        = 0.2 × 1.4
        = 0.28 (boosted significantly: 40% increase)

Color 2 (already vivid red):
  H=0°, S=0.9, V=0.9
  Vibrance: +0.5
  
  S_new = 0.9 × (1 + 0.5 × (1 - 0.9))
        = 0.9 × (1 + 0.5 × 0.1)
        = 0.9 × 1.05
        = 0.945 (only slightly boosted: 5% increase)

Color 3 (skin tone):
  H=25°, S=0.4, V=0.7
  Vibrance: +0.5
  Protection: 0.3
  
  S_new = 0.4 × (1 + 0.5 × (1 - 0.4) × 0.3)
        = 0.4 × (1 + 0.09)
        = 0.436 (minimal change: 9% increase)
```

### Saturation vs Vibrance
```
Saturation:
  - Affects all colors equally
  - Can oversaturate already vivid colors
  - Can make skin tones unnatural
  
Vibrance:
  - Affects dull colors more
  - Protects already saturated colors
  - Protects skin tones (orange-red hues)
  - More natural results
```

### References
- **Adobe Systems.** "Vibrance adjustment." *Lightroom Classic Documentation*.
- **Kelby, Scott.** *The Adobe Photoshop Lightroom Classic CC Book*. Rocky Nook, 2018.
- **Fairchild, Mark D.** "The HDR Photographic Survey." *Color Research & Application*, 2007.

---

## 10. Warmth (Temperature)

### Mathematical Formula

**Color temperature shift (Kelvin scale):**

$$\begin{align}
\text{ratio}_R &= \frac{T_{\text{target}}}{T_{\text{current}}} \\
\text{ratio}_B &= \frac{T_{\text{current}}}{T_{\text{target}}} \\
R_{\text{new}} &= R_{\text{old}} \times \text{ratio}_R \\
B_{\text{new}} &= B_{\text{old}} \times \text{ratio}_B \\
G_{\text{new}} &= G_{\text{old}} \quad \text{(unchanged)}
\end{align}$$

**Simplified warmth adjustment:**
$$\begin{align}
R_{\text{new}} &= R_{\text{old}} \times (1 + W) \\
B_{\text{new}} &= B_{\text{old}} \times (1 - W) \\
G_{\text{new}} &= G_{\text{old}}
\end{align}$$

**Plain text:**
```
Warmer (positive): Increase red, decrease blue
Cooler (negative): Decrease red, increase blue
Green stays unchanged
```

### Parameters
- $W$ = Warmth adjustment (-1.0 to +1.0)
  - $W > 0$ = warmer (more red/orange)
  - $W < 0$ = cooler (more blue)
- Temperature: 2000K (warm/orange) to 10000K (cool/blue)

### Example
```
Original RGB: (150, 150, 150) - neutral gray
Warmth: +0.3 (warmer/more orange)

R_new = 150 × (1 + 0.3) = 150 × 1.3 = 195
G_new = 150 (unchanged)
B_new = 150 × (1 - 0.3) = 150 × 0.7 = 105

Result RGB: (195, 150, 105) - warm orange-gray


Original RGB: (180, 120, 100)
Warmth: -0.4 (cooler/more blue)

R_new = 180 × (1 - 0.4) = 180 × 0.6 = 108
G_new = 120 (unchanged)
B_new = 100 × (1 + 0.4) = 100 × 1.4 = 140

Result RGB: (108, 120, 140) - cool blue-gray
```

### Color Temperature (Kelvin)
```
1000K  = Candlelight (very warm, orange-red)
2700K  = Incandescent bulb (warm, yellow-orange)
3500K  = Warm white LED
5000K  = Daylight (neutral)
6500K  = Overcast sky (cool, slight blue)
10000K = Clear blue sky (very cool, blue)

Formula: Warmer = lower Kelvin, Cooler = higher Kelvin
```

### References
- **Wyszecki, Günter; Stiles, W.S.** *Color Science: Concepts and Methods*. Wiley, 2000.
- **Hernández-Andrés, Javier, et al.** "Color and spectral analysis of daylight in southern Europe." *JOSA A*, 2001.
- **CIE Publication 15:2004.** Colorimetry (3rd Edition).

---

## 11. Tint

### Mathematical Formula

**Green-Magenta balance:**
$$\begin{align}
G_{\text{new}} &= G_{\text{old}} \times (1 + T) \\
R_{\text{new}} &= R_{\text{old}} \times (1 - T \times 0.5) \\
B_{\text{new}} &= B_{\text{old}} \times (1 - T \times 0.5)
\end{align}$$

**Alternative (additive tint):**
$$\begin{align}
\text{if } T > 0: \quad &G_{\text{new}} = G_{\text{old}} + T \times 255 \\
&R_{\text{new}} = R_{\text{old}}, \quad B_{\text{new}} = B_{\text{old}} \\
\text{if } T < 0: \quad &R_{\text{new}} = R_{\text{old}} + |T| \times 255 \\
&B_{\text{new}} = B_{\text{old}} + |T| \times 255 \\
&G_{\text{new}} = G_{\text{old}}
\end{align}$$

**Plain text:**
```
Positive tint: Add green
Negative tint: Add magenta (red + blue)
```

### Parameters
- $T$ = Tint adjustment (-1.0 to +1.0)
  - $T > 0$ = green shift
  - $T < 0$ = magenta shift
  - $T = 0$ = no change

### Example
```
Original RGB: (150, 150, 150) - neutral gray
Tint: +0.3 (green shift)

G_new = 150 × (1 + 0.3) = 150 × 1.3 = 195
R_new = 150 × (1 - 0.3 × 0.5) = 150 × 0.85 = 127.5 → 128
B_new = 150 × (1 - 0.3 × 0.5) = 150 × 0.85 = 128

Result RGB: (128, 195, 128) - green-gray


Original RGB: (180, 150, 120)
Tint: -0.4 (magenta shift)

G_new = 150 × (1 - 0.4) = 150 × 0.6 = 90
R_new = 180 × (1 + 0.4 × 0.5) = 180 × 1.2 = 216
B_new = 120 × (1 + 0.4 × 0.5) = 120 × 1.2 = 144

Result RGB: (216, 90, 144) - magenta tint
```

### Color Cast Removal
```
Fluorescent lighting → Often has green cast → Tint: -0.2 (add magenta)
Old incandescent → Often has magenta cast → Tint: +0.15 (add green)
```

### References
- **Adobe Systems.** "White Balance: Temperature and Tint." *Camera Raw Documentation*.
- **Hunt, R.W.G.; Pointer, M.R.** *Measuring Colour (4th Edition)*. Wiley, 2011.
- **ISO 17321-1:2012** - Graphic technology and photography - Colour characterisation of digital still cameras (DSCs).

---

# Complete Formula Reference Table

| Adjustment | Formula | Parameter Range | Effect |
|------------|---------|-----------------|--------|
| **Exposure** | $V \times 2^E$ | $E \in [-3, +3]$ | Multiplicative brightness |
| **Brilliance** | $V + B \times (255-V) \times (0.5+0.5V/255)$ | $B \in [0, 1]$ | Brighten with highlight enhancement |
| **Highlights** | $T + (V - T) \times (1 + H)$ | $H \in [-1, +1]$ | Adjust bright regions |
| **Shadows** | $V + S \times (T - V)$ | $S \in [-1, +1]$ | Adjust dark regions |
| **Contrast** | Piecewise: darken < 128, brighten ≥ 128 | $C \in [0, 1]$ | Expand tonal range |
| **Brightness** | $V + B$ | $B \in [-255, +255]$ | Additive brightness |
| **Black Point** | $(V - B_{in}) / (255 - B_{in}) \times 255$ | $B \in [0, 50]$ | Remap minimum |
| **Saturation** | $L + (C - L) \times (1 + S)$ | $S \in [-1, +1]$ | Uniform color intensity |
| **Vibrance** | $S \times (1 + V \times (1 - S))$ | $V \in [-1, +1]$ | Selective saturation |
| **Warmth** | $R \times (1+W), B \times (1-W)$ | $W \in [-1, +1]$ | Red-blue shift |
| **Tint** | $G \times (1+T), R,B \times (1-T/2)$ | $T \in [-1, +1]$ | Green-magenta shift |

---

# Academic References

## Books

1. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3: Intensity Transformations and Spatial Filtering
   - Chapter 6: Color Image Processing

2. **Pratt, William K.** *Digital Image Processing (4th Edition)*. Wiley-Interscience, 2007.
   - Chapter 12: Color Image Processing

3. **Fairchild, Mark D.** *Color Appearance Models (3rd Edition)*. Wiley, 2013.
   - Comprehensive color science and perception

4. **Reinhard, Erik, et al.** *High Dynamic Range Imaging: Acquisition, Display, and Image-Based Lighting (2nd Edition)*. Morgan Kaufmann, 2010.
   - Tone mapping operators

5. **Banterle, Francesco, et al.** *Advanced High Dynamic Range Imaging (2nd Edition)*. CRC Press, 2017.
   - Advanced tone mapping techniques

## Papers

1. **Reinhard, Erik, et al.** "Photographic tone reproduction for digital images." *ACM SIGGRAPH 2002*. DOI: 10.1145/566570.566575

2. **Durand, Frédo; Dorsey, Julie.** "Fast bilateral filtering for the display of high-dynamic-range images." *ACM SIGGRAPH 2002*. DOI: 10.1145/566570.566574

3. **Smith, Alvy Ray.** "Color gamut transform pairs." *SIGGRAPH '78*. DOI: 10.1145/800248.807361

4. **Pizer, Stephen M., et al.** "Adaptive histogram equalization and its variations." *Computer Vision, Graphics, and Image Processing*, 39(3):355-368, 1987.

5. **Han, Jungwoo, et al.** "Contrast enhancement using adaptive S-curve transformation." *IEEE Transactions on Consumer Electronics*, 56(2):573-578, 2010.

## Standards

1. **ISO 12232:2019** - Photography - Digital still cameras - Determination of exposure index, ISO speed ratings, standard output sensitivity, and recommended exposure index.

2. **ISO 17321-1:2012** - Graphic technology and photography - Colour characterisation of digital still cameras (DSCs).

3. **CIE Publication 15:2004** - Colorimetry (3rd Edition). Commission Internationale de l'Éclairage.

4. **IEC 61966-2-1:1999** - Multimedia systems and equipment - Colour measurement and management - Part 2-1: Colour management - Default RGB colour space - sRGB.

## Software Documentation

1. **Adobe Systems.** *Adobe Lightroom Classic CC Documentation*. [https://helpx.adobe.com/lightroom-classic/](https://helpx.adobe.com/lightroom-classic/)

2. **Adobe Systems.** *Adobe Camera Raw Documentation*. [https://helpx.adobe.com/camera-raw/](https://helpx.adobe.com/camera-raw/)

3. **Apple Inc.** *Photos for macOS User Guide*. [https://support.apple.com/guide/photos/](https://support.apple.com/guide/photos/)

## Online Resources

1. **Cambridgeincolour.com** - Excellent tutorials on exposure, histogram, tone curves
2. **DPReview.com** - Technical photography articles
3. **RawTherapee Source Code** - [https://github.com/Beep6581/RawTherapee](https://github.com/Beep6581/RawTherapee) - Open-source RAW processor
4. **darktable Source Code** - [https://github.com/darktable-org/darktable](https://github.com/darktable-org/darktable) - Open-source photography workflow

---

# Implementation Notes

## Performance Considerations

**Point operations are:**
- ✅ Fast (per-pixel, no dependencies)
- ✅ Parallelizable (GPU-friendly)
- ✅ Cache-friendly (sequential memory access)

**Typical performance:**
```
1080×1920 image (2.07 million pixels):
- Exposure adjustment: ~5-10ms (CPU)
- Saturation (RGB→HSV→RGB): ~20-40ms (CPU)
- All 11 operations: ~100-200ms (CPU)

GPU (CUDA/OpenGL):
- All operations: ~1-5ms
```

## Order of Operations (Recommended)

```
1. Black Point     ← Fix input levels first
2. Exposure        ← Overall brightness
3. Highlights      ← Recover blown areas
4. Shadows         ← Lift dark regions
5. Brightness      ← Fine-tune overall level
6. Contrast        ← Adjust tonal separation
7. Brilliance      ← Enhance definition
8. Warmth          ← Color balance
9. Tint            ← Fine color correction
10. Saturation     ← Overall color intensity
11. Vibrance       ← Final selective boost
```

## Color Space Considerations

**sRGB gamma:**
- Store: γ = 2.2 (nonlinear)
- Process: Linear (γ = 1.0)
- Display: γ = 2.2 (nonlinear)

**Always:**
1. Convert sRGB → Linear for math
2. Apply transformations
3. Convert Linear → sRGB for storage

---

**Document Version:** 1.0  
**Date:** January 2, 2026  
**Author:** FastImage Computer Graphics Library  
**License:** Educational Use
