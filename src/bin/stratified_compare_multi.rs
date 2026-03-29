use anyhow::Result;
use opencv::prelude::*;
use opencv::{core, imgcodecs};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Convert RGB to LAB color space
fn rgb_to_lab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
  // Convert RGB to XYZ (assuming sRGB)
  let r = if r > 0.04045 {
    ((r + 0.055) / 1.055).powf(2.4)
  } else {
    r / 12.92
  };
  let g = if g > 0.04045 {
    ((g + 0.055) / 1.055).powf(2.4)
  } else {
    g / 12.92
  };
  let b = if b > 0.04045 {
    ((b + 0.055) / 1.055).powf(2.4)
  } else {
    b / 12.92
  };

  let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
  let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
  let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

  // Convert XYZ to LAB
  let xn = 0.95047;
  let yn = 1.00000;
  let zn = 1.08883;

  let fx = if x / xn > 0.008856 {
    (x / xn).powf(1.0 / 3.0)
  } else {
    7.787 * (x / xn) + 16.0 / 116.0
  };
  let fy = if y / yn > 0.008856 {
    (y / yn).powf(1.0 / 3.0)
  } else {
    7.787 * (y / yn) + 16.0 / 116.0
  };
  let fz = if z / zn > 0.008856 {
    (z / zn).powf(1.0 / 3.0)
  } else {
    7.787 * (z / zn) + 16.0 / 116.0
  };

  let l = 116.0 * fy - 16.0;
  let a = 500.0 * (fx - fy);
  let b = 200.0 * (fy - fz);

  (l, a, b)
}

/// Compute bucket index in 8x8x8 LAB grid
fn compute_bucket(l: f32, a: f32, b: f32) -> (usize, usize, usize) {
  // LAB ranges: L [0, 100], a [-128, 127], b [-128, 127]
  let l_bucket = ((l / 100.0) * 7.999).floor().max(0.0).min(7.0) as usize;
  let a_bucket = (((a + 128.0) / 255.0) * 7.999).floor().max(0.0).min(7.0) as usize;
  let b_bucket = (((b + 128.0) / 255.0) * 7.999).floor().max(0.0).min(7.0) as usize;
  (l_bucket, a_bucket, b_bucket)
}

