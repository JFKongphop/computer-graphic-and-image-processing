use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Black point adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust black point: crush values below 50 to pure black (dramatic!)
    tone.adjust_black_point(50.0, 0.0); // Input black=50, output black=0 (strong shadow crush)

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite(
      "outputs/black_point_adjusted.jpg",
      &result,
      &core::Vector::new(),
    )?;

    println!("✅ Black point adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
