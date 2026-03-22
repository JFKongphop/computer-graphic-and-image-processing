/// Tone mapping and color adjustment operations (like "Animal extends Leg")
/// Wraps BasedImage and provides tone/color adjustment methods
use super::image::BasedImage;

/// Tone struct - extends BasedImage with tone adjustment capabilities
/// Like "Animal extends Leg" in TypeScript
#[derive(Clone)]
pub struct Tone {
  pub base: BasedImage, // The "parent class" - like super() in TypeScript
}

impl Tone {
  /// Constructor - creates Tone from BasedImage (like calling super())
  pub fn new(base: BasedImage) -> Self {
    Self { base }
  }

  /// Create Tone directly from OpenCV Mat
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

  /// Adjust image exposure (brightness in stops)
  ///
  /// Formula: V_new = V_old × 2^exposure
  ///
  /// # Arguments
  /// * `exposure` - Exposure adjustment in stops (-3.0 to +3.0)
  ///   - Positive values = brighter
  ///   - Negative values = darker
  ///   - +1.0 = twice as bright (one stop)
  ///   - -1.0 = half as bright (one stop darker)
  ///
  /// # Recommended Values
  /// * **Brighten**: 0.5 to 2.0 (subtle to strong brightening)
  ///   - 0.5 = gentle brighten (~1.4× brighter)
  ///   - 1.0 = moderate brighten (2× brighter)
  ///   - 2.0 = strong brighten (4× brighter)
  /// * **Darken**: -0.5 to -2.0 (subtle to strong darkening)
  ///   - -0.5 = gentle darken (~0.7× darker)
  ///   - -1.0 = moderate darken (0.5× darker)
  ///   - -2.0 = strong darken (0.25× darker)
  /// * **Extreme**: ±3.0 (8× brighter or 0.125× darker) - may clip highlights/shadows
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_exposure(1.0);  // Brighten by one stop (2×)
  /// ```
  pub fn adjust_exposure(&mut self, exposure: f32) {
    // Calculate multiplier: 2^exposure
    let multiplier = 2.0_f32.powf(exposure);

    // Apply to each pixel
    for i in (0..self.base.data.len()).step_by(3) {
      let b = self.base.data[i] as f32;
      let g = self.base.data[i + 1] as f32;
      let r = self.base.data[i + 2] as f32;

      // Multiply by exposure multiplier and clamp to [0, 255]
      self.base.data[i] = (b * multiplier).min(255.0).max(0.0) as u8;
      self.base.data[i + 1] = (g * multiplier).min(255.0).max(0.0) as u8;
      self.base.data[i + 2] = (r * multiplier).min(255.0).max(0.0) as u8;
    }
  }

  /// Adjust exposure with gamma correction (more natural results)
  ///
  /// Formula:
  /// 1. sRGB → Linear: V_linear = (V_sRGB / 255)^2.2
  /// 2. Apply exposure: V_exposed = V_linear × 2^exposure
  /// 3. Linear → sRGB: V_sRGB = (V_exposed^(1/2.2)) × 255
  ///
  /// # Arguments
  /// * `exposure` - Exposure adjustment in stops (-3.0 to +3.0)
  ///
  /// # Recommended Values (More natural than linear)
  /// * **Brighten**: 0.3 to 1.5 (subtle to strong natural brightening)
  ///   - 0.3 = very gentle brighten (preserves midtones)
  ///   - 0.7 = moderate brighten (photographic look)
  ///   - 1.5 = strong brighten (HDR-like effect)
  /// * **Darken**: -0.3 to -1.5 (subtle to strong natural darkening)
  ///   - -0.3 = gentle darken (preserves details)
  ///   - -0.7 = moderate darken (moody look)
  ///   - -1.5 = strong darken (dramatic shadows)
  /// * **Note**: Gamma correction gives smoother transitions than linear adjustment
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_exposure_gamma(0.5);  // Brighten naturally
  /// ```
  pub fn adjust_exposure_gamma(&mut self, exposure: f32) {
    let multiplier = 2.0_f32.powf(exposure);
    let gamma = 2.2_f32;
    let inv_gamma = 1.0 / gamma;

    for i in (0..self.base.data.len()).step_by(3) {
      // Convert to linear space (remove gamma)
      let b = (self.base.data[i] as f32 / 255.0).powf(gamma);
      let g = (self.base.data[i + 1] as f32 / 255.0).powf(gamma);
      let r = (self.base.data[i + 2] as f32 / 255.0).powf(gamma);

      // Apply exposure in linear space
      let b_linear = (b * multiplier).min(1.0).max(0.0);
      let g_linear = (g * multiplier).min(1.0).max(0.0);
      let r_linear = (r * multiplier).min(1.0).max(0.0);

      // Convert back to sRGB (apply gamma)
      self.base.data[i] = (b_linear.powf(inv_gamma) * 255.0) as u8;
      self.base.data[i + 1] = (g_linear.powf(inv_gamma) * 255.0) as u8;
      self.base.data[i + 2] = (r_linear.powf(inv_gamma) * 255.0) as u8;
    }
  }

