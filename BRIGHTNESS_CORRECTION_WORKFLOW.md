# Brightness Bias Correction & Multi-Image Training Workflow

## Overview
Your analysis shows the LUT output is **+1.49% brighter** than Classic Chrome. This guide provides two complementary solutions.

---

## ✅ Option 1: Immediate Bias Correction (5 minutes)

Apply brightness correction to your existing LUT right now.

### Step 1: Run Bias Correction

```bash
# Use the detected bias (+1.489)
cargo run --release --bin correct_lut_bias

# Or specify a custom correction amount
cargo run --release --bin correct_lut_bias -- 1.5
```

**Output:** `outputs/lut_33_corrected.cube`

### Step 2: Test Corrected LUT

Update `apply_lut.rs` to use the corrected LUT:
```rust
let lut_path = "outputs/lut_33_corrected.cube";
let output_path = "outputs/lut_33_corrected.jpg";
```

Then apply:
```bash
cargo run --release --bin apply_lut
```

### Step 3: Verify Correction Worked

```bash
# Update compare_lut.rs to use corrected output
# Then analyze new bias
cargo run --bin analyze_brightness_bias
```

**Expected result:** Bias should drop from +1.49 to ~±0.2 (nearly zero)

### Pros ✅
- Instant fix
- No new data needed
- Mathematically precise correction
- Preserves all existing work

### Cons ❌
- Doesn't address root cause
- May introduce slight color shifts at extremes
- Temporary fix until better training data

---

## ✅ Option 2: Multi-Image Training (Long-term solution)

Collect 100+ diverse images to naturally reduce bias and improve overall quality.

### Step 1: Prepare Image Pairs

Organize your images:
```
source/compare/standard/
  1.JPG, 2.JPG, ..., 100.JPG
source/compare/classic-chrome/
  1.JPG, 2.JPG, ..., 100.JPG
```

**Image Selection Tips:**
- Mix of bright & dark scenes
- Various subjects (portraits, landscapes, products, nature)
- Different exposures
- Indoor and outdoor lighting
- Various color palettes (warm, cool, neutral)

### Step 2: Process All Images

```bash
# Automated processing
./process_multi_images.sh 1 100 200

# This will:
# - Process each image pair with stratified LAB sampling
# - Combine all CSVs into pixel_comparison_combined.csv
# - Build new LUT automatically
```

**Expected duration:** ~10-15 minutes for 100 images

### Step 3: Compare Quality

```bash
# Apply new multi-image LUT
cargo run --release --bin apply_lut

# Compare quality metrics
cargo run --bin compare_lut

# Check brightness bias
cargo run --bin analyze_brightness_bias
```

### Expected Improvements with 100 Images

| Metric | Current (8 images) | Expected (100 images) |
|--------|---------------------|------------------------|
| **Training Samples** | 103,427 | ~10,000,000 |
| **LUT Coverage** | 32% | 90-95% |
| **PSNR** | 35.90 dB | 38-40 dB |
| **Avg ΔE** | 1.71 | 1.0-1.3 |
| **Brightness Bias** | +1.49% | ±0.3% |

### Pros ✅
- Naturally reduces bias through diversity
- Dramatically improves overall quality
- Better generalization to new images
- More robust LUT
- Minimal interpolation needed

### Cons ❌
- Requires 100 image pairs
- Time-intensive data collection
- ~15 minutes processing time
- Large storage (~20GB for 7MP images)

---

## 🎯 Recommended Combined Workflow

Use **both** approaches for best results:

### Phase 1: Immediate Fix (Now)
1. ✅ Run bias correction: `cargo run --bin correct_lut_bias`
2. ✅ Test corrected LUT
3. ✅ Use corrected version in production

**Time:** 5 minutes  
**Benefit:** Fixes brightness issue immediately

### Phase 2: Long-term Improvement (Next few days/weeks)
1. 📸 Collect 50-100 diverse image pairs
2. 🔄 Process with multi-image script
3. 📊 Compare quality metrics
4. ✅ Replace corrected LUT with multi-image LUT

