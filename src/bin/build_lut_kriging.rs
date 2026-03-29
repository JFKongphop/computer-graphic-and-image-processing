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

const N: usize = 33;

/// Exponential variogram model for Kriging
/// gamma(h) = nugget + sill * (1 - exp(-h/range))
fn variogram(distance: f32, nugget: f32, sill: f32, range: f32) -> f32 {
  if distance == 0.0 {
    0.0
  } else {
    nugget + sill * (1.0 - (-distance / range).exp())
  }
}

/// Ordinary Kriging interpolation
/// Returns interpolated RGB values for an empty cell
fn kriging_interpolate(
  target_pos: (usize, usize, usize),
  neighbors: &[(usize, usize, usize, [f32; 3])],
  nugget: f32,
  sill: f32,
  range: f32,
) -> [f32; 3] {
  let n = neighbors.len();
  
  if n == 0 {
    // Fallback: identity mapping
    return [
      target_pos.0 as f32 / (N - 1) as f32,
      target_pos.1 as f32 / (N - 1) as f32,
      target_pos.2 as f32 / (N - 1) as f32,
    ];
  }
  
  if n == 1 {
    // Only one neighbor, use its value
    return neighbors[0].3;
  }
  
  // Build Kriging system: K * w = k
  // K is (n+1)x(n+1) covariance matrix
  // w is (n+1)x1 weights vector (including Lagrange multiplier)
  // k is (n+1)x1 covariance vector
  
  let mut k_matrix = vec![vec![0.0f32; n + 1]; n + 1];
  let mut k_vector = vec![0.0f32; n + 1];
  
  // Compute distance between target and each neighbor
  let target_i = target_pos.0 as f32;
  let target_j = target_pos.1 as f32;
  let target_k = target_pos.2 as f32;
  
  // Fill K matrix (covariances between neighbors)
  for i in 0..n {
    for j in 0..n {
      let dist_ij = (
        (neighbors[i].0 as f32 - neighbors[j].0 as f32).powi(2)
        + (neighbors[i].1 as f32 - neighbors[j].1 as f32).powi(2)
        + (neighbors[i].2 as f32 - neighbors[j].2 as f32).powi(2)
      ).sqrt();
      
      // Covariance = sill - variogram
      k_matrix[i][j] = sill - variogram(dist_ij, nugget, sill, range);
    }
    k_matrix[i][n] = 1.0; // Lagrange constraint
    k_matrix[n][i] = 1.0;
  }
  k_matrix[n][n] = 0.0;
  
  // Fill k vector (covariances between target and neighbors)
  for i in 0..n {
    let dist = (
      (target_i - neighbors[i].0 as f32).powi(2)
      + (target_j - neighbors[i].1 as f32).powi(2)
      + (target_k - neighbors[i].2 as f32).powi(2)
    ).sqrt();
    
    k_vector[i] = sill - variogram(dist, nugget, sill, range);
  }
  k_vector[n] = 1.0; // Lagrange constraint
  
  // Solve K * w = k using Gaussian elimination (simplified)
  let weights = solve_linear_system(&k_matrix, &k_vector);
  
  // Apply weights to compute interpolated values
  let mut result = [0.0f32; 3];
  for i in 0..n {
    result[0] += weights[i] * neighbors[i].3[0];
    result[1] += weights[i] * neighbors[i].3[1];
    result[2] += weights[i] * neighbors[i].3[2];
  }
  
  result
}

/// Solve linear system Ax = b using Gaussian elimination with partial pivoting
fn solve_linear_system(a: &[Vec<f32>], b: &[f32]) -> Vec<f32> {
  let n = b.len();
  let mut aug = vec![vec![0.0f32; n + 1]; n];
  
  // Create augmented matrix [A|b]
  for i in 0..n {
    for j in 0..n {
      aug[i][j] = a[i][j];
    }
    aug[i][n] = b[i];
  }
  
  // Forward elimination with partial pivoting
  for k in 0..n {
    // Find pivot
    let mut max_row = k;
    let mut max_val = aug[k][k].abs();
    for i in (k + 1)..n {
      if aug[i][k].abs() > max_val {
        max_val = aug[i][k].abs();
        max_row = i;
      }
    }
    
    // Swap rows
    if max_row != k {
      aug.swap(k, max_row);
    }
    
    // Check for singular matrix
    if aug[k][k].abs() < 1e-10 {
      continue;
    }
    
    // Eliminate column
    for i in (k + 1)..n {
      let factor = aug[i][k] / aug[k][k];
      for j in k..=n {
        aug[i][j] -= factor * aug[k][j];
      }
    }
  }
  
  // Back substitution
  let mut x = vec![0.0f32; n];
  for i in (0..n).rev() {
    let mut sum = aug[i][n];
    for j in (i + 1)..n {
      sum -= aug[i][j] * x[j];
    }
    x[i] = if aug[i][i].abs() > 1e-10 {
      sum / aug[i][i]
    } else {
      0.0
    };
  }
  
  x
}