  /// Adjust exposure with highlight protection (like Lightroom)
  ///
  /// Formula:
  /// - If value ≤ threshold: V_new = V_old × 2^exposure
  /// - If value > threshold: Soft compression using (excess / (1 + excess))
  ///
  /// # Arguments
  /// * `exposure` - Exposure adjustment in stops (-3.0 to +3.0)
  /// * `highlights_protect` - Threshold for highlight protection (0.0 to 1.0)
  ///   - 0.8 = protect top 20% of brightness range (typical)
  ///   - 0.7 = protect top 30% (more aggressive)
  ///   - 0.9 = protect top 10% (minimal protection)
  ///
  /// # Recommended Values (Best for high contrast scenes)
  /// * **Brighten**: 0.5 to 2.5 with highlights_protect=0.8
  ///   - 0.5 = gentle brighten (safe for all images)
  ///   - 1.0 = moderate brighten (good for underexposed)
  ///   - 2.0 = strong brighten (recovers dark images)
  ///   - 2.5 = extreme brighten (with highlight protection)
  /// * **Darken**: -0.5 to -1.5 with highlights_protect=0.8
  ///   - -0.5 = gentle darken (subtle mood)
  ///   - -1.0 = moderate darken (dramatic look)
  ///   - -1.5 = strong darken (low-key style)
  /// * **Best use**: Brightening high-contrast images without blowing out highlights
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_exposure_smooth(1.5, 0.8);  // Brighten but protect highlights
  /// ```
  pub fn adjust_exposure_smooth(&mut self, exposure: f32, highlights_protect: f32) {
    let multiplier = 2.0_f32.powf(exposure);
    let threshold = highlights_protect.clamp(0.0, 1.0);

    for i in (0..self.base.data.len()).step_by(3) {
      for channel in 0..3 {
        let val = self.base.data[i + channel] as f32 / 255.0;

        // Apply exposure
        let mut adjusted = val * multiplier;

        // Protect highlights (compress values near 1.0)
        if adjusted > threshold {
          let excess = adjusted - threshold;
          let compressed = excess / (1.0 + excess); // Smooth compression
          adjusted = threshold + compressed * (1.0 - threshold);
        }

        self.base.data[i + channel] = (adjusted.clamp(0.0, 1.0) * 255.0) as u8;
      }
    }
  }

