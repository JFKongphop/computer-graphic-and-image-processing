# Computer Graphics: 21 Advanced Features for FastImage

**Main Topic:** Computer Graphics (2D Rendering & Image Processing)  
**Subtopic:** Vector Graphics, Rasterization, Anti-aliasing, Image Effects

This document outlines 21 advanced features to extend the FastImage graphics library, all within the field of **Computer Graphics**. Each feature includes its mathematical foundation, practical applications, and implementation complexity.

---

## Table of Contents

### Part 1: Shape Drawing Primitives (5 features)
1. Hollow Circle (Circle Outline)
2. Ellipse (Filled & Outline)
3. Arc (Partial Circle)
4. Rectangle (Filled & Outline)
5. Rounded Rectangle

### Part 2: Advanced Line Features (4 features)
6. Dashed/Dotted Lines
7. Line with Arrows
8. Bezier Curves
9. Gradient Lines

### Part 3: Text & Annotations (2 features)
10. Text Rendering
11. Distance Markers

### Part 4: Color & Effects (3 features)
12. Fill Patterns
13. Gaussian Blur
14. Drop Shadow

### Part 5: GPS/Route Specific (3 features)
15. Elevation Profile
16. Heat Map Overlay
17. Route Simplification (Douglas-Peucker)

### Part 6: Performance & Quality (4 features)
18. Grid / Graticule
19. Clip Region (Cohen-Sutherland)
20. Level of Detail (LOD)
21. Supersampling / MSAA

---

# Part 1: Shape Drawing Primitives

## 1. Hollow Circle (Circle Outline)

### Mathematics
**Annulus equation** (ring between two circles):

$$
(r - \frac{t}{2})^2 \leq x^2 + y^2 \leq (r + \frac{t}{2})^2
$$

Where:
- $r$ = circle radius
- $t$ = line thickness
- $(x, y)$ = point offset from center

**Plain text:**
```
(radius - thickness/2)² ≤ distance² ≤ (radius + thickness/2)²
```

### Application
- Route markers (start/end points)
- Waypoint indicators
- Target highlights
- Selection circles

### Complexity
⭐⭐☆☆☆ (Easy - similar to existing `draw_circle`)

### Example
```
Circle: center=(100, 100), radius=20, thickness=3
Inner radius: 18.5, Outer radius: 21.5

Point at (120, 100): distance = 20
  20² = 400
  18.5² = 342.25, 21.5² = 462.25
  342.25 ≤ 400 ≤ 462.25 ✓ Draw (on ring)
```

---

## 2. Ellipse (Filled & Outline)

### Mathematics
**Ellipse equation:**

$$
\left(\frac{x}{a}\right)^2 + \left(\frac{y}{b}\right)^2 \leq 1
$$

Where:
- $a$ = horizontal semi-axis (half-width)
- $b$ = vertical semi-axis (half-height)
- $(x, y)$ = point offset from center

**Plain text:**
```
(x/a)² + (y/b)² ≤ 1
```

**Special case:** Circle is an ellipse where $a = b$

### Application
- Geographic features (lakes, regions)
- Uncertainty/error ellipses
- Directional indicators
- Anisotropic markers

### Complexity
⭐⭐☆☆☆ (Easy - generalization of circle)

### Example
```
Ellipse: center=(100, 100), a=30, b=20

Point (120, 100): offset=(20, 0)
  (20/30)² + (0/20)² = 0.444 + 0 = 0.444 ≤ 1 ✓ Inside

Point (115, 112): offset=(15, 12)
  (15/30)² + (12/20)² = 0.25 + 0.36 = 0.61 ≤ 1 ✓ Inside
```

---

## 3. Arc (Partial Circle)

### Mathematics
**Parametric circle + angle test:**

$$
x = c_x + r \cos(\theta), \quad y = c_y + r \sin(\theta)
$$

$$
\theta_{\text{point}} = \arctan2(y - c_y, x - c_x)
$$

Check: $\theta_{\text{start}} \leq \theta_{\text{point}} \leq \theta_{\text{end}}$

**Plain text:**
```
x = center_x + radius × cos(angle)
y = center_y + radius × sin(angle)
point_angle = atan2(y - center_y, x - center_x)
```

