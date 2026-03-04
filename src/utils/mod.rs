pub mod image;
pub mod drawer;
pub mod tone;

// Re-export main types for convenience
pub use image::BasedImage;  // Base class (like "Leg")
pub use drawer::Drawer;      // Extends BasedImage (like "Human")
pub use tone::Tone;          // Extends BasedImage (like "Animal")