fn main() -> Result<()> {
  println!("🎨 Building 3D LUT from CSV data (Kriging Interpolation)");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  // Step 1: Initialize LUT and COUNT arrays
  let mut lut = vec![vec![vec![[0.0f32; 3]; N]; N]; N];
  let mut count = vec![vec![vec![0u32; N]; N]; N];

  // Step 2: Read CSV and accumulate values
  println!("📖 Reading CSV file...");
  let file = File::open("outputs/pixel_comparison.csv")?;
  let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

  let mut row_count = 0;

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

  // Step 4: Fill empty cells using Ordinary Kriging interpolation
  if empty_cells > 0 {
    println!("\n🔧 Filling empty cells with Ordinary Kriging interpolation...");
    println!("   Variogram parameters:");
    
    // Variogram parameters (tuned for color space)
    let nugget = 0.01;  // Small nugget effect
    let sill = 0.1;     // Total variance
    let range = 5.0;    // Correlation range in LUT space
    
    println!("   - Nugget: {}", nugget);
    println!("   - Sill: {}", sill);
    println!("   - Range: {}", range);
    
    let mut filled_count = 0;
    let max_neighbors = 20; // Use up to 20 nearest neighbors for Kriging
    
    for i in 0..N {
      for j in 0..N {
        for k in 0..N {
          // Skip cells that already have data
          if count[i][j][k] > 0 {
            continue;
          }
          
          // Find nearest filled neighbors within search radius
          let mut neighbors = Vec::new();
          
          for radius in 1..=N {
            // Search all cells within this radius
            for di in -(radius as i32)..=(radius as i32) {
              for dj in -(radius as i32)..=(radius as i32) {
                for dk in -(radius as i32)..=(radius as i32) {
                  let ni = i as i32 + di;
                  let nj = j as i32 + dj;
                  let nk = k as i32 + dk;
                  
                  // Skip out-of-bounds
                  if ni < 0 || nj < 0 || nk < 0 || ni >= N as i32 || nj >= N as i32 || nk >= N as i32 {
                    continue;
                  }
                  
                  let ni = ni as usize;
                  let nj = nj as usize;
                  let nk = nk as usize;
                  
                  // Skip empty neighbors or self
                  if count[ni][nj][nk] == 0 || (ni == i && nj == j && nk == k) {
                    continue;
                  }
                  
                  neighbors.push((ni, nj, nk, lut[ni][nj][nk]));
                }
              }
            }
            
            // Stop when we have enough neighbors
            if neighbors.len() >= max_neighbors {
              break;
            }
          }
          
          // Limit to max_neighbors for efficiency
          if neighbors.len() > max_neighbors {
            neighbors.truncate(max_neighbors);
          }
          
          // Apply Kriging interpolation
          let interpolated = kriging_interpolate((i, j, k), &neighbors, nugget, sill, range);
          lut[i][j][k] = interpolated;
          filled_count += 1;
        }
      }
    }
    
    println!("   ✅ Filled {} empty cells", filled_count);
  }
  
  // Final LUT composition statistics
  println!("\n📊 Final LUT Composition:");
  let cells_from_data = filled_cells;
  let cells_interpolated = if empty_cells > 0 { empty_cells } else { 0 };
  
  println!("   From training data: {} ({:.2}%)", 
    cells_from_data, 
    (cells_from_data as f64 / total_cells as f64) * 100.0);
  println!("   From Kriging:       {} ({:.2}%)", 
    cells_interpolated, 
    (cells_interpolated as f64 / total_cells as f64) * 100.0);
  println!("   ─────────────────────────────────");
  println!("   Total completion:   {} (100.00%)", total_cells);

  // Save LUT to file
  println!("\n💾 Writing LUT to file...");
  let output_file = File::create("outputs/lut_33_kriging.cube")?;
  write_cube_file(output_file, &lut)?;

  println!("✅ LUT saved to: outputs/lut_33_kriging.cube");

  // Show some sample LUT values
  println!("\n🔍 Sample LUT values:");
  println!("   Black [0,0,0] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    lut[0][0][0][0], lut[0][0][0][1], lut[0][0][0][2], count[0][0][0]);
  println!("   White [{},{},{}] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    N-1, N-1, N-1, lut[N-1][N-1][N-1][0], lut[N-1][N-1][N-1][1], lut[N-1][N-1][N-1][2], count[N-1][N-1][N-1]);
  println!("   Mid [{},{},{}] -> [{:.4}, {:.4}, {:.4}] (count: {})",
    N/2, N/2, N/2, lut[N/2][N/2][N/2][0], lut[N/2][N/2][N/2][1], lut[N/2][N/2][N/2][2], count[N/2][N/2][N/2]);

  Ok(())
}

/// Write LUT in .cube format
fn write_cube_file(mut file: File, lut: &Vec<Vec<Vec<[f32; 3]>>>) -> Result<()> {
  use std::io::Write;

  // Write header
  writeln!(file, "# 3D LUT for Classic Chrome Film Simulation (Kriging)")?;
  writeln!(file, "# Generated from pixel comparison data")?;
  writeln!(file, "TITLE \"Classic Chrome LUT - Kriging\"")?;
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