  /// Adjust brilliance using S-curve (darkens darks, brightens brights)
  ///
  /// Formula: V_new = (1 / (1 + e^(-k(V_norm - 0.5)))) × 255
  /// Where V_norm = V_old / 255, k = steepness parameter
  ///
  /// # Arguments
  /// * `strength` - Brilliance strength (0.0 to 3.0, typically 0.0 to 2.0)
  ///   - 0.0 = no change
  ///   - 1.0 = moderate S-curve
  ///   - 2.0 = strong S-curve
  ///   - 3.0 = very strong S-curve
  ///
  /// # Recommended Values
  /// * **Subtle enhancement**: 0.5 to 1.0
  ///   - 0.5 = gentle contrast boost (natural)
  ///   - 1.0 = moderate brilliance (photographic)
  /// * **Strong enhancement**: 1.5 to 2.5
  ///   - 1.5 = strong brilliance (punchy)
  ///   - 2.0 = very strong (dramatic)
  ///   - 2.5 = extreme (HDR-like)
  /// * **Note**: Keeps midtones around 128, darkens shadows, brightens highlights
  ///
  /// # Used By
  /// - **Apple Photos** - Brilliance slider
  /// - **Instagram filters** - Many use S-curve variations
  /// - **HDR processing** - Local contrast enhancement
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_brilliance(1.0);  // Moderate brilliance
  /// ```
  pub fn adjust_brilliance(&mut self, strength: f32) {
    let b = strength.clamp(0.0, 1.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;
      let v_norm = v / 255.0;

      // Apple Photos-style brilliance: brighten with enhanced definition
      // Brightens all tones, but enhances highlights more (creates "brilliance")
      let result = v + b * (255.0 - v) * (0.5 + 0.5 * v_norm);

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel (unrolled loop for speed)
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize]; // B channel
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize]; // G channel
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize]; // R channel
    }
  }

  /// Adjust contrast (expand or compress tonal range around midpoint)
  ///
  /// Formula:
  /// - If v < 128: V_new = v × (1 - c × (1 - v/255))  (darken darks)
  /// - If v ≥ 128: V_new = v + c × (255 - v)          (brighten brights)
  ///
  /// # Arguments
  /// * `strength` - Contrast strength (0.0 to 1.0)
  ///   - 0.0 = no change
  ///   - 0.5 = moderate contrast boost
  ///   - 1.0 = maximum contrast (blacks → 0, whites → 255)
  ///
  /// # Recommended Values
  /// * **Subtle**: 0.1 to 0.3
  ///   - 0.1 = very gentle contrast increase
  ///   - 0.2 = subtle contrast boost
  ///   - 0.3 = moderate contrast
  /// * **Strong**: 0.4 to 0.7
  ///   - 0.4 = strong contrast
  ///   - 0.5 = very strong contrast
  ///   - 0.7 = dramatic contrast
  /// * **Extreme**: 0.8 to 1.0
  ///   - 0.8 = extreme contrast
  ///   - 1.0 = maximum (black/white emphasis)
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_contrast(0.3);  // Moderate contrast boost
  /// ```
  pub fn adjust_contrast(&mut self, strength: f32) {
    let c = strength.clamp(0.0, 1.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;

      // Piecewise contrast: darken darks, brighten brights
      let result = if v < 128.0 {
        // Darken dark regions
        v * (1.0 - c * (1.0 - v / 255.0))
      } else {
        // Brighten bright regions
        v + c * (255.0 - v)
      };

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel (unrolled loop for speed)
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize]; // B channel
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize]; // G channel
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize]; // R channel
    }
  }

  /// Adjust highlights (recover or boost bright regions)
  ///
  /// Formula (parametric curve):
  /// - If v < threshold: V_new = v (no change)
  /// - If v ≥ threshold: V_new = T + (v - T) × (1 + H)
  ///
  /// Formula (soft clipping for recovery):
  /// V_new = T + ((v - T) × (1 + H)) / (1 + |(v - T) × H|)
  ///
  /// # Arguments
  /// * `strength` - Highlight adjustment (-1.0 to +1.0)
  ///   - Negative values = recover highlights (compress bright regions)
  ///   - Positive values = boost highlights (expand bright regions)
  ///   - 0.0 = no change
  /// * `threshold` - Where highlights start (typically 180-200 in 0-255 range)
  ///
  /// # Recommended Values
  /// * **Recovery**: -0.3 to -1.0 with threshold=200
  ///   - -0.3 = gentle highlight recovery
  ///   - -0.5 = moderate recovery (typical)
  ///   - -1.0 = maximum recovery (bring back blown highlights)
  /// * **Boost**: +0.2 to +0.6 with threshold=180
  ///   - +0.2 = gentle highlight boost
  ///   - +0.4 = moderate boost
  ///   - +0.6 = strong boost (brighten sky/clouds)
  /// * **Best use**: Recovering overexposed areas or emphasizing bright elements
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_highlights(-0.5, 200.0);  // Recover blown highlights
  /// ```
  pub fn adjust_highlights(&mut self, strength: f32, threshold: f32) {
    let h = strength.clamp(-1.0, 1.0);
    let t = threshold.clamp(0.0, 255.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;

      // Parametric curve: adjust only values above threshold
      let result = if v < t {
        v // No change below threshold
      } else {
        // Adjust highlights: negative = recover (compress), positive = boost (expand)
        t + (v - t) * (1.0 + h)
      };

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize];
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize];
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize];
    }
  }

  /// Adjust shadows (lift or crush dark regions)
  ///
  /// Formula (parametric curve):
  /// - If v < threshold: V_new = T + (v - T) × (1 + S)
  /// - If v ≥ threshold: V_new = v (no change)
  ///
  /// Formula (lift with compression):
  /// V_new = v + S × (T - v) × (v / T)
  ///
  /// # Arguments
  /// * `strength` - Shadow adjustment (-1.0 to +1.0)
  ///   - Positive values = lift shadows (brighten dark regions)
  ///   - Negative values = crush shadows (darken dark regions)
  ///   - 0.0 = no change
  /// * `threshold` - Where shadows end (typically 50-80 in 0-255 range)
  ///
  /// # Recommended Values
  /// * **Lift**: +0.3 to +1.0 with threshold=80
  ///   - +0.3 = gentle shadow lift
  ///   - +0.6 = moderate lift (typical)
  ///   - +1.0 = maximum lift (reveal hidden details)
  /// * **Crush**: -0.2 to -0.6 with threshold=70
  ///   - -0.2 = gentle shadow crush
  ///   - -0.4 = moderate crush
  ///   - -0.6 = strong crush (dramatic blacks)
  /// * **Best use**: Revealing detail in underexposed areas or creating dramatic mood
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_shadows(0.6, 80.0);  // Lift shadows to reveal detail
  /// ```
  pub fn adjust_shadows(&mut self, strength: f32, threshold: f32) {
    let s = strength.clamp(-1.0, 1.0);
    let t = threshold.clamp(0.0, 255.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;

      // Parametric curve: adjust only values below threshold
      let result = if v >= t {
        v // No change above threshold
      } else {
        // Lift with compression: positive = lift (brighten), negative = crush (darken)
        v + s * (t - v) * (v / t)
      };

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize];
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize];
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize];
    }
  }

  /// Adjust brightness (additive brightness shift)
  ///
  /// Formula: V_new = V_old + B
  /// With clamping: V_final = max(0, min(255, V_new))
  ///
  /// # Arguments
  /// * `brightness` - Brightness offset (-255.0 to +255.0, typically -100 to +100)
  ///   - Positive values = brighter
  ///   - Negative values = darker
  ///   - 0.0 = no change
  ///
  /// # Recommended Values
  /// * **Brighten**: +10 to +50
  ///   - +10 = very subtle brighten
  ///   - +25 = moderate brighten
  ///   - +50 = strong brighten
  /// * **Darken**: -10 to -50
  ///   - -10 = very subtle darken
  ///   - -25 = moderate darken
  ///   - -50 = strong darken
  /// * **Extreme**: ±100 (may clip blacks/whites)
  ///
  /// # Note
  /// Brightness is additive (V + B), while Exposure is multiplicative (V × 2^E).
  /// - Brightness shifts all values equally
  /// - Exposure preserves relative differences (more photographic)
  /// - Use Brightness for quick adjustments, Exposure for natural results
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_brightness(30.0);  // Add 30 to all pixel values
  /// ```
  pub fn adjust_brightness(&mut self, brightness: f32) {
    let b = brightness.clamp(-255.0, 255.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;

      // Simple additive brightness
      let result = v + b;

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize];
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize];
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize];
    }
  }

  /// Adjust black point (remap minimum black level)
  ///
  /// Formula: V_new = (V_old - B_in) / (255 - B_in) × 255
  ///
  /// With output black point:
  /// V_new = B_out + ((V_old - B_in) / (255 - B_in)) × (255 - B_out)
  ///
  /// # Arguments
  /// * `input_black` - Input black level (0-255, typically 0-50)
  ///   - Values below this become pure black
  ///   - Values above are stretched to fill range
  /// * `output_black` - Output black level (0-255, typically 0-30)
  ///   - What black becomes in output
  ///   - Usually 0 (pure black)
  ///
  /// # Recommended Values
  /// * **Crush shadows**: input_black=20-40, output_black=0
  ///   - input=20: Slight shadow crush
  ///   - input=30: Moderate shadow crush
  ///   - input=40: Strong shadow crush
  /// * **Lift blacks**: input_black=0, output_black=10-30
  ///   - output=10: Slight fade (film look)
  ///   - output=20: Moderate fade
  ///   - output=30: Strong fade (washed out)
  /// * **Best use**: Remove color cast in shadows or create film-like fade
  ///
  /// # Example
  /// ```
  /// let mut img = Tone::from_mat(&mat);
  /// img.adjust_black_point(30.0, 0.0);  // Values below 30 become black
  /// ```
  pub fn adjust_black_point(&mut self, input_black: f32, output_black: f32) {
    let b_in = input_black.clamp(0.0, 255.0);
    let b_out = output_black.clamp(0.0, 255.0);

    // Pre-compute lookup table for all 256 possible values
    let mut lut = [0u8; 256];
    for i in 0..256 {
      let v = i as f32;

      // Remap [b_in, 255] → [b_out, 255]
      let result = if v <= b_in {
        b_out // Values at or below input black become output black
      } else {
        // Stretch remaining range
        b_out + ((v - b_in) / (255.0 - b_in)) * (255.0 - b_out)
      };

      lut[i] = result.clamp(0.0, 255.0) as u8;
    }

    // Apply lookup table to each pixel
    for i in (0..self.base.data.len()).step_by(3) {
      self.base.data[i] = lut[self.base.data[i] as usize];
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize];
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize];
    }
  }
}
