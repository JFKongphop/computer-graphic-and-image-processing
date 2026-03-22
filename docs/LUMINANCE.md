# Luminance: Human Vision and Color Perception

Complete guide to luminance calculation, color perception, and the biological basis of the ITU-R BT.709 standard.

---

## Table of Contents
- [What is Luminance?](#what-is-luminance)
- [Biological Basis](#biological-basis)
- [Mathematical Formulas](#mathematical-formulas)
- [Standards and History](#standards-and-history)
- [Applications](#applications)
- [References](#references)

---

## What is Luminance?

**Luminance** is the perceived brightness of a color as seen by the human eye. It represents how "light" or "dark" a color appears to us, independent of its hue or saturation.

### Key Concepts:

**Luminance vs. Brightness:**
- **Luminance (Y)**: Objective, measurable, based on human vision science
- **Brightness**: Subjective perception, varies by individual
- **Lightness**: Perceptual, nonlinear response to luminance

**Why Not Simple Average?**

❌ **Wrong:** `(R + G + B) / 3 = 170` for RGB(255, 170, 85)
- Treats all colors equally
- Doesn't match human perception

✅ **Correct:** `0.2126×R + 0.7152×G + 0.0722×B` 
- Weighted by human eye sensitivity
- Green contributes most (71%)
- Matches what we actually see

---

## Biological Basis

The ITU-R BT.709 coefficients (0.2126, 0.7152, 0.0722) are **directly derived from human eye biology**.

### 1. Anatomy of Color Vision

**Retinal Photoreceptors:**

```
Human Eye
├── Rods (120 million)
│   └── Low-light vision (scotopic), no color
└── Cones (6 million)
    ├── L-cones (~64%) - Long wavelength (red)      ~560 nm
    ├── M-cones (~32%) - Medium wavelength (green)  ~530 nm
    └── S-cones (~4%)  - Short wavelength (blue)    ~420 nm
```

**Key Insight:** More M-cones (green) ≠ higher green coefficient!

### 2. Photopic Luminosity Function V(λ)

The **CIE photopic luminosity function** measures human eye sensitivity to different wavelengths:

```
Sensitivity Peak: 555 nm (yellow-green)

   1.0 |        ╭──────╮
       |       ╱        ╲
   0.7 |      ╱          ╲         ← Green peak
       |     ╱            ╲
   0.5 |    ╱              ╲
       |   ╱                ╲
   0.2 |  ╱                  ╲      ← Red & Blue lower
       | ╱                    ╲
   0.0 |╱______________________╲___
       400  500  555  600  700 (nm)
       Blue      Green    Red
```

**Why 555nm?**
- Maximum sensitivity of M-cones + L-cones combined
- Optimal for daylight vision (sun spectrum peaks here)
- Evolutionary advantage: detect vegetation, natural scenes

### 3. Cone Cell Spectral Sensitivities

Each cone type responds to a range of wavelengths:

**L-cones (Red):**
- Peak: ~560 nm (yellow-green, not pure red!)
- Range: 500-700 nm
- Contribution to luminance: ~21%

**M-cones (Green):**
- Peak: ~530 nm (green)
- Range: 450-630 nm
- Contribution to luminance: ~71% ← **Highest**

**S-cones (Blue):**
- Peak: ~420 nm (blue-violet)
- Range: 400-500 nm
- Contribution to luminance: ~7% ← **Lowest**

### 4. Neural Processing

**From Cone Signals to Perceived Brightness:**

```
Cone Activation
     ↓
Retinal Ganglion Cells (combine signals)
     ↓
Lateral Geniculate Nucleus (LGN)
     ↓
Visual Cortex V1
     ↓
Perceived Luminance
```

The visual system creates **opponent channels**:
- **Luminance channel (Y)**: L + M signals (primarily)
- **Red-Green channel**: L - M signals
- **Blue-Yellow channel**: S - (L+M) signals

**Key Finding:** Luminance perception is dominated by L + M cones (red + green), with minimal blue contribution!

### 5. Evolutionary Biology

**Why Green Dominates:**

1. **Solar spectrum**: Sun's peak output is ~500nm (cyan-green)
2. **Vegetation**: Most natural environments are green
3. **Acuity**: M-cones are densest near fovea (central vision)
4. **Motion detection**: Green-sensitive pathways detect movement better
5. **Historical selection**: Better survival with enhanced green sensitivity

**Why Blue is Weak:**
1. **Atmospheric scatter**: Blue light scatters more (lower contrast)
2. **Fewer S-cones**: Only 2-4% of total cones
3. **No foveal S-cones**: Central vision has zero blue receptors
4. **Chromatic aberration**: Blue focuses differently than red/green

---

## Mathematical Formulas

### 1. ITU-R BT.709 Luminance (Digital Imaging)

$$Y = 0.2126 \cdot R + 0.7152 \cdot G + 0.0722 \cdot B$$

**Where:**
- $Y$ = Luminance (0.0 to 1.0 or 0 to 255)
- $R, G, B$ = Linear RGB values (gamma-corrected)

**Coefficients derivation:**
```
From CIE XYZ color space:
Y_CIE = 0.2126 × X_R + 0.7152 × X_G + 0.0722 × X_B

Where X_R, X_G, X_B are CIE XYZ tristimulus values
for BT.709 RGB primaries
```

### 2. Alternative Standards

**ITU-R BT.601 (SDTV - Older Standard):**
$$Y = 0.299 \cdot R + 0.587 \cdot G + 0.114 \cdot B$$

**Used for:**
- NTSC, PAL analog television
- JPEG images (legacy)
- Older video codecs

**ITU-R BT.2020 (UHDTV - Newer Standard):**
$$Y = 0.2627 \cdot R + 0.6780 \cdot G + 0.0593 \cdot B$$

**Used for:**
- 4K/8K Ultra HD
- HDR content
- Wide color gamut displays

### 3. CIE 1931 Color Space

**XYZ to Luminance:**
$$Y_{CIE} = \int_\lambda S(\lambda) \cdot \bar{y}(\lambda) \, d\lambda$$

**Where:**
- $S(\lambda)$ = spectral power distribution
- $\bar{y}(\lambda)$ = CIE luminous efficiency function (V(λ))

**RGB to XYZ (for BT.709):**
$$
\begin{bmatrix} X \\ Y \\ Z \end{bmatrix} = 
\begin{bmatrix}
0.4124 & 0.3576 & 0.1805 \\
0.2126 & 0.7152 & 0.0722 \\
0.0193 & 0.1192 & 0.9505
\end{bmatrix}
\begin{bmatrix} R \\ G \\ B \end{bmatrix}
$$

Notice: Second row contains BT.709 luminance coefficients!

### 4. Gamma Correction Relationship

**Linear RGB vs. Gamma-Corrected RGB:**

```rust
// Gamma-corrected (sRGB) to Linear
fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

// Calculate luminance (should use linear RGB)
fn luminance_correct(r: u8, g: u8, b: u8) -> f32 {
    let r_linear = srgb_to_linear(r as f32 / 255.0);
    let g_linear = srgb_to_linear(g as f32 / 255.0);
    let b_linear = srgb_to_linear(b as f32 / 255.0);
    
    0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
}
```

**In practice:** Many applications use gamma-corrected RGB directly (approximation).

### 5. Perceptual Lightness (L*)

**CIELAB Lightness (perceptually uniform):**
$$L^* = 116 \cdot \left(\frac{Y}{Y_n}\right)^{1/3} - 16$$

Where $Y_n$ is reference white luminance.

**Relationship:**
- Linear luminance: Y (physical)
- Perceptual lightness: L* (what we perceive)
- L* accounts for nonlinear human brightness perception

---

## Standards and History

### Timeline of Luminance Standards

**1931 - CIE XYZ Color Space**
- First international standard for color
- Based on color matching experiments
- Established photopic luminosity function V(λ)
- Foundation for all modern color science

**1953 - NTSC (BT.470)**
- First color television standard
- Coefficients: 0.299, 0.587, 0.114
- Used phosphors available at the time

**1990 - ITU-R BT.709 (HDTV)**
- Modern HDTV standard
- Coefficients: 0.2126, 0.7152, 0.0722
- Improved phosphor primaries
- **Most widely used today**

**1996 - sRGB (IEC 61966-2-1)**
- Standard for computer displays
- Uses BT.709 primaries and luminance
- Default color space for web/images

**2012 - ITU-R BT.2020 (UHDTV)**
- 4K/8K Ultra HD standard
- Coefficients: 0.2627, 0.6780, 0.0593
- Wider color gamut

### Why Different Standards?

Standards differ due to:
1. **Display technology**: Different phosphor/LED primaries
2. **Color gamut**: Wider gamut = different coefficients
3. **Historical constraints**: Technology limitations when standard created
4. **Application**: Broadcast TV vs. cinema vs. web

**All standards share:** Based on human vision biology (CIE 1931)

---

## Applications

### 1. Image Processing

**Grayscale Conversion:**
```rust
// Convert RGB to grayscale using luminance
fn rgb_to_gray(r: u8, g: u8, b: u8) -> u8 {
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8
}
```

**Why use luminance?**
- Preserves perceived brightness
- Better than simple average
- Maintains contrast relationships

### 2. Video Compression

**YCbCr Color Space:**
```
Y  = Luminance (0.2126R + 0.7152G + 0.0722B)
Cb = Blue chroma  (B - Y)
Cr = Red chroma   (R - Y)
```

**Benefits:**
- Human eye more sensitive to Y than Cb/Cr
- Can downsample chroma (4:2:0) without visible loss
- Used in JPEG, MPEG, H.264, H.265

### 3. Color Grading (Our Grain Implementation)

**Luminance-Weighted Effects:**
```rust
// Apply more grain in midtones, less in highlights
let luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;
let weight = luminance_weight(luminance);
let grain_strength = base_intensity * weight * noise_val * 255.0;
```

**Why:**
- Film grain naturally varies by brightness
- Matches analog film behavior
- Perceptually correct effect

### 4. HDR Tone Mapping

**Preserve Luminance During Tone Mapping:**
```rust
// Compress HDR luminance to LDR range
fn tone_map(hdr_rgb: RGB) -> RGB {
    let Y_hdr = luminance(hdr_rgb);
    let Y_ldr = tone_curve(Y_hdr);  // Apply tone curve to luminance
    
    // Scale RGB to match new luminance
    let scale = Y_ldr / Y_hdr;
    hdr_rgb * scale
}
```

### 5. Contrast Enhancement

**Local Contrast (CLAHE):**
- Adjusts histogram based on luminance
- Preserves color hue and saturation
- Enhances detail in shadows/highlights

### 6. Edge Detection

**Luminance Gradients:**
- Sobel, Canny edge detectors work on luminance channel
- More robust than per-channel edge detection
- Matches human perception of edges

### 7. Color Correction

**White Balance:**
- Adjust RGB so neutral gray has equal luminance contribution
- Ensures white appears white under different lighting

**Color Grading:**
- Adjust shadows/midtones/highlights based on luminance ranges
- Professional video editing (DaVinci Resolve, Premiere)

---

## Code Examples

### Basic Luminance Calculation

```rust
/// Calculate luminance using ITU-R BT.709 standard
fn luminance_bt709(r: u8, g: u8, b: u8) -> f32 {
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 255.0
}

// Example usage
let rgb = (128, 100, 80);
let luma = luminance_bt709(rgb.0, rgb.1, rgb.2);
println!("Luminance: {:.3}", luma);  // 0.410
```

### Comparing Standards

```rust
fn compare_standards(r: u8, g: u8, b: u8) {
    // BT.709 (modern HDTV)
    let bt709 = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 255.0;
    
    // BT.601 (legacy SDTV)
    let bt601 = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    
    // BT.2020 (UHDTV)
    let bt2020 = (0.2627 * r as f32 + 0.6780 * g as f32 + 0.0593 * b as f32) / 255.0;
    
    println!("RGB({}, {}, {})", r, g, b);
    println!("  BT.709:  {:.4}", bt709);
    println!("  BT.601:  {:.4}", bt601);
    println!("  BT.2020: {:.4}", bt2020);
}

// Example: Pure green
compare_standards(0, 255, 0);
// Output:
//   BT.709:  0.7152
//   BT.601:  0.5870
//   BT.2020: 0.6780
```

### Gamma-Correct Luminance

```rust
/// Convert sRGB (gamma-corrected) to linear RGB
fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate perceptually correct luminance
fn luminance_linear(r: u8, g: u8, b: u8) -> f32 {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;
    
    let r_lin = srgb_to_linear(r_norm);
    let g_lin = srgb_to_linear(g_norm);
    let b_lin = srgb_to_linear(b_norm);
    
    0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin
}
```

### RGB to YCbCr (Video)

```rust
/// Convert RGB to YCbCr (ITU-R BT.709)
fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;
    
    // Luminance (Y)
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    
    // Chroma blue (Cb)
    let cb = (b - y) / (2.0 * (1.0 - 0.0722)) + 128.0;
    
    // Chroma red (Cr)
    let cr = (r - y) / (2.0 * (1.0 - 0.2126)) + 128.0;
    
    (
        y.clamp(0.0, 255.0) as u8,
        cb.clamp(0.0, 255.0) as u8,
        cr.clamp(0.0, 255.0) as u8,
    )
}
```

---

## Common Misconceptions

### Myth 1: "Green coefficient should match cone density"
❌ **Wrong:** More M-cones (32%) → 32% coefficient

✅ **Correct:** Coefficient reflects **sensitivity**, not quantity
- Peak sensitivity at 555nm (between M and L cones)
- Neural processing combines L + M for luminance
- Result: 71% green contribution

### Myth 2: "All luminance standards are the same"
❌ **Wrong:** Always use 0.299, 0.587, 0.114

✅ **Correct:** Different standards for different applications
- BT.601: Legacy TV
- BT.709: Modern HDTV, Web (most common)
- BT.2020: Ultra HD, HDR

### Myth 3: "Luminance = Brightness"
❌ **Wrong:** They're the same thing

✅ **Correct:** 
- **Luminance**: Objective, measurable, based on photometry
- **Brightness**: Subjective perception, varies by context
- **Lightness**: Perceptual scale (L*)

### Myth 4: "Simple average is good enough"
❌ **Wrong:** `(R + G + B) / 3` works fine

✅ **Correct:** Produces perceptually incorrect results
- Pure yellow (255, 255, 0) should be bright, average says 170
- Proper luminance: 0.2126×255 + 0.7152×255 = 237
- Difference is visible to viewers

### Myth 5: "Gamma correction doesn't matter"
❌ **Wrong:** Can ignore gamma when calculating luminance

✅ **Correct:** Technically should use linear RGB
- Most images are gamma-corrected (sRGB)
- For accuracy, convert to linear first
- In practice, many apps use gamma-corrected directly (approximation)

---

## References

### Official Standards

1. **ITU-R Recommendation BT.709-6** (2015)
   - Title: "Parameter values for the HDTV standards"
   - URL: https://www.itu.int/rec/R-REC-BT.709/
   - PDF: https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.709-6-201506-I!!PDF-E.pdf

2. **IEC 61966-2-1:1999** (sRGB)
   - Title: "Multimedia systems and equipment - Colour management - Part 2-1: sRGB"
   - URL: https://webstore.iec.ch/publication/6169

3. **CIE 015:2004** - Colorimetry, 3rd Edition
   - Publisher: Commission Internationale de l'Éclairage
   - URL: https://cie.co.at/publications/colorimetry-3rd-edition

### Academic Papers

4. **Stockman, A., & Sharpe, L. T.** (2000)
   - "Spectral sensitivities of the middle- and long-wavelength sensitive cones"
   - *Journal of the Optical Society of America A*, 17(4), 571-582
   - DOI: 10.1364/JOSAA.17.000571

5. **Fairchild, M. D.** (2013)
   - *Color Appearance Models*, 3rd Edition
   - Wiley-IS&T Series in Imaging Science and Technology
   - ISBN: 978-1119967033

6. **Hunt, R. W. G., & Pointer, M. R.** (2011)
   - *Measuring Colour*, 4th Edition
   - Wiley
   - ISBN: 978-0470974216

### Books

7. **Poynton, Charles** (2012)
   - *Digital Video and HD: Algorithms and Interfaces*, 2nd Edition
   - Morgan Kaufmann
   - ISBN: 978-0123919267
   - **Best comprehensive reference**

8. **Wandell, Brian A.** (1995)
   - *Foundations of Vision*
   - Sinauer Associates
   - ISBN: 978-0878938537
   - Free online: https://foundationsofvision.stanford.edu/

9. **Reinhard, Erik, et al.** (2010)
   - *High Dynamic Range Imaging: Acquisition, Display, and Image-Based Lighting*, 2nd Edition
   - Morgan Kaufmann
   - ISBN: 978-0123749147

### Online Resources

10. **Poynton's Color FAQ**
    - URL: http://poynton.ca/notes/colour_and_gamma/ColorFAQ.html
    - Excellent technical Q&A on color science

11. **Bruce Lindbloom's Website**
    - URL: http://brucelindbloom.com/
    - Color space conversions, calculators

12. **Cambridge in Colour**
    - URL: https://www.cambridgeincolour.com/tutorials/color-spaces.htm
    - Accessible tutorials on color theory

13. **Scratchapixel 2.0**
    - URL: https://www.scratchapixel.com/lessons/digital-imaging/colors
    - Computer graphics perspective

### Implementation References

14. **OpenCV Color Conversions**
    - URL: https://docs.opencv.org/4.x/de/d25/imgproc_color_conversions.html
    - Shows BT.709 in practice

15. **FFmpeg Source Code**
    - File: `libavutil/colorspace.h`
    - URL: https://github.com/FFmpeg/FFmpeg/blob/master/libavutil/colorspace.h
    - Industry-standard video implementation

16. **darktable Source Code**
    - File: `src/common/colorspaces_inline_conversions.h`
    - URL: https://github.com/darktable-org/darktable
    - Professional photo editor implementation

---

## Summary

### Key Takeaways

1. **Luminance coefficients are biological**
   - Based on human cone cells and V(λ) function
   - Not arbitrary math

2. **ITU-R BT.709 is the modern standard**
   - 0.2126, 0.7152, 0.0722
   - Used for HDTV, web, most digital imaging

3. **Green dominates (71%)**
   - Peak eye sensitivity at 555nm
   - L + M cone combination
   - Evolutionary optimization

4. **Different standards for different applications**
   - BT.601: Legacy TV
   - BT.709: Modern (most common)
   - BT.2020: UHD/HDR

5. **Gamma correction matters**
   - Technically should use linear RGB
   - Many apps approximate with gamma-corrected

### When to Use Luminance

✅ **Use luminance weighting for:**
- Grayscale conversion
- Edge detection
- Contrast enhancement
- Video compression
- Color grading effects
- Perceptual image metrics

❌ **Don't use for:**
- Hue manipulation (use HSL/HSV)
- Saturation adjustment
- Color matching
- Chromatic operations

---

*Last updated: March 19, 2026*
*Used in: `/src/utils/grain.rs` for luminance-weighted grain application*
