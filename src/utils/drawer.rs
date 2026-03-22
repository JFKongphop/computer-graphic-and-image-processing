/// Drawing operations for images (like "Human extends Leg")
/// Wraps BasedImage and provides drawing methods
use super::image::BasedImage;

/// Drawer struct - extends BasedImage with drawing capabilities
/// Like "Human extends Leg" in TypeScript
#[derive(Clone)]
pub struct Drawer {
  pub base: BasedImage, // The "parent class" - like super() in TypeScript
}

impl Drawer {
  /// Constructor - creates Drawer from BasedImage (like calling super())
  pub fn new(base: BasedImage) -> Self {
    Self { base }
  }

  /// Create Drawer directly from OpenCV Mat
  pub fn from_mat(mat: &opencv::core::Mat) -> Self {
    Self::new(BasedImage::from_mat(mat))
  }

  /// Convert back to OpenCV Mat
  pub fn to_mat(&self) -> opencv::core::Mat {
    self.base.to_mat()
  }

  /// Clone the base image (useful for creating frames)
  pub fn clone_base(&self) -> BasedImage {
    self.base.clone()
  }

  /// Sets a pixel with alpha blending (anti-aliasing support).
  /// Uses alpha blending to merge new color with existing pixel color.
  /// Performs bounds checking to avoid drawing outside image boundaries.
  ///
  /// # Arguments
  /// * `x`, `y` - Pixel coordinates
  /// * `b`, `g`, `r` - Blue, Green, Red color values (0-255)
  /// * `a` - Alpha/opacity value (0.0-1.0, where 1.0 is fully opaque)
  #[inline]
  pub fn put_pixel_bgr(&mut self, x: i32, y: i32, b: u8, g: u8, r: u8, a: f32) {
    // Bounds check: ensure coordinates are within image dimensions
    if x < 0 || y < 0 {
      return;
    }
    let x = x as usize;
    let y = y as usize;
    if x >= self.base.w || y >= self.base.h {
      return;
    }

    // Calculate pixel index in flat array (BGR format: 3 bytes per pixel)
    let idx = (y * self.base.w + x) * 3;

    // Clamp alpha to valid range [0.0, 1.0]
    let ai = a.clamp(0.0, 1.0);

    // Get existing pixel values (for alpha blending)
    let ob = self.base.data[idx] as f32; // Original blue
    let og = self.base.data[idx + 1] as f32; // Original green
    let or = self.base.data[idx + 2] as f32; // Original red

    // Alpha blend: new_color = old_color + (new_color - old_color) * alpha
    self.base.data[idx] = (ob + (b as f32 - ob) * ai) as u8;
    self.base.data[idx + 1] = (og + (g as f32 - og) * ai) as u8;
    self.base.data[idx + 2] = (or + (r as f32 - or) * ai) as u8;
  }

  /// Draws a single pixel at the specified coordinates.
  ///
  /// # Arguments
  /// * `x`, `y` - Pixel coordinates
  /// * `b`, `g`, `r` - Blue, Green, Red color values (0-255)
  pub fn draw_point(&mut self, x: i32, y: i32, b: u8, g: u8, r: u8) {
    self.put_pixel_bgr(x, y, b, g, r, 1.0); // Alpha = 1.0 for solid color
  }