### Application
- Compass directions
- Heading indicators
- Partial pie charts
- Turn angle visualization

### Complexity
⭐⭐⭐☆☆ (Moderate - involves trigonometry)

### Example
```
Arc: center=(100, 100), radius=50, angles=[0°, 90°]

Quarter circle in upper-right quadrant:
  Point (150, 100): angle = 0° ✓ (east)
  Point (135, 135): angle = 45° ✓ (northeast)
  Point (100, 50): angle = -90° = 270° ✗ (south, outside range)
```

---

## 4. Rectangle (Filled & Outline)

### Mathematics
**Axis-aligned bounding box (AABB):**

$$
x_{\min} \leq x \leq x_{\max} \quad \text{AND} \quad y_{\min} \leq y \leq y_{\max}
$$

**Plain text:**
```
Point inside if:
  x_min ≤ x ≤ x_max AND y_min ≤ y ≤ y_max
```

### Application
- Legends and labels
- Background boxes
- Clipping regions
- Grid cells

### Complexity
⭐☆☆☆☆ (Very easy - basic shape)

### Example
```
Rectangle: (10, 20) to (50, 80)

Filled: Loop through all (x, y) where 10 ≤ x ≤ 50, 20 ≤ y ≤ 80
Outline: Draw 4 lines forming perimeter
```

---

## 5. Rounded Rectangle

### Mathematics
**Rectangle + corner circles:**

For each corner with radius $r$, use circle equation for corner regions:

$$
(x - c_x)^2 + (y - c_y)^2 \leq r^2
$$

Corner centers are offset inward by $r$ from edges.

**Plain text:**
```
Corner regions: Use circle equation
Straight edges: Use rectangle equation
```

### Application
- Modern UI elements
- Smooth label backgrounds
- Rounded markers
- Aesthetic improvements

### Complexity
⭐⭐⭐☆☆ (Moderate - combines rectangles and circles)

### Example
```
Rounded rect: (0, 0) to (100, 60), corner_radius=10

Corner centers:
  Top-left: (10, 10)
  Top-right: (90, 10)
  Bottom-left: (10, 50)
  Bottom-right: (90, 50)

For point (5, 5):
  Distance to top-left corner: √50 ≈ 7.07 < 10 ✓ Inside
```

---

# Part 2: Advanced Line Features

## 6. Dashed/Dotted Lines

### Mathematics
**Distance-based pattern:**

$$
d = \sqrt{(x - x_0)^2 + (y - y_0)^2}
$$

$$
\text{pattern\_pos} = d \bmod (\text{dash} + \text{gap})
$$

Draw if $\text{pattern\_pos} < \text{dash}$

**Plain text:**
```
distance_traveled mod (dash_length + gap_length)
Draw if position < dash_length
```

### Application
- Secondary routes
- Boundaries
- Construction/planning lines
- Differentiate line types

### Complexity
⭐⭐☆☆☆ (Easy - modify existing line algorithm)

### Example
```
Pattern: dash=10, gap=5 (cycle=15)

x=0:  d=0  → 0 mod 15 = 0  → Draw
x=10: d=10 → 10 mod 15 = 10 → Skip (gap)
x=15: d=15 → 15 mod 15 = 0  → Draw (new cycle)
x=25: d=25 → 25 mod 15 = 10 → Skip (gap)

Visual: ──────────     ──────────     ──────────
```

---

## 7. Line with Arrows

### Mathematics
**Vector rotation for arrow wings:**

2D rotation matrix:

$$
\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix} \begin{pmatrix} x \\ y \end{pmatrix}
$$

For arrow at line end:
1. Unit direction: $\vec{u} = (dx, dy) / \|\vec{v}\|$
2. Reverse: $\vec{back} = -\vec{u}$
3. Rotate by ±135° for wings
4. Scale by arrow length

**Plain text:**
```
direction = normalize(end - start)
back = -direction
left_wing = rotate(back, -135°) × arrow_length
right_wing = rotate(back, +135°) × arrow_length
```

### Application
- Show route direction
- Flow indicators
- Navigation arrows
- Movement vectors

### Complexity
⭐⭐⭐☆☆ (Moderate - trigonometry required)

