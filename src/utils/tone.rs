/// Tone mapping and color adjustment operations (like "Animal extends Leg")
/// Wraps BasedImage and provides tone/color adjustment methods

use super::image::BasedImage;

/// Tone struct - extends BasedImage with tone adjustment capabilities
/// Like "Animal extends Leg" in TypeScript
#[derive(Clone)]
pub struct Tone {
  pub base: BasedImage,  // The "parent class" - like super() in TypeScript
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
      self.base.data[i] = lut[self.base.data[i] as usize];         // B channel
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
      self.base.data[i] = lut[self.base.data[i] as usize];         // B channel
      self.base.data[i + 1] = lut[self.base.data[i + 1] as usize]; // G channel
      self.base.data[i + 2] = lut[self.base.data[i + 2] as usize]; // R channel
    }
  }
}
