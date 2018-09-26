// TODO - switch over to dimensioned crate and proper units

use super::na;

pub fn x_axis() -> na::Vector3<f32> {
    na::Vector3::new(1.0, 0.0, 0.0)
}

pub fn y_axis() -> na::Vector3<f32> {
    na::Vector3::new(0.0, 1.0, 0.0)
}

pub fn z_axis() -> na::Vector3<f32> {
    na::Vector3::new(0.0, 0.0, 1.0)
}

pub fn v3zero() -> na::Vector3<f32> {
    na::Vector3::new(0.0, 0.0, 1.0)
}

pub fn in_to_cm(v: f32) -> f32 {
    v * 2.54
}

pub fn ft_to_cm(v: f32) -> f32 {
    v * 30.48
}