### Example
```
Line: (10, 10) → (50, 30)
Direction: (40, 20), normalized: (0.894, 0.447)
Arrow length: 10

Back direction: (-0.894, -0.447)
Left wing rotated -135°: ends at (59.48, 33.16)
Right wing rotated +135°: ends at (46.16, 40.48)

Draw: line + two wing lines forming arrow head
```

---

## 8. Bezier Curves

### Mathematics
**Quadratic Bezier (3 control points):**

$$
B(t) = (1-t)^2 P_0 + 2(1-t)t P_1 + t^2 P_2, \quad t \in [0,1]
$$

**Cubic Bezier (4 control points):**

$$
B(t) = (1-t)^3 P_0 + 3(1-t)^2t P_1 + 3(1-t)t^2 P_2 + t^3 P_3
$$

Where:
- $P_0$ = start point
- $P_1, P_2$ = control points (curve pulls toward these)
- $P_3$ = end point (cubic only)
- $t$ = parameter from 0 to 1

**Plain text:**
```
Quadratic: B(t) = (1-t)² × start + 2(1-t)t × control + t² × end
Cubic: More control points for smoother curves
```

### Application
- Smooth route interpolation
- Curve fitting between waypoints
- Artistic paths
- Vector graphics

### Complexity
⭐⭐⭐⭐☆ (Complex - parametric curves)

### Example
```
Quadratic Bezier:
  P₀ = (0, 0), P₁ = (50, 100), P₂ = (100, 0)

t=0:   B(0) = (0, 0) - start
t=0.5: B(0.5) = (50, 50) - curve peak
t=1:   B(1) = (100, 0) - end

Renders as smooth arc from (0,0) to (100,0)
pulling toward control point (50,100)
```

---

## 9. Gradient Lines (Color Transition)

### Mathematics
**Linear color interpolation (lerp):**

$$
C(t) = C_{\text{start}} + (C_{\text{end}} - C_{\text{start}}) \times t
$$

For each RGB channel:

$$
R(t) = R_0 + (R_1 - R_0) \times t
$$

Where $t \in [0, 1]$ is position along line.

**Plain text:**
```
color = start_color + (end_color - start_color) × position
Apply to each RGB channel separately
```

### Application
- Visualize speed along route
- Show elevation changes
- Display heart rate data
- Temperature gradients

### Complexity
⭐⭐⭐☆☆ (Moderate - color math + line drawing)

### Example
```
Line: (0, 0) → (100, 0)
Color: Red (255, 0, 0) → Blue (0, 0, 255)

x=0 (t=0):    RGB(255, 0, 0) - Pure red
x=50 (t=0.5): RGB(128, 0, 128) - Purple
x=100 (t=1):  RGB(0, 0, 255) - Pure blue

Visual: ████████████████████████████████████████
        Red → Purple → Blue
```

---

# Part 3: Text & Annotations

## 10. Text Rendering

### Mathematics
**Glyph atlas lookup:**

$$
\text{row} = \lfloor \frac{\text{char\_code}}{\text{columns}} \rfloor
$$

$$
\text{col} = \text{char\_code} \bmod \text{columns}
$$

Source position in atlas:
$$
\text{src\_x} = \text{col} \times \text{glyph\_width}
$$

$$
\text{src\_y} = \text{row} \times \text{glyph\_height}
$$

**Plain text:**
```
row = char_code / atlas_columns
col = char_code % atlas_columns
source_x = col × glyph_width
source_y = row × glyph_height
```

### Application
- Labels and annotations
- Distance markers
- Legend text
- Debugging info

### Complexity
⭐⭐⭐☆☆ (Moderate - bitmap font management)

### Example
```
Font atlas: 16×8 grid (128 ASCII chars)
Each glyph: 8×12 pixels

Character 'A' (ASCII 65):
  row = 65 / 16 = 4
  col = 65 % 16 = 1
  source = (8, 48) in atlas

Render "HELLO":
  H, E, L, L, O sequentially with 8px spacing
```

---

## 11. Distance Markers

### Mathematics
**Arc length + text placement:**

$$
\text{total\_distance} = \sum_{i=0}^{n-1} \sqrt{(x_{i+1} - x_i)^2 + (y_{i+1} - y_i)^2}
$$

Place markers every $d_{\text{interval}}$ meters/kilometers.

