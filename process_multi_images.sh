#!/bin/bash
# Process multiple images for LUT training
# Usage: ./process_multi_images.sh <start> <end> [max_samples_per_bucket]
# Example: ./process_multi_images.sh 1 100 200

START=${1:-1}
END=${2:-9}
MAX_SAMPLES=${3:-200}

echo "🎨 Multi-Image LUT Training Pipeline"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📸 Processing images: $START to $END"
echo "🔢 Max samples per bucket: $MAX_SAMPLES"
echo ""

# Step 1: Process each image pair
echo "📷 Step 1: Processing individual images..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for i in $(seq $START $END); do
  echo ""
  echo "Processing image $i/$END..."
  
  # Check if images exist
  if [ ! -f "source/compare/standard/$i.JPG" ]; then
    echo "⚠️  Warning: source/compare/standard/$i.JPG not found, skipping..."
    continue
  fi
  
  if [ ! -f "source/compare/classic-chrome/$i.JPG" ]; then
    echo "⚠️  Warning: source/compare/classic-chrome/$i.JPG not found, skipping..."
    continue
  fi
  
  # Process the image
  cargo run --release --bin stratified_compare_multi -- $i $MAX_SAMPLES
  
  if [ $? -ne 0 ]; then
    echo "❌ Error processing image $i"
    exit 1
  fi
done

echo ""
echo "✅ All images processed!"

# Step 2: Combine all CSV files
echo ""
echo "🔗 Step 2: Combining CSV files..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Create combined CSV with header
echo "index,sr,sg,sb,cr,cg,cb,dr,dg,db" > outputs/pixel_comparison_combined.csv

# Add all individual CSVs (skip headers)
TOTAL_SAMPLES=0
for i in $(seq $START $END); do
  CSV_FILE="outputs/pixel_comparison_$i.csv"
  
  if [ -f "$CSV_FILE" ]; then
    # Count samples in this file (minus header)
    SAMPLES=$(tail -n +2 "$CSV_FILE" | wc -l | tr -d ' ')
    echo "  Adding $CSV_FILE: $SAMPLES samples"
    
    # Append data (skip header)
    tail -n +2 "$CSV_FILE" >> outputs/pixel_comparison_combined.csv
    
    TOTAL_SAMPLES=$((TOTAL_SAMPLES + SAMPLES))
  fi
done

echo ""
echo "✅ Combined CSV created!"
echo "   Total samples: $TOTAL_SAMPLES"
echo "   Output: outputs/pixel_comparison_combined.csv"

# Step 3: Build LUT from combined data
echo ""
echo "🔧 Step 3: Building 33×33×33 LUT..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Backup current pixel_comparison.csv
if [ -f "outputs/pixel_comparison.csv" ]; then
  mv outputs/pixel_comparison.csv outputs/pixel_comparison_backup.csv
  echo "📦 Backed up existing pixel_comparison.csv"
fi

# Use combined CSV
cp outputs/pixel_comparison_combined.csv outputs/pixel_comparison.csv

# Build LUT
cargo run --release --bin build_lut

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Multi-image LUT training complete!"
echo ""
echo "📊 Summary:"
echo "   Images processed: $(($END - $START + 1))"
echo "   Total samples: $TOTAL_SAMPLES"
echo "   LUT output: outputs/lut_33.cube"
echo ""
echo "🔍 Next steps:"
echo "   1. Apply LUT: cargo run --bin apply_lut"
echo "   2. Compare quality: cargo run --bin compare_lut"
echo "   3. Check improvement vs single-image LUT"
echo ""
