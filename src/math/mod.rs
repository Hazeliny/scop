pub mod vec3;
pub mod mat4;

// Re-export for convenience: use crate::math::Vec3 instead of crate::math::vec3::Vec3
pub use vec3::Vec3;
pub use mat4::Mat4;