use anyhow::Result;
use opencv::prelude::*;
use opencv::{core, imgproc};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Load a 3D LUT from .cube file
fn load_cube_file(path: &str) -> Result<(usize, Vec<Vec<Vec<[f32; 3]>>>)> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  let mut size = 0;
  let mut rgb_values = Vec::new();

  for line in reader.lines() {
    let line = line?;
    let line = line.trim();

    if line.is_empty() || line.starts_with('#') {
      continue;
    }

    if line.starts_with("LUT_3D_SIZE") {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 2 {
        size = parts[1].parse()?;
      }
      continue;
    }

    if line.starts_with("TITLE") || line.starts_with("DOMAIN_") {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
      let r: f32 = parts[0].parse()?;
      let g: f32 = parts[1].parse()?;
      let b: f32 = parts[2].parse()?;
      rgb_values.push([r, g, b]);
    }
  }

  if size == 0 {
    return Err(anyhow::anyhow!("LUT_3D_SIZE not found"));
  }

  if rgb_values.len() != size * size * size {
    return Err(anyhow::anyhow!(
      "Expected {} values, got {}",
      size * size * size,
      rgb_values.len()
    ));
  }

  // Reshape into 3D array
  let mut lut = vec![vec![vec![[0.0f32; 3]; size]; size]; size];
  let mut idx = 0;
  for r in 0..size {
    for g in 0..size {
      for b in 0..size {
        lut[r][g][b] = rgb_values[idx];
        idx += 1;
      }
    }
  }

  Ok((size, lut))
}

/// Apply brightness bias correction in LAB space
fn apply_brightness_correction(
  lut: &mut Vec<Vec<Vec<[f32; 3]>>>,
  bias_correction: f32,
) -> Result<()> {
  let n = lut.len();
  
  println!("🔧 Applying brightness correction...");
  println!("   Correction: {:+.3} (LAB L* units, 0-100 scale)", bias_correction);
  
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        let rgb = lut[i][j][k];
        
        // Convert RGB to LAB
        let mut bgr_mat = unsafe { Mat::new_rows_cols(1, 1, core::CV_32FC3)? };
        let pixel = bgr_mat.at_2d_mut::<core::Vec3f>(0, 0)?;
        pixel[0] = rgb[2]; // B
        pixel[1] = rgb[1]; // G
        pixel[2] = rgb[0]; // R
        
        let mut lab_mat = Mat::default();
        imgproc::cvt_color(
          &bgr_mat,
          &mut lab_mat,
          imgproc::COLOR_BGR2Lab,
          0,
          core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;
        
        let lab_pixel = lab_mat.at_2d_mut::<core::Vec3f>(0, 0)?;
        
        // Apply correction to L* channel
        // OpenCV LAB: L is [0, 100], but stored as float
        lab_pixel[0] = (lab_pixel[0] - bias_correction).clamp(0.0, 100.0);
        
        // Convert back to RGB
        let mut corrected_bgr = Mat::default();
        imgproc::cvt_color(
          &lab_mat,
          &mut corrected_bgr,
          imgproc::COLOR_Lab2BGR,
          0,
          core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;
        
        let corrected_pixel = corrected_bgr.at_2d::<core::Vec3f>(0, 0)?;
        
        // Update LUT with corrected values (clamped to [0, 1])
        lut[i][j][k][0] = corrected_pixel[2].clamp(0.0, 1.0); // R
        lut[i][j][k][1] = corrected_pixel[1].clamp(0.0, 1.0); // G
        lut[i][j][k][2] = corrected_pixel[0].clamp(0.0, 1.0); // B
      }
    }
  }
  
  println!("   ✅ Correction applied to {} cells", n * n * n);
  
  Ok(())
}

