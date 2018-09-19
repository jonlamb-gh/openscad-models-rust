// TODO - switch over to dimensioned crate and proper units

pub const fn in_to_cm(v: f32) -> f32 {
    v * 2.54
}

pub const fn ft_to_cm(v: f32) -> f32 {
    v * 30.48
}
