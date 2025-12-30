//! WASM-compatible route video generation
//! 
//! This module provides pure Rust drawing functions that can be compiled to WebAssembly.
//! No OpenCV dependencies - works entirely in the browser!
//! 
//! ## Usage from JavaScript:
//! ```javascript
//! import init, { 
//!   generate_route_frame, 
//!   normalize_gps_to_pixels 
//! } from './pkg/runarium.js';
//! 
//! await init();
//! 
//! // Load image from canvas
//! const ctx = canvas.getContext('2d');
//! const imageData = ctx.getImageData(0, 0, width, height);
//! 
//! // Generate frame
//! const frame = generate_route_frame(
//!   imageData.data,
//!   width,
//!   height,
//!   routePoints,
//!   frameIndex,
//!   7,  // line thickness
//!   10  // point radius
//! );
//! 
//! // Put frame back to canvas
//! ctx.putImageData(new ImageData(frame, width, height), 0, 0);
//! ```

use wasm_bindgen::prelude::*;

/// Fast image buffer for efficient pixel manipulation
/// Stores image data in BGR format (3 bytes per pixel)
#[wasm_bindgen]
pub struct FastImage {
  w: usize,
  h: usize,
  data: Vec<u8>, // BGR format
}

#[wasm_bindgen]
impl FastImage {
  /// Create a new FastImage from raw BGR data
  #[wasm_bindgen(constructor)]
  pub fn new(width: usize, height: usize, bgr_data: Vec<u8>) -> Self {
    assert_eq!(
      bgr_data.len(),
      width * height * 3,
      "BGR data size mismatch"
    );
    Self {
      w: width,
      h: height,
      data: bgr_data,
    }
  }

  /// Create FastImage from RGBA data (e.g., from Canvas ImageData)
  #[wasm_bindgen]
  pub fn from_rgba(width: usize, height: usize, rgba_data: Vec<u8>) -> Self {
    let mut bgr_data = Vec::with_capacity(width * height * 3);

    for chunk in rgba_data.chunks(4) {
      bgr_data.push(chunk[2]); // B
      bgr_data.push(chunk[1]); // G
      bgr_data.push(chunk[0]); // R
      // Skip alpha
    }

    Self {
      w: width,
      h: height,
      data: bgr_data,
    }
  }

  /// Convert FastImage to RGBA data (for Canvas ImageData)
  #[wasm_bindgen]
  pub fn to_rgba(&self) -> Vec<u8> {
    let mut rgba_data = Vec::with_capacity(self.w * self.h * 4);

    for chunk in self.data.chunks(3) {
      rgba_data.push(chunk[2]); // R
      rgba_data.push(chunk[1]); // G
      rgba_data.push(chunk[0]); // B
      rgba_data.push(255); // A (fully opaque)
    }

    rgba_data
  }

  /// Get image width
  #[wasm_bindgen]
  pub fn width(&self) -> usize {
    self.w
  }

  /// Get image height
  #[wasm_bindgen]
  pub fn height(&self) -> usize {
    self.h
  }

  /// Draw a single pixel at the specified coordinates
  #[wasm_bindgen]
  pub fn draw_point(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
    self.put_pixel_bgr(x, y, b, g, r, 1.0);
  }

  /// Draw an anti-aliased line with variable thickness
  #[wasm_bindgen]
  pub fn draw_line(
    &mut self,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: i32,
    r: u8,
    g: u8,
    b: u8,
  ) {
    self.draw_line_aa(x0, y0, x1, y1, thickness, b, g, r);
  }

  /// Draw a filled circle at the specified center point
  #[wasm_bindgen]
  pub fn draw_circle(
    &mut self,
    cx: i32,
    cy: i32,
    radius: i32,
    r: u8,
    g: u8,
    b: u8,
  ) {
    self.draw_circle_bgr(cx, cy, radius, b, g, r);
  }
}

// Internal implementation (not exposed to JS)
impl FastImage {
  /// Sets a pixel with alpha blending (anti-aliasing support)
  #[inline]
  fn put_pixel_bgr(&mut self, x: i32, y: i32, b: u8, g: u8, r: u8, a: f32) {
    if x < 0 || y < 0 {
      return;
    }
    let x = x as usize;
    let y = y as usize;
    if x >= self.w || y >= self.h {
      return;
    }

    let idx = (y * self.w + x) * 3;
    let ai = a.clamp(0.0, 1.0);

    let ob = self.data[idx] as f32;
    let og = self.data[idx + 1] as f32;
    let or = self.data[idx + 2] as f32;

    self.data[idx] = (ob + (b as f32 - ob) * ai) as u8;
    self.data[idx + 1] = (og + (g as f32 - og) * ai) as u8;
    self.data[idx + 2] = (or + (r as f32 - or) * ai) as u8;
  }

