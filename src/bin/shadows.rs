use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Shadows adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust shadows: +0.9 = strong lift (brighten dark areas dramatically)
    tone.adjust_shadows(0.9, 100.0); // Lift shadows below 100 (higher threshold = more pixels affected)

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite(
      "outputs/shadows_adjusted.jpg",
      &result,
      &core::Vector::new(),
    )?;

    println!("✅ Shadows adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