  /// Draws a filled circle at the specified center point.
  /// Uses a simple distance check to determine which pixels are inside the circle.
  ///
  /// # Arguments
  /// * `cx`, `cy` - Circle center coordinates
  /// * `radius` - Circle radius in pixels
  /// * `b`, `g`, `r` - Blue, Green, Red color values (0-255)
  pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, b: u8, g: u8, r: u8) {
    let r2 = radius * radius; // Square of radius for distance comparison

    // Iterate through bounding box of circle
    for dy in -radius..=radius {
      for dx in -radius..=radius {
        // Check if point is inside circle using distance formula: dx² + dy² ≤ r²
        if dx * dx + dy * dy <= r2 {
          self.put_pixel_bgr(cx + dx, cy + dy, b, g, r, 1.0);
        }
      }
    }
  }

  /// Draws an anti-aliased line with variable thickness using Wu's algorithm.
  /// For thickness > 1, draws multiple parallel lines to create a thick line.
  ///
  /// # Arguments
  /// * `x0`, `y0` - Starting point coordinates
  /// * `x1`, `y1` - Ending point coordinates
  /// * `thickness` - Line thickness in pixels (1 for thin, higher for thick)
  /// * `b`, `g`, `r` - Blue, Green, Red color values (0-255)
  pub fn draw_line_aa(
    &mut self,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: i32,
    b: u8,
    g: u8,
    r: u8,
  ) {
    // For thin lines, use single anti-aliased line
    if thickness <= 1 {
      self.draw_line_aa_single(x0, y0, x1, y1, b, g, r);
      return;
    }

    // Calculate perpendicular direction for parallel lines
    let dx = x1 - x0; // X component of line direction
    let dy = y1 - y0; // Y component of line direction
    let len = (dx * dx + dy * dy).sqrt(); // Line length
    if len == 0.0 {
      return; // Zero-length line, nothing to draw
    }

    // Perpendicular unit vector (rotated 90° from line direction)
    let px = -dy / len;
    let py = dx / len;

    // Draw multiple parallel lines to create thickness
    let half_thickness = thickness as f32 / 2.0;
    for i in 0..thickness {
      // Calculate offset from center line
      let offset = i as f32 - half_thickness + 0.5;
      let ox = offset * px; // X offset in perpendicular direction
      let oy = offset * py; // Y offset in perpendicular direction

      // Draw parallel line offset from original
      self.draw_line_aa_single(x0 + ox, y0 + oy, x1 + ox, y1 + oy, b, g, r);
    }
  }

  /// Draws a single anti-aliased line using Xiaolin Wu's line algorithm.
  /// This produces smooth, anti-aliased lines without jagged edges.
  ///
  /// Wu's algorithm works by:
  /// 1. Drawing pixels at fractional coordinates
  /// 2. Using alpha blending based on distance from actual line
  /// 3. Handling steep/shallow lines differently for optimal quality
  ///
  /// # Arguments
  /// * `x0`, `y0` - Starting point coordinates
  /// * `x1`, `y1` - Ending point coordinates  
  /// * `b`, `g`, `r` - Blue, Green, Red color values (0-255)
  pub fn draw_line_aa_single(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, b: u8, g: u8, r: u8) {
    // Determine if line is steep (more vertical than horizontal)
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);

    // For steep lines, swap x and y coordinates
    if steep {
      std::mem::swap(&mut x0, &mut y0);
      std::mem::swap(&mut x1, &mut y1);
    }
    // Ensure line goes from left to right
    if x0 > x1 {
      std::mem::swap(&mut x0, &mut x1);
      std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0; // Horizontal distance
    let dy = y1 - y0; // Vertical distance
    let gradient = if dx == 0.0 { 1.0 } else { dy / dx }; // Slope of line

    // Helper functions for Wu's algorithm
    let ip = |x: f32| x.floor(); // Integer part
    let fp = |x: f32| x - x.floor(); // Fractional part

    // First endpoint
    let xend = ip(x0 + 0.5);
    let yend = y0 + gradient * (xend - x0);
    let xgap = 1.0 - fp(x0 + 0.5);
    let xpxl1 = xend;
    let ypxl1 = ip(yend);

    // Draw first endpoint with anti-aliasing
    if steep {
      self.put_pixel_bgr(ypxl1 as i32, xpxl1 as i32, b, g, r, (1.0 - fp(yend)) * xgap);
      self.put_pixel_bgr((ypxl1 + 1.0) as i32, xpxl1 as i32, b, g, r, fp(yend) * xgap);
    } else {
      self.put_pixel_bgr(xpxl1 as i32, ypxl1 as i32, b, g, r, (1.0 - fp(yend)) * xgap);
      self.put_pixel_bgr(xpxl1 as i32, (ypxl1 + 1.0) as i32, b, g, r, fp(yend) * xgap);
    }

    let mut intery = yend + gradient; // Y-intersection for main loop

    // Second endpoint
    let xend2 = ip(x1 + 0.5);
    let yend2 = y1 + gradient * (xend2 - x1);
    let xgap2 = fp(x1 + 0.5);
    let xpxl2 = xend2;
    let ypxl2 = ip(yend2);

    // Main loop: draw line between endpoints
    for x in ((xpxl1 + 1.0) as i32)..(xpxl2 as i32) {
      // Draw two pixels per x to create anti-aliasing
      if steep {
        self.put_pixel_bgr(ip(intery) as i32, x, b, g, r, 1.0 - fp(intery));
        self.put_pixel_bgr((ip(intery) + 1.0) as i32, x, b, g, r, fp(intery));
      } else {
        self.put_pixel_bgr(x, ip(intery) as i32, b, g, r, 1.0 - fp(intery));
        self.put_pixel_bgr(x, (ip(intery) + 1.0) as i32, b, g, r, fp(intery));
      }
      intery += gradient; // Move to next y-intersection
    }

    // Draw second endpoint with anti-aliasing
    if steep {
      self.put_pixel_bgr(
        ypxl2 as i32,
        xpxl2 as i32,
        b,
        g,
        r,
        (1.0 - fp(yend2)) * xgap2,
      );
      self.put_pixel_bgr(
        (ypxl2 + 1.0) as i32,
        xpxl2 as i32,
        b,
        g,
        r,
        fp(yend2) * xgap2,
      );
    } else {
      self.put_pixel_bgr(
        xpxl2 as i32,
        ypxl2 as i32,
        b,
        g,
        r,
        (1.0 - fp(yend2)) * xgap2,
      );
      self.put_pixel_bgr(
        xpxl2 as i32,
        (ypxl2 + 1.0) as i32,
        b,
        g,
        r,
        fp(yend2) * xgap2,
      );
    }
  }
}