Marker position along curve:
$$
t = \frac{d_{\text{accumulated}}}{d_{\text{total}}}
$$

**Plain text:**
```
Accumulate distance along path
Every N km, place text marker
Calculate rotation from path tangent
```

### Application
- Auto-label route distances
- Milestone markers
- Progress indicators
- Scale references

### Complexity
⭐⭐⭐☆☆ (Moderate - requires text rendering + path math)

---

# Part 4: Color & Effects

## 12. Fill Patterns

### Mathematics
**Repeating pattern with modulo:**

$$
\text{pattern\_x} = x \bmod \text{pattern\_width}
$$

$$
\text{pattern\_y} = y \bmod \text{pattern\_height}
$$

For hatching at angle $\theta$:
$$
(x \cos\theta + y \sin\theta) \bmod \text{spacing} < \text{line\_width}
$$

**Plain text:**
```
Tile pattern: use (x mod width, y mod height) to index pattern
Hatching: rotate coordinate system, apply stripe pattern
```

### Application
- Distinguish map regions
- Texture fills
- Visual separation
- Artistic effects

### Complexity
⭐⭐☆☆☆ (Easy - pattern repetition)

---

## 13. Gaussian Blur

### Mathematics
**2D Gaussian function:**

$$
G(x, y) = \frac{1}{2\pi\sigma^2} e^{-\frac{x^2 + y^2}{2\sigma^2}}
$$

**Separable implementation (optimization):**

$$
G_{2D}(x, y) = G_{1D}(x) \times G_{1D}(y)
$$

Where $\sigma$ is the blur radius/strength.

**Convolution:**
$$
\text{blurred}(x, y) = \sum_{i,j} \text{image}(x+i, y+j) \times G(i, j)
$$

**Plain text:**
```
Generate Gaussian kernel based on sigma
Convolve kernel with image
Separable: blur horizontally, then vertically (faster)
```

### Application
- Background blur (depth effect)
- Soft shadows
- Smooth edges
- Artistic blur

### Complexity
⭐⭐⭐⭐☆ (Complex - convolution operation)

### Example
```
3×3 Gaussian kernel (σ=1.0, normalized):
  [0.094  0.156  0.094]
  [0.156  0.256  0.156]
  [0.094  0.156  0.094]

Apply to each pixel: multiply neighbors by weights, sum
Result: smoothed/blurred image
```

---

## 14. Drop Shadow

### Mathematics
**Shadow = Offset + Blur + Alpha composite:**

1. Offset by $(dx, dy)$
2. Apply Gaussian blur with $\sigma$
3. Composite with alpha: $\alpha(d) = \alpha_0 e^{-d^2/(2\sigma^2)}$

**Plain text:**
```
1. Render shape to temp buffer
2. Offset position
3. Blur with Gaussian
4. Draw with transparency behind original
```

### Application
- 3D depth illusion
- Elevation indication
- Visual hierarchy
- Modern UI appearance

### Complexity
⭐⭐⭐⭐☆ (Complex - requires blur + compositing)

---

# Part 5: GPS/Route Specific

## 15. Elevation Profile

### Mathematics
**Vertical scale mapping:**

$$
y_{\text{pixel}} = y_{\text{bottom}} - \frac{h - h_{\min}}{h_{\max} - h_{\min}} \times H
$$

Where:
- $h$ = elevation at point
- $h_{\min}, h_{\max}$ = elevation range
- $H$ = graph height in pixels

**Plain text:**
```
y_pixel = bottom - ((elevation - min) / (max - min)) × height
Normalize elevation to [0, 1], then scale to pixel height
```

### Application
- Elevation graphs
- Climb visualization
- Gradient analysis
- Route difficulty assessment

### Complexity
⭐⭐⭐☆☆ (Moderate - data scaling + graph drawing)

### Example
```
Elevations: [100m, 150m, 200m, 180m, 120m]
Graph: 500×100 pixels

Range: 100m to 200m
Point at 200m: y = 100 - ((200-100)/100) × 100 = 0 (top)
Point at 100m: y = 100 - ((100-100)/100) × 100 = 100 (bottom)
```

---

## 16. Heat Map Overlay

### Mathematics
**Density accumulation with Gaussian kernel:**

$$
w(d) = e^{-\frac{d^2}{2r^2}}
$$

