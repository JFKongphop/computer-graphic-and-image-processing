use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Brightness adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust brightness: +60 = strong uniform shift (will blow out highlights!)
    tone.adjust_brightness(60.0); // Strong brightness - notice how it clips!

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite(
      "outputs/brightness_adjusted.jpg",
      &result,
      &core::Vector::new(),
    )?;

    println!("✅ Brightness adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
