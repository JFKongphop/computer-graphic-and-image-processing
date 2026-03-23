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
  println!("🚀 Starting parallel processing of 8 image pairs...");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Process all image pairs in parallel
  let results: Vec<Vec<PixelData>> = (1..=8)
    .into_par_iter()
    .map(|img_num| {
      process_image_pair(img_num)
    })
    .collect();

  // Flatten all results into one vector
  let all_pixel_data: Vec<PixelData> = results.into_iter().flatten().collect();

  // Write all pixel data to CSV file
  println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
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

fn process_image_pair(img_num: i32) -> Vec<PixelData> {
  let filename = format!("{}.JPG", img_num);
  let standard_path = format!("source/compare/standard/{}", filename);
  let chrome_path = format!("source/compare/classic-chrome/{}", filename);
  
  println!("📸 Processing image pair: {}", filename);
  
  // Load two images
  let img1_mat = match imgcodecs::imread(&standard_path, imgcodecs::IMREAD_COLOR) {
    Ok(mat) => mat,
    Err(e) => {
      eprintln!("❌ Error loading {}: {}", standard_path, e);
      return Vec::new();
    }
  };
  
  let img2_mat = match imgcodecs::imread(&chrome_path, imgcodecs::IMREAD_COLOR) {
    Ok(mat) => mat,
    Err(e) => {
      eprintln!("❌ Error loading {}: {}", chrome_path, e);
      return Vec::new();
    }
  };

  // Convert to BasedImage for pixel access
  let img1 = BasedImage::from_mat(&img1_mat);
  let img2 = BasedImage::from_mat(&img2_mat);

  // Check if images have the same dimensions
  if img1.w != img2.w || img1.h != img2.h {
    eprintln!(
      "❌ Images have different dimensions: {}x{} vs {}x{}",
      img1.w, img1.h, img2.w, img2.h
    );
    return Vec::new();
  }

  println!("   Image {}: {}x{} - Sampling pixels...", img_num, img1.w, img1.h);

  // Generate random sample of 10000 pixels (or less if image is smaller)
  let total_pixels = img1.w * img1.h;
  let sample_size = 10000.min(total_pixels);
  
  // Create array of all pixel indices
  let mut pixel_indices: Vec<usize> = (0..total_pixels).collect();
  
  // Randomly shuffle and take first 10000
  let mut rng = thread_rng();
  pixel_indices.shuffle(&mut rng);
  let sampled_indices = &pixel_indices[..sample_size];

  let mut pixel_data_vec: Vec<PixelData> = Vec::new();
  let mut diff_count = 0;
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
    pixel_data_vec.push(PixelData {
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
    }
  }

  println!("   Image {}: {} different pixels ({:.2}%)", 
    img_num, diff_count, (diff_count as f64 / sample_size as f64) * 100.0);
  
  pixel_data_vec
}
