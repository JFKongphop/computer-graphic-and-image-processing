/// Film grain effect implementation (Fujifilm-style)
/// Wraps BasedImage and provides grain application methods
use super::image::BasedImage;
use rand_distr::{Distribution, Normal};

/// Grain intensity levels (Fujifilm-style)
#[derive(Debug, Clone, Copy)]
pub enum GrainIntensity {
  Weak,
  Strong,
}

/// Grain size levels (Fujifilm-style)
#[derive(Debug, Clone, Copy)]
pub enum GrainSize {
  Small,
  Large,
}

/// Grain struct - extends BasedImage with film grain capabilities
#[derive(Clone)]
pub struct Grain {
  pub base: BasedImage,
}

impl Grain {
  /// Constructor - creates Grain from BasedImage
  pub fn new(base: BasedImage) -> Self {
    Self { base }
  }

  /// Create Grain directly from OpenCV Mat
  pub fn from_mat(mat: &opencv::core::Mat) -> Self {
    Self::new(BasedImage::from_mat(mat))
  }

  /// Convert back to OpenCV Mat
  pub fn to_mat(&self) -> opencv::core::Mat {
    self.base.to_mat()
  }

  /// Clone the base image
  pub fn clone_base(&self) -> BasedImage {
    self.base.clone()
  }

  /// Calculate luminance-based grain weight (Fujifilm-style curve)
  ///
  /// Formula: More grain in midtones/shadows, less in highlights
  /// - Deep shadows: 0.3 (minimal)
  /// - Midtones: 1.0 (maximum)
  /// - Highlights: 0.2 (minimal)
  ///
  /// # Arguments
  /// * `luminance` - Normalized luminance value (0.0 to 1.0)
  fn luminance_weight(luminance: f32) -> f32 {
    match luminance {
      l if l < 0.05 => 0.3, // Deep shadows
      l if l >= 0.05 && l <= 0.20 => {
        // Shadows to midtones: ramp up
        0.3 + (l - 0.05) / 0.15 * 0.7
      }
      l if l > 0.20 && l <= 0.80 => 1.0, // Midtones: maximum grain
      l if l > 0.80 && l <= 0.95 => {
        // Highlights: ramp down
        1.0 - (l - 0.80) / 0.15 * 0.8
      }
      _ => 0.2, // Bright highlights
    }
  }

  /// Apply film grain effect (Good Enough method: Gaussian noise + luminance mask)
  ///
  /// Algorithm:
  /// 1. Generate Gaussian noise
  /// 2. Apply Gaussian blur (for grain size)
  /// 3. Calculate luminance mask
  /// 4. Blend with luminance-weighted intensity
  ///
  /// # Arguments
  /// * `intensity` - Grain intensity (Weak or Strong)
  /// * `size` - Grain size (Small or Large)
  ///
  /// # Recommended Values
  /// * **Small + Weak**: Subtle fine grain (Provia, Velvia style)
  /// * **Small + Strong**: Pronounced fine grain (pushed film)
  /// * **Large + Weak**: Subtle chunky grain (Classic Chrome)
  /// * **Large + Strong**: Heavy chunky grain (high ISO film)
  ///
  /// # Example
  /// ```
  /// let mut img = Grain::from_mat(&mat);
  /// img.apply_grain(GrainIntensity::Weak, GrainSize::Small);
  /// ```
  pub fn apply_grain(
    &mut self,
    intensity: GrainIntensity,
    size: GrainSize,
  ) -> Result<(), opencv::Error> {
    let width = self.base.w;
    let height = self.base.h;

    // Parameters based on intensity and size
    let (base_intensity, blur_radius) = match (intensity, size) {
      (GrainIntensity::Weak, GrainSize::Small) => (0.10, 1),
      (GrainIntensity::Weak, GrainSize::Large) => (0.08, 3),
      (GrainIntensity::Strong, GrainSize::Small) => (0.30, 1),
      (GrainIntensity::Strong, GrainSize::Large) => (0.25, 3),
    };

    // Step 1: Generate Gaussian noise
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut noise: Vec<f32> = (0..width * height)
      .map(|_| normal.sample(&mut rng))
      .collect();

    // Step 2: Simple box blur for grain clumping (instead of Gaussian blur)
    // This creates a more efficient approximation
    if blur_radius > 0 {
      let mut blurred = vec![0.0f32; width * height];
      for y in 0..height {
        for x in 0..width {
          let mut sum = 0.0;
          let mut count = 0;

          // Box blur kernel
          for dy in -(blur_radius as i32)..=(blur_radius as i32) {
            for dx in -(blur_radius as i32)..=(blur_radius as i32) {
              let ny = (y as i32 + dy).max(0).min((height - 1) as i32) as usize;
              let nx = (x as i32 + dx).max(0).min((width - 1) as i32) as usize;
              sum += noise[ny * width + nx];
              count += 1;
            }
          }
          blurred[y * width + x] = sum / count as f32;
        }
      }
      noise = blurred;
    }

    // Step 3 & 4: Apply to each pixel with luminance weighting
    for y in 0..height {
      for x in 0..width {
        let idx = (y * width + x) * 3;
        let noise_idx = y * width + x;

        // Get pixel values
        let b = self.base.data[idx] as f32;
        let g = self.base.data[idx + 1] as f32;
        let r = self.base.data[idx + 2] as f32;

        // Calculate luminance (ITU-R BT.709)
        let luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;

        // Get luminance weight
        let weight = Self::luminance_weight(luminance);

        // Get blurred noise value
        let noise_val = noise[noise_idx];

        // Calculate final grain strength
        let grain_strength = base_intensity * weight * noise_val * 255.0;

        // Apply grain to each channel
        self.base.data[idx] = ((b + grain_strength).max(0.0).min(255.0)) as u8;
        self.base.data[idx + 1] = ((g + grain_strength).max(0.0).min(255.0)) as u8;
        self.base.data[idx + 2] = ((r + grain_strength).max(0.0).min(255.0)) as u8;
      }
    }

    Ok(())
  }