For each GPS point, add weight to nearby cells:
$$
\text{density}(x, y) = \sum_{\text{points}} w(\text{distance}(x, y, \text{point}))
$$

Map density to color using colormap (e.g., viridis, hot).

**Plain text:**
```
For each point: add Gaussian weight to nearby grid cells
Smooth accumulated density
Map density values to colors (blue=low, red=high)
```

### Application
- Activity frequency
- Popular route segments
- Traffic density
- Usage patterns

### Complexity
⭐⭐⭐⭐☆ (Complex - accumulation + color mapping)

---

## 17. Route Simplification (Douglas-Peucker)

### Mathematics
**Perpendicular distance from point to line:**

$$
t = \frac{(P - A) \cdot (B - A)}{|B - A|^2}
$$

Closest point on segment:
$$
\text{closest} = \begin{cases}
A & \text{if } t < 0 \\
B & \text{if } t > 1 \\
A + t(B - A) & \text{otherwise}
\end{cases}
$$

Distance:
$$
d = |P - \text{closest}|
$$

**Algorithm:**
1. Find point farthest from line segment
2. If distance > tolerance: keep point, recurse on sub-segments
3. Else: discard all intermediate points

**Plain text:**
```
Calculate perpendicular distance from each point to line
Keep points that exceed tolerance threshold
Recursively simplify sub-segments
```

### Application
- Reduce route complexity
- Faster rendering
- Storage optimization
- LOD (level of detail)

### Complexity
⭐⭐⭐⭐☆ (Complex - recursive algorithm)

### Example
```
Route: 1000 points
Tolerance: 5 pixels
Result: 50 points (95% reduction!)
Visual: nearly identical, much faster to render
```

---

# Part 6: Performance & Quality

## 18. Grid / Graticule

### Mathematics
**Evenly spaced lines:**

Vertical lines at: $x = x_{\min} + i \times \text{spacing}$
Horizontal lines at: $y = y_{\min} + j \times \text{spacing}$

For geographic grid (lat/lon):
$$
\text{pixel} = \text{project}(\text{lat}, \text{lon})
$$

**Plain text:**
```
Draw lines at regular intervals
For maps: convert lat/lon to pixels, draw grid
```

### Application
- Reference grid
- Distance scale
- Coordinate system
- Navigation aid

### Complexity
⭐⭐☆☆☆ (Easy - repetitive line drawing)

---

## 19. Clip Region (Cohen-Sutherland)

### Mathematics
**Outcode calculation (4-bit code):**

$$
\text{outcode} = \begin{cases}
\text{1000} & \text{if } y > y_{\max} \text{ (above)} \\
\text{0100} & \text{if } y < y_{\min} \text{ (below)} \\
\text{0010} & \text{if } x > x_{\max} \text{ (right)} \\
\text{0001} & \text{if } x < x_{\min} \text{ (left)}
\end{cases}
$$

**Clipping logic:**
- If $\text{code}_0 | \text{code}_1 = 0000$: both inside → draw
- If $\text{code}_0 \& \text{code}_1 \neq 0000$: both outside → reject
- Otherwise: clip and recurse

**Plain text:**
```
Assign 4-bit code to each endpoint (TBRL: top/bottom/right/left)
Use bitwise operations to test visibility
Clip lines to viewport boundaries
```

### Application
- Performance optimization
- Viewport culling
- Prevent out-of-bounds drawing
- Clean rendering

### Complexity
⭐⭐⭐☆☆ (Moderate - bit operations + line clipping)

---

## 20. Level of Detail (LOD)

### Mathematics
**Distance-based simplification:**

$$
\text{detail\_level} = \begin{cases}
\text{high} & \text{if } \text{scale} > \text{threshold}_1 \\
\text{medium} & \text{if } \text{scale} > \text{threshold}_2 \\
\text{low} & \text{otherwise}
\end{cases}
$$

Adjust parameters:
- Line thickness: $t(\text{scale}) = t_0 \times \text{scale}$
- Point density: $n(\text{scale}) = n_0 / \text{scale}$

**Plain text:**
```
Zoom out → reduce detail (fewer points, thinner lines)
Zoom in → increase detail (more points, thicker lines)
```

