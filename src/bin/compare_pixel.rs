use computer_graphic_and_vision::utils::BasedImage;
use opencv::imgcodecs;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use serde::Serialize;
use std::fs::File;
use anyhow::Result;

#[derive(Serialize)]
struct PixelData {
  index: usize,
  sr: f32,
  sg: f32,
  sb: f32,
  cr: f32,
  cg: f32,
  cb: f32,
  dr: f32,
  dg: f32,
  db: f32,
}

fn main() -> Result<()> {
  let mut all_pixel_data: Vec<PixelData> = Vec::new();
  
  // Process all image pairs (1.JPG through 8.JPG)
  for img_num in 1..=8 {
    let filename = format!("{}.JPG", img_num);
    let standard_path = format!("source/compare/standard/{}", filename);
    let chrome_path = format!("source/compare/classic-chrome/{}", filename);
    
    println!("📸 Processing image pair: {}", filename);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Load two images
    let img1_mat = imgcodecs::imread(&standard_path, imgcodecs::IMREAD_COLOR)?;
    let img2_mat = imgcodecs::imread(&chrome_path, imgcodecs::IMREAD_COLOR)?;

    // Convert to BasedImage for pixel access
    let img1 = BasedImage::from_mat(&img1_mat);
    let img2 = BasedImage::from_mat(&img2_mat);

    // Check if images have the same dimensions
    if img1.w != img2.w || img1.h != img2.h {
      println!(
        "❌ Images have different dimensions: {}x{} vs {}x{}",
        img1.w, img1.h, img2.w, img2.h
      );
      continue;
    }

    println!("   Image size: {}x{}", img1.w, img1.h);

    // Generate random sample of 10000 pixels (or less if image is smaller)
    let total_pixels = img1.w * img1.h;
    let sample_size = 40000.min(total_pixels);
    
    // Create array of all pixel indices
    let mut pixel_indices: Vec<usize> = (0..total_pixels).collect();
    
    // Randomly shuffle and take first 10000
    let mut rng = thread_rng();
    pixel_indices.shuffle(&mut rng);
    let sampled_indices = &pixel_indices[..sample_size];

    println!("   Sampling {} pixels from {} total", sample_size, total_pixels);

    let mut diff_count = 0;
    let mut max_diff_r = 0i32;
    let mut max_diff_g = 0i32;
    let mut max_diff_b = 0i32;
    let mut total_diff_r = 0i64;
    let mut total_diff_g = 0i64;
    let mut total_diff_b = 0i64;

    // Compare only the sampled pixels
    for &pixel_idx in sampled_indices {
      let idx = pixel_idx * 3; // Convert pixel index to byte index (3 bytes per pixel)

      // Get BGR values (OpenCV uses BGR format)
      let b1 = img1.data[idx];
      let g1 = img1.data[idx + 1];
      let r1 = img1.data[idx + 2];

      let b2 = img2.data[idx];
      let g2 = img2.data[idx + 1];
      let r2 = img2.data[idx + 2];

      let sr = r1 as f32 / 255.0;
      let sg = g1 as f32 / 255.0;
      let sb = b1 as f32 / 255.0;
      let cr = r2 as f32 / 255.0;
      let cg = g2 as f32 / 255.0;
      let cb = b2 as f32 / 255.0;
      let dr = sr - cr;
      let dg = sg - cg;
      let db = sb - cb;

      // Store pixel data normalized to 0-1 range
      all_pixel_data.push(PixelData {
        index: pixel_idx,
        sr,
        sg,
        sb,
        cr,
        cg,
        cb,
        dr,
        dg,
        db
      });

      // Compare RGB values
      if r1 != r2 || g1 != g2 || b1 != b2 {
        diff_count += 1;

        // Calculate absolute differences
        let diff_r = (r1 as i32 - r2 as i32).abs();
        let diff_g = (g1 as i32 - g2 as i32).abs();
        let diff_b = (b1 as i32 - b2 as i32).abs();

        total_diff_r += diff_r as i64;
        total_diff_g += diff_g as i64;
        total_diff_b += diff_b as i64;

        max_diff_r = max_diff_r.max(diff_r);
        max_diff_g = max_diff_g.max(diff_g);
        max_diff_b = max_diff_b.max(diff_b);
      }
    }

    println!("   Different pixels: {} ({:.2}%)", diff_count, (diff_count as f64 / sample_size as f64) * 100.0);
    if diff_count > 0 {
      println!("   Avg difference: R={:.2} G={:.2} B={:.2}",
        total_diff_r as f64 / diff_count as f64,
        total_diff_g as f64 / diff_count as f64,
        total_diff_b as f64 / diff_count as f64
      );
    }
    println!();
  } // End of image pair loop

  // Write all pixel data to CSV file
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  println!("📈 Combined Results:");
  println!("   Total samples collected: {}", all_pixel_data.len());
  
  println!("\n📝 Writing pixel data to CSV...");
  let file = File::create("outputs/pixel_comparison.csv")?;
  let mut wtr = csv::Writer::from_writer(file);

  // Write all pixel data
  for pixel_data in &all_pixel_data {
    wtr.serialize(pixel_data)?;
  }

  wtr.flush()?;
  println!("✅ CSV file saved: outputs/pixel_comparison.csv");
  println!("   Total records: {}", all_pixel_data.len());

  Ok(())
}
