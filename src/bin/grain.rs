use computer_graphic_and_vision::utils::{
  Grain,
  grain::{GrainIntensity, GrainSize},
};
use opencv::{Error, core, imgcodecs};
use runarium::utils::performance::measure;
use std::fs;

fn main() -> Result<(), Error> {
  measure("Film Grain Application", || -> Result<(), Error> {
    // Create outputs/grains directory if it doesn't exist
    fs::create_dir_all("outputs/ggg").expect("Failed to create outputs/grains directory");

    // Load image
    let img = imgcodecs::imread("source/example.jpg", imgcodecs::IMREAD_COLOR)?;

    println!("🎞️  Applying Fujifilm-style grain effects...\n");

    // // Test 1: Small + Weak (Provia/Velvia style)
    // println!("📸 Small + Weak (subtle fine grain)");
    // let mut grain_sw = Grain::from_mat(&img);
    // grain_sw.apply_grain(GrainIntensity::Weak, GrainSize::Small)?;
    // let result_sw = grain_sw.to_mat();
    // imgcodecs::imwrite(
    //   "outputs/grains/grain_small_weak.jpg",
    //   &result_sw,
    //   &core::Vector::new(),
    // )?;

    // // Test 2: Small + Strong (pushed film style)
    // println!("📸 Small + Strong (pronounced fine grain)");
    // let mut grain_ss = Grain::from_mat(&img);
    // grain_ss.apply_grain(GrainIntensity::Strong, GrainSize::Small)?;
    // let result_ss = grain_ss.to_mat();
    // imgcodecs::imwrite(
    //   "outputs/grains/grain_small_strong.jpg",
    //   &result_ss,
    //   &core::Vector::new(),
    // )?;

    // // Test 3: Large + Weak (Classic Chrome style)
    // println!("📸 Large + Weak (subtle chunky grain)");
    // let mut grain_lw = Grain::from_mat(&img);
    // grain_lw.apply_grain(GrainIntensity::Weak, GrainSize::Large)?;
    // let result_lw = grain_lw.to_mat();
    // imgcodecs::imwrite(
    //   "outputs/grains/grain_large_weak.jpg",
    //   &result_lw,
    //   &core::Vector::new(),
    // )?;

    // // Test 4: Large + Strong (high ISO film style)
    // println!("📸 Large + Strong (heavy chunky grain)");
    // let mut grain_ls = Grain::from_mat(&img);
    // grain_ls.apply_grain(GrainIntensity::Strong, GrainSize::Large)?;
    // let result_ls = grain_ls.to_mat();
    // imgcodecs::imwrite(
    //   "outputs/grains/grain_large_strong.jpg",
    //   &result_ls,
    //   &core::Vector::new(),
    // )?;

    // Bonus: Custom grain
    for i in 1..6 {
      println!("📸 Custom (medium grain with custom parameters)");
      let mut grain_custom = Grain::from_mat(&img);
      let bt = i as f32 / 100.0;
      grain_custom.apply_grain_custom(bt, 0)?;
      let result_custom = grain_custom.to_mat();
      imgcodecs::imwrite(
        &format!("outputs/ggg/{}.jpg", bt),
        &result_custom,
        &core::Vector::new(),
      )?;
    }

    println!("\n✅ All grain effects applied successfully!");
    println!("📁 Check outputs/grains/ folder for results:");
    println!("   - grain_small_weak.jpg (Provia/Velvia style)");
    println!("   - grain_small_strong.jpg (pushed film)");
    println!("   - grain_large_weak.jpg (Classic Chrome style)");
    println!("   - grain_large_strong.jpg (high ISO film)");
    println!("   - grain_custom.jpg (custom parameters)");

    Ok(())
  })?;

  Ok(())
}
