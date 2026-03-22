use computer_graphic_and_vision::utils::Drawer;
use opencv::{Error, core, imgcodecs, imgproc, prelude::*, videoio};
use runarium::utils::{converter::get_bounds, performance::measure, read_file::fit_reader};

/// ---------------------------
/// 1. Load & Resize (your code)
/// ---------------------------
pub fn load_and_resize_image(path: &str, max_dim: i32) -> Result<(Mat, i32, i32), Error> {
  let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
  let size = img.size()?;
  let (orig_w, orig_h) = (size.width as f64, size.height as f64);

  let max_side = orig_w.max(orig_h);
  let scale = (max_dim as f64 / max_side).min(1.0);

  let width = (orig_w * scale) as i32;
  let height = (orig_h * scale) as i32;

  let mut resized = Mat::default();
  imgproc::resize(
    &img,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  Ok((resized, width, height))
}

/// Read route from FIT file
fn read_route_from_fit(
  fit_path: &str,
  route_scale: f64,
  offset_x_percent: f64,
  offset_y_percent: f64,
  width: i32,
) -> Result<Vec<(f32, f32)>, Box<dyn std::error::Error>> {
  // Read FIT file
  let (route, _lap) = fit_reader(fit_path)?;
  let points = route.gps_points;

  // Normalize coordinates
  let ((lat_min, lat_max), (lon_min, lon_max)) = get_bounds(&points);

  // Convert GPS coordinates to pixel coordinates
  let pixel_points: Vec<(f32, f32)> = points
    .iter()
    .map(|&(lat, lon)| {
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
      let y = ((offset_y_percent + (1.0 - ny) * route_scale) * width as f64) as f32;
      (x, y)
    })
    .collect();

  Ok(pixel_points)
}

/// ---------------------------
/// 5. MAIN WITH VIDEO CREATION
/// ---------------------------
fn main() -> Result<(), Box<dyn std::error::Error>> {
  measure(
    "Video generation with antialiased route",
    || -> Result<(), Box<dyn std::error::Error>> {
      // Load and resize
      let (bg_mat, w, h) = load_and_resize_image("source/example.jpg", 1080)?;
      // Read route from FIT file instead of generating fake route
      let route = read_route_from_fit(
        "source/example.fit",
        0.2, // route_scale
        0.1, // offset_x_percent
        0.1, // offset_y_percent
        w,
      )?;

      // Convert to fast buffer
      let mut fast = Drawer::from_mat(&bg_mat);

      // Prepare video writer (MP4)
      let fps = (route.len() / 15) as f64;
      let fourcc = videoio::VideoWriter::fourcc('a', 'v', 'c', '1')?; // h264

      let mut writer = videoio::VideoWriter::new(
        "outputs/scratch.mp4",
        fourcc,
        fps,
        core::Size::new(w, h),
        true,
      )?;

      if !writer.is_opened()? {
        panic!("VideoWriter failed to open!");
      }

      println!("📍 Loaded {} GPS points from FIT file", route.len());

      let line_thickness = 7;
      let point_radius = 10;

      // Draw frame-by-frame
      for i in 1..route.len() {
        let (x0, y0) = route[i - 1];
        let (x1, y1) = route[i];

        // Draw line segment on accumulated buffer (Red)
        fast.draw_line_aa(x0, y0, x1, y1, line_thickness, 0, 0, 255);

        // Clone the current accumulated route for this frame
        let mut frame_buffer = Drawer::new(fast.clone_base());

        // Draw the moving point as a circle on the frame copy (Green)
        frame_buffer.draw_circle(x1 as i32, y1 as i32, point_radius, 0, 255, 0);

        // Convert to Mat
        let frame = frame_buffer.to_mat();

        if (i + 1).is_multiple_of(100) {
          println!("Processed {} points", i + 1,);
        }

        // Write frame
        writer.write(&frame)?;
      }

      println!("🎉 Video saved: new.mp4");

      Ok(())
    },
  )?;

  Ok(())
}
