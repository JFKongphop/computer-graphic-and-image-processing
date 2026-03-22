use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Highlights adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust highlights: -0.8 = strong recovery (compress bright areas dramatically)
    tone.adjust_highlights(-0.8, 180.0); // Recover highlights above 180 (lower threshold = more pixels affected)

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite(
      "outputs/highlights_adjusted.jpg",
      &result,
      &core::Vector::new(),
    )?;

    println!("✅ Highlights adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
