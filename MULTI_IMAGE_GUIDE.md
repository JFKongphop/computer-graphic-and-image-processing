# Multi-Image LUT Training Guide

## Overview
Process multiple image pairs to create a high-quality 3D LUT with better color space coverage.

## Workflow

### 1. Prepare Your Images
Place image pairs in these folders:
- `source/compare/standard/` - Original images (1.JPG, 2.JPG, ..., 100.JPG)
- `source/compare/classic-chrome/` - Target film simulation (1.JPG, 2.JPG, ..., 100.JPG)

**Image Requirements:**
- Same dimensions for each pair
- Same numbering (e.g., standard/50.JPG matches classic-chrome/50.JPG)
- Diverse content: portraits, landscapes, products, etc.
- Good representation of different colors and exposures

### 2. Process All Images

#### Option A: Use the automated script (recommended)
```bash
# Process images 1-100 with 200 samples per LAB bucket
./process_multi_images.sh 1 100 200

# Or process a subset
./process_multi_images.sh 1 50 200
```

#### Option B: Manual processing
```bash
# Step 1: Process individual images
for i in {1..100}; do
  cargo run --release --bin stratified_compare_multi -- $i 200
done

# Step 2: Combine CSVs
echo "index,sr,sg,sb,cr,cg,cb,dr,dg,db" > outputs/pixel_comparison_combined.csv
for i in {1..100}; do
  tail -n +2 outputs/pixel_comparison_$i.csv >> outputs/pixel_comparison_combined.csv
done

# Step 3: Build LUT
cp outputs/pixel_comparison_combined.csv outputs/pixel_comparison.csv
cargo run --release --bin build_lut
```

### 3. Apply and Compare

```bash
# Apply LUT to test image
cargo run --release --bin apply_lut

# Compare with ground truth
cargo run --release --bin compare_lut
```

## Expected Results

### Single Image (current)
- Samples: ~103,000
- LUT coverage: 32%
- PSNR: 35.9 dB
- Avg ΔE: 1.71

### 100 Images (expected)
- Samples: ~10,000,000
- LUT coverage: 90-95%
- PSNR: 38-40 dB (estimated)
- Avg ΔE: 1.0-1.3 (estimated)

## Performance Tips

1. **Use --release mode** for faster processing (~5x speedup)
2. **Process in batches** if memory is limited
3. **Parallel processing**: Process images on multiple machines, combine CSVs later
4. **Incremental updates**: Add new images and rebuild periodically

## File Structure

```
outputs/
├── pixel_comparison_1.csv          # Image 1 samples
├── pixel_comparison_2.csv          # Image 2 samples
├── ...
├── pixel_comparison_100.csv        # Image 100 samples
├── pixel_comparison_combined.csv   # All samples combined
├── pixel_comparison.csv            # Current working CSV (used by build_lut)
└── lut_33.cube                     # Final 33×33×33 LUT
```

## Troubleshooting

### "Image not found" errors
- Verify images exist in both standard/ and classic-chrome/ folders
- Check file naming (must be 1.JPG, 2.JPG, etc.)
- Ensure .JPG extension (not .jpg)

### Out of memory
- Reduce samples per bucket: use 100 instead of 200
- Process in smaller batches
- Use 64GB+ RAM for 100 images at 200 samples/bucket

### Build takes too long
- Use `cargo run --release` instead of `cargo run`
- Consider reducing LUT size from 33³ to 25³ initially

## Advanced: Adaptive Sampling

To identify undersampled regions:
```bash
# Build initial LUT with current data
cargo run --bin build_lut

# Apply to many test images
# Identify regions with high error
# Capture more images with those colors
```

## Quality Metrics Guide

| PSNR (dB) | Quality | Typical Use |
|-----------|---------|-------------|
| < 30 | Poor | Unacceptable |
| 30-35 | Fair | Acceptable for previews |
| 35-40 | Good | Production quality |
| 40-45 | Excellent | Professional grade |
| > 45 | Outstanding | Near-perfect |

| ΔE | Perception |
|----|------------|
| < 1.0 | Not perceptible |
| 1.0-2.0 | Perceptible through close observation |
| 2.0-3.5 | Perceptible at a glance |
| 3.5-5.0 | Clear color difference |
| > 5.0 | Very obvious difference |