  /// Draw filled circle (internal, BGR format)
  fn draw_circle_bgr(&mut self, cx: i32, cy: i32, radius: i32, b: u8, g: u8, r: u8) {
    let r2 = radius * radius;
    for dy in -radius..=radius {
      for dx in -radius..=radius {
        if dx * dx + dy * dy <= r2 {
          self.put_pixel_bgr(cx + dx, cy + dy, b, g, r, 1.0);
        }
      }
    }
  }

  /// Draw anti-aliased line with thickness
  fn draw_line_aa(
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
    if thickness <= 1 {
      self.draw_line_aa_single(x0, y0, x1, y1, b, g, r);
      return;
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
      return;
    }

    let px = -dy / len;
    let py = dx / len;

    let half_thickness = thickness as f32 / 2.0;
    for i in 0..thickness {
      let offset = i as f32 - half_thickness + 0.5;
      let ox = offset * px;
      let oy = offset * py;

      self.draw_line_aa_single(x0 + ox, y0 + oy, x1 + ox, y1 + oy, b, g, r);
    }
  }

  /// Xiaolin Wu's line algorithm for anti-aliased lines
  fn draw_line_aa_single(
    &mut self,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    b: u8,
    g: u8,
    r: u8,
  ) {
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);

    if steep {
      std::mem::swap(&mut x0, &mut y0);
      std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
      std::mem::swap(&mut x0, &mut x1);
      std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

    let ip = |x: f32| x.floor();
    let fp = |x: f32| x - x.floor();

    let draw = |img: &mut FastImage, steep: bool, x: f32, y: f32, c: f32| {
      if steep {
        img.put_pixel_bgr(y as i32, x as i32, b, g, r, c);
      } else {
        img.put_pixel_bgr(x as i32, y as i32, b, g, r, c);
      }
    };

    // First endpoint
    let xend = ip(x0 + 0.5);
    let yend = y0 + gradient * (xend - x0);
    let xgap = 1.0 - fp(x0 + 0.5);
    let xpxl1 = xend;
    let ypxl1 = ip(yend);

    draw(self, steep, xpxl1, ypxl1, (1.0 - fp(yend)) * xgap);
    draw(self, steep, xpxl1, ypxl1 + 1.0, fp(yend) * xgap);

    let mut intery = yend + gradient;

    // Second endpoint
    let xend2 = ip(x1 + 0.5);
    let yend2 = y1 + gradient * (xend2 - x1);
    let xgap2 = fp(x1 + 0.5);
    let xpxl2 = xend2;
    let ypxl2 = ip(yend2);

    // Main loop
    for x in ((xpxl1 + 1.0) as i32)..(xpxl2 as i32) {
      draw(self, steep, x as f32, ip(intery), 1.0 - fp(intery));
      draw(self, steep, x as f32, ip(intery) + 1.0, fp(intery));
      intery += gradient;
    }

    draw(self, steep, xpxl2, ypxl2, (1.0 - fp(yend2)) * xgap2);
    draw(self, steep, xpxl2, ypxl2 + 1.0, fp(yend2) * xgap2);
  }
}

/// Generate a single frame with progressive route drawing
/// 
/// # Arguments
/// * `background_rgba` - Background image in RGBA format (from Canvas)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `route_points` - Flat array of route coordinates [x0, y0, x1, y1, ...]
/// * `frame_index` - Current frame (how many route segments to draw)
/// * `line_thickness` - Thickness of the route line
/// * `point_radius` - Radius of the current position marker
/// 
/// # Returns
/// RGBA image data that can be put into Canvas ImageData
#[wasm_bindgen]
pub fn generate_route_frame(
  background_rgba: Vec<u8>,
  width: u32,
  height: u32,
  route_points: Vec<f32>,
  frame_index: usize,
  line_thickness: i32,
  point_radius: i32,
) -> Vec<u8> {
  // Convert RGBA to FastImage (BGR)
  let mut fast = FastImage::from_rgba(width as usize, height as usize, background_rgba);

  // Parse route points from flat array
  let route: Vec<(f32, f32)> = route_points
    .chunks(2)
    .map(|chunk| (chunk[0], chunk[1]))
    .collect();

  if route.is_empty() {
    return fast.to_rgba();
  }

  // Draw all line segments up to current frame (Red)
  for i in 1..=frame_index.min(route.len() - 1) {
    let (x0, y0) = route[i - 1];
    let (x1, y1) = route[i];

    fast.draw_line_aa(x0, y0, x1, y1, line_thickness, 0, 0, 255);
  }

  // Draw current position marker (Green circle)
  if frame_index < route.len() {
    let (x, y) = route[frame_index];
    fast.draw_circle_bgr(x as i32, y as i32, point_radius, 0, 255, 0);
  }

  // Convert back to RGBA for JavaScript
  fast.to_rgba()
}

