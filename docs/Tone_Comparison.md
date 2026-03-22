# Tone Mapping Functions - Mathematical Comparison

## Visual Differences Explained

### 1. Highlights vs Shadows - Different Regions

**Highlights** (acts on bright regions, v ≥ 200):
```
Formula: threshold + (v - threshold) × (1 + strength)

Input  | Strength=-0.5 | Strength=0     | Strength=+0.5
-------|---------------|----------------|---------------
180    | 180 (no change - below threshold)
200    | 200           | 200            | 200 (at threshold)
220    | 210           | 220            | 230
240    | 220           | 240            | 260 → 255
255    | 227.5         | 255            | 282.5 → 255

Effect: Linear compression/expansion of bright areas
```

**Shadows** (acts on dark regions, v < 80):
```
Formula: v + strength × (threshold - v) × (v / threshold)

Input  | Strength=-0.5 | Strength=0     | Strength=+0.6
-------|---------------|----------------|---------------
20     | 12.5          | 20             | 27
40     | 30            | 40             | 52
60     | 52.5          | 60             | 78
80     | 80            | 80             | 80 (at threshold)
100    | 100 (no change - above threshold)

Effect: Non-linear lift with compression (darker = less lift)
```

**Key Difference**: 
- Highlights: Simple linear adjustment (same % change for all bright pixels)
- Shadows: Weighted lift (the `v/t` term means darker pixels get proportionally less lift)

---

### 2. Brightness vs Black Point - Different Purposes

**Brightness** (shifts all values equally):
```
Formula: v + brightness

Input  | Brightness=-30 | Brightness=0   | Brightness=+30
-------|----------------|----------------|---------------
0      | 0 (clamped)    | 0              | 30
50     | 20             | 50             | 80
128    | 98             | 128            | 158
200    | 170            | 200            | 230
255    | 225            | 255            | 255 (clamped)

Effect: Parallel shift (all values move by same amount)
Loses blacks/whites when values clip!
```

**Black Point** (remaps range):
```
Formula: output_black + ((v - input_black) / (255 - input_black)) × (255 - output_black)

Input  | in=30,out=0    | in=0,out=20    | in=40,out=0
-------|----------------|----------------|---------------
0      | 0              | 20             | 0
30     | 0 (crushed)    | 31             | 0 (crushed)
40     | 4.4            | 38             | 0 (crushed)
128    | 43.6           | 133            | 40.9
200    | 75.6           | 203            | 74.4
255    | 100            | 255            | 100

Effect: Stretches or compresses tonal range
Preserves relative differences!
```

**Key Difference**:
- Brightness: Additive (can blow out highlights/crush shadows)
- Black Point: Remapping (preserves tonal relationships, just shifts range)

---

### 3. All Four Functions - Side by Side

```
Original pixel value: 64 (dark-ish)

Highlights(-0.5, threshold=200):  64 → 64 (no change, below threshold)
Shadows(+0.6, threshold=80):       64 → 71.68 (lifted)
Brightness(+30):                   64 → 94 (shifted up)
Black Point(input=30, output=0):   64 → 15.11 (remapped down)

---

Original pixel value: 220 (bright)

Highlights(-0.5, threshold=200):  220 → 210 (compressed)
Shadows(+0.6, threshold=80):       220 → 220 (no change, above threshold)
Brightness(+30):                   220 → 250 (shifted up)
Black Point(input=30, output=0):   220 → 84.44 (remapped down)
```

---

## Mathematical Properties

| Function | Type | Range Affected | Linearity | Preserves Ratios |
|----------|------|----------------|-----------|------------------|
| **Highlights** | Parametric | v ≥ threshold | Linear in region | No (clipping) |
| **Shadows** | Weighted lift | v < threshold | Non-linear (v/t term) | No (weighted) |
| **Brightness** | Additive | All values | Linear globally | No (shifts) |
| **Black Point** | Remapping | All values | Piecewise linear | Yes (in segments) |

---

## When to Use Each

### Highlights
- **Problem**: Overexposed sky/clouds
- **Solution**: Negative strength to recover detail
- **Example**: Wedding photo with blown-out dress → recover texture

### Shadows
- **Problem**: Underexposed faces/details in dark areas
- **Solution**: Positive strength to lift shadows
- **Example**: Indoor photo → reveal details in dark corners
- **Note**: The `v/t` term prevents making pure blacks look "gray"

### Brightness
- **Problem**: Image is slightly too dark or light overall
- **Solution**: Quick uniform shift
- **Example**: Screen too dim → add +20 for quick fix
- **Warning**: Can blow out highlights if not careful

### Black Point
- **Problem**: Washed out blacks (no true black)
- **Solution**: Remap minimum value to pure black
- **Example**: Old scanned photo with gray "blacks" → crush to 0
- **Note**: Preserves relative tonal differences unlike brightness

---

## Formula Complexity Ranking

1. **Brightness** (simplest):  
   `v + b`

2. **Highlights** (simple parametric):  
   `threshold + (v - threshold) × (1 + h)`  [if above threshold]

3. **Black Point** (linear remapping):  
   `out + ((v - in) / (255 - in)) × (255 - out)`

4. **Shadows** (weighted non-linear):  
   `v + s × (threshold - v) × (v / threshold)`  [compression term!]

---

## Conclusion

**Are they too similar?**  
**No** - Each serves a distinct purpose:

- **Highlights/Shadows**: Opposite regions (bright vs dark), different math (linear vs weighted)
- **Brightness/Black Point**: Different operations (shift vs remap), different quality (clips vs preserves)

They may *look* similar in structure (threshold-based, LUT optimization), but:
1. They operate on **different regions** of the tonal range
2. They use **different mathematical transformations**
3. They produce **different visual results**
4. They solve **different photographic problems**

This is intentional - professional photo editing software (Lightroom, Capture One) has all these sliders because each solves a unique problem!
