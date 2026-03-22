pub mod drawer;
pub mod grain;
pub mod image;
pub mod tone;

// Re-export main types for convenience
pub use drawer::Drawer; // Extends BasedImage (like "Human")
pub use grain::Grain;
pub use image::BasedImage; // Base class (like "Leg")
pub use tone::Tone; // Extends BasedImage (like "Animal") // Extends BasedImage (film grain)
