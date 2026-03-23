use anyhow::Result;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
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

const N: usize = 17;

fn main() -> Result<()> {
  println!("🎨 Building 3D LUT from CSV data");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  // Step 1: Initialize LUT and COUNT arrays
  let mut lut = vec![vec![vec![[0.0f32; 3]; N]; N]; N];
  let mut count = vec![vec![vec![0u32; N]; N]; N];

  // Step 2: Read CSV and accumulate values
  println!("📖 Reading CSV file...");
  let file = File::open("outputs/pixel_comparison.csv")?;
  let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

  let mut row_count = 0;
  let mut skipped = 0;

  for result in rdr.deserialize() {
    let record: PixelData = result?;
    row_count += 1;

    // Convert source RGB to LUT indices
    let i = ((record.sr * (N - 1) as f32).floor() as usize).min(N - 1);
    let j = ((record.sg * (N - 1) as f32).floor() as usize).min(N - 1);
    let k = ((record.sb * (N - 1) as f32).floor() as usize).min(N - 1);

    // Accumulate target RGB values
    lut[i][j][k][0] += record.cr;
    lut[i][j][k][1] += record.cg;
    lut[i][j][k][2] += record.cb;

    count[i][j][k] += 1;
  }

  println!("✅ Processed {} rows", row_count);

  // Step 3: Average accumulated values
  println!("🧮 Computing averages...");
  let mut filled_cells = 0;
  let mut empty_cells = 0;

  for i in 0..N {
    for j in 0..N {
      for k in 0..N {
        if count[i][j][k] > 0 {
          lut[i][j][k][0] /= count[i][j][k] as f32;
          lut[i][j][k][1] /= count[i][j][k] as f32;
          lut[i][j][k][2] /= count[i][j][k] as f32;
          filled_cells += 1;
        } else {
          empty_cells += 1;
        }
      }
    }
  }

  let total_cells = N * N * N;
  println!("📊 LUT Statistics:");
  println!("   Total cells: {}", total_cells);
  println!("   Filled cells: {} ({:.2}%)", filled_cells, (filled_cells as f64 / total_cells as f64) * 100.0);
  println!("   Empty cells: {} ({:.2}%)", empty_cells, (empty_cells as f64 / total_cells as f64) * 100.0);

  // Show sample counts
  println!("\n📈 Sample distribution:");
  let mut max_samples = 0;
  let mut min_samples = u32::MAX;
  let mut total_samples = 0u64;

  for i in 0..N {
    for j in 0..N {
      for k in 0..N {
        let c = count[i][j][k];
        if c > 0 {
          max_samples = max_samples.max(c);
          min_samples = min_samples.min(c);
          total_samples += c as u64;
        }
      }
    }
  }

  if filled_cells > 0 {
    println!("   Max samples per cell: {}", max_samples);
    println!("   Min samples per cell: {}", min_samples);
    println!("   Avg samples per filled cell: {:.2}", total_samples as f64 / filled_cells as f64);
  }

  // Save LUT to file
  println!("\n💾 Writing LUT to file...");
  let output_file = File::create("outputs/lut_17.cube")?;
  write_cube_file(output_file, &lut)?;

  println!("✅ LUT saved to: outputs/lut_17.cube");

  // Show some sample LUT values
  println!("\n🔍 Sample LUT values:");
  println!("   Black [0,0,0] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    lut[0][0][0][0], lut[0][0][0][1], lut[0][0][0][2], count[0][0][0]);
  println!("   White [16,16,16] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    lut[16][16][16][0], lut[16][16][16][1], lut[16][16][16][2], count[16][16][16]);
  println!("   Mid [8,8,8] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    lut[8][8][8][0], lut[8][8][8][1], lut[8][8][8][2], count[8][8][8]);

  Ok(())
}

/// Write LUT in .cube format
fn write_cube_file(mut file: File, lut: &Vec<Vec<Vec<[f32; 3]>>>) -> Result<()> {
  use std::io::Write;

  // Write header
  writeln!(file, "# 3D LUT for Classic Chrome Film Simulation")?;
  writeln!(file, "# Generated from pixel comparison data")?;
  writeln!(file, "TITLE \"Classic Chrome LUT\"")?;
  writeln!(file, "LUT_3D_SIZE {}", N)?;
  writeln!(file)?;

  // Write LUT data in BGR order (Blue changes fastest)
  for r in 0..N {
    for g in 0..N {
      for b in 0..N {
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