fn main() -> Result<()> {
  // Parse command line arguments
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <image_number> [max_samples_per_bucket]", args[0]);
    eprintln!("Example: {} 1 200", args[0]);
    std::process::exit(1);
  }

  let image_num: usize = args[1].parse().expect("Invalid image number");
  let max_samples_per_bucket = if args.len() >= 3 {
    args[2].parse().expect("Invalid max samples")
  } else {
    200
  };

  println!("🎨 Stratified Color Sampling in LAB Space");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  println!("📸 Processing image pair: {}.JPG", image_num);
  println!("🔢 Max samples per bucket: {}", max_samples_per_bucket);

  // Paths
  let standard_path = format!("source/compare/standard/{}.JPG", image_num);
  let chrome_path = format!("source/compare/classic-chrome/{}.JPG", image_num);
  let output_csv = format!("outputs/pixel_comparison_{}.csv", image_num);

  // Step 1: Load images
  println!("\n📷 Loading images...");
  let standard = imgcodecs::imread(&standard_path, imgcodecs::IMREAD_COLOR)?;
  let chrome = imgcodecs::imread(&chrome_path, imgcodecs::IMREAD_COLOR)?;

  let rows = standard.rows();
  let cols = standard.cols();
  println!("   Image size: {}x{}", cols, rows);
  println!("   Total pixels: {}", rows * cols);

  // Verify dimensions match
  if standard.size()? != chrome.size()? {
    return Err(anyhow::anyhow!("Image dimensions don't match!"));
  }

  // Step 2: Create 8x8x8 = 512 buckets in LAB space
  println!("\n🗂️  Creating stratified buckets (8x8x8 in LAB)...");
  let mut buckets: HashMap<(usize, usize, usize), Vec<(usize, usize)>> = HashMap::new();

  // Scan all pixels and assign to buckets
  for y in 0..rows {
    for x in 0..cols {
      // Read pixel from standard image
      let pixel = standard.at_2d::<core::Vec3b>(y, x)?;
      let b = pixel[0] as f32 / 255.0;
      let g = pixel[1] as f32 / 255.0;
      let r = pixel[2] as f32 / 255.0;

      // Convert to LAB
      let (l, a, b_val) = rgb_to_lab(r, g, b);

      // Compute bucket
      let bucket = compute_bucket(l, a, b_val);

      // Add pixel position to bucket
      buckets.entry(bucket).or_insert_with(Vec::new).push((y as usize, x as usize));
    }
  }

  println!("   Total buckets created: {}", buckets.len());
  let total_pixels: usize = buckets.values().map(|v| v.len()).sum();
  println!("   Total pixels categorized: {}", total_pixels);

  // Step 3: Sample from each bucket
  println!("\n🎲 Sampling from buckets...");
  let mut rng = StdRng::seed_from_u64(42 + image_num as u64); // Different seed per image
  let mut sampled_pixels = Vec::new();

  for (_bucket, pixels) in buckets.iter() {
    let sample_count = pixels.len().min(max_samples_per_bucket);
    
    if pixels.len() <= max_samples_per_bucket {
      // Take all pixels if bucket is small
      for &(y, x) in pixels {
        sampled_pixels.push((y, x));
      }
    } else {
      // Random sampling without replacement
      let mut indices: Vec<usize> = (0..pixels.len()).collect();
      for i in 0..sample_count {
        let j = rng.gen_range(i..pixels.len());
        indices.swap(i, j);
        let (y, x) = pixels[indices[i]];
        sampled_pixels.push((y, x));
      }
    }
  }

  println!("   Sampled pixels: {}", sampled_pixels.len());
  println!("   Sampling ratio: {:.2}%", 
    (sampled_pixels.len() as f64 / total_pixels as f64) * 100.0);

  // Step 4: Extract pixel data and save to CSV
  println!("\n💾 Writing samples to CSV...");
  let mut csv_file = File::create(&output_csv)?;
  writeln!(csv_file, "index,sr,sg,sb,cr,cg,cb,dr,dg,db")?;

  for (idx, &(y, x)) in sampled_pixels.iter().enumerate() {
    // Standard image pixel
    let s_pixel = standard.at_2d::<core::Vec3b>(y as i32, x as i32)?;
    let sb = s_pixel[0] as f32 / 255.0;
    let sg = s_pixel[1] as f32 / 255.0;
    let sr = s_pixel[2] as f32 / 255.0;

    // Chrome image pixel
    let c_pixel = chrome.at_2d::<core::Vec3b>(y as i32, x as i32)?;
    let cb = c_pixel[0] as f32 / 255.0;
    let cg = c_pixel[1] as f32 / 255.0;
    let cr = c_pixel[2] as f32 / 255.0;

    // Difference
    let dr = cr - sr;
    let dg = cg - sg;
    let db = cb - sb;

    writeln!(
      csv_file,
      "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
      idx, sr, sg, sb, cr, cg, cb, dr, dg, db
    )?;
  }

  println!("✅ CSV saved to: {}", output_csv);

  // Step 5: Statistics
  println!("\n📊 Bucket Statistics:");
  let bucket_sizes: Vec<usize> = buckets.values().map(|v| v.len()).collect();
  let min_bucket = bucket_sizes.iter().min().unwrap_or(&0);
  let max_bucket = bucket_sizes.iter().max().unwrap_or(&0);
  let avg_bucket = if !bucket_sizes.is_empty() {
    bucket_sizes.iter().sum::<usize>() / bucket_sizes.len()
  } else {
    0
  };

  println!("   Min pixels per bucket: {}", min_bucket);
  println!("   Max pixels per bucket: {}", max_bucket);
  println!("   Avg pixels per bucket: {}", avg_bucket);

  println!("\n🎉 Done!");

  Ok(())
}