  /// Apply grain with custom parameters (advanced control)
  ///
  /// # Arguments
  /// * `intensity` - Base grain intensity (0.0 to 1.0, typical: 0.05-0.40)
  /// * `blur_radius` - Box blur radius for grain size (0-5 pixels)
  ///
  /// # Example
  /// ```
  /// let mut img = Grain::from_mat(&mat);
  /// img.apply_grain_custom(0.15, 2)?;  // Custom medium grain
  /// ```
  pub fn apply_grain_custom(
    &mut self,
    base_intensity: f32,
    blur_radius: i32,
  ) -> Result<(), opencv::Error> {
    let width = self.base.w;
    let height = self.base.h;

    // Generate Gaussian noise
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut noise: Vec<f32> = (0..width * height)
      .map(|_| normal.sample(&mut rng))
      .collect();

    // Apply box blur to noise
    if blur_radius > 0 {
      let mut blurred = vec![0.0f32; width * height];
      for y in 0..height {
        for x in 0..width {
          let mut sum = 0.0;
          let mut count = 0;

          // Box blur kernel
          for dy in -blur_radius..=blur_radius {
            for dx in -blur_radius..=blur_radius {
              let ny = (y as i32 + dy).max(0).min((height - 1) as i32) as usize;
              let nx = (x as i32 + dx).max(0).min((width - 1) as i32) as usize;
              sum += noise[ny * width + nx];
              count += 1;
            }
          }
          blurred[y * width + x] = sum / count as f32;
        }
      }
      noise = blurred;
    }

    // Apply to each pixel with luminance weighting
    for y in 0..height {
      for x in 0..width {
        let idx = (y * width + x) * 3;
        let noise_idx = y * width + x;

        let b = self.base.data[idx] as f32;
        let g = self.base.data[idx + 1] as f32;
        let r = self.base.data[idx + 2] as f32;

        let luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;
        let weight = Self::luminance_weight(luminance);

        let noise_val = noise[noise_idx];
        let grain_strength = base_intensity * weight * noise_val * 255.0;

        self.base.data[idx] = ((b + grain_strength).max(0.0).min(255.0)) as u8;
        self.base.data[idx + 1] = ((g + grain_strength).max(0.0).min(255.0)) as u8;
        self.base.data[idx + 2] = ((r + grain_strength).max(0.0).min(255.0)) as u8;
      }
    }

    Ok(())
  }
}
