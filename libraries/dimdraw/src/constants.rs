use super::na::Vector3;

// taken from:
// https://github.com/TheZoq2/Rust-scad-util
pub fn x_axis() -> Vector3<f32> {
    Vector3::new(1.0, 0.0, 0.0)
}
pub fn y_axis() -> Vector3<f32> {
    Vector3::new(0.0, 1.0, 0.0)
}
pub fn z_axis() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 1.0)
}

pub fn vec3z() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 0.0)
}
