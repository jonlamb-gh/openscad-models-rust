// TODO - switch over to dimensioned crate and proper units

//pub fn x_axis() -> Vector3<f32> {Vector3::new(1.0, 0.0, 0.0)}
//pub fn y_axis() -> Vector3<f32> {Vector3::new(0.0, 1.0, 0.0)}
//pub fn z_axis() -> Vector3<f32> {Vector3::new(0.0, 0.0, 1.0)}

pub const fn in_to_cm(v: f32) -> f32 {
    v * 2.54
}

pub const fn ft_to_cm(v: f32) -> f32 {
    v * 30.48
}
