use computer_graphic_and_vision::utils::Tone;
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;

fn main() -> Result<(), Error> {
  measure("Tone adjust", || -> Result<(), Error> {
    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    // Create Tone instance
    let mut tone = Tone::from_mat(&img);

    // Adjust exposure
    tone.adjust_exposure(1.0); // Brighten by 1 stop

    // Save result
    let result = tone.to_mat();
    imgcodecs::imwrite("outputs/exposure.jpg", &result, &core::Vector::new())?;

    println!("✅ Tone adjustment complete!");
    Ok(())
  })?;

  Ok(())
}