/// Write LUT in .cube format
fn write_cube_file(path: &str, size: usize, lut: &Vec<Vec<Vec<[f32; 3]>>>) -> Result<()> {
  let mut file = File::create(path)?;

  writeln!(file, "# 3D LUT for Classic Chrome Film Simulation (Brightness Corrected)")?;
  writeln!(file, "# Generated with bias correction")?;
  writeln!(file, "TITLE \"Classic Chrome LUT - Bias Corrected\"")?;
  writeln!(file, "LUT_3D_SIZE {}", size)?;
  writeln!(file)?;

  for r in 0..size {
    for g in 0..size {
      for b in 0..size {
        writeln!(
          file,
          "{:.6} {:.6} {:.6}",
          lut[r][g][b][0], lut[r][g][b][1], lut[r][g][b][2]
        )?;
      }
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  println!("🔧 LUT Brightness Bias Correction");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  // Parse command line arguments
  let args: Vec<String> = std::env::args().collect();
  
  let bias_correction = if args.len() >= 2 {
    args[1].parse::<f32>().unwrap_or(1.489)
  } else {
    1.489 // Default from analyze_brightness_bias
  };
  
  let input_lut = "outputs/lut_33.cube";
  let output_lut = "outputs/lut_33_corrected.cube";

  println!("\n📖 Loading LUT from: {}", input_lut);
  let (size, mut lut) = load_cube_file(input_lut)?;
  println!("✅ Loaded {}x{}x{} LUT ({} cells)", size, size, size, size * size * size);

  // Show sample values before correction
  println!("\n🔍 Sample values BEFORE correction:");
  println!("   Black [0,0,0] -> [{:.4}, {:.4}, {:.4}]",
    lut[0][0][0][0], lut[0][0][0][1], lut[0][0][0][2]);
  println!("   White [{},{},{}] -> [{:.4}, {:.4}, {:.4}]",
    size-1, size-1, size-1,
    lut[size-1][size-1][size-1][0],
    lut[size-1][size-1][size-1][1],
    lut[size-1][size-1][size-1][2]);
  println!("   Mid [{},{},{}] -> [{:.4}, {:.4}, {:.4}]",
    size/2, size/2, size/2,
    lut[size/2][size/2][size/2][0],
    lut[size/2][size/2][size/2][1],
    lut[size/2][size/2][size/2][2]);

  println!("\n🎯 Brightness Correction Settings:");
  println!("   Target bias correction: {:+.3} LAB L* units", bias_correction);
  println!("   This will make output DARKER to match ground truth");

  // Apply correction
  apply_brightness_correction(&mut lut, bias_correction)?;

  // Show sample values after correction
  println!("\n🔍 Sample values AFTER correction:");
  println!("   Black [0,0,0] -> [{:.4}, {:.4}, {:.4}]",
    lut[0][0][0][0], lut[0][0][0][1], lut[0][0][0][2]);
  println!("   White [{},{},{}] -> [{:.4}, {:.4}, {:.4}]",
    size-1, size-1, size-1,
    lut[size-1][size-1][size-1][0],
    lut[size-1][size-1][size-1][1],
    lut[size-1][size-1][size-1][2]);
  println!("   Mid [{},{},{}] -> [{:.4}, {:.4}, {:.4}]",
    size/2, size/2, size/2,
    lut[size/2][size/2][size/2][0],
    lut[size/2][size/2][size/2][1],
    lut[size/2][size/2][size/2][2]);

  // Save corrected LUT
  println!("\n💾 Saving corrected LUT...");
  write_cube_file(output_lut, size, &lut)?;
  println!("✅ Corrected LUT saved to: {}", output_lut);

  println!("\n🎉 Brightness correction complete!");
  println!("\n📝 Next steps:");
  println!("   1. Update apply_lut.rs to use: {}", output_lut);
  println!("   2. Apply to test image: cargo run --bin apply_lut");
  println!("   3. Re-analyze bias: cargo run --bin analyze_brightness_bias");
  println!("   4. Should see bias reduced from +1.49 to ~0.0");
  println!("\n💡 Tip: You can adjust correction amount:");
  println!("   cargo run --bin correct_lut_bias -- 1.5");
  println!("   cargo run --bin correct_lut_bias -- 2.0");

  Ok(())
}
