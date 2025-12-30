# FastImage Mathematical Documentation

This document explains all mathematical operations in the `FastImage` struct with detailed examples, formulas, and corresponding code.

---

## Table of Contents

### Helper Functions
1. [load_and_resize_image() - Image Scaling](#1-load_and_resize_image---image-scaling)
2. [read_route_from_fit() - GPS Coordinate Normalization](#2-read_route_from_fit---gps-coordinate-normalization)

### FastImage Struct Methods
3. [from_mat() - Matrix to Buffer Conversion](#3-from_mat---matrix-to-buffer-conversion)
4. [to_mat() - Buffer to Matrix Conversion](#4-to_mat---buffer-to-matrix-conversion)
5. [put_pixel_bgr() - Pixel Indexing & Alpha Blending](#5-put_pixel_bgr---pixel-indexing--alpha-blending)
6. [draw_circle() - Circle Drawing](#6-draw_circle---circle-drawing)
7. [draw_line_aa() - Thick Line with Perpendicular Vectors](#7-draw_line_aa---thick-line-with-perpendicular-vectors)
8. [draw_line_aa_single() - Wu's Algorithm](#8-draw_line_aa_single---wus-algorithm)

---

# Part 1: Helper Functions

---

## 1. load_and_resize_image() - Image Scaling

### Purpose
Load an image and resize it to fit within a maximum dimension while maintaining aspect ratio.

### Code
```rust
let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
let size = img.size()?;
let (orig_w, orig_h) = (size.width as f64, size.height as f64);

let max_side = orig_w.max(orig_h);
let scale = (max_dim as f64 / max_side).min(1.0);

let width = (orig_w * scale) as i32;
let height = (orig_h * scale) as i32;

imgproc::resize(
  &img,
  &mut resized,
  core::Size::new(width, height),
  0.0,
  0.0,
  imgproc::INTER_LANCZOS4,
)?;
```

### Formula: Aspect Ratio Preserving Scale

**Plain text:**
```
max_side = max(width, height)
scale = min(max_dim / max_side, 1.0)
new_width = original_width × scale
new_height = original_height × scale
```

**LaTeX:**

$$\text{max\_side} = \max(\text{width}, \text{height})$$

$$\text{scale} = \min\left(\frac{\text{max\_dim}}{\text{max\_side}}, 1.0\right)$$

$$\text{new\_width} = \text{original\_width} \times \text{scale}$$

$$\text{new\_height} = \text{original\_height} \times \text{scale}$$

The `min(scale, 1.0)` ensures we never upscale images, only downscale.

### Example 1: Landscape Image
```
Original image: 3840×2160 pixels (4K)
Target max_dim: 1080

Step 1: Find longest side
max_side = max(3840, 2160) = 3840

Step 2: Calculate scale factor
scale = 1080 / 3840 = 0.28125

Step 3: Apply scale to both dimensions
new_width = 3840 × 0.28125 = 1080
new_height = 2160 × 0.28125 = 607.5 → 607

Result: 1080×607 (fits within 1080, maintains 16:9 ratio)
```

### Example 2: Portrait Image
```
Original image: 1200×1600 pixels
Target max_dim: 800

Step 1: Find longest side
max_side = max(1200, 1600) = 1600

Step 2: Calculate scale factor
scale = 800 / 1600 = 0.5

Step 3: Apply scale to both dimensions
new_width = 1200 × 0.5 = 600
new_height = 1600 × 0.5 = 800

Result: 600×800 (fits within 800, maintains 3:4 ratio)
```

### Example 3: Small Image (No Upscaling)
```
Original image: 640×480 pixels
Target max_dim: 1080

Step 1: Find longest side
max_side = max(640, 480) = 640

Step 2: Calculate scale factor
scale = 1080 / 640 = 1.6875
BUT: min(1.6875, 1.0) = 1.0  ← Prevents upscaling

Step 3: Apply scale to both dimensions
new_width = 640 × 1.0 = 640
new_height = 480 × 1.0 = 480

Result: 640×480 (unchanged, no upscaling)
```

### Example 4: Square Image
```
Original image: 2048×2048 pixels (square)
Target max_dim: 1080

Step 1: Find longest side
max_side = max(2048, 2048) = 2048

Step 2: Calculate scale factor
scale = 1080 / 2048 = 0.52734375

Step 3: Apply scale to both dimensions
new_width = 2048 × 0.52734375 = 1080
new_height = 2048 × 0.52734375 = 1080

Result: 1080×1080 (square remains square)
```

### Why This Formula Works
```
Aspect ratio = width / height

Original: 3840 / 2160 = 1.778 (16:9)
Scaled:   1080 / 607  = 1.778 (16:9) ✓

The ratio is preserved because we multiply both dimensions
by the same scale factor.
```

### Visual Example
```
Original (3840×2160):
┌─────────────────────────────────────┐
│                                     │
│         16:9 Landscape              │
│                                     │
└─────────────────────────────────────┘

Scaled (1080×607):
┌──────────────────┐
│  16:9 Landscape  │
└──────────────────┘

Same aspect ratio, smaller size!
```

---

## 2. read_route_from_fit() - GPS Coordinate Normalization

### Purpose
Convert GPS coordinates from a FIT file to pixel coordinates on an image, with scaling and positioning control.

### Code: Part 1 - Get GPS Bounds
```rust
let (route, _lap) = fit_reader(fit_path)?;
let points = route.gps_points;

let ((lat_min, lat_max), (lon_min, lon_max)) = get_bounds(&points);
```

### Example: GPS Bounds
```
Sample GPS route (5 points):
Point 1: (37.7749, -122.4194)  San Francisco
Point 2: (37.7750, -122.4180)
Point 3: (37.7760, -122.4170)
Point 4: (37.7770, -122.4165)
Point 5: (37.7780, -122.4160)

Bounds:
lat_min = 37.7749
lat_max = 37.7780
lon_min = -122.4194
lon_max = -122.4160

Latitude range: 37.7780 - 37.7749 = 0.0031 degrees
Longitude range: -122.4160 - (-122.4194) = 0.0034 degrees
```

### Code: Part 2 - Normalize Coordinates
```rust
let nx = if lon_max != lon_min {
  (lon - lon_min) / (lon_max - lon_min)
} else {
  0.5
};
let ny = if lat_max != lat_min {
  (lat - lat_min) / (lat_max - lat_min)
} else {
  0.5
};
```

### Formula: Min-Max Normalization

**Plain text:**
```
normalized = (value - min) / (max - min)
```

**LaTeX:**

$$\text{normalized} = \frac{\text{value} - \text{min}}{\text{max} - \text{min}}$$

This maps any range $[\text{min}, \text{max}]$ to $[0, 1]$.

### Example: Normalization
```
Using the GPS bounds from above:
lon_min = -122.4194, lon_max = -122.4160

Point 1: lon = -122.4194
nx = (-122.4194 - (-122.4194)) / (-122.4160 - (-122.4194))
nx = 0 / 0.0034
nx = 0.0 ← Leftmost point

Point 3: lon = -122.4170
nx = (-122.4170 - (-122.4194)) / 0.0034
nx = 0.0024 / 0.0034
nx ≈ 0.706 ← 70.6% across

Point 5: lon = -122.4160
nx = (-122.4160 - (-122.4194)) / 0.0034
nx = 0.0034 / 0.0034
nx = 1.0 ← Rightmost point

Similarly for latitude:
lat_min = 37.7749, lat_max = 37.7780

Point 1: lat = 37.7749
ny = (37.7749 - 37.7749) / (37.7780 - 37.7749)
ny = 0 / 0.0031
ny = 0.0 ← Southmost point

Point 5: lat = 37.7780
ny = (37.7780 - 37.7749) / 0.0031
ny = 0.0031 / 0.0031
ny = 1.0 ← Northmost point
```

### Code: Part 3 - Map to Pixel Coordinates
```rust
let x = ((offset_x_percent + nx * route_scale) * width as f64) as f32;
let y = ((offset_y_percent + (1.0 - ny) * route_scale) * width as f64) as f32;
```

### Formula: Pixel Coordinate Mapping

**Plain text:**
```
x_pixel = (offset_x + normalized_x × scale) × image_width
y_pixel = (offset_y + (1 - normalized_y) × scale) × image_width
```

**LaTeX:**

$$x_{\text{pixel}} = (\text{offset}_x + n_x \times \text{scale}) \times \text{image\_width}$$

$$y_{\text{pixel}} = (\text{offset}_y + (1 - n_y) \times \text{scale}) \times \text{image\_width}$$

Note: `1 - ny` flips the y-axis because:
- GPS: North is positive (higher latitude = north)
- Images: Y=0 is at top, Y increases downward

### Example: Complete Transformation
```
Image size: 1080×1080 pixels
Parameters:
- route_scale = 0.2 (route takes 20% of image width)
- offset_x_percent = 0.1 (start 10% from left edge)
- offset_y_percent = 0.1 (start 10% from top edge)

Point 1 (normalized: nx=0.0, ny=0.0):
x = (0.1 + 0.0 × 0.2) × 1080 = 0.1 × 1080 = 108 pixels
y = (0.1 + (1.0 - 0.0) × 0.2) × 1080 = (0.1 + 0.2) × 1080 = 324 pixels
→ (108, 324)

Point 3 (normalized: nx=0.706, ny=0.355):
x = (0.1 + 0.706 × 0.2) × 1080 = (0.1 + 0.1412) × 1080 = 260 pixels
y = (0.1 + (1.0 - 0.355) × 0.2) × 1080 = (0.1 + 0.129) × 1080 = 247 pixels
→ (260, 247)

Point 5 (normalized: nx=1.0, ny=1.0):
x = (0.1 + 1.0 × 0.2) × 1080 = 0.3 × 1080 = 324 pixels
y = (0.1 + (1.0 - 1.0) × 0.2) × 1080 = 0.1 × 1080 = 108 pixels
→ (324, 108)
```

### Visual Example: Route Placement
```
Image (1080×1080), offset=0.1, scale=0.2

┌────────────────────────────────────┐
│ 10%                                │
│ ┌──────┐                           │
│ │ 20%  │  Route fits in this box   │
│ │  ×   │  (216×216 pixels)         │
│ │ of   │                           │
│ │image │  Route scaled to 20%      │
│ └──────┘  Offset by 10%            │
│                                    │
│                                    │
│                                    │
└────────────────────────────────────┘

Calculation:
Route box starts at: (0.1 × 1080, 0.1 × 1080) = (108, 108)
Route box size: 0.2 × 1080 = 216 pixels
Route box ends at: (108 + 216, 108 + 216) = (324, 324)
```

### Example: Different Scale Values
```
Same route, same image (1080×1080), offset=0.1

route_scale = 0.1 (10%):
Box size = 108 pixels
Route appears small

route_scale = 0.5 (50%):
Box size = 540 pixels
Route appears large

route_scale = 0.8 (80%):
Box size = 864 pixels
Route fills most of the area
```

### Why Y-Axis is Flipped
```
GPS Coordinates:          Image Coordinates:
North ↑                   (0,0) ────→ X
      │                   │
      │                   │
      │                   ↓ Y
      └────→ East

GPS: Higher latitude = North (up)
Image: Higher Y = Down

Solution: y = 1.0 - normalized_y

Example:
GPS North (ny=1.0) → y = 1.0 - 1.0 = 0.0 → Top of image ✓
GPS South (ny=0.0) → y = 1.0 - 0.0 = 1.0 → Bottom of image ✓
```

### Edge Case: Single Point
```
If route has only one GPS point:
lon_min = lon_max = -122.4194
lat_min = lat_max = 37.7749

Division by zero protection:
if lon_max != lon_min {
  nx = (lon - lon_min) / (lon_max - lon_min)  // Would be 0/0
} else {
  nx = 0.5  // Center the single point
}

Result: Single point placed at center of route box
```

---

# Part 2: FastImage Struct Methods

---

## 3. from_mat() - Matrix to Buffer Conversion

### Purpose
Convert OpenCV Mat (2D matrix) to flat 1D array for direct pixel manipulation.

### Code
```rust
let w = mat.cols() as usize;  // Image width in pixels
let h = mat.rows() as usize;  // Image height in pixels
let slice = mat.data_bytes().unwrap();
let data = slice.to_vec();  // Copy to owned Vec for mutation
```

### Mathematics
**Total bytes calculation:**

$$\text{Total bytes} = \text{width} \times \text{height} \times 3$$
- Each pixel has 3 bytes (Blue, Green, Red)
- Stored row-by-row from top to bottom

### Example
```
Image: 800×600 pixels
Total bytes = 800 × 600 × 3 = 1,440,000 bytes

Layout in memory:
[B₀₀, G₀₀, R₀₀, B₁₀, G₁₀, R₁₀, ..., B₇₉₉,₅₉₉, G₇₉₉,₅₉₉, R₇₉₉,₅₉₉]
 Row 0, Pixel 0    Row 0, Pixel 1    ...    Row 599, Pixel 799
```

---

## 2. to_mat() - Buffer to Matrix Conversion

### Purpose
Convert flat 1D array back to OpenCV Mat for video/image operations.

### Code
```rust
let mut mat = unsafe {
  Mat::new_rows_cols(
    self.h as i32,  // Height in pixels
    self.w as i32,  // Width in pixels
    core::CV_8UC3,  // 8-bit BGR format
  )
  .unwrap()
};
mat.data_bytes_mut().unwrap().copy_from_slice(&self.data);
```

### Mathematics
**Reconstruction:**
- Flat array → 2D matrix with dimensions (height, width)
- CV_8UC3 = 8-bit unsigned, 3 channels (BGR)

### Example
```
Buffer: [0, 255, 0, 128, 64, 200, ...]
        ↓
Mat:    Row 0: [(0,255,0), (128,64,200), ...]
        Row 1: [(...), (...), ...]
```

---

## 5. put_pixel_bgr() - Pixel Indexing & Alpha Blending

### Purpose
Set a pixel color with alpha blending for anti-aliasing.

### Code: Part 1 - Pixel Index Calculation
```rust
let idx = (y * self.w + x) * 3;
```

### Formula: 2D to 1D Index Conversion

**Plain text:**
```
idx = (y × width + x) × 3
```

**LaTeX:**

$$\text{idx} = (y \times \text{width} + x) \times 3$$

**Step-by-step breakdown:**
1. `y × width` = Skip to the correct row
2. `+ x` = Move to the correct column
3. `× 3` = Each pixel has 3 bytes (BGR)

### Example 1: Small Image
```
Image: 4×3 pixels (4 wide, 3 tall)
Find index for pixel at (2, 1)

Coordinate grid:
  x=0  x=1  x=2  x=3
y=0 [0]  [1]  [2]  [3]
y=1 [4]  [5]  [6]  [7]
y=2 [8]  [9]  [10] [11]

Calculation:
idx = (1 × 4 + 2) × 3
idx = (4 + 2) × 3
idx = 6 × 3
idx = 18

Flat array indices:
[B₀, G₀, R₀, B₁, G₁, R₁, ..., B₆, G₆, R₆, ...]
 0   1   2   3   4   5       18  19  20
                              ↑
                        Pixel (2,1) starts here
```

### Example 2: Large Image
```
Image: 1920×1080 pixels
Find index for pixel at (500, 300)

idx = (300 × 1920 + 500) × 3
idx = (576,000 + 500) × 3
idx = 576,500 × 3
idx = 1,729,500

Accessing the pixel:
data[1,729,500] = Blue channel
data[1,729,501] = Green channel
data[1,729,502] = Red channel
```

### Code: Part 2 - Alpha Blending
```rust
let ob = self.data[idx] as f32;      // Original blue
let og = self.data[idx + 1] as f32;  // Original green
let or = self.data[idx + 2] as f32;  // Original red

self.data[idx]     = (ob + (b as f32 - ob) * ai) as u8;
self.data[idx + 1] = (og + (g as f32 - og) * ai) as u8;
self.data[idx + 2] = (or + (r as f32 - or) * ai) as u8;
```

### Formula: Alpha Blending

**Plain text:**
```
new_color = old_color + (new_color - old_color) × alpha
```

Alternative form:
```
new_color = old_color × (1 - alpha) + new_color × alpha
```

**LaTeX:**

$$C_{\text{new}} = C_{\text{old}} + (C_{\text{new}} - C_{\text{old}}) \times \alpha$$

Alternative form:

$$C_{\text{new}} = C_{\text{old}} \times (1 - \alpha) + C_{\text{new}} \times \alpha$$

### Example: Alpha Blending
```
Scenario: Draw semi-transparent red over blue background

Old pixel: BGR = (255, 0, 0)  [Pure Blue]
New color: BGR = (0, 0, 255)  [Pure Red]
Alpha: 0.5 (50% transparency)

Blue channel:
new_blue = 255 + (0 - 255) × 0.5
new_blue = 255 + (-255) × 0.5
new_blue = 255 - 127.5
new_blue = 127.5 → 128

Red channel:
new_red = 0 + (255 - 0) × 0.5
new_red = 0 + 255 × 0.5
new_red = 127.5 → 128

Result: BGR = (128, 0, 128) [Purple - 50/50 mix]
```

### Example: Full Opacity vs Transparency
```
Old pixel: BGR = (100, 150, 200)
New color: BGR = (50, 75, 255)

Case 1: Alpha = 1.0 (fully opaque)
Blue:  100 + (50 - 100) × 1.0 = 100 - 50 = 50
Green: 150 + (75 - 150) × 1.0 = 150 - 75 = 75
Red:   200 + (255 - 200) × 1.0 = 200 + 55 = 255
Result: (50, 75, 255) ← Exactly the new color

Case 2: Alpha = 0.0 (fully transparent)
Blue:  100 + (50 - 100) × 0.0 = 100 + 0 = 100
Green: 150 + (75 - 150) × 0.0 = 150 + 0 = 150
Red:   200 + (255 - 200) × 0.0 = 200 + 0 = 200
Result: (100, 150, 200) ← Keeps old color

Case 3: Alpha = 0.3 (30% opacity)
Blue:  100 + (50 - 100) × 0.3 = 100 - 15 = 85
Green: 150 + (75 - 150) × 0.3 = 150 - 22.5 = 127.5 → 128
Red:   200 + (255 - 200) × 0.3 = 200 + 16.5 = 216.5 → 217
Result: (85, 128, 217) ← Blend of both
```

---

## 6. draw_circle() - Circle Drawing

### Purpose
Draw a filled circle using distance-based pixel testing.

### Code
```rust
let r2 = radius * radius;  // Square of radius

for dy in -radius..=radius {
  for dx in -radius..=radius {
    if dx * dx + dy * dy <= r2 {
      self.put_pixel_bgr(cx + dx, cy + dy, b, g, r, 1.0);
    }
  }
}
```

### Formula: Circle Equation

**Plain text:**
```
x² + y² ≤ r²
```

A point (x, y) is inside a circle if:
```
distance² ≤ radius²
```

We use squared distance to avoid expensive sqrt() operations:
```
√(dx² + dy²) ≤ r    ←  Slow (requires sqrt)
dx² + dy² ≤ r²      ←  Fast (only multiplication)
```

**LaTeX:**

$$x^2 + y^2 \leq r^2$$

A point $(x, y)$ is inside a circle if:

$$\text{distance}^2 \leq \text{radius}^2$$

We use squared distance to avoid expensive sqrt() operations:

$$\sqrt{dx^2 + dy^2} \leq r \quad \text{(Slow - requires sqrt)}$$

$$dx^2 + dy^2 \leq r^2 \quad \text{(Fast - only multiplication)}$$

### Example 1: Small Circle
```
Circle: center = (100, 100), radius = 3

r² = 3² = 9

Testing points around center:
Point (100, 100): dx=0, dy=0 → 0² + 0² = 0 ≤ 9 ✓ (inside)
Point (101, 100): dx=1, dy=0 → 1² + 0² = 1 ≤ 9 ✓ (inside)
Point (102, 100): dx=2, dy=0 → 2² + 0² = 4 ≤ 9 ✓ (inside)
Point (103, 100): dx=3, dy=0 → 3² + 0² = 9 ≤ 9 ✓ (on edge)
Point (104, 100): dx=4, dy=0 → 4² + 0² = 16 > 9 ✗ (outside)

Point (102, 102): dx=2, dy=2 → 2² + 2² = 8 ≤ 9 ✓ (inside)
Point (103, 103): dx=3, dy=3 → 3² + 3² = 18 > 9 ✗ (outside)
```

### Example 2: Visual Representation
```
Circle with radius = 5, center = (0, 0)
r² = 25

    -5 -4 -3 -2 -1  0  1  2  3  4  5
-5   .  .  .  .  ○  ○  ○  .  .  .  .
-4   .  .  ○  ○  ●  ●  ●  ○  ○  .  .
-3   .  ○  ●  ●  ●  ●  ●  ●  ●  ○  .
-2   .  ○  ●  ●  ●  ●  ●  ●  ●  ○  .
-1   ○  ●  ●  ●  ●  ●  ●  ●  ●  ●  ○
 0   ○  ●  ●  ●  ●  ●  ●  ●  ●  ●  ○
 1   ○  ●  ●  ●  ●  ●  ●  ●  ●  ●  ○
 2   .  ○  ●  ●  ●  ●  ●  ●  ●  ○  .
 3   .  ○  ●  ●  ●  ●  ●  ●  ●  ○  .
 4   .  .  ○  ○  ●  ●  ●  ○  ○  .  .
 5   .  .  .  .  ○  ○  ○  .  .  .  .

● = inside (dx² + dy² < 25)
○ = on edge (dx² + dy² ≤ 25)
. = outside (dx² + dy² > 25)

Example calculations:
Point (0, 5):  0² + 5² = 25 ≤ 25 ✓
Point (3, 4):  3² + 4² = 9 + 16 = 25 ≤ 25 ✓
Point (4, 4):  4² + 4² = 16 + 16 = 32 > 25 ✗
Point (-3, -3): (-3)² + (-3)² = 9 + 9 = 18 ≤ 25 ✓
```

---

## 7. draw_line_aa() - Thick Line with Perpendicular Vectors

### Purpose
Draw thick anti-aliased lines by drawing multiple parallel thin lines.

### Code: Part 1 - Direction Vector
```rust
let dx = x1 - x0;  // X component of line direction
let dy = y1 - y0;  // Y component of line direction
let len = (dx * dx + dy * dy).sqrt();  // Line length
```

### Formula: Vector Length (Pythagorean Theorem)

**Plain text:**
```
length = √(dx² + dy²)
```

**LaTeX:**

$$\text{length} = \sqrt{dx^2 + dy^2}$$

### Example: Vector Length
```
Line from (10, 20) to (40, 80)

dx = 40 - 10 = 30
dy = 80 - 20 = 60

length = √(30² + 60²)
length = √(900 + 3600)
length = √4500
length ≈ 67.08 pixels
```

### Code: Part 2 - Perpendicular Unit Vector
```rust
let px = -dy / len;
let py = dx / len;
```

### Formula: 90° Rotation & Normalization

**Plain text:**
```
Rotate (dx, dy) by 90° counterclockwise:
(dx, dy) → (-dy, dx)

Normalize to unit vector (length = 1):
perpendicular = (-dy/length, dx/length)
```

**LaTeX:**

Rotate $(dx, dy)$ by 90° counterclockwise:

$$(dx, dy) \rightarrow (-dy, dx)$$

Normalize to unit vector (length = 1):

$$\vec{p} = \left(\frac{-dy}{\text{length}}, \frac{dx}{\text{length}}\right)$$

### Example: Perpendicular Vector
```
Original line direction: (30, 60), length = 67.08

Perpendicular vector:
px = -60 / 67.08 ≈ -0.894
py = 30 / 67.08 ≈ 0.447

Verification (unit vector check):
√(px² + py²) = √(0.894² + 0.447²) = √(0.799 + 0.200) ≈ 1.0 ✓
```

### Code: Part 3 - Parallel Lines
```rust
let half_thickness = thickness as f32 / 2.0;
for i in 0..thickness {
  let offset = i as f32 - half_thickness + 0.5;
  let ox = offset * px;  // X offset
  let oy = offset * py;  // Y offset
  
  self.draw_line_aa_single(x0 + ox, y0 + oy, x1 + ox, y1 + oy, b, g, r);
}
```

### Formula: Parallel Line Offset

**Plain text:**
```
offset = i - (thickness/2) + 0.5
new_position = original_position + offset × perpendicular_vector
```

**LaTeX:**

$$\text{offset} = i - \frac{\text{thickness}}{2} + 0.5$$

$$\vec{p}_{\text{new}} = \vec{p}_{\text{original}} + \text{offset} \times \vec{p}_{\perp}$$

### Example: Thickness = 7
```
Original line: (10, 20) → (40, 80)
Perpendicular: px = -0.894, py = 0.447
Thickness: 7 (draw 7 parallel lines)

half_thickness = 7/2 = 3.5

Offsets for each line:
i=0: offset = 0 - 3.5 + 0.5 = -3.0
i=1: offset = 1 - 3.5 + 0.5 = -2.0
i=2: offset = 2 - 3.5 + 0.5 = -1.0
i=3: offset = 3 - 3.5 + 0.5 = 0.0  ← Center line
i=4: offset = 4 - 3.5 + 0.5 = 1.0
i=5: offset = 5 - 3.5 + 0.5 = 2.0
i=6: offset = 6 - 3.5 + 0.5 = 3.0

For i=0 (offset = -3.0):
ox = -3.0 × -0.894 = 2.682
oy = -3.0 × 0.447 = -1.341
New start: (10 + 2.682, 20 - 1.341) = (12.682, 18.659)
New end:   (40 + 2.682, 80 - 1.341) = (42.682, 78.659)

For i=6 (offset = 3.0):
ox = 3.0 × -0.894 = -2.682
oy = 3.0 × 0.447 = 1.341
New start: (10 - 2.682, 20 + 1.341) = (7.318, 21.341)
New end:   (40 - 2.682, 80 + 1.341) = (37.318, 81.341)
```

### Visual Example
```
Original line: ─────────
Thickness = 5, draws 5 parallel lines:

Line 0 (offset -2): ─────────
Line 1 (offset -1):  ─────────
Line 2 (offset  0):   ─────────  ← Center (original)
Line 3 (offset +1):    ─────────
Line 4 (offset +2):     ─────────

Result:    ▓▓▓▓▓▓▓▓▓
           (thick line)
```

---

## 8. draw_line_aa_single() - Wu's Algorithm

### Purpose
Draw a single anti-aliased line using Xiaolin Wu's algorithm.

### Code: Part 1 - Steep Line Detection
```rust
let steep = (y1 - y0).abs() > (x1 - x0).abs();
```

### Formula: Steepness Test

**Plain text:**
```
steep = |Δy| > |Δx|
```

**LaTeX:**

$$\text{steep} = |\Delta y| > |\Delta x|$$

### Example: Steep Detection
```
Line 1: (10, 20) → (15, 80)
dx = 5, dy = 60
|60| > |5| → steep = true (vertical line)

Line 2: (10, 20) → (80, 25)
dx = 70, dy = 5
|5| > |70| → steep = false (horizontal line)
```

### Code: Part 2 - Gradient (Slope)
```rust
let gradient = if dx == 0.0 { 1.0 } else { dy / dx };
```

### Formula: Slope

**Plain text:**
```
gradient = Δy / Δx = (y₁ - y₀) / (x₁ - x₀)
```

**LaTeX:**

$$\text{gradient} = \frac{\Delta y}{\Delta x} = \frac{y_1 - y_0}{x_1 - x_0}$$

### Example: Gradient Calculation
```
Line from (10, 20) to (50, 100)
dx = 40, dy = 80
gradient = 80 / 40 = 2.0

Meaning: For every 1 pixel moved in X, move 2 pixels in Y.

At x=10: y = 20
At x=11: y = 22 (moved 2 pixels up)
At x=12: y = 24 (moved 2 pixels up)
At x=15: y = 30 (moved 10 pixels up)
```

### Code: Part 3 - Integer & Fractional Parts
```rust
let ip = |x: f32| x.floor();           // Integer part
let fp = |x: f32| x - x.floor();       // Fractional part
```

### Formula: Number Decomposition

**Plain text:**
```
x = floor(x) + frac(x)
x = integer_part + fractional_part
```

**LaTeX:**

$$x = \lfloor x \rfloor + \text{frac}(x)$$

$$x = \text{integer\_part} + \text{fractional\_part}$$

### Example: Integer/Fractional Parts
```
x = 12.3 → ip(12.3) = 12, fp(12.3) = 0.3
x = 7.89 → ip(7.89) = 7,  fp(7.89) = 0.89
x = 5.0  → ip(5.0) = 5,   fp(5.0) = 0.0
x = -3.6 → ip(-3.6) = -4, fp(-3.6) = 0.4  (floor goes down!)
```

### Code: Part 4 - First Endpoint
```rust
let xend = ip(x0 + 0.5);
let yend = y0 + gradient * (xend - x0);
let xgap = 1.0 - fp(x0 + 0.5);
```

### Example: Endpoint Calculation
```
Line from (10.3, 20.7) to (50.2, 80.4)
gradient = (80.4 - 20.7) / (50.2 - 10.3) = 59.7 / 39.9 ≈ 1.496

First endpoint:
xend = floor(10.3 + 0.5) = floor(10.8) = 10
yend = 20.7 + 1.496 × (10 - 10.3) = 20.7 + 1.496 × (-0.3) = 20.7 - 0.449 ≈ 20.251
xgap = 1.0 - frac(10.3 + 0.5) = 1.0 - frac(10.8) = 1.0 - 0.8 = 0.2

ypxl1 = floor(20.251) = 20
```

### Code: Part 5 - Drawing Endpoint with Alpha
```rust
draw(self, steep, xpxl1, ypxl1, (1.0 - fp(yend)) * xgap);
draw(self, steep, xpxl1, ypxl1 + 1.0, fp(yend) * xgap);
```

### Formula: Endpoint Alpha Blending

**Plain text:**
```
lower_pixel_alpha = (1 - frac(y)) × xgap
upper_pixel_alpha = frac(y) × xgap
```

**LaTeX:**

$$\alpha_{\text{lower}} = (1 - \text{frac}(y)) \times x_{\text{gap}}$$

$$\alpha_{\text{upper}} = \text{frac}(y) \times x_{\text{gap}}$$

### Example: Endpoint Alpha
```
yend = 20.251
ypxl1 = 20
xgap = 0.2

Pixel at y=20:
alpha = (1.0 - 0.251) × 0.2 = 0.749 × 0.2 = 0.1498 ≈ 15% opacity

Pixel at y=21:
alpha = 0.251 × 0.2 = 0.0502 ≈ 5% opacity

Total coverage = 0.1498 + 0.0502 = 0.2 = xgap ✓
```

### Code: Part 6 - Main Loop
```rust
for x in ((xpxl1 + 1.0) as i32)..(xpxl2 as i32) {
  draw(self, steep, x as f32, ip(intery), 1.0 - fp(intery));
  draw(self, steep, x as f32, ip(intery) + 1.0, fp(intery));
  intery += gradient;
}
```

### Example: Main Loop Iteration
```
gradient = 1.496
Start: x=11, intery=21.747

Iteration 1: x=11
  y = 21.747
  ypxl = floor(21.747) = 21
  Pixel (11, 21): alpha = 1.0 - 0.747 = 0.253 (25%)
  Pixel (11, 22): alpha = 0.747 (75%)
  intery += 1.496 → intery = 23.243

Iteration 2: x=12
  y = 23.243
  ypxl = floor(23.243) = 23
  Pixel (12, 23): alpha = 1.0 - 0.243 = 0.757 (76%)
  Pixel (12, 24): alpha = 0.243 (24%)
  intery += 1.496 → intery = 24.739

Iteration 3: x=13
  y = 24.739
  ypxl = floor(24.739) = 24
  Pixel (13, 24): alpha = 1.0 - 0.739 = 0.261 (26%)
  Pixel (13, 25): alpha = 0.739 (74%)
  intery += 1.496 → intery = 26.235
```

### Visual Representation of Wu's Algorithm
```
Line from (0, 0) to (10, 6), gradient = 0.6

Without anti-aliasing (jagged):
6 │     ■
5 │    ■
4 │   ■
3 │  ■ ■
2 │  ■
1 │ ■
0 │■
  └─────────────
  0 1 2 3 4 5 6 7 8 9 10

With Wu's algorithm (smooth):
6 │         ▓░
5 │       ▓▓░
4 │     ▓▓░
3 │   ▓▓░
2 │  ▓░
1 │ ▓░
0 │▓
  └─────────────
  0 1 2 3 4 5 6 7 8 9 10

▓ = high alpha (darker)
░ = low alpha (lighter)

Detailed alpha values at x=5:
y = 0 + 0.6 × 5 = 3.0
Pixel (5, 3): alpha = 1.0 - 0.0 = 1.0 (100% - fully opaque)
Pixel (5, 4): alpha = 0.0 (0% - fully transparent)

At x=6:
y = 0 + 0.6 × 6 = 3.6
Pixel (6, 3): alpha = 1.0 - 0.6 = 0.4 (40%)
Pixel (6, 4): alpha = 0.6 (60%)
```

---

## Summary of Key Formulas

### 1. Pixel Indexing
**Plain text:** `idx = (y × width + x) × 3`

**LaTeX:** $$\text{idx} = (y \times \text{width} + x) \times 3$$

### 2. Alpha Blending
**Plain text:** `new_color = old_color + (new_color - old_color) × alpha`

**LaTeX:** $$C_{\text{new}} = C_{\text{old}} + (C_{\text{new}} - C_{\text{old}}) \times \alpha$$

### 3. Circle Equation
**Plain text:** `x² + y² ≤ r²`

**LaTeX:** $$x^2 + y^2 \leq r^2$$

### 4. Vector Length
**Plain text:** `length = √(dx² + dy²)`

**LaTeX:** $$\text{length} = \sqrt{dx^2 + dy^2}$$

### 5. Perpendicular Unit Vector
**Plain text:** `perpendicular = (-dy/length, dx/length)`

**LaTeX:** $$\vec{p} = \left(\frac{-dy}{\text{length}}, \frac{dx}{\text{length}}\right)$$

### 6. Gradient (Slope)
**Plain text:** `gradient = Δy / Δx`

**LaTeX:** $$\text{gradient} = \frac{\Delta y}{\Delta x}$$

### 7. Wu's Alpha for Two Pixels
**Plain text:**
```
alpha_lower = 1 - frac(y)
alpha_upper = frac(y)
alpha_lower + alpha_upper = 1.0 (total coverage)
```

**LaTeX:**
$$\alpha_{\text{lower}} = 1 - \text{frac}(y)$$
$$\alpha_{\text{upper}} = \text{frac}(y)$$
$$\alpha_{\text{lower}} + \alpha_{\text{upper}} = 1.0 \quad \text{(total coverage)}$$

---

## Performance Notes

### Why Wu's Algorithm?
- **Problem**: Jagged lines (aliasing) look unprofessional
- **Solution**: Anti-aliasing by drawing fractional pixels
- **Cost**: 2 pixels per x-coordinate (instead of 1)
- **Benefit**: Smooth, professional-looking lines

### Why Thick Lines Use Parallel Lines?
- **Alternative 1**: Draw wider pixels → Creates square/blocky lines
- **Alternative 2**: Brush-based approach → Slower, more complex
- **Our approach**: Multiple parallel antialiased lines → Fast + smooth

### Optimization: Squared Distance

**Plain text:**
```
Slow:  √(dx² + dy²) ≤ radius
Fast:  dx² + dy² ≤ radius²
```

**LaTeX:**

**Slow:** $\sqrt{dx^2 + dy^2} \leq \text{radius}$

**Fast:** $dx^2 + dy^2 \leq \text{radius}^2$

Squaring both sides eliminates expensive sqrt() operation.

---

## Additional Resources

- **Xiaolin Wu's Line Algorithm**: [Wikipedia](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm)
- **Bresenham's Line Algorithm**: Faster but no anti-aliasing
- **Alpha Compositing**: [Porter-Duff operators](https://en.wikipedia.org/wiki/Alpha_compositing)