**Time:** Data collection dependent  
**Benefit:** Professional-grade quality

---

## 📊 Bias Correction Technical Details

### How It Works

1. **Load existing LUT** (`lut_33.cube`)
2. **For each LUT cell:**
   - Convert RGB → LAB
   - Subtract bias from L* channel (e.g., -1.489)
   - Clamp L* to [0, 100]
   - Convert back LAB → RGB
   - Clamp RGB to [0, 1]
3. **Save corrected LUT** (`lut_33_corrected.cube`)

### Why LAB Space?

- **Perceptually uniform:** 1 unit change = consistent perceived difference
- **Separates luminance:** L* channel is pure brightness
- **Preserves color:** a* (red-green) and b* (blue-yellow) unchanged
- **More accurate** than RGB correction

### Custom Correction Amounts

```bash
# Stronger correction (darker output)
cargo run --bin correct_lut_bias -- 2.0

# Weaker correction
cargo run --bin correct_lut_bias -- 1.0

# Experimental: slight overcorrection
cargo run --bin correct_lut_bias -- 1.7
```

**Tip:** After correction, always verify with `analyze_brightness_bias`

---

## 🔄 Iterative Refinement Process

### Round 1: Current State
- 8 training images
- Bias: +1.49%
- PSNR: 35.90 dB

### Round 2: Apply Bias Correction
- Same 8 training images
- Bias: ~0.0% (corrected)
- PSNR: ~36.0 dB (slight improvement)

### Round 3: Add More Images (25 total)
- Diverse scenes
- Bias: ~0.5% (naturally reduced)
- PSNR: 36.5-37.0 dB

### Round 4: Full Dataset (100+ images)
- Excellent coverage
- Bias: <0.3%
- PSNR: 38-40 dB
- **Professional quality achieved**

---

## 📁 File Reference

### New Tools
- `src/bin/analyze_brightness_bias.rs` - Detect bias in LUT output
- `src/bin/correct_lut_bias.rs` - Apply LAB-based brightness correction
- `src/bin/stratified_compare_multi.rs` - Process individual images
- `process_multi_images.sh` - Batch processing script

### Outputs
- `outputs/lut_33.cube` - Original LUT (biased)
- `outputs/lut_33_corrected.cube` - Bias-corrected LUT
- `outputs/pixel_comparison_combined.csv` - Multi-image training data
- `outputs/pixel_comparison_1.csv` to `pixel_comparison_N.csv` - Individual images

---

## 🎯 Quick Start Commands

```bash
# === OPTION 1: Immediate Correction ===
cargo run --release --bin correct_lut_bias
# Update apply_lut.rs to use lut_33_corrected.cube
cargo run --release --bin apply_lut
cargo run --bin analyze_brightness_bias  # Verify ~0% bias

# === OPTION 2: Multi-Image Training ===
./process_multi_images.sh 1 100 200
cargo run --release --bin apply_lut
cargo run --bin compare_lut
cargo run --bin analyze_brightness_bias

# === BOTH: Use correction now, train later ===
# Do Option 1 today, Option 2 when you have images
```

---

## 💡 Pro Tips

1. **Test on Multiple Images**
   - Don't just test on 9.JPG
   - Try various scenes to ensure correction generalizes

2. **Document Your Bias**
   - Record bias measurements before/after
   - Track improvements over time

3. **Incremental Approach**
   - Start with 25 images, measure improvement
   - Add 25 more, measure again
   - Find the point of diminishing returns

4. **Compare Methods**
   - Keep both corrected LUT and multi-image LUT
   - A/B test on various images
   - Choose what looks best visually

5. **Version Control Your LUTs**
   ```bash
   mv outputs/lut_33.cube outputs/lut_33_v1_8images.cube
   mv outputs/lut_33_corrected.cube outputs/lut_33_v2_corrected.cube
   # After 100 images:
   mv outputs/lut_33.cube outputs/lut_33_v3_100images.cube
   ```

---

**Last Updated:** March 29, 2026  
**Detected Bias:** +1.489 LAB L* units (+1.49%)  
**Recommended Action:** Apply both options
