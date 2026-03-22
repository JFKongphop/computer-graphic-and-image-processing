use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Brilliance adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust brilliance
    tone.adjust_brilliance(0.5); // Moderate brilliance (S-curve)

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite("outputs/brilliance.jpg", &result, &core::Vector::new())?;

    println!("✅ Brilliance adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