### Application
- Performance at zoom levels
- Scalable rendering
- Adaptive quality
- Responsive UI

### Complexity
⭐⭐⭐☆☆ (Moderate - multi-version management)

---

## 21. Supersampling / MSAA

### Mathematics
**Multi-sample averaging:**

For $n \times n$ samples per pixel:
$$
C_{\text{output}} = \frac{1}{n^2} \sum_{i=0}^{n-1} \sum_{j=0}^{n-1} C_{\text{sample}}(i, j)
$$

**2×2 SSAA (4 samples):**
$$
C = \frac{C_{00} + C_{01} + C_{10} + C_{11}}{4}
$$

**Plain text:**
```
Render at higher resolution (e.g., 2×)
Average subpixel colors to get final pixel
Result: smooth edges, no jaggies
```

### Application
- High-quality rendering
- Smooth edges
- Professional appearance
- Export quality

### Complexity
⭐⭐⭐⭐☆ (Complex - multi-pass rendering)

### Example
```
Edge passing through pixel:
  4 samples: 3 inside (white), 1 outside (black)
  Average: (255 + 255 + 255 + 0) / 4 = 191
  Result: gray pixel (75% coverage) → smooth edge!
```

---

# Summary

## All 21 Topics: Yes, They're All Computer Graphics!

**Computer Graphics** is the field of visual computing that deals with:
- Creating, manipulating, and rendering images
- 2D/3D geometry and rasterization
- Image processing and effects
- Visual representation of data

All 21 features fall under this umbrella, specifically:

### Subcategories

| Category | Topics | Computer Graphics Subfield |
|----------|--------|---------------------------|
| **Primitives (1-5)** | Shapes | Vector graphics, rasterization |
| **Lines (6-9)** | Advanced lines | Line algorithms, anti-aliasing |
| **Text (10-11)** | Typography | Font rendering, text layout |
| **Effects (12-14)** | Visual effects | Image processing, filtering |
| **GPS (15-17)** | Data visualization | Information visualization |
| **Performance (18-21)** | Optimization | Rendering optimization, quality |

### Complexity Distribution

- ⭐☆☆☆☆ **Very Easy**: Rectangle
- ⭐⭐☆☆☆ **Easy**: Hollow circle, Ellipse, Dashed lines, Fill patterns, Grid
- ⭐⭐⭐☆☆ **Moderate**: Arc, Rounded rectangle, Arrows, Gradients, Text, Elevation, Clipping, LOD
- ⭐⭐⭐⭐☆ **Complex**: Bezier curves, Blur, Shadows, Heat map, Douglas-Peucker, MSAA

### Recommended Implementation Order

1. **Quick wins**: Rectangle, Hollow circle, Dashed lines (1-2 days)
2. **Medium features**: Ellipse, Arc, Arrows, Text (1 week)
3. **Advanced**: Bezier, Gradient lines, Elevation (1-2 weeks)
4. **Complex**: Blur, Heat map, MSAA (2-3 weeks)

---

## Academic References

### Core Computer Graphics
- **Foley, James D., et al.** *Computer Graphics: Principles and Practice*. Addison-Wesley, 1995.
- **Hughes, John F., et al.** *Computer Graphics: Principles and Practice (3rd Edition)*. Addison-Wesley, 2013.
- **Marschner & Shirley.** *Fundamentals of Computer Graphics (5th Edition)*. CRC Press, 2021.

### Specific Algorithms
- **Bresenham, J. E.** "Algorithm for computer control of a digital plotter." *IBM Systems Journal*, 1965.
- **Wu, Xiaolin.** "An efficient antialiasing technique." *Computer Graphics (SIGGRAPH '91)*, 1991.
- **Douglas, David H.; Peucker, Thomas K.** "Algorithms for the reduction of the number of points required to represent a digitized line." *Cartographica*, 1973.
- **Cohen, Danny; Sutherland, Ivan.** Line clipping algorithm (1967).
- **Bézier, Pierre.** *Courbes et surfaces*. Hermès, 1986.

### Image Processing
- **Gonzalez & Woods.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
- **Burger & Burge.** *Digital Image Processing: An Algorithmic Introduction*. Springer, 2016.

---

**Document Version:** 1.0  
**Date:** January 2, 2026  
**License:** Educational Use