/// Normalize GPS coordinates to pixel coordinates
/// 
/// # Arguments
/// * `lat_coords` - Array of latitude values
/// * `lon_coords` - Array of longitude values
/// * `width` - Canvas width
/// * `height` - Canvas height
/// * `route_scale` - How much of the canvas the route should fill (0.0-1.0)
/// * `offset_x_percent` - X offset as percentage (0.0-1.0)
/// * `offset_y_percent` - Y offset as percentage (0.0-1.0)
/// 
/// # Returns
/// Flat array of pixel coordinates [x0, y0, x1, y1, ...]
#[wasm_bindgen]
pub fn normalize_gps_to_pixels(
  lat_coords: Vec<f64>,
  lon_coords: Vec<f64>,
  width: u32,
  height: u32,
  route_scale: f64,
  offset_x_percent: f64,
  offset_y_percent: f64,
) -> Vec<f32> {
  if lat_coords.len() != lon_coords.len() || lat_coords.is_empty() {
    return Vec::new();
  }

  // Find coordinate bounds
  let lat_min = lat_coords
    .iter()
    .copied()
    .fold(f64::INFINITY, f64::min);
  let lat_max = lat_coords
    .iter()
    .copied()
    .fold(f64::NEG_INFINITY, f64::max);
  let lon_min = lon_coords
    .iter()
    .copied()
    .fold(f64::INFINITY, f64::min);
  let lon_max = lon_coords
    .iter()
    .copied()
    .fold(f64::NEG_INFINITY, f64::max);

  // Convert to pixel coordinates
  let mut result = Vec::with_capacity(lat_coords.len() * 2);

  for (&lat, &lon) in lat_coords.iter().zip(lon_coords.iter()) {
    let nx = if lon_max != lon_min {
      (lon - lon_min) / (lon_max - lon_min)
    } else {
      0.5
    };
    let ny = if lat_max != lat_min {
      (lat - lat_min) / (lat_max - lat_min)
    } else {
      0.5
    };

    let x = ((offset_x_percent + nx * route_scale) * width as f64) as f32;
    let y = ((offset_y_percent + (1.0 - ny) * route_scale) * height as f64) as f32;

    result.push(x);
    result.push(y);
  }

  result
}

/// Draw multiple line segments on an image
/// Useful for drawing entire routes at once
/// 
/// # Arguments
/// * `background_rgba` - Background image in RGBA format
/// * `width` - Image width
/// * `height` - Image height
/// * `route_points` - Flat array [x0, y0, x1, y1, ...]
/// * `line_thickness` - Line thickness
/// * `r`, `g`, `b` - RGB color values (0-255)
#[wasm_bindgen]
pub fn draw_route_on_image(
  background_rgba: Vec<u8>,
  width: u32,
  height: u32,
  route_points: Vec<f32>,
  line_thickness: i32,
  r: u8,
  g: u8,
  b: u8,
) -> Vec<u8> {
  let mut fast = FastImage::from_rgba(width as usize, height as usize, background_rgba);

  let route: Vec<(f32, f32)> = route_points
    .chunks(2)
    .map(|chunk| (chunk[0], chunk[1]))
    .collect();

  for i in 1..route.len() {
    let (x0, y0) = route[i - 1];
    let (x1, y1) = route[i];
    fast.draw_line_aa(x0, y0, x1, y1, line_thickness, b, g, r);
  }

  fast.to_rgba()
}

/// Add a marker (circle) to an image
/// 
/// # Arguments
/// * `image_rgba` - Image in RGBA format
/// * `width` - Image width
/// * `height` - Image height
/// * `x`, `y` - Marker position
/// * `radius` - Marker radius
/// * `r`, `g`, `b` - RGB color values (0-255)
#[wasm_bindgen]
pub fn add_marker_to_image(
  image_rgba: Vec<u8>,
  width: u32,
  height: u32,
  x: i32,
  y: i32,
  radius: i32,
  r: u8,
  g: u8,
  b: u8,
) -> Vec<u8> {
  let mut fast = FastImage::from_rgba(width as usize, height as usize, image_rgba);
  fast.draw_circle_bgr(x, y, radius, b, g, r);
  fast.to_rgba()
}
